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
                 <div class="color-accent-yellow" :style="{
                    borderBottom: (i === 0 && props.horse.past_performances?.[i]?.days_since_last_race && props.horse.past_performances[i].days_since_last_race > 45)
                        ? '1px solid var(--accent-red)'
                        : undefined
                }">                    
                    {{ Transformers.formatDateShort(props.horse.past_performances[i].race_date) }}{{ props.horse.past_performances[i].track_code }}<span class="use-superscript">{{ props.horse.past_performances[i].race_number }}</span>
                </div>
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
    grid-template-columns: minmax(7rem, min-content) auto auto auto auto auto auto auto;
    grid-template-rows: repeat(11, auto);
}

.title-row,
.perf-row {
    display: contents;
}
</style>