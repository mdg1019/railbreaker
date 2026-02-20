<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick, watch, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { getName, getVersion } from "@tauri-apps/api/app";
import { useGlobalStateStore } from "../stores/globalStateStore";
import { useConfigFileStore } from "../stores/configFileStore";
import { Racecard } from "../models/racecard";
import RacecardHeader from "../components/racecard/RacecardHeader.vue";
import RaceDetails from "../components/racecard/RaceDetails.vue";
import EqualizerLoader from "../components/ui/EqualizerLoader.vue";
import MessageDialog from "../components/ui/MessageDialog.vue";
import PrintDialog from "../components/ui/PrintDialog.vue";
import { openPrintWindowAndSendPayload } from "../utils/openPrintWindowEvent";
import { computePrimePowerComparisons } from "../utils/computePrimePowerComparisons";
import Horse from "../components/racecard/Horse.vue";
import SelectRacecardDialog from "../components/ui/SelectRacecardDialog.vue";
import SelectHorseSortDialog from "../components/ui/SelectHorseSortDialog.vue";
import Analysis from "../components/racecard/Analysis.vue";
import "../scss/_main.scss";
import TripAnalysis from "../components/racecard/TripAnalysis.vue";
import { storeToRefs } from "pinia";
import { TripInfo, useRacecardStateStore } from "../stores/racecardStateStore";
import {
    HORSE_SORTING_METHOD_PROGRAM_NUMBER,
    HORSE_SORTING_METHOD_NAME,
    HORSE_SORTING_METHOD_MORNING_LINE,
    HORSE_SORTING_METHOD_CSPM,
    HORSE_SORTING_METHOD_TRIP,
} from "../constants/horseSortingMethods";
import RaceClassification from "../components/racecard/RaceClassification.vue";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();
const racecardStateStore = useRacecardStateStore();
const { getCurrentRacecard, getCurrentRacecardIdx, getRacecards, currentRaceNumber, getRaceMeta, getTripData } = storeToRefs(racecardStateStore);

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenPrintRacecard: (() => void);
let unlistenNextPage: (() => void);
let unlistenPrevPage: (() => void);
let unlistenSortHorses: (() => void);
let unlistenExit: (() => void);
let unlistenHelp: (() => void);
let unlistenAbout: (() => void);

const isProcessingZip = ref(false);
const isProcessingRacecard = ref(false);
const isSwitchingRace = ref(false);

const racecard = getCurrentRacecard;
const lastNoteUpdateAt = ref(0);

const race_number = currentRaceNumber;

const currentRacecardIndex = computed({
    get: () => getCurrentRacecardIdx.value,
    set: (value) => racecardStateStore.setCurrentRacecardIdx(value),
});

const racecards = getRacecards;

const racecardLabel = computed(() => {
    if (racecards.value.racecardEntries.length === 0) {
        return "No racecards loaded";
    }
    const entry = racecards.value.racecardEntries[currentRacecardIndex.value];
    if (!entry) {
        return "Select racecard";
    }
    const track = entry.racecard.track || entry.racecard.track_code || "Racecard";
    const date = entry.racecard.long_date || entry.racecard.date || entry.racecard.zip_file_name;
    return `${track} — ${date}`;
});

const raceOptions = computed(() => racecard.value?.races?.length ?? 0);

const primePowerComparisons = ref<Array<[number | string, string, string]>>([]);

const raceContainerRef = ref<HTMLElement | null>(null);
const raceContentRef = ref<HTMLElement | null>(null);
const racecardDropdownRef = ref<HTMLElement | null>(null);
const raceDropdownRef = ref<HTMLElement | null>(null);

const isRacecardDropdownOpen = ref(false);
const isRaceDropdownOpen = ref(false);

