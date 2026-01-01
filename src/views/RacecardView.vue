
<script setup lang="ts">
import { useGlobalStateStore } from "../stores/globalStateStore";

const globalStateStore = useGlobalStateStore();
</script>

<template>
    <div class="container">

    </div>
</template>

<style lang="scss" scoped>
.container {
  margin: 0 auto;
  padding: 2em;
  max-width: 600px;
}
</style>