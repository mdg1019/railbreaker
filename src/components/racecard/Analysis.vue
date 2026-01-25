<script setup lang="ts">
import { computed } from "vue";
import { CardAnalysis } from "../../models/analysis";
import Panel from "../ui/Panel.vue";

const props = withDefaults(defineProps<{
    raceNumber: number;
    analysis: CardAnalysis,
    print: boolean;
}>(), {
    print: false,
});

const horsesForRace = computed(() => {
    const horses = props.analysis?.races?.[props.raceNumber - 1]?.horses ?? [];
    return [...horses]
        .filter(horse => horse.score !== undefined && horse.score !== null)
        .sort((a, b) => (b.score ?? Number.POSITIVE_INFINITY) - (a.score ?? Number.POSITIVE_INFINITY))
        .slice(0, 6);
});
</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="color-accent-yellow">Contextual Speed and Pace Model</div>
            <div class="track-date color-accent-yellow">{{ props.analysis.track }} - {{ props.analysis.date }}</div>
            <div class="race-info">
                <div class="color-accent-yellow">Race <span class="color-accent-green">{{ raceNumber }}</span></div>
                <div class="color-accent-yellow">Shape: <span class="color-accent-green">{{ props.analysis.races[raceNumber - 1].shape }}</span></div>
                <div class="color-accent-yellow">EPI: <span class="color-accent-green">{{ props.analysis.races[raceNumber - 1].epi.toFixed(2) }}</span></div>
            </div>
            <div class="horses">
                <div class="horse-row">
                    <div class="color-accent-yellow">#</div>
                    <div class="color-accent-yellow">Horse Name</div>
                    <div class="color-accent-yellow">Score</div>
                </div>
                <div class="horse-row" v-for="(horse, idx) in horsesForRace" :key="idx">
                    <div class="color-accent-yellow">{{ horse.programNumber }}</div>
                    <div>{{ horse.horseName }}</div>
                    <div>{{ horse.score?.toFixed(2) }}</div>
                </div>
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.contents {
    display: flex;
    flex-direction: column;
}

.track-date {
    margin-bottom: 1rem;
}

.race-info {
    display: flex;
    gap: 4rem;
}

.horses {
    margin-top: 1rem;
    display: grid;
    grid-template-columns: 1fr;
    gap: 0.5rem;
}

.horse-row {
    display: grid;
    grid-template-columns: 5rem 20rem 5rem;
    column-gap: 0.5rem;
    align-items: baseline;
}
</style>
