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
import RacecardSideMenu from "../components/racecard/RacecardSideMenu.vue";
import { openPrintWindowAndSendPayload } from "../utils/openPrintWindowEvent";
import { computePrimePowerComparisons } from "../utils/computePrimePowerComparisons";
import Horse from "../components/racecard/Horse.vue";
import SelectRacecardDialog from "../components/ui/SelectRacecardDialog.vue";
import Analysis from "../components/racecard/Analysis.vue";
import "../scss/_main.scss";
import TripAnalysis from "../components/racecard/TripAnalysis.vue";
import { storeToRefs } from "pinia";
import { useRacecardStateStore } from "../stores/racecardStateStore";
import { HORSE_SORTING_METHOD_PROGRAM_NUMBER } from "../constants/horseSortingMethods";

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();
const racecardStateStore = useRacecardStateStore();
const { getCurrentRacecard, getCurrentRacecardIdx, getCurrentRaceNumber, getRacecards } = storeToRefs(racecardStateStore);

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenPrintRacecard: (() => void);
let unlistenExit: (() => void);
let unlistenAbout: (() => void);

const isProcessingZip = ref(false);
const isProcessingRacecard = ref(false);

const racecard = getCurrentRacecard;
const lastNoteUpdateAt = ref(0);

const race_number = computed({
    get: () => getCurrentRaceNumber.value,
    set: (value) => racecardStateStore.setLastOpenedRace(value),
});

const currentRacecardIndex = computed({
    get: () => getCurrentRacecardIdx.value,
    set: (value) => racecardStateStore.setCurrentRacecardIdx(value),
});

const racecards = getRacecards;

const primePowerComparisons = ref<Array<[number | string, string, string]>>([]);

const isRacecardMenuOpen = ref(false);

const raceContainerRef = ref<HTMLElement | null>(null);

const showErrorDialog = ref(false);
const errorMessage = ref("");
const showAboutDialog = ref(false);
const aboutTitle = ref("About");
const aboutMessage = ref("RailBreaker");
const showPrintDialog = ref(false);
const showSelectRacecardDialog = ref(false);
const filteredRacecards = ref<Racecard[]>([]);
let pendingPrintResolve: ((value: number[] | null) => void) | null = null;

const sortedHorses = computed(() => {
    const horses = racecard.value?.races?.[race_number.value - 1]?.horses ?? [];
    const method = configFileStore.configState.horseSortingMethod;
    const sorted = [...horses];

    switch (method) {
        case HORSE_SORTING_METHOD_PROGRAM_NUMBER:
            sorted.sort((a, b) =>
                (a.program_number ?? "").localeCompare(b.program_number ?? "", undefined, {
                    numeric: true,
                    sensitivity: "base",
                })
            );
            break;
        default:
            sorted.sort((a, b) =>
                (a.program_number ?? "").localeCompare(b.program_number ?? "", undefined, {
                    numeric: true,
                    sensitivity: "base",
                })
            );
            break;
    }

    return sorted;
});

function requestPrintRaces(): Promise<number[] | null> {
    showPrintDialog.value = true;
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

function handleSelectedRace(value: number) {
    racecardStateStore.setLastOpenedRace(value);
    isRacecardMenuOpen.value = false;
}

function handleDeleteRacecard(index: number) {
    racecardStateStore.deleteRacecardAt(index);
    if (racecards.value.racecardEntries.length === 0) {
        racecardStateStore.setCurrentRacecardIdx(0);
        return;
    }

    const newIndex = index - 1 >= 0 ? index - 1 : 0;
    racecardStateStore.setCurrentRacecardIdx(Math.min(newIndex, racecards.value.racecardEntries.length - 1));
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
        showErrorDialog.value = true;
    }
}

watch(racecard, async (rc) => {
    isRacecardMenuOpen.value = false;

    await invoke('set_print_racecard_enabled', { enabled: !!rc }).catch(() => { });
});

watch([racecard, race_number], () => {
    primePowerComparisons.value = computePrimePowerComparisons(racecard.value, race_number.value);
});

watch(race_number, async (_newVal, _oldVal) => {
    await nextTick();
    raceContainerRef.value?.scrollIntoView({ behavior: "smooth", block: "start" });
});

document.documentElement.classList.add('dark');

onMounted(async () => {
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
            showErrorDialog.value = true;
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
            showErrorDialog.value = true;
            return;
        }

        filteredRacecards.value = filteredRacecardsList;
        showSelectRacecardDialog.value = true;
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
                showErrorDialog.value = true;

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
                showErrorDialog.value = true;
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

    unlistenExit = await listen("menu-exit", async () => {
        await invoke('exit_app');
    });

    unlistenAbout = await listen("menu-about", () => {
        showAboutDialog.value = true;
    });

    await globalStateStore.loadGlobalState();
    await configFileStore.loadConfigFile();
});

onUnmounted(() => {
    unlistenOpen();
    unlistenOpenZip();
    unlistenPrintRacecard();
    unlistenExit();
    unlistenAbout();
});
</script>



<template>
    <main class="container">
        <RacecardSideMenu v-if="racecards.racecardEntries.length > 0" :racecards="racecards"
            v-model:currentRacecardIndex="currentRacecardIndex" v-model:open="isRacecardMenuOpen"
            :currentRace="race_number" @update:selectedRace="handleSelectedRace"
            @delete:racecard="handleDeleteRacecard" />
        <div class="processing" v-if="isProcessingZip">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" :title="'Processing ZIP File'" />
        </div>
        <div class="processing" v-if="isProcessingRacecard">
            <EqualizerLoader :bars="5" :width="70" :height="100" color="#4ade80" :title="'Processing Racecard File'" />
        </div>
        <div class="race-container" v-if="racecard" ref="raceContainerRef">
            <RacecardHeader :racecard="racecard" :race="race_number" />
            <RaceDetails :racecard="racecard" :race="race_number" :print="false" />
            <Analysis :race_number="race_number" :race="racecard.races[race_number - 1]" :racecard_date="racecard.date" :track="racecard.track" :print="false" />
            <TripAnalysis :race="racecard.races[race_number - 1]" :print="false" />
            <Horse v-for="(horse, idx) in sortedHorses"
                :key="`${race_number}-${horse.id || horse.program_number || horse.post_position || idx}`" :horse="horse"
                :primePowerComparisons="primePowerComparisons" :print="false"></Horse>
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
        <SelectRacecardDialog v-model="showSelectRacecardDialog" :racecards="filteredRacecards"
            :selectedRacecardId="racecard?.id"
            @update:modelValue="showSelectRacecardDialog = $event"
            @open="handleOpenRacecard" />
    </main>
</template>

<style lang="scss" scoped>
.container {
    padding: 1rem;
    font-family: "MGSans", sans-serif;
}

.race-container {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
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
