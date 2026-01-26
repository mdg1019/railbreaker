<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RaceRankResult } from "../../models/analysis";
import { Race } from "../../models/racecard";
import { deriveRaceMeta } from "../../utils/cspm";
import Panel from "../ui/Panel.vue";

const props = withDefaults(defineProps<{
    raceNumber: number;
    race: Race;
    racecardDate?: string;
    track?: string;
    print: boolean;
}>(), {
    print: false,
    racecardDate: undefined,
    track: "",
});

const raceResult = ref<RaceRankResult | null>(null);
const scratchedIndices = ref(new Set<number>());

const raceIndexByProgram = computed(() => {
    const map = new Map<string, number>();
    props.race?.horses?.forEach((horse, idx) => {
        map.set(horse.programNumber, idx);
    });
    return map;
});

const scratchedList = computed(() => {
    return Array.from(scratchedIndices.value).sort((a, b) => a - b);
});

const displayHeader = computed(() => {
    const parts = [];
    if (props.track) parts.push(props.track);
    if (props.racecardDate) parts.push(props.racecardDate);
    return parts.join(" - ");
});

const isHorseScratched = (programNumber: string) => {
    const idx = raceIndexByProgram.value.get(programNumber);
    return idx !== undefined && scratchedIndices.value.has(idx);
};

const metadata = computed(() => {
    const race = raceResult.value;
    if (!race) {
        return deriveRaceMeta(new RaceRankResult());
    }
    return deriveRaceMeta(new RaceRankResult({
        ...race,
        horses: race.horses.filter(horse => !isHorseScratched(horse.programNumber)),
    }));
});

const toggleScratch = (programNumber: string, checked: boolean) => {
    const idx = raceIndexByProgram.value.get(programNumber);
    if (idx === undefined) return;
    const next = new Set(scratchedIndices.value);
    if (checked) {
        next.add(idx);
    } else {
        next.delete(idx);
    }
    scratchedIndices.value = next;
};

const horsesForRace = computed(() => {
    const horses = raceResult.value?.horses ?? [];
    return [...horses]
        .filter(horse => horse.score !== undefined && horse.score !== null)
        .sort((a, b) => (b.score ?? Number.POSITIVE_INFINITY) - (a.score ?? Number.POSITIVE_INFINITY));
});

const fetchRaceRank = async () => {
    if (!props.race) {
        raceResult.value = null;
        return;
    }
    const racePayload = Race.fromObject(props.race).toObject();
    try {
        const result = await invoke<any>("rank_race", {
            race: racePayload,
            racecardDate: props.racecardDate ?? null,
            scratchedHorses: scratchedList.value,
        });
        raceResult.value = RaceRankResult.fromObject(result);
    } catch (err) {
        console.error("Failed to rank race", err);
        raceResult.value = null;
    }
};

watch(
    () => props.race,
    () => {
        scratchedIndices.value = new Set();
    },
    { immediate: true }
);

watch(
    [() => props.race, () => props.racecardDate, scratchedList],
    () => {
        fetchRaceRank();
    },
    { immediate: true }
);

</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="color-accent-yellow">Contextual Speed and Pace Model</div>
            <div class="color-accent-yellow">{{ displayHeader }}</div>
            <div class="caution color-accent-yellow">(CAUTION: This is a mathematical model and should be used as one of many tools in your analysis. Scratches can affect the results.)</div>
            <div class="race-info">
                <div class="color-accent-yellow">Race <span class="color-accent-green">{{ raceNumber }}</span></div>
                <div class="color-accent-yellow">Shape: <span class="color-accent-green">{{
                    raceResult?.shape }}</span></div>
                <div class="color-accent-yellow">EPI: <span class="color-accent-green">{{
                    raceResult?.epi?.toFixed(2) }}</span></div>
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
                    :class="{ scratched: isHorseScratched(horse.programNumber) }"
                    v-for="(horse, idx) in horsesForRace"
                    :key="idx"
                >
                    <div class="horse-checkbox">
                        <input
                            class="horse-checkbox-input"
                            type="checkbox"
                            :checked="isHorseScratched(horse.programNumber)"
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