const showErrorDialog = ref(false);
const errorMessage = ref("");
const showAboutDialog = ref(false);
const aboutTitle = ref("About");
const aboutMessage = ref("RailBreaker");
const showHelpDialog = ref(false);
const helpTitle = ref("Help");
const helpMessageRows = ref<Array<[string, string]>>([
    ["Open Racecard", "Cmd/Ctrl+O"],
    ["Open Zip", "Cmd/Ctrl+Shift+O"],
    ["Print Racecard", "Cmd/Ctrl+Shift+P"],
    ["Next Page", "Cmd/Ctrl+N"],
    ["Previous Page", "Cmd/Ctrl+P"],
    ["Sort Horses", "Cmd/Ctrl+S"],
    ["Help", "Cmd/Ctrl+H"],
    ["Exit", "Cmd/Ctrl+Q"],
]);
const helpMessage = ref("This output from the Contextual Speed and Pace Model is based on a mathematical model and is intended to be used as one of several analytical tools. Projected winners have an increased likelihood of success if the race unfolds in accordance with the model’s assumptions. However, horse racing is inherently unpredictable. Do not rely solely on computer projections, as late scratches and race-day variables can materially affect the outcome.\n\nThe Trip Handicapping Model is very much a work in progress. Be sure to study the PP info in the tooltip to get a more accurate depiction of the trip.");
const showPrintDialog = ref(false);
const showSelectRacecardDialog = ref(false);
const showSortMethodDialog = ref(false);
const filteredRacecards = ref<Racecard[]>([]);
let pendingPrintResolve: ((value: number[] | null) => void) | null = null;
let switchingRaceTimeout: ReturnType<typeof setTimeout> | null = null;
let switchingRaceStartedAt = 0;
let switchingRaceFrameId: number | null = null;

const horseSortingOptions = [
    { value: HORSE_SORTING_METHOD_PROGRAM_NUMBER, label: "Program Number" },
    { value: HORSE_SORTING_METHOD_NAME, label: "Horse Name" },
    { value: HORSE_SORTING_METHOD_MORNING_LINE, label: "Morning Line" },
    { value: HORSE_SORTING_METHOD_CSPM, label: "CSPM Score" },
    { value: HORSE_SORTING_METHOD_TRIP, label: "Trip Score" },
];

function normalizeSortKey(value?: string | null): string {
    return (value ?? "").trim().toLowerCase();
}

const cspmScoreLookup = computed(() => {
    const byProgram = new Map<string, number>();
    const byName = new Map<string, number>();
    const horses = getRaceMeta.value?.race_rank_result?.horses ?? [];

    for (const horse of horses) {
        if (!Number.isFinite(horse.score)) {
            continue;
        }
        const score = horse.score as number;
        const program = normalizeSortKey(horse.program_number);
        const name = normalizeSortKey(horse.horse_name);
        if (program) {
            byProgram.set(program, score);
        }
        if (name) {
            byName.set(name, score);
        }
    }

    return { byProgram, byName };
});

const tripScoreLookup = computed(() => {
    const byProgram = new Map<string, number>();
    const byName = new Map<string, number>();
    const trips: TripInfo[] = getTripData.value ?? [];

    for (const trip of trips) {
        const score = trip.score;
        if (typeof score !== "number" || !Number.isFinite(score)) continue;

        const program = normalizeSortKey(trip.program_number);
        const name = normalizeSortKey(trip.horse_name);
        if (program) {
            byProgram.set(program, score);
        }
        if (name) {
            byName.set(name, score);
        }
    }

    return { byProgram, byName };
});

function getCspmScore(horse: { program_number?: string; horse_name?: string }): number {
    const program = normalizeSortKey(horse.program_number);
    if (program) {
        const byProgram = cspmScoreLookup.value.byProgram.get(program);
        if (byProgram != null) return byProgram;
    }
    const name = normalizeSortKey(horse.horse_name);
    if (name) {
        const byName = cspmScoreLookup.value.byName.get(name);
        if (byName != null) return byName;
    }
    return Number.NEGATIVE_INFINITY;
}

function getTripScore(horse: { program_number?: string; horse_name?: string }): number {
    const program = normalizeSortKey(horse.program_number);
    if (program) {
        const byProgram = tripScoreLookup.value.byProgram.get(program);
        if (byProgram != null) return byProgram;
    }
    const name = normalizeSortKey(horse.horse_name);
    if (name) {
        const byName = tripScoreLookup.value.byName.get(name);
        if (byName != null) return byName;
    }
    return Number.NEGATIVE_INFINITY;
}

function compareProgramNumber(a: { program_number?: string }, b: { program_number?: string }): number {
    return (a.program_number ?? "").localeCompare(b.program_number ?? "", undefined, {
        numeric: true,
        sensitivity: "base",
    });
}

const sortedHorses = computed(() => {
    const horses = racecard.value?.races?.[race_number.value - 1]?.horses ?? [];
    const method = configFileStore.configState.horseSortingMethod;
    const sorted = [...horses];

    switch (method) {
        case HORSE_SORTING_METHOD_PROGRAM_NUMBER:
            sorted.sort(compareProgramNumber);
            break;
        case HORSE_SORTING_METHOD_NAME:
            sorted.sort((a, b) =>
                normalizeSortKey(a.horse_name).localeCompare(normalizeSortKey(b.horse_name), undefined, {
                    sensitivity: "base",
                })
            );
            break;
        case HORSE_SORTING_METHOD_MORNING_LINE:
            sorted.sort((a, b) => {
                const aOdds = Number.isFinite(a.morning_line_odds) ? (a.morning_line_odds as number) : Number.POSITIVE_INFINITY;
                const bOdds = Number.isFinite(b.morning_line_odds) ? (b.morning_line_odds as number) : Number.POSITIVE_INFINITY;
                if (aOdds !== bOdds) {
                    return aOdds - bOdds;
                }
                return compareProgramNumber(a, b);
            });
            break;
        case HORSE_SORTING_METHOD_CSPM:
            sorted.sort((a, b) => {
                const aScore = getCspmScore(a);
                const bScore = getCspmScore(b);
                if (aScore !== bScore) {
                    return bScore - aScore;
                }
                return compareProgramNumber(a, b);
            });
            break;
        case HORSE_SORTING_METHOD_TRIP:
            sorted.sort((a, b) => {
                const aTrip = getTripScore(a);
                const bTrip = getTripScore(b);
                if (aTrip !== bTrip) {
                    return bTrip - aTrip;
                }
                return compareProgramNumber(a, b);
            });
            break;
        default:
            sorted.sort(compareProgramNumber);
            break;
    }

    return sorted;
});

function requestPrintRaces(): Promise<number[] | null> {
    openDialog("print");
    return new Promise((resolve) => {
        pendingPrintResolve = resolve;
    });
}

function handlePrintDialogUpdate(value: boolean) {
    showPrintDialog.value = value;
    if (!value && pendingPrintResolve) {
        pendingPrintResolve(null);
        pendingPrintResolve = null;
    }
}

function handlePrintDialogPrint(value: number[]) {
    showPrintDialog.value = false;
    if (pendingPrintResolve) {
        pendingPrintResolve(value);
        pendingPrintResolve = null;
    }
}

function handleSortMethodSelect(value: string) {
    if (!value) {
        return;
    }
    configFileStore.configState.horseSortingMethod = value;
    void configFileStore.saveConfigFile();
}

function closeAllDialogs() {
    if (showPrintDialog.value) {
        handlePrintDialogUpdate(false);
    }
    showErrorDialog.value = false;
    showAboutDialog.value = false;
    showHelpDialog.value = false;
    showSelectRacecardDialog.value = false;
    showSortMethodDialog.value = false;
}

function openDialog(target: "error" | "about" | "help" | "print" | "select" | "sort") {
    closeAllDialogs();
    switch (target) {
        case "error":
            showErrorDialog.value = true;
            break;
        case "about":
            showAboutDialog.value = true;
            break;
        case "help":
            showHelpDialog.value = true;
            break;
        case "print":
            showPrintDialog.value = true;
            break;
        case "select":
            showSelectRacecardDialog.value = true;
            break;
        case "sort":
            showSortMethodDialog.value = true;
            break;
    }
}

function handleSelectedRace(value: number) {
    racecardStateStore.setLastOpenedRace(value);
}

function toggleRacecardDropdown() {
    if (racecards.value.racecardEntries.length === 0) {
        return;
    }
    isRacecardDropdownOpen.value = !isRacecardDropdownOpen.value;
    if (isRacecardDropdownOpen.value) {
        isRaceDropdownOpen.value = false;
    }
}

