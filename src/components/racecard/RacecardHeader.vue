<script setup lang="ts">
import { computed } from "vue";
import type { Racecard } from "../../models/racecard";
import Transformers from "../../utils/transformers";
import RaceClassification from "./RaceClassification.vue";
import Panel from "../ui/Panel.vue";

const props = withDefaults(defineProps<{
    racecard: Racecard;
    race: number;
    print?: boolean;
}>(), {
    print: false,
});

const raceIndex = computed(() => Math.max(0, props.race - 1));
const currentRace = computed(() => props.racecard.races[raceIndex.value]);
</script>

<template>
    <Panel :print="props.print">
        <div class="race-container-header">
            <div class="track-name color-accent-green font-large">{{ props.racecard.track }}</div>
            <RaceClassification class="raceType color-accent-yellow" :race="currentRace" :prefix-color="'var(--accent-green)'" />
            <div class=" color-accent-yellow">{{ Transformers.getDistanceLength(currentRace?.distance ?? 0) }}</div>
            <div class=" color-accent-yellow">{{ Transformers.getAgeSexRestrictionString(currentRace?.ageSexRestrictions) }}</div>
            <div class="color-accent-yellow">{{ props.racecard.longDate }}</div>
            <div class="race-number color-accent-green font-large">Race {{ props.race }}</div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.race-container-header {
    font-size: var(--headerFontSize);
    display: flex;
    align-items: baseline;
    justify-content: center;
    gap: 3rem;

    .track-name {
        font-weight: 700;
    }

    .race-number {
        font-weight: 700;
    }
}
</style>
