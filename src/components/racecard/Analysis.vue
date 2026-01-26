<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { CardAnalysis, RaceRankResult } from "../../models/analysis";
import { deriveRaceMeta, topNHorsesByScore } from "../../utils/cspm";
import Panel from "../ui/Panel.vue";

const props = withDefaults(defineProps<{
    raceNumber: number;
    analysis: CardAnalysis,
    print: boolean;
}>(), {
    print: false,
});

const scratchedByRace = ref(new Map<number, Set<string>>());

const currentScratched = computed(() => {
    return scratchedByRace.value.get(props.raceNumber) ?? new Set<string>();
});

const metadata = computed(() => {
    const race = props.analysis.races[props.raceNumber - 1];
    const scratched = currentScratched.value;
    return deriveRaceMeta(new RaceRankResult({
        ...race,
        horses: race.horses.filter(horse => !scratched.has(horse.programNumber)),
    }));
});

const toggleScratch = (programNumber: string, checked: boolean) => {
    const raceKey = props.raceNumber;
    const current = scratchedByRace.value.get(raceKey) ?? new Set<string>();
    const next = new Set(current);
    if (checked) {
        next.add(programNumber);
    } else {
        next.delete(programNumber);
    }
    const updated = new Map(scratchedByRace.value);
    updated.set(raceKey, next);
    scratchedByRace.value = updated;
};

const horsesForRace = computed(() => {
    const horses = props.analysis?.races?.[props.raceNumber - 1]?.horses ?? [];
    return [...horses]
        .filter(horse => horse.score !== undefined && horse.score !== null)
        .sort((a, b) => (b.score ?? Number.POSITIVE_INFINITY) - (a.score ?? Number.POSITIVE_INFINITY));
});

</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="color-accent-yellow">Contextual Speed and Pace Model</div>
            <div class="color-accent-yellow">{{ props.analysis.track }} - {{ props.analysis.date }}</div>
            <div class="caution color-accent-yellow">(CAUTION: This is a mathematical model and should be used as one of many tools in your analysis. Scratches can affect the results.)</div>
            <div class="race-info">
                <div class="color-accent-yellow">Race <span class="color-accent-green">{{ raceNumber }}</span></div>
                <div class="color-accent-yellow">Shape: <span class="color-accent-green">{{
                    props.analysis.races[props.raceNumber - 1].shape }}</span></div>
                <div class="color-accent-yellow">EPI: <span class="color-accent-green">{{
                    props.analysis.races[props.raceNumber - 1].epi.toFixed(2) }}</span></div>
                <div class="color-accent-yellow">Confidence: <span class="color-accent-green">{{ metadata.confidence
                        }}</span></div>
                <div class="color-accent-yellow">Winner: <span class="color-accent-green">{{
                    metadata.win_bet?.horse_name != null ? metadata.win_bet.horse_name : "None Selected" }}</span>
                </div>
            </div>
            <div class="horses">
                <div class="horse-row">
                    <div class="color-accent-yellow">Scratch</div>
                    <div class="color-accent-yellow">#</div>
                    <div class="color-accent-yellow">Horse Name</div>
                    <div class="color-accent-yellow numeric-right">Score</div>
                    <div class="color-accent-yellow numeric-right">RepS</div>
                    <div class="color-accent-yellow numeric-right">RepE</div>
                    <div class="color-accent-yellow numeric-right">RepL</div>
                    <div class="color-accent-yellow text-center">Style</div>
                    <div class="color-accent-yellow text-center">Quirin</div>
                </div>
                <div
                    class="horse-row"
                    :class="{ scratched: currentScratched.has(horse.programNumber) }"
                    v-for="(horse, idx) in horsesForRace"
                    :key="idx"
                >
                    <div class="horse-checkbox">
                        <input
                            class="horse-checkbox-input"
                            type="checkbox"
                            :checked="currentScratched.has(horse.programNumber)"
                            @change="toggleScratch(horse.programNumber, ($event.target as HTMLInputElement).checked)"
                        />
                    </div>
                    <div>{{ horse.programNumber }}</div>
                    <div>{{ horse.horseName }}</div>
                    <div class="numeric-right">{{ horse.score?.toFixed(2) }}</div>
                    <div class="numeric-right">{{ horse.rep.repSpeed?.toFixed(2) }}</div>
                    <div class="numeric-right">{{ horse.rep.repEarly?.toFixed(2) }}</div>
                    <div class="numeric-right">{{ horse.rep.repLate?.toFixed(2) }}</div>
                    <div class="text-center">{{ horse.runStyle !== "Unk" ? horse.runStyle : "" }}</div>
                    <div class="text-center">{{ horse.quirin }}</div>
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

.caution {
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
    grid-template-columns: 6rem 5rem 20rem 5rem 5rem 5rem 5rem 5rem 5rem;
    column-gap: 1rem;
    align-items: baseline;
    position: relative;
}

.horse-checkbox {
    display: flex;
    align-items: center;
    justify-content: center;
}

.horse-checkbox-input {
    transform: scale(1.5);
}

.numeric-right {
    text-align: right;
}

.text-center {
    text-align: center;
}

.horse-row.scratched::after {
    content: "";
    position: absolute;
    left: 7rem;
    right: calc(100% - 69rem);
    top: 50%;
    height: 2px;
    background: currentColor;
    z-index: 2;
    pointer-events: none;
}
</style>