function toggleRaceDropdown() {
    if (!racecard.value) {
        return;
    }
    isRaceDropdownOpen.value = !isRaceDropdownOpen.value;
    if (isRaceDropdownOpen.value) {
        isRacecardDropdownOpen.value = false;
    }
}

function handleSelectRacecard(index: number) {
    currentRacecardIndex.value = index;
    isRacecardDropdownOpen.value = false;
}

function handleSelectRace(value: number) {
    handleSelectedRace(value);
    isRaceDropdownOpen.value = false;
}

function handleDropdownOutsideClick(event: MouseEvent) {
    const target = event.target as Node | null;
    if (!target) {
        return;
    }
    if (racecardDropdownRef.value && !racecardDropdownRef.value.contains(target)) {
        isRacecardDropdownOpen.value = false;
    }
    if (raceDropdownRef.value && !raceDropdownRef.value.contains(target)) {
        isRaceDropdownOpen.value = false;
    }
}

function handleDropdownKeydown(event: KeyboardEvent) {
    if (event.key !== "Escape") {
        return;
    }
    if (isRacecardDropdownOpen.value) {
        isRacecardDropdownOpen.value = false;
    }
    if (isRaceDropdownOpen.value) {
        isRaceDropdownOpen.value = false;
    }
}

async function handleOpenRacecard(id: number | null) {
    if (id === null) {
        return;
    }

    isProcessingRacecard.value = true;
    try {
        const rc = Racecard.fromObject(await invoke<any>('get_racecard_by_id', { racecardId: id }));

        racecardStateStore.addRacecard(rc);

        racecardStateStore.setCurrentRacecardIdx(racecards.value.racecardEntries.length - 1);

        isProcessingRacecard.value = false;
    } catch (error) {
        isProcessingRacecard.value = false;
        errorMessage.value = String(error);
        openDialog("error");
    }
}

watch(racecard, async (rc) => {
    await invoke('set_print_racecard_enabled', { enabled: !!rc }).catch(() => { });
    await invoke('set_view_menu_enabled', { enabled: !!rc }).catch(() => { });
});

watch([racecard, race_number], () => {
    primePowerComparisons.value = computePrimePowerComparisons(racecard.value, race_number.value);
    racecardStateStore.updateTripData();
});

watch(race_number, async (_newVal, _oldVal) => {
    if (racecard.value) {
        isSwitchingRace.value = true;
        switchingRaceStartedAt = Date.now();
    }
    await nextTick();
    await nextTick();
    if (switchingRaceFrameId) {
        cancelAnimationFrame(switchingRaceFrameId);
    }
    await new Promise<void>((resolve) => {
        switchingRaceFrameId = requestAnimationFrame(() => resolve());
    });
    raceContainerRef.value?.scrollIntoView({ behavior: "smooth", block: "start" });
    raceContentRef.value?.scrollTo({ top: 0, behavior: "smooth" });
    if (switchingRaceTimeout) {
        clearTimeout(switchingRaceTimeout);
    }
    switchingRaceTimeout = setTimeout(() => {
        isSwitchingRace.value = false;
        switchingRaceTimeout = null;
    }, Math.max(0, 600 - (Date.now() - switchingRaceStartedAt)));
});

document.documentElement.classList.add('dark');

