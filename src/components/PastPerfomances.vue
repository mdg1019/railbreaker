<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Transformers from '../utils/transformers';

const props = defineProps<{
    horse: Horse;
}>();
</script>

<template>
    <div class="container">
        <div
            v-if="props.horse.past_performances && props.horse.past_performances[0] && props.horse.past_performances[0].race_date !== ''"
            class="title-row color-accent-green"
        >
            <div>DATE TRK</div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
        </div>

        <template v-for="(_, i) in Array(10)" :key="i">
            <div
                class="perf-row"
                v-if="props.horse.past_performances && props.horse.past_performances[i] && props.horse.past_performances[i].race_date !== ''"
            >
                <div>{{ props.horse.past_performances[i].race_date }}</div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
            </div>
        </template>
    </div>
</template>

<style lang="scss" scoped>
.container {
    font-size: 1.4rem;
    margin-top: 1rem;
    display: grid;
    grid-template-columns: auto auto auto auto auto auto auto auto;
    grid-template-rows: repeat(11, auto);
}

/* make the row wrappers disappear so their children participate in the parent's grid */
.title-row,
.perf-row {
    display: contents;
}
</style>