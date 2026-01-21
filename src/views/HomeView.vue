<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "../stores/globalStateStore";
import { useConfigFileStore } from "../stores/configFileStore";
import { Racecard } from "../models/racecard";
import { Racecards } from "../models/racecards";
import RacecardHeader from "../components/racecard/RacecardHeader.vue";
import RaceDetails from "../components/racecard/RaceDetails.vue";
import EqualizerLoader from "../components/ui/EqualizerLoader.vue";
import MessageDialog from "../components/ui/MessageDialog.vue";
import RacecardSideMenu from "../components/racecard/RacecardSideMenu.vue";
import { openPrintWindowAndSendPayload } from "../utils/openPrintWindowEvent";
import { computePrimePowerComparisons } from "../utils/computePrimePowerComparisons";
import Horse from "../components/racecard/Horse.vue";
import "../scss/_main.scss";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenPrintRacecard: (() => void);
let unlistenExit: (() => void);

const isProcessingZip = ref(false);
const isProcessingRacecard = ref(false);

const racecards = new Racecards();
const racecard = ref<Racecard | null>(null);

const raceNumber = ref(1);

const primePowerComparisons = ref<Array<[number | string, string, string]>>([]);

const currentRacecardIndex = ref(0);

const isRacecardMenuOpen = ref(false);

const raceContainerRef = ref<HTMLElement | null>(null);

const showErrorDialog = ref(false);
const errorMessage = ref("");

function handleSelectedRace(value: number) {
    raceNumber.value = value;
    const entry = racecards.racecardEntries[currentRacecardIndex.value];
    if (entry) {
        entry.last_opened_race = value;
    }
    isRacecardMenuOpen.value = false;
}

function handleDeleteRacecard(index: number) {
    racecards.deleteRacecardAt(index);
    if (racecards.racecardEntries.length === 0) {
        currentRacecardIndex.value = 0;
        racecard.value = null;
        raceNumber.value = 1;
        return;
    }

    const newIndex = index - 1 >= 0 ? index - 1 : 0;
    currentRacecardIndex.value = Math.min(newIndex, racecards.racecardEntries.length - 1);
}

watch(racecard, async (rc) => {
    isRacecardMenuOpen.value = false;

    await invoke('set_print_racecard_enabled', {enabled: !!rc }).catch(() => { });
});

watch([racecard, raceNumber], () => {
    primePowerComparisons.value = computePrimePowerComparisons(racecard.value, raceNumber.value);
});

watch(currentRacecardIndex, (idx, oldIdx) => {
    if (typeof oldIdx === 'number' && oldIdx >= 0) {
        const prev = racecards.racecardEntries[oldIdx];
        if (prev) prev.last_opened_race = raceNumber.value;
    }

    const entry = racecards.racecardEntries[idx];
    racecard.value = entry?.racecard ?? null;
    if (entry && entry.last_opened_race && entry.last_opened_race > 0) {
        raceNumber.value = entry.last_opened_race;
    } else {
        raceNumber.value = 1;
    }
});

watch(raceNumber, async (_newVal, _oldVal) => {
    await nextTick();
    raceContainerRef.value?.scrollIntoView({ behavior: "smooth", block: "start" });
});

document.documentElement.classList.add('dark');

onMounted(async () => {
    unlistenOpen = await listen("menu-open", async () => {
        const file = await open({
            multiple: false,
            filters: [
                {
                    name: "Racecard Files",
                    extensions: ["json"],
                },
            ],
            defaultPath: globalStateStore.globalState.racecardsDirectory
        });

        if (file) {
            racecard.value = null;
            await nextTick();

            isProcessingRacecard.value = true;
            try {
                const openedRacecard = Racecard.fromObject(
                    await invoke<any>('load_racecard_file', { path: file })
                );
                racecards.addRacecard(openedRacecard);
                currentRacecardIndex.value = racecards.racecardEntries.length - 1;
                racecard.value = openedRacecard;
                isProcessingRacecard.value = false;
            } catch (error) {
                isProcessingRacecard.value = false;

                errorMessage.value = String(error);
                showErrorDialog.value = true;
            }
        }
    });

    unlistenOpenZip = await listen("menu-open-zip", async () => {
        const file = await open({
            multiple: false,
            filters: [
                {
                    name: "Zip Files",
                    extensions: ["zip"],
                },
            ],
            defaultPath: configFileStore.configState.lastDirectory
        });


        if (file) {
            racecard.value = null;
            await nextTick();

            isProcessingZip.value = true;

            try { 
                let racecard_path = await invoke('process_zip_file', { path: file });
                isProcessingZip.value = false;
                isProcessingRacecard.value = true;               
                const openedRacecard = Racecard.fromObject(
                    await invoke<any>('process_racecard_file', { path: racecard_path })
                );
                racecards.addRacecard(openedRacecard);
                currentRacecardIndex.value = racecards.racecardEntries.length - 1;
                racecard.value = openedRacecard;
                isProcessingRacecard.value = false;
            } catch (error) {
                isProcessingZip.value = false;
                isProcessingRacecard.value = false;

                errorMessage.value = String(error);
                showErrorDialog.value = true;
            }
        }
    });

    unlistenPrintRacecard = await listen("menu-print", async () => {
        if (!racecard.value) {
            return;
        }
    
        const raceCount = racecard.value?.races?.length ?? 0;
        let printRaces = Array.from({ length: raceCount }, (_, idx) => idx + 1);

        const raceCardPrintPayload = {
            raceCard: racecard.value,
            printRaces: printRaces
        };

        await openPrintWindowAndSendPayload(raceCardPrintPayload, {});
    });

    unlistenExit = await listen("menu-exit", async () => {
        await invoke('exit_app');
    });

    await globalStateStore.loadGlobalState();
    await configFileStore.loadConfigFile();
});

onUnmounted(() => {
    unlistenOpen();
    unlistenOpenZip();
    unlistenPrintRacecard();
    unlistenExit();
});
</script>

<template>
    <main class="container">
        <RacecardSideMenu v-if="racecards.racecardEntries.length > 0" :racecards="racecards" v-model:currentRacecardIndex="currentRacecardIndex" v-model:open="isRacecardMenuOpen" :currentRace="raceNumber"
            @update:selectedRace="handleSelectedRace" @delete:racecard="handleDeleteRacecard" />
        <div class="processing" v-if="isProcessingZip">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" />
            <br />
            Processing ZIP File
        </div>
        <div class="processing" v-if="isProcessingRacecard">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" />
            <br />
            Processing Racecard File
        </div>
        <div class="race-container" v-if="racecard" ref="raceContainerRef">
            <RacecardHeader :racecard="racecard" :race="raceNumber" />
            <RaceDetails :racecard="racecard" :race="raceNumber" :print="false" />
            <Horse
                v-for="(horse, idx) in (racecard.races[raceNumber - 1]?.horses || [])"
                :key="horse.programNumber || horse.postPosition || idx"
                :horse="horse"
                :primePowerComparisons="primePowerComparisons"
                :print="false"
            ></Horse>
        </div>
        <MessageDialog v-model="showErrorDialog" :message="errorMessage" title="Error" />
    </main>
</template>

<style lang="scss" scoped>
.container {
    padding: 1rem;
    font-family: "MGSans", sans-serif;
}

.race-container {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
}

.processing {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    font-size: 1.5rem;
    z-index: 1000;
}
</style>
