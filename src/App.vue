<script setup lang="ts">

import { onMounted, onUnmounted, ref, nextTick, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "./stores/globalStateStore";
import { useConfigFileStore } from "./stores/configFileStore";
import { Racecard } from "./models/racecard";
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
const racecard = ref<Racecard | null>(null);

const race = ref(1);

const isRacecardMenuOpen = ref(false);

const showErrorDialog = ref(false);
const errorMessage = ref("");

function handleSelectedRace(value: number) {
    race.value = value;
    isRacecardMenuOpen.value = false;
}

watch(racecard, (rc) => {
    if (rc) {
        isRacecardMenuOpen.value = false;
        race.value = 1;
    } else {
        isRacecardMenuOpen.value = false;
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
                racecard.value = await invoke<Racecard>('load_racecard_file', { path: file });
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
                racecard.value = await invoke<Racecard>('process_racecard_file', { path: racecard_path });
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
        <RacecardSideMenu v-if="racecard" :racecard="racecard!" v-model:open="isRacecardMenuOpen" :selectedRace="race"
            @update:selectedRace="handleSelectedRace" />
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
                    <div class="track-name">{{ racecard.track }}</div>
                    <RaceClassification class="race_type" :race="racecard.races[race - 1]" />
                    <div class="race-date">{{ racecard.long_date }}</div>
                    <div class="race-number">Race {{ race }}</div>
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
