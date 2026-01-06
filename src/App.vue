<script setup lang="ts">

import { onMounted, onUnmounted, ref, nextTick, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "./stores/globalStateStore";
import { useConfigFileStore } from "./stores/configFileStore";
import { Racecard } from "./models/racecard";
import { Racecards } from "./models/racecards";
import EqualizerLoader from "./components/EqualizerLoader.vue";
import MessageDialog from "./components/MessageDialog.vue";
import RacecardSideMenu from "./components/RacecardSideMenu.vue";
import RaceClassification from "./components/RaceClassification.vue";
import Panel from "./components/Panel.vue";
import "./scss/_main.scss";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenExit: (() => void);

const isProcessingZip = ref(false);
const isProcessingRacecard = ref(false);

const racecards = new Racecards();
const racecard = ref<Racecard | null>(null);

const race = ref(1);

const currentRacecardIndex = ref(0);

const isRacecardMenuOpen = ref(false);

const showErrorDialog = ref(false);
const errorMessage = ref("");

function handleSelectedRace(value: number) {
    race.value = value;
    const entry = racecards.racecardEntries[currentRacecardIndex.value];
    if (entry) {
        entry.last_opened_race = value;
    }
    isRacecardMenuOpen.value = false;
}

function handleDeleteRacecard(index: number) {
    // delete the entry
    racecards.deleteRacecardAt(index);
    // if no racecards, clear selection
    if (racecards.racecardEntries.length === 0) {
        currentRacecardIndex.value = 0;
        racecard.value = null;
        race.value = 1;
        return;
    }

    // prefer previous entry if exists, otherwise clamp to 0
    const newIndex = index - 1 >= 0 ? index - 1 : 0;
    currentRacecardIndex.value = Math.min(newIndex, racecards.racecardEntries.length - 1);
}

watch(racecard, (rc) => {
    isRacecardMenuOpen.value = false;
});

watch(currentRacecardIndex, (idx, oldIdx) => {
    if (typeof oldIdx === 'number' && oldIdx >= 0) {
        const prev = racecards.racecardEntries[oldIdx];
        if (prev) prev.last_opened_race = race.value;
    }

    const entry = racecards.racecardEntries[idx];
    racecard.value = entry?.racecard ?? null;
    if (entry && entry.last_opened_race && entry.last_opened_race > 0) {
        race.value = entry.last_opened_race;
    } else {
        race.value = 1;
    }
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
            defaultPath: globalStateStore.globalState.racecards_directory
        });

        if (file) {
            racecard.value = null;
            await nextTick();

            isProcessingRacecard.value = true;
            try {
                let openedRacecard = await invoke<Racecard>('load_racecard_file', { path: file });
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
                let openedRacecard = await invoke<Racecard>('process_racecard_file', { path: racecard_path });
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

    unlistenExit = await listen("menu-exit", async () => {
        await invoke('exit_app');
    });

    await globalStateStore.loadGlobalState();
    await configFileStore.loadConfigFile();
});

onUnmounted(() => {
    unlistenOpen();
    unlistenOpenZip();
    unlistenExit();
});
</script>

<template>
    <main class="container">
        <RacecardSideMenu v-if="racecards.racecardEntries.length > 0" :racecards="racecards" v-model:currentRacecardIndex="currentRacecardIndex" v-model:open="isRacecardMenuOpen" :currentRace="race"
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
        <div class="race-container" v-if="racecard">
            <Panel>
                <div class="race-container-header">
                    <div class="track-name color-accent-green-strong">{{ racecard.track }}</div>
                    <RaceClassification class="race_type" :race="racecard.races[race - 1]" />
                    <div class="race-date">{{ racecard.long_date }}</div>
                    <div class="race-number color-accent-green-strong">Race {{ race }}</div>
                </div>
            </Panel>
        </div>
        <MessageDialog v-model="showErrorDialog" :message="errorMessage" title="Error" />
    </main>
</template>

<style lang="scss" scoped>
.container {
    padding: 1rem;
}

.race-container {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;

    .race-container-header {
        display: flex;
        align-items: baseline;
        justify-content: center;
        gap: 3rem;

        .track-name {
            font-size: 2rem;
            font-weight: 700;
        }

        .race-date {
            opacity: 0.95;
        }

        .race-number {
            font-size: 2rem;
            font-weight: 700;
        }
    }
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
