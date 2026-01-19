<script setup lang="ts">
import type { Horse } from "../../models/racecard";
import Panel from "../ui/Panel.vue";
import HorseHeaderLeft from "../horse-body/horse-header/HorseHeaderLeft.vue";
import HorseHeaderRight from "../horse-body/horse-header/HorseHeaderRight.vue";
import HorseHeaderCenter from "../horse-body/horse-header/HorseHeaderCenter.vue";
import PastPerfomances from "../horse-body/PastPerfomances.vue";
import Workouts from "../horse-body/Workouts.vue";
import TrainerStats from "../horse-body/TrainerStats.vue";
import TrainerJockey from "../horse-body/TrainerJockey.vue";

const props = withDefaults(defineProps<{ horse: Horse, primePowerComparisons: Array<[number | string, string, string]>; print: boolean; }>(), {
    print: false,
});

</script>

<template>
    <Panel :print="props.print">
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
                
                <Workouts :workouts="props.horse.workouts" />

                <TrainerStats :horse="props.horse" />

                <TrainerJockey :horse="props.horse" />
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
    margin-bottom: 1rem;
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
