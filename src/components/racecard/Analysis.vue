<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RaceMeta } from "../../models/analysis";
import { Race } from "../../models/racecard";
import Panel from "../ui/Panel.vue";
import Transformers from "../../utils/transformers";

const props = withDefaults(defineProps<{
    race_number: number;
    race: Race;
    racecard_date?: string;
    track?: string;
    print: boolean;
}>(), {
    print: false,
    racecard_date: undefined,
    track: "",
});

const raceMeta = ref<RaceMeta | null>(null);
const scratchedIndices = ref(new Set<number>());

const raceIndexByProgram = computed(() => {
    const map = new Map<string, number>();
    props.race?.horses?.forEach((horse, idx) => {
        const program = horse.program_number?.trim();
        if (program) map.set(program, idx);
    });
    return map;
});

const raceIndexByName = computed(() => {
    const map = new Map<string, number>();
    props.race?.horses?.forEach((horse, idx) => {
        const name = horse.horse_name?.trim().toLowerCase();
        if (name) map.set(name, idx);
    });
    return map;
});

const scratchedList = computed(() => {
    return Array.from(scratchedIndices.value).sort((a, b) => a - b);
});

const findRaceIndex = (program_number: string, horse_name: string) => {
    const program = program_number?.trim();
    if (program) {
        const idx = raceIndexByProgram.value.get(program);
        if (idx !== undefined) return idx;
    }
    const name = horse_name?.trim().toLowerCase();
    if (name) {
        const idx = raceIndexByName.value.get(name);
        if (idx !== undefined) return idx;
    }
    return undefined;
};

const isHorseScratched = (program_number: string, horse_name: string) => {
    const idx = findRaceIndex(program_number, horse_name);
    return idx !== undefined && scratchedIndices.value.has(idx);
};

const metadata = computed(() => {
    return raceMeta.value ?? new RaceMeta();
});

const toggleScratch = (program_number: string, horse_name: string, checked: boolean) => {
    const idx = findRaceIndex(program_number, horse_name);
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
    const horses = metadata.value.race_rank_result?.horses ?? [];
    return [...horses]
        .filter(horse => horse.score !== undefined && horse.score !== null)
        .sort((a, b) => (b.score ?? Number.POSITIVE_INFINITY) - (a.score ?? Number.POSITIVE_INFINITY));
});

const fetchRaceRank = async () => {
    if (!props.race) {
        raceMeta.value = null;
        return;
    }

    for (const [idx, horse] of props.race.horses.entries()) {
        horse.scratched = scratchedList.value.includes(idx);
    }

    const racePayload = Race.fromObject(props.race).toObject();
    
    try {
        const result = await invoke<any>("rank_race", {
            race: racePayload,
            racecardDate: props.racecard_date ?? null,
        });
        raceMeta.value = RaceMeta.fromObject(result);
    } catch (err) {
        console.error("Failed to rank race", err);
        raceMeta.value = null;
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
    [() => props.race, () => props.racecard_date, () => scratchedList.value],
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
            <div class="race-info">
                <div class="color-accent-yellow race">Race<span class="left-margin color-accent-green">{{ race_number }}</span></div>
                <div class="color-accent-yellow">Shape:<span class="left-margin color-accent-green">{{
                    metadata.shape }}</span></div>
                <div class="color-accent-yellow">EPI:<span class="left-margin color-accent-green">{{
                    metadata.epi?.toFixed(2) }}</span></div>
                <div class="color-accent-yellow">Confidence:<span class="left-margin color-accent-green">{{ metadata.confidence?.replace(/([a-z])([A-Z])/g, '$1 $2')
                }}</span></div>
                <div class="color-accent-yellow">Winner:<span class="left-margin color-accent-green">{{
                    metadata.win_bet?.horse_name != null ? Transformers.capitalize(metadata.win_bet.horse_name) : "None Selected" }}</span>
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
                <div class="horse-row" :class="{ scratched: isHorseScratched(horse.program_number, horse.horse_name) }"
                    v-for="(horse, idx) in horsesForRace" :key="`${horse.program_number || ''}-${horse.horse_name || ''}-${idx}`">
                    <div class="horse-checkbox">
                        <input class="horse-checkbox-input" type="checkbox"
                            :checked="isHorseScratched(horse.program_number, horse.horse_name)"
                            @change="toggleScratch(horse.program_number, horse.horse_name, ($event.target as HTMLInputElement).checked)" />
                    </div>
                    <div>{{ horse.program_number }}</div>
                    <div>{{ Transformers.capitalize(horse.horse_name) }}</div>
                    <div class="numeric-right">{{ horse.score?.toFixed(2) }}</div>
                    <div class="numeric-right">{{ horse.rep.rep_speed?.toFixed(2) }}</div>
                    <div class="numeric-right">{{ horse.rep.rep_early?.toFixed(2) }}</div>
                    <div class="numeric-right">{{ horse.rep.rep_late?.toFixed(2) }}</div>
                    <div class="text-center">{{ horse.run_style !== "Unk" ? horse.run_style : "" }}</div>
                    <div class="text-center">{{ horse.quirin }}</div>
                </div>
            </div>
            <div class="caution color-accent-yellow">(Caution: <span class="color-accent-green">This output is based on a mathematical model and is intended to be used as one of several analytical tools. Projected winners have an increased likelihood of success if the race unfolds in accordance with the model’s assumptions. However, horse racing is inherently unpredictable. Do not rely solely on computer projections, as late scratches and race-day variables can materially affect the outcome.</span>)</div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.contents {
    display: flex;
    flex-direction: column;
}

.caution {
    margin-top: 1rem;
}

.track-date {
    margin-bottom: 1rem;
}

.race-info {
    margin-top: 1rem;
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
    left: calc(6rem + 1rem);
    width: calc(5rem + 20rem + 5rem + 5rem + 5rem + 5rem + 5rem + 5rem + 7rem);
    top: 50%;
    height: 2px;
    background: var(--accent-red);
    z-index: 2;
    pointer-events: none;
}

.left-margin {
    margin-left: 1rem;
}
</style>