onMounted(async () => {
    window.addEventListener("click", handleDropdownOutsideClick, true);
    window.addEventListener("keydown", handleDropdownKeydown);
    try {
        const [_, version] = await Promise.all([getName(), getVersion()]);
        aboutTitle.value = `About RailBreaker`;
        aboutMessage.value = `RailBreaker ${version}\nCopyright © 2026 By Mark Goodwin\nMIT License`;
    } catch {
        aboutTitle.value = "About RailBreaker";
        aboutMessage.value = "RailBreaker\nCopyright © 2026 By Mark Goodwin\nMIT License";
    }

    unlistenOpen = await listen("menu-open", async () => {
        let racecardsInDatabase= await invoke<Array<Racecard>>('get_all_racecards').catch(() => null);

        if (!racecardsInDatabase || racecardsInDatabase.length === 0) {
            errorMessage.value = "No racecards found in the database. Please import a ZIP file.";
            openDialog("error");
            return;
        }

        let filteredRacecardsList = racecardsInDatabase.filter(rc => {
            return !racecards.value.racecardEntries.some(entry => {
                if (entry.racecard.id != null && rc.id != null) {
                    return entry.racecard.id === rc.id;
                }
                return entry.racecard.zip_file_name === rc.zip_file_name;
            });
        });

        if (filteredRacecardsList.length === 0) {
            errorMessage.value = "All racecards in the database are already opened.";
            openDialog("error");
            return;
        }

        filteredRacecards.value = filteredRacecardsList;
        openDialog("select");
    });

    unlistenOpenZip = await listen("menu-open-zip", async () => {
        const path = await open({
            multiple: false,
            filters: [
                {
                    name: "Zip Files",
                    extensions: ["zip"],
                },
            ],
            defaultPath: configFileStore.configState.lastDirectory
        });

        if (path) {
            const filename = path.split(/[\\/]/).pop() ?? "";
            const exists = await invoke("racecard_exists_by_zip_name", { zipFileName: filename });
            
            if (exists) {
                errorMessage.value = `Racecard with ${filename} already exists in the database.`;
                openDialog("error");

                return;
            }

            isProcessingZip.value = true;

            try {
                const processedPath = await invoke<string>('process_zip_file', { path: path });

                isProcessingZip.value = false;

                isProcessingRacecard.value = true;

                const racecardValue = await invoke<Racecard>(
                    'process_racecard_file',
                    { path: processedPath, zipFileName: path }
                );

                const openedRacecard = Racecard.fromObject(racecardValue);

                racecardStateStore.addRacecard(openedRacecard);
                currentRacecardIndex.value = racecards.value.racecardEntries.length - 1;
                isProcessingRacecard.value = false;
            } catch (error) {
                isProcessingZip.value = false;
                isProcessingRacecard.value = false;

                errorMessage.value = String(error);
                openDialog("error");
            }
        }
    });

    unlistenPrintRacecard = await listen("menu-print", async () => {
        if (!racecard.value) {
            return;
        }
        const selectedRaces = await requestPrintRaces();

        if (!selectedRaces) {
            return;
        }

        await nextTick();
        const elapsed = Date.now() - lastNoteUpdateAt.value;
        if (elapsed < 200) {
            await new Promise((resolve) => setTimeout(resolve, 200 - elapsed));
        }

        const raceCardPrintPayload = {
            raceCard: racecard.value,
            printRaces: selectedRaces
        };

        await openPrintWindowAndSendPayload(raceCardPrintPayload, {});
    });

    unlistenNextPage = await listen("menu-next-page", () => {
        racecardStateStore.setNextRaceNumber();
    });

    unlistenPrevPage = await listen("menu-prev-page", () => {
        racecardStateStore.setPrevRaceNumber();
    });

    unlistenSortHorses = await listen("menu-sort-horses", () => {
        openDialog("sort");
    });

    unlistenExit = await listen("menu-exit", async () => {
        await invoke('exit_app');
    });

    unlistenHelp = await listen("menu-help", () => {
        openDialog("help");
    });

    unlistenAbout = await listen("menu-about", () => {
        openDialog("about");
    });

    await globalStateStore.loadGlobalState();
    await configFileStore.loadConfigFile();
});

onUnmounted(() => {
    window.removeEventListener("click", handleDropdownOutsideClick, true);
    window.removeEventListener("keydown", handleDropdownKeydown);
    unlistenOpen();
    unlistenOpenZip();
    unlistenPrintRacecard();
    unlistenNextPage();
    unlistenPrevPage();
    unlistenSortHorses();
    unlistenExit();
    unlistenHelp();
    unlistenAbout();
    if (switchingRaceTimeout) {
        clearTimeout(switchingRaceTimeout);
    }
    if (switchingRaceFrameId) {
        cancelAnimationFrame(switchingRaceFrameId);
    }
});
</script>



