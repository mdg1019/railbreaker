<script setup lang="ts">
import { computed } from "vue";
import type { Racecard } from "../models/racecard";
import Panel from "./Panel.vue";
import Transformers from "../utils/transformers";

const props = defineProps<{ racecard: Racecard; race: number }>();

const raceIndex = computed(() => Math.max(0, props.race - 1));
const currentRace = computed(() => props.racecard.races[raceIndex.value]);
</script>

<template>
    <Panel>
        <div class="race-details">
            <div class="race-details-left">
                <div class="race-number color-accent-green-strong">{{ props.race }}</div>
            </div>
            <div class="race-details-center">
                <div class="wager-list">
                    <div v-if="currentRace.wager_type_line1 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line1) }}</div>
                    <div v-if="currentRace.wager_type_line2 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line2) }}</div>
                    <div v-if="currentRace.wager_type_line3 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line3) }}</div>
                    <div v-if="currentRace.wager_type_line4 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line4) }}</div>
                    <div v-if="currentRace.wager_type_line5 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line5) }}</div>
                    <div v-if="currentRace.wager_type_line6 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line6) }}</div>
                    <div v-if="currentRace.wager_type_line7 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line7) }}</div>
                    <div v-if="currentRace.wager_type_line8 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line8) }}</div>
                    <div v-if="currentRace.wager_type_line9 !== ''">{{ Transformers.capitalize(currentRace.wager_type_line9) }}</div>
                </div>
            </div>
            <div class="race-details-right">3</div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.race-details {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: flex-start;
}

.race-details-left,
.race-details-center,
.race-details-right {
    padding: 0.5rem;
}

.race-details-left {
    font-size: 5rem;
    text-align: left;
    flex: 0 0 5%;
    display: flex;
    align-items: center;
    text-align: left;
}
.race-details-right {
    flex: 0 0 25%;
}

.race-details-center {
    flex: 1 1 auto;
    text-align: left;
    align-items: flex-start;
}

@media (max-width: 640px) {
    .race-details {
        flex-direction: column;
    }
    .race-details-left,
    .race-details-center,
    .race-details-right {
        flex: none;
        width: 100%;
    }
}
</style>
