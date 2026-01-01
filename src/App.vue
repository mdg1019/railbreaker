<script setup lang="ts">

import { onMounted, onUnmounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "./stores/globalStateStore";
import { useConfigFileStore } from "./stores/configFileStore";
import "./scss/_main.scss";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenExit: (() => void);

function handleMenuOpen() {
    console.log("Open menu clicked");
}

function handleMenuExit() {
    console.log("Exit menu clicked");
}

onMounted(async () => {
    unlistenOpen = await listen("menu-open", handleMenuOpen);

    unlistenOpenZip = await listen("menu-open-zip", async () => {
        console.log("Open Zip menu clicked");
        const file = await open({
            multiple: false,
            filters: [
                {
                    name: "Zip Files",
                    extensions: ["zip"],
                },
            ],
        });

        if (file && typeof file === "string") {
            console.log("Selected zip file:", file);
            let racecard = await invoke('process_zip_file', { path: file });
            console.log("Racecard data:", racecard);
        }
    });

    unlistenExit = await listen("menu-exit", handleMenuExit);

    await globalStateStore.loadGlobalState();
    await configFileStore.loadConfigFile();

    console.log(globalStateStore.globalState);
    console.log(configFileStore.configState);
});

onUnmounted(() => {
    unlistenOpen();
    unlistenOpenZip();
    unlistenExit();
});
</script>

<template>
    <main>
        <router-view />
    </main>
</template>

<style lang="scss" scoped></style>