<template>
    <main class="container">
        <div class="menubar" v-if="racecards.racecardEntries.length > 0">
            <div class="menu-group" ref="racecardDropdownRef">
                <span class="menu-label">Racecard</span>
                <button
                    type="button"
                    class="menu-trigger"
                    :class="{ disabled: racecards.racecardEntries.length === 0 }"
                    :disabled="racecards.racecardEntries.length === 0"
                    :aria-expanded="isRacecardDropdownOpen"
                    @click="toggleRacecardDropdown"
                >
                    <span class="menu-trigger-text">{{ racecardLabel }}</span>
                </button>
                <div v-if="isRacecardDropdownOpen" class="menu-list" role="listbox">
                    <button
                        v-for="(entry, idx) in racecards.racecardEntries"
                        :key="entry.racecard.id ?? entry.racecard.zip_file_name ?? idx"
                        type="button"
                        class="menu-item"
                        :class="{ active: idx === currentRacecardIndex }"
                        @click="handleSelectRacecard(idx)"
                    >
                        {{ entry.racecard.track || entry.racecard.track_code || "Racecard" }} — {{ entry.racecard.long_date || entry.racecard.date || entry.racecard.zip_file_name }}
                    </button>
                </div>
            </div>
            <div class="menu-group" ref="raceDropdownRef">
                <span class="menu-label">Race</span>
                <button
                    type="button"
                    class="menu-trigger"
                    :class="{ disabled: !racecard }"
                    :disabled="!racecard"
                    :aria-expanded="isRaceDropdownOpen"
                    @click="toggleRaceDropdown"
                >
                    <span class="menu-trigger-text" v-if="racecard">
                        <span class="color-accent-yellow">Race {{ race_number }} - </span><RaceClassification class="color-accent-yellow" :race="racecard.races[race_number - 1]" />
                    </span>
                    <span class="menu-trigger-text" v-else>No racecard selected</span>
                </button>
                <div v-if="isRaceDropdownOpen" class="menu-list race-list" role="listbox">
                    <button
                        v-for="n in raceOptions"
                        :key="n"
                        type="button"
                        class="menu-item race-item"
                        :class="{ active: n === race_number }"
                        @click="handleSelectRace(n)"
                    >
                        <span class="race-item-left">Race {{ n }}</span>
                        <span class="race-item-right">
                            <RaceClassification :race="racecard!.races[n - 1]" />
                        </span>
                    </button>
                </div>
            </div>
        </div>
        <div class="processing" v-if="isProcessingZip">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" :title="'Processing ZIP File'" />
        </div>
        <div class="processing" v-if="isProcessingRacecard">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" :title="'Processing Racecard File'" />
        </div>
        <div class="processing" v-if="isSwitchingRace">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" :title="'Switching Race'" />
        </div>
        <div class="race-container" v-if="racecard" ref="raceContainerRef">
            <div class="race-header">
                <RacecardHeader :racecard="racecard" :race="race_number" />
            </div>
        <div class="race-content" ref="raceContentRef">
                <RaceDetails :racecard="racecard" :race="race_number" :print="false" />
                <Analysis :race_number="race_number" :race="racecard.races[race_number - 1]" :racecard_date="racecard.date" :track="racecard.track" :print="false" />
                <TripAnalysis :race="racecard.races[race_number - 1]" :print="false" />
                <Horse v-for="(horse, idx) in sortedHorses"
                    :key="`${race_number}-${horse.id || horse.program_number || horse.post_position || idx}`" :horse="horse"
                    :primePowerComparisons="primePowerComparisons" :print="false"></Horse>
            </div>
        </div>
        
        <PrintDialog v-model="showPrintDialog" :racecard="racecard" @update:modelValue="handlePrintDialogUpdate"
            @print="handlePrintDialogPrint" />
        <MessageDialog v-model="showErrorDialog" :message="errorMessage" messageColor="--accent-green" title="Error" titleColor="--accent-red" />
        <MessageDialog
            v-model="showAboutDialog"
            :message="aboutMessage"
            :title="aboutTitle"
            titleColor="--accent-yellow"
            messageColor="--accent-green"
            linkLabel="Source code: "
            linkText="https://github.com/mdg1019/railbreaker"
            linkHref="https://github.com/mdg1019/railbreaker"
            linkColor="--accent-yellow"
            link2Label="Releases: "
            link2Text="https://github.com/mdg1019/railbreaker/releases"
            link2Href="https://github.com/mdg1019/railbreaker/releases"
            link2Color="--accent-yellow"
        />
        <MessageDialog
            v-model="showHelpDialog"
            :message="helpMessage"
            :messageRows="helpMessageRows"
            messageRowsTitle="Keyboard Shortcuts"
            :title="helpTitle"
            dialogWidth="min(90vw, 1040px)"
            titleColor="--accent-yellow"
            messageColor="--accent-green"
        />
        <SelectRacecardDialog v-model="showSelectRacecardDialog" :racecards="filteredRacecards"
            :selectedRacecardId="racecard?.id"
            @update:modelValue="showSelectRacecardDialog = $event"
            @open="handleOpenRacecard" />
        <SelectHorseSortDialog
            v-model="showSortMethodDialog"
            :options="horseSortingOptions"
            :selected="configFileStore.configState.horseSortingMethod"
            @select="handleSortMethodSelect"
        />
    </main>
