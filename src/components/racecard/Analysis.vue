<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { RaceMeta } from "../../models/analysis";
import { Race } from "../../models/racecard";
import Panel from "../ui/Panel.vue";
import Transformers from "../../utils/transformers";
import { useRacecardStateStore } from "../../stores/racecardStateStore";

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
const racecardStateStore = useRacecardStateStore();

const findHorse = (program_number: string, horse_name: string) => {
    const program = program_number?.trim();
    if (program) {
        const horse = props.race?.horses?.find(h => h.program_number?.trim() === program);
        if (horse) return horse;
    }
    const name = horse_name?.trim().toLowerCase();
    if (name) {
        const horse = props.race?.horses?.find(h => h.horse_name?.trim().toLowerCase() === name);
        if (horse) return horse;
    }
    return undefined;
};

const isHorseScratched = (program_number: string, horse_name: string) => {
    const horse = findHorse(program_number, horse_name);
    return !!horse?.scratched;
};

const isRankHorseScratched = (horse: { program_number: string; horse_name: string }) =>
    isHorseScratched(horse.program_number, horse.horse_name);

const metadata = computed(() => {
    return raceMeta.value ?? new RaceMeta();
});

const toggleScratch = (program_number: string, horse_name: string, checked: boolean) => {
    if (props.print) {
        return;
    }
    const horse = findHorse(program_number, horse_name);
    if (!horse?.id) {
        return;
    }
    racecardStateStore.setScratch(horse.id, checked);
};

const horsesForRace = computed(() => {
    const horses = metadata.value.race_rank_result?.horses ?? [];
    return [...horses]
        .filter(horse => horse.score !== undefined && horse.score !== null)
        .sort((a, b) => (b.score ?? Number.POSITIVE_INFINITY) - (a.score ?? Number.POSITIVE_INFINITY));
});

const horseColumns = computed(() => {
    const horses = horsesForRace.value;
    if (horses.length <= 6) {
        return [horses];
    }
    const mid = Math.ceil(horses.length / 2);
    return [horses.slice(0, mid), horses.slice(mid)];
});

const fetchRaceRank = async () => {
    if (!props.race) {
        raceMeta.value = null;
        return;
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

const scratchedKey = computed(() => {
    if (!props.race?.horses) {
        return "";
    }
    return props.race.horses.map(h => (h.scratched ? "1" : "0")).join("");
});

watch(
    [() => props.race, () => props.racecard_date, () => scratchedKey.value],
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
            <div class="horses-columns">
                <div class="horses-column" v-for="(column, colIdx) in horseColumns" :key="colIdx">
                    <div class="horse-row header">
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
                    <div class="horse-row" :class="{ scratched: isRankHorseScratched(horse) }"
                        v-for="(horse, idx) in column" :key="`${horse.program_number || ''}-${horse.horse_name || ''}-${idx}`">
                        <div class="horse-checkbox">
                            <input class="horse-checkbox-input" type="checkbox"
                                :checked="isRankHorseScratched(horse)"
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

.horses-columns {
    margin-top: 1rem;
    display: flex;
    gap: 2rem;
}

.horses-column {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    flex: 1 1 0;
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
