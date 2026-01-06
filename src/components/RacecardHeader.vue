<script setup lang="ts">
import { computed } from "vue";
import type { Racecard } from "../models/racecard";
import Transformers from "../utils/transformers";
import RaceClassification from "./RaceClassification.vue";
import Panel from "./Panel.vue";

const props = defineProps<{ racecard: Racecard; race: number }>();

const raceIndex = computed(() => Math.max(0, props.race - 1));
const currentRace = computed(() => props.racecard.races[raceIndex.value]);
</script>

<template>
    <Panel>
        <div class="race-container-header">
            <div class="track-name color-accent-green-strong">{{ props.racecard.track }}</div>
            <RaceClassification class="race_type" :race="currentRace" />
            <div class="use-mgsans">{{ Transformers.getDistanceLength(currentRace?.distance ?? 0) }}</div>
            <div>{{ Transformers.getAgeSexRestrictionString(currentRace?.age_sex_restrictions) }}</div>
            <div class="race-date">{{ props.racecard.long_date }}</div>
            <div class="race-number color-accent-green-strong">Race {{ props.race }}</div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.race-container-header {
    display: flex;
    align-items: baseline;
    justify-content: center;
    gap: 3rem;

    .track-name {
        font-size: 2rem;
        font-weight: 700;
    }

    .race-date {
        opacity: 0.95;
    }

    .race-number {
        font-size: 2rem;
        font-weight: 700;
    }
}
</style>
