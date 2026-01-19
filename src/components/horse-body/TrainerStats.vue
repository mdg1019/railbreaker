<script setup lang="ts">
import { Horse, KeyTrainerStat } from "../../models/racecard";

const props = defineProps<{
    horse: Horse;
}>();

</script>

<template>
    <div class="container font-small">
        <div class="color-accent-purple">Trainer:</div>
        <div class="stats">
            <div v-for="(s, i) in horse.keyTrainerStats.filter(s => s.category !== '')" :key="i" class="stat">
                {{ s.category }} ({{ s.starts }} {{ s.winPct?.toFixed(0) }}% {{ s.roi?.toFixed(2) }})
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.container {
    display: flex;
    flex-direction: row;
    gap: 0.25rem;
    margin-top: 1rem;
}

.stats {
    margin-left: 3.5rem;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 0.25rem;
    align-items: center;
}

.stat {
    margin-left: 1rem;
}

.stats > .stat:nth-child(odd) {
    color: var(--accent-yellow);
}

.stats > .stat:nth-child(even) {
    color: var(--accent-green);
}
</style>
