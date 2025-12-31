
<script setup lang="ts">
import { onMounted } from "vue";
import { useTracksStore } from "./stores/tracks";
import "./scss/_main.scss";

const tracksStore = useTracksStore();

onMounted(() => {
  tracksStore.loadTracks();
});
</script>

<template>
  <main>
    <router-view />
  </main>
</template>

<style lang="scss" scoped>
</style>

