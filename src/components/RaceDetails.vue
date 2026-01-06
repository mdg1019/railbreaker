<script setup lang="ts">
import { computed } from "vue";
import type { Racecard } from "../models/racecard";
import Panel from "./Panel.vue";

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
            <div class="race-details-center">2</div>
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
    font-size: 8rem;
    text-align: left;
    flex: 0 0 5%;
}
.race-details-right {
    flex: 0 0 25%;
}

.race-details-center {
    flex: 1 1 auto;
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
