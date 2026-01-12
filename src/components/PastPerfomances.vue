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
            <div></div>
            <div></div>
        </div>

        <template v-for="(_, i) in Array(10)" :key="i">          
            <div
                class="claimed-row"
                v-if="props.horse.past_performances[i].alternate_comment_line.startsWith('Claimed')"
            >
                 <div class="color-accent-purple">                    
                    {{ props.horse.past_performances[i].alternate_comment_line }}( as of  {{  props.horse.past_performances[i].claimed_and_trainer_switches_1 }} ): ( {{ props.horse.past_performances[i].claimed_and_trainer_switches_2 }}  {{ props.horse.past_performances[i].claimed_and_trainer_switches_3 }}-{{ props.horse.past_performances[i].claimed_and_trainer_switches_4 }}-{{ props.horse.past_performances[i].claimed_and_trainer_switches_5 }}  
                    {{ Transformers.createPercentageString(Transformers.parseNumberOrNull(props.horse.past_performances[i].claimed_and_trainer_switches_3), Transformers.parseNumberOrNull(props.horse.past_performances[i].claimed_and_trainer_switches_2))  }} )
                </div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
                <div></div>
            </div>
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
                <div>{{ Transformers.getSurfaceString(props.horse.past_performances[i]) }}</div>
                <div></div>
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
    grid-template-columns: minmax(7rem, min-content) auto auto auto auto auto auto auto auto auto;
    grid-template-rows: repeat(11, auto);
}

.title-row,
.perf-row {
    display: contents;
}

.claimed-row {
    grid-column: 2 / -1;
}
</style>