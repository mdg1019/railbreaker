<script setup lang="ts">

import { onMounted, onBeforeUnmount } from "vue";
import { useTracksStore } from "./stores/tracks";
import "./scss/_main.scss";
import { listen } from "@tauri-apps/api/event";

const tracksStore = useTracksStore();

let unlistenNew: (() => void) | null = null;
let unlistenOpen: (() => void) | null = null;
let unlistenExit: (() => void) | null = null;

function handleMenuNew() {
  tracksStore.tracks.clear();
}

function handleMenuOpen() {
  console.log("Open menu clicked");
}

function handleMenuExit() {
  console.log("Exit menu clicked");
}

onMounted(async () => {
  tracksStore.loadTracks();
  unlistenNew = await listen("menu-new", handleMenuNew);
  unlistenOpen = await listen("menu-open", handleMenuOpen);
  unlistenExit = await listen("menu-exit", handleMenuExit);
});

onBeforeUnmount(() => {
  if (unlistenNew) unlistenNew();
  if (unlistenOpen) unlistenOpen();
  if (unlistenExit) unlistenExit();
});
</script>

<template>
  <main>
    <router-view />
  </main>
</template>

<style lang="scss" scoped>
</style>

