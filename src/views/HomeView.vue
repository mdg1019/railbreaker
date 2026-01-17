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

function ordinal(n: number): string {
    const s = ["th", "st", "nd", "rd"], v = n % 100;
    return n + (s[(v - 20) % 10] || s[v] || s[0]);
}

function computePrimePowerComparisons() {
    const result: Array<[number | string, string, string]> = [];
    if (!racecard.value) {
        primePowerComparisons.value = result;
        return
    }
    const raceIdx = raceNumber.value - 1;
    const race = racecard.value.races?.[raceIdx];
    if (!race || !Array.isArray(race.horses)) {
        primePowerComparisons.value = result;
        return
    }

    const entries: { post: number | string; rating: number }[] = [];
    race.horses.forEach((h: any, idx: number) => {
        const r = h.brisPrimePowerRating;
        if (r === null || r === undefined) return;
        if (Number(r) === 0) return;
        const post = h.postPosition ?? h.programNumber ?? idx + 1;
        entries.push({ post, rating: Number(r) });
    });

    if (entries.length === 0) {
        primePowerComparisons.value = result;
    }

    entries.sort((a, b) => b.rating - a.rating);

    const N = entries.length;
    const tier = Math.ceil(N / 3);

    entries.forEach((e, i) => {
        const position = i + 1; 
        let color = "var(--accent-yellow)";
        if (position <= tier) {
            color = "var(--accent-green)";
        } else if (position > 2 * tier) {
            color = "var(--accent-red)";
        }

        const tuple: [number | string, string, string] = [e.post, ordinal(position), color];
        result.push(tuple);
    });

    primePowerComparisons.value = result;
}

watch(racecard, (_rc) => {
    isRacecardMenuOpen.value = false;
});

watch([racecard, raceNumber], () => {
    computePrimePowerComparisons();
    console.log("Prime Power Comparisons:", primePowerComparisons.value);
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
        console.log("Print Racecard menu item clicked");
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
            <RaceDetails :racecard="racecard" :race="raceNumber" />
            <Horse
                v-for="(horse, idx) in (racecard.races[raceNumber - 1]?.horses || [])"
                :key="horse.programNumber || horse.postPosition || idx"
                :horse="horse"
                :primePowerComparisons="primePowerComparisons"
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
