<script setup lang="ts">
import { computed } from 'vue';
import type { Horse } from "../models/racecard";
import Transformers from '../utils/transformers';

const props = defineProps<{
    horse: Horse;
}>();

const fraction_1 = computed(() => {
  const pps = props.horse?.past_performances || [];
  return pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction_1 ?? null));
});

const fraction_2 = computed(() => {
  const pps = props.horse?.past_performances || [];
  return pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction_2 ?? null));
});

const fraction_3 = computed(() => {
  const pps = props.horse?.past_performances || [];
  return pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction_3 ?? null));
});

const final_time = computed(() => {
  const pps = props.horse?.past_performances || [];
  return pps.map(pp => Transformers.getFractionalTimeString(pp?.final_time ?? null));
});

const race_classification = computed(() => {
  const pps = props.horse?.past_performances || [];
  return pps.map(pp => Transformers.getPPRaceClassification(pp));
});
</script>

<template>
    <div class="container">
        <div
            v-if="props.horse.past_performances && props.horse.past_performances[0] && props.horse.past_performances[0].race_date !== ''"
            class="title-row color-accent-green"
        >
            <div>DATE TRK</div>
            <div></div>
            <div>DIST</div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div>RACETYPE</div>
            <div></div>
            <div></div>
        </div>

        <template v-for="(_, i) in Array(10)" :key="i">          
            <div
                    class="claimed-row"
                    v-if="props.horse.past_performances?.[i]?.alternate_comment_line?.startsWith('Claimed')"
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
                 <div :style="{
                    borderBottom: (i === 0 && props.horse.past_performances?.[i]?.days_since_last_race && props.horse.past_performances[i].days_since_last_race > 45)
                        ? '1px solid var(--accent-red)'
                        : undefined
                }">                    
                    {{ Transformers.formatDateShort(props.horse.past_performances[i].race_date) }}{{ props.horse.past_performances[i].track_code }}<span class="use-superscript">{{ props.horse.past_performances[i].race_number }}</span>
                </div>
                <div class="color-accent-yellow surface-indicator">{{ Transformers.getSurfaceString(props.horse.past_performances[i]) }}</div>
                <div>{{ Transformers.getShortLength(props.horse.past_performances[i].distance) }} <span>{{ props.horse.past_performances[i].track_condition.toLowerCase() }}</span><span v-if="props.horse.past_performances[i].sealed_track_indicator" class="use-superscript">s</span></div>
                <div class="left-padding">{{ fraction_1[i]?.[0] }}<span class="use-superscript">{{ fraction_1[i]?.[1] }}</span></div>
                <div>{{ fraction_2[i]?.[0] }}<span class="use-superscript">{{ fraction_2[i]?.[1] }}</span></div>
                <div>{{ fraction_3[i]?.[0] }}<span class="use-superscript">{{ fraction_3[i]?.[1] }}</span></div>
                <div>{{ final_time[i]?.[0] }}<span class="use-superscript">{{ final_time[i]?.[1] }}</span></div>
                <div><span class="color-accent-yellow">{{ race_classification[i]?.[0] }}</span>{{ race_classification[i]?.[1] }}</div>
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
    grid-template-columns: minmax(8rem, max-content) minmax(2rem, max-content) minmax(3rem, max-content) minmax(4rem, max-content) minmax(3rem, max-content) minmax(3rem, max-content) minmax(3rem, max-content) auto auto auto;
    grid-template-rows: repeat(11, auto);
}

.title-row,
.perf-row {
    display: contents;
}

.claimed-row {
    grid-column: 2 / -1;
}


.claimed-row .color-accent-purple {
    max-height: 1.6rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.95em;
}

.surface-indicator {
    padding-right: 0.25rem;
    text-align: right;
}

.use-superscript {
    vertical-align: super;
    font-size: 0.7em;
    line-height: 0;
}

.left-padding {
    padding-left: 1rem;
}
</style>