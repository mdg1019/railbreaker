<script setup lang="ts">

import { onMounted, onUnmounted, ref, nextTick } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "./stores/globalStateStore";
import { useConfigFileStore } from "./stores/configFileStore";
import { Racecard } from "./models/racecard";
import EqualizerLoader from "./components/EqualizerLoader.vue";
import "./scss/_main.scss";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenExit: (() => void);

const isProcessingZip = ref(false); 
const isProcessingRacecard = ref(false); 
const racecard = ref<Racecard | null>(null);

function handleMenuOpen() {
    console.log("Open menu clicked");
}

function handleMenuExit() {
    console.log("Exit menu clicked");
}

document.documentElement.classList.add('dark');

onMounted(async () => {
    unlistenOpen = await listen("menu-open", handleMenuOpen);

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

        racecard.value = null;
        await nextTick();
        

        if (file) {
            isProcessingZip.value = true;

            try {
                let racecard_path = await invoke('process_zip_file', { path: file });
                isProcessingZip.value = false;
                isProcessingRacecard.value = true;
                racecard.value = await invoke<Racecard>('process_racecard_file', { path: racecard_path });                
                isProcessingRacecard.value = false;

        await nextTick();
            } catch (error) {
                isProcessingZip.value = false;
                isProcessingRacecard.value = false;

                // TODO: Create a message dialog component instead of using alert()
                alert(error);
            }
        }
    });

    unlistenExit = await listen("menu-exit", handleMenuExit);

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
        <div class="processing-zip" v-if="isProcessingZip">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" />
            <br />
            Processing ZIP File
        </div>
        <div class="processing-racecard" v-if="isProcessingRacecard">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" />
            <br />
            Processing Racecard File
        </div>
        <div v-if="racecard">
            <h2>Racecard Data:</h2>
            <pre>{{ racecard }}</pre>
        </div>
    </main>
</template>

<style lang="scss" scoped>
    .processing-zip {
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
    .processing-racecard {
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
