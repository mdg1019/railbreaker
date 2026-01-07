<script setup lang="ts">
import { computed } from "vue";
import type { Racecard } from "../models/racecard";
import Panel from "./Panel.vue";
import Transformers from "../utils/transformers";
import RaceClassification from "./RaceClassification.vue";

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
                    <div v-if="currentRace.wager_type_line1 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line1) }}</div>
                    <div v-if="currentRace.wager_type_line2 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line2) }}</div>
                    <div v-if="currentRace.wager_type_line3 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line3) }}</div>
                    <div v-if="currentRace.wager_type_line4 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line4) }}</div>
                    <div v-if="currentRace.wager_type_line5 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line5) }}</div>
                    <div v-if="currentRace.wager_type_line6 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line6) }}</div>
                    <div v-if="currentRace.wager_type_line7 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line7) }}</div>
                    <div v-if="currentRace.wager_type_line8 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line8) }}</div>
                    <div v-if="currentRace.wager_type_line9 !== ''">{{
                        Transformers.capitalize(currentRace.wager_type_line9) }}</div>
                </div>
                <div class="race-conditions">
                    <div v-if="currentRace.race_conditions_line1 !== ''">
                        <span class="distance">{{ Transformers.getDistanceLength(currentRace?.distance ?? 0) + ". " }}</span>
                        <span class="race-classification"><RaceClassification :race="currentRace" /></span>
                        {{ Transformers.buildRaceConditions(currentRace) }}
                    </div>
                    <div v-else>{{ Transformers.commas(currentRace.race_conditions) }}</div>
                </div>
            </div>
            <div class="race-details-right">
                <div class="right-c2-r1">E1</div>
                <div class="right-c3-r1">E2/Late</div>
                <div class="right-c4-r1">Speed</div>
                <div class="right-c1-r2">Pars:</div> 
                <div class="right-c2-r2">{{ currentRace.two_f_bris_pace_par ?? '-' }}</div>  
                <div class="right-c3-r2">{{ (currentRace.six_f_bris_pace_par ?? currentRace.four_f_bris_pace_par ?? '-') + '/' + (currentRace.bris_late_pace_par ?? '-') }}</div>
                <div class="right-c4-r2">{{ currentRace.bris_speed_for_class ?? '-' }}</div>
            </div>
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
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    grid-template-rows: repeat(2, auto);
    gap: 0.5rem;
    align-items: start;

    .right-c2-r1 {
        color: var(--accent-green);
        font-weight: 600;
        grid-column: 2 / span 1;
        grid-row: 1 / span 1;
    }

    .right-c3-r1 {
        color: var(--accent-green);
        font-weight: 600;
        grid-column: 3 / span 1;
        grid-row: 1 / span 1;
    }

    .right-c4-r1 {
        color: var(--accent-green);
        font-weight: 600;
        grid-column: 4 / span 1;
        grid-row: 1 / span 1;
    }

    .right-c1-r2 {
        color: var(--accent-green);
        font-weight: 600;
        grid-column: 1 / span 1;
        grid-row: 2 / span 1;
    }

    .right-c2-r2 {
        color: var(--accent-yellow);
        font-weight: 600;
        grid-column: 2 / span 1;
        grid-row: 2 / span 1;
    }

    .right-c3-r2 {
        color: var(--accent-yellow);
        font-weight: 600;
        grid-column: 3 / span 1;
        grid-row: 2 / span 1;
    }

    .right-c4-r2 {
        color: var(--accent-yellow);
        font-weight: 600;
        grid-column: 4 / span 1;
        grid-row: 2 / span 1;
    }
}

.race-details-center {
    color: var(--accent-yellow);
    flex: 1 1 auto;
    display: flex;
    flex-direction: column;
    text-align: left;
    align-items: flex-start;
    gap: 1rem;
}

.distance {
    color: var(--accent-green);
}

.race-classification {
    color: var(--accent-green);
    position: relative;
    text-decoration: none;

    &::after {
        content: '';
        position: absolute;
        left: 0;
        right: 0;
        height: 1px;
        background-color: var(--accent-green);
        bottom: 0px;
    }
}

@media (max-width: 640px) {
    .race-details {
        flex-direction: column;
    }

    .race-details-left,
    .race-details-center,
    .race-details-right {
        display: grid;
        grid-template-columns: 1fr;
        grid-auto-rows: auto;
        gap: 0.5rem;
    }

    .race-details-left,
    .race-details-center {
        flex: none;
        width: 100%;
    }
}
</style>