</template>

<style lang="scss" scoped>
.container {
    padding: 1rem;
    font-family: "MGSans", sans-serif;
    box-sizing: border-box;
    height: 100vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
}

.menubar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 1rem;
    border: 2px solid var(--accent-green);
    border-radius: 0.75rem;
    background: linear-gradient(135deg, rgba(11, 11, 11, 0.92), rgba(24, 24, 24, 0.92));
    font-family: "MGSans", sans-serif;
}

.menu-group {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
}

.menu-label {
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-yellow);
    white-space: nowrap;
    font-family: "MGSans", sans-serif;
}

.menu-trigger {
    position: relative;
    min-width: 220px;
    max-width: 420px;
    padding: 0.45rem 2rem 0.45rem 0.65rem;
    border-radius: 0.5rem;
    border: 2px solid var(--accent-green);
    background-color: #000;
    color: #fff;
    font-size: 0.95rem;
    font-family: "MGSans", sans-serif;
    text-align: left;
    cursor: pointer;
}

.menu-trigger::after {
    content: "";
    position: absolute;
    right: 0.6rem;
    top: 50%;
    width: 1rem;
    height: 1rem;
    transform: translateY(-50%);
    background-color: var(--accent-green);
    mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M5.5 7.5 10 12l4.5-4.5'/%3E%3C/svg%3E") no-repeat center / contain;
    -webkit-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M5.5 7.5 10 12l4.5-4.5'/%3E%3C/svg%3E") no-repeat center / contain;
}

.menu-trigger:disabled,
.menu-trigger.disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.menu-trigger-text {
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--accent-yellow);
}

.menu-list {
    position: absolute;
    top: calc(100% + 0.4rem);
    left: 0;
    z-index: 20;
    min-width: 100%;
    max-width: 520px;
    max-height: none;
    height: auto;
    overflow: visible;
    background: #000;
    border: 2px solid var(--accent-green);
    border-radius: 0.5rem;
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.45);
    padding: 0.25rem 0;
    box-sizing: border-box;
}

.menu-item {
    width: 100%;
    text-align: left;
    padding: 0.45rem 0.65rem;
    border: none;
    background: transparent;
    color: var(--accent-yellow);
    font-size: 0.95rem;
    font-family: "MGSans", sans-serif;
    cursor: pointer;
}

.menu-item:hover,
.menu-item:focus {
    background: rgba(74, 222, 128, 0.18);
    outline: none;
}

.menu-item.active {
    background: rgba(74, 222, 128, 0.3);
}

.race-item {
    display: grid;
    grid-template-columns: 8ch 1fr;
    align-items: center;
    column-gap: 0.5rem;
}

.race-item-left {
    text-align: left;
    justify-self: start;
    font-variant-numeric: tabular-nums;
}

.race-item-right {
    text-align: left;
    min-width: 0;
}

.race-container {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    flex: 1 1 auto;
    min-height: 0;
}

.race-header {
    position: sticky;
    top: 0;
    z-index: 10;
    background: var(--bg, #0b0b0b);
}

.race-content {
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    flex: 1 1 auto;
    padding-bottom: 1rem;
    min-height: 0;
}

.processing {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    font-size: 1.5rem;
    z-index: 1000;
}
</style>
