<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Transformers from '../utils/transformers';

const props = defineProps<{
    horse: Horse;
}>();

</script>

<template>
    <div class="container">
        Past Performances Component
    </div>
</template>

<style lang="scss" scoped>
.container {
    margin-top: 1rem;
}
</style>
