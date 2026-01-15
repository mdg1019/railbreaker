<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Panel from "./Panel.vue";
import HorseHeaderLeft from "./HorseHeaderLeft.vue";
import HorseHeaderRight from "./HorseHeaderRight.vue";
import HorseHeaderCenter from "./HorseHeaderCenter.vue";
import PastPerfomances from "./PastPerfomances.vue";
import Workout from "./Workout.vue";

const props = defineProps<{ horse: Horse, primePowerComparisons: Array<[number | string, string, string]> }>();
</script>

<template>
    <Panel>
        <div class="horse">
            <div class="horse-header">

                <HorseHeaderLeft :horse="props.horse" />  

                <HorseHeaderCenter
                    :horse="props.horse"
                    :primePowerComparisons="props.primePowerComparisons" />

                <HorseHeaderRight :horse="props.horse" />
            </div>

            <div class="horse-body">
                <div class="horizontal-line"></div>
                <PastPerfomances :horse="props.horse" />
                
                <div v-if="props.horse.workouts.length > 0" class="workouts">
                    <div v-for="(workout, i) in props.horse.workouts" :key="i" class="workout-item">
                        <Workout :workout="workout" />
                    </div>
                </div>
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.horse {
    font-family: "MGSansCondensed", sans-serif;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.horse-header {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: center;
}

.horizontal-line {
    height: calc(2px + 0.5rem);
    margin-top: 0.5rem;
    border-bottom: 2px solid var(--ubuntu-blue);  
    width: calc(100% + 2 * 1rem);
    margin: 0 calc(-1 * 1rem);
}

.workouts {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 0.25rem;
    align-items: flex-start;
}

.workout-item {
    display: flex;
    align-items: center;
}
</style>
