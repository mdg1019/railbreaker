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
    headerFontSize?: string;
    trackFontSize?: string;
    raceNumberFontSize?: string;
}>(), {
    print: false,
});


const headerFontSize = computed(() => props.headerFontSize ?? (props.print ? "1rem" : "1.6rem"));
const trackFontSize = computed(() => props.trackFontSize ?? (props.print ? "1.2rem" : "2rem"));
const raceNumberFontSize = computed(() => props.raceNumberFontSize ?? (props.print ? "1.2rem" : "2rem"));
const raceIndex = computed(() => Math.max(0, props.race - 1));
const currentRace = computed(() => props.racecard.races[raceIndex.value]);
</script>

<template>
    <Panel v-if="!props.print">
        <div class="race-container-header" :style="{ '--headerFontSize': headerFontSize }">
            <div class="track-name color-accent-green" :style="{ '--trackFontSize': trackFontSize }">{{ props.racecard.track }}</div>
            <RaceClassification class="raceType color-accent-yellow" :race="currentRace" :prefix-color="'var(--accent-green)'" />
            <div class=" color-accent-yellow">{{ Transformers.getDistanceLength(currentRace?.distance ?? 0) }}</div>
            <div class=" color-accent-yellow">{{ Transformers.getAgeSexRestrictionString(currentRace?.ageSexRestrictions) }}</div>
            <div class="race-date color-accent-yellow">{{ props.racecard.longDate }}</div>
            <div class="race-number color-accent-green" :style="{ '--raceNumberFontSize': raceNumberFontSize }">Race {{ props.race }}</div>
        </div>
    </Panel>

    <div v-else class="race-container-header" :style="{ '--headerFontSize': headerFontSize }">
        <div class="track-name color-accent-green" :style="{ '--trackFontSize': trackFontSize }">{{ props.racecard.track }}</div>
        <RaceClassification class="raceType color-accent-yellow" :race="currentRace" :prefix-color="'var(--accent-green)'" />
        <div class=" color-accent-yellow">{{ Transformers.getDistanceLength(currentRace?.distance ?? 0) }}</div>
        <div class=" color-accent-yellow">{{ Transformers.getAgeSexRestrictionString(currentRace?.ageSexRestrictions) }}</div>
        <div class="race-date color-accent-yellow">{{ props.racecard.longDate }}</div>
        <div class="race-number color-accent-green" :style="{ '--raceNumberFontSize': raceNumberFontSize }">Race {{ props.race }}</div>
    </div>
</template>

<style scoped lang="scss">
.race-container-header {
    font-size: var(--headerFontSize);
    display: flex;
    align-items: baseline;
    justify-content: center;
    gap: 3rem;

    .track-name {
        font-size: var(--trackFontSize);
        font-weight: 700;
    }

    .race-number {
        font-size: var(--raceNumberFontSize);
        font-weight: 700;
    }
}
</style>
