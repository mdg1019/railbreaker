<script setup lang="ts">

import { onMounted, onUnmounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "./stores/globalStateStore";
import { useConfigFileStore } from "./stores/configFileStore";
import "./scss/_main.scss";
import EqualizerLoader from "./components/EqualizerLoader.vue";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenExit: (() => void);

const isProcessingZip = ref(false); 
const isProcessingRacecard = ref(false); 

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

        if (file) {
            isProcessingZip.value = true;

            try {
                let racecard = await invoke('process_zip_file', { path: file });
                isProcessingZip.value = false;
                isProcessingRacecard.value = true;
                let path = await invoke('process_racecard_file', { path: racecard });
                console.log("Racecard processed at path:", path);
                
                //isProcessingRacecard.value = false;
            } catch (error) {
                isProcessingZip.value = false;
                console.error("Error processing zip file:", error);

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
        <!-- <router-view /> -->
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
