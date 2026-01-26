<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useGlobalStateStore } from "../stores/globalStateStore";
import { useConfigFileStore } from "../stores/configFileStore";
import { Racecard } from "../models/racecard";
import { Racecards } from "../models/racecards";
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

const globalStateStore = useGlobalStateStore();
const configFileStore = useConfigFileStore();

let unlistenOpen: (() => void);
let unlistenOpenZip: (() => void);
let unlistenPrintRacecard: (() => void);
let unlistenExit: (() => void);

const isProcessingZip = ref(false);
const isProcessingRacecard = ref(false);

const racecards = new Racecards();
const racecard = ref<Racecard | null>(null);
const lastNoteUpdateAt = ref(0);

const race_number = ref(1);

const primePowerComparisons = ref<Array<[number | string, string, string]>>([]);

const currentRacecardIndex = ref(0);

const isRacecardMenuOpen = ref(false);

const raceContainerRef = ref<HTMLElement | null>(null);

const showErrorDialog = ref(false);
const errorMessage = ref("");
const showPrintDialog = ref(false);
const showSelectRacecardDialog = ref(false);
const filteredRacecards = ref<Racecard[]>([]);
let pendingPrintResolve: ((value: number[] | null) => void) | null = null;

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
    race_number.value = value;
    const entry = racecards.racecardEntries[currentRacecardIndex.value];
    if (entry) {
        entry.last_opened_race = value;
    }
    isRacecardMenuOpen.value = false;
}

function handleDeleteRacecard(index: number) {
    racecards.deleteRacecardAt(index);
    if (racecards.racecardEntries.length === 0) {
        currentRacecardIndex.value = 0;
        racecard.value = null;
        race_number.value = 1;
        return;
    }

    const newIndex = index - 1 >= 0 ? index - 1 : 0;
    currentRacecardIndex.value = Math.min(newIndex, racecards.racecardEntries.length - 1);
}

function updateNote([note, horse_id]: [string, number]) {
    const entry = racecards.racecardEntries[currentRacecardIndex.value];
    if (!entry) {
        return;
    }

    const updateHorseNote = (rc: Racecard) => {
        for (const race of rc.races ?? []) {
            const horse = race.horses?.find(h => h.id === horse_id);
            if (horse) {
                horse.note = note;
                return true;
            }
        }
        return false;
    };

    updateHorseNote(entry.racecard);
    if (racecard.value && racecard.value !== entry.racecard) {
        updateHorseNote(racecard.value);
    }
}

async function handleOpenRacecard(id: number | null) {
    if (id === null) {
        return;
    }

    isProcessingRacecard.value = true;
    try {
        const rc = Racecard.fromObject(await invoke<any>('get_racecard_by_id', { racecardId: id }));

        racecards.addRacecard(rc);

        currentRacecardIndex.value = racecards.racecardEntries.length - 1;
        racecard.value = rc;

        isProcessingRacecard.value = false;
    } catch (error) {
        console.error("Failed to open racecard", error);
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

watch(currentRacecardIndex, (idx, oldIdx) => {
    if (typeof oldIdx === 'number' && oldIdx >= 0) {
        const prev = racecards.racecardEntries[oldIdx];
        if (prev) prev.last_opened_race = race_number.value;
    }

    const entry = racecards.racecardEntries[idx];
    racecard.value = entry?.racecard ?? null;
    if (entry && entry.last_opened_race && entry.last_opened_race > 0) {
        race_number.value = entry.last_opened_race;
    } else {
        race_number.value = 1;
    }
});

watch(race_number, async (_newVal, _oldVal) => {
    await nextTick();
    raceContainerRef.value?.scrollIntoView({ behavior: "smooth", block: "start" });
});

document.documentElement.classList.add('dark');

onMounted(async () => {
    unlistenOpen = await listen("menu-open", async () => {
        let racecardsInDatabase= await invoke<Array<Racecard>>('get_all_racecards').catch(() => null);

        if (!racecardsInDatabase || racecardsInDatabase.length === 0) {
            errorMessage.value = "No racecards found in the database. Please import a ZIP file.";
            showErrorDialog.value = true;
            return;
        }

        let filteredRacecardsList = racecardsInDatabase.filter(rc => {
            return !racecards.racecardEntries.some(entry => {
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
            console.log(filename);
            const exists = await invoke("racecard_exists_by_zip_name", { zipFileName: filename });
            console.log('here');
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

                racecards.addRacecard(openedRacecard);
                currentRacecardIndex.value = racecards.racecardEntries.length - 1;
                racecard.value = openedRacecard;
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

    await globalStateStore.loadGlobalState();
    await configFileStore.loadConfigFile();
});

onUnmounted(() => {
    unlistenOpen();
    unlistenOpenZip();
    unlistenPrintRacecard();
    unlistenExit();
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
            <Horse v-for="(horse, idx) in (racecard.races[race_number - 1]?.horses || [])"
                :key="horse.program_number || horse.post_position || idx" :horse="horse"
                :primePowerComparisons="primePowerComparisons" :print="false" @update:note="updateNote"></Horse>
        </div>
        
        <PrintDialog v-model="showPrintDialog" :racecard="racecard" @update:modelValue="handlePrintDialogUpdate"
            @print="handlePrintDialogPrint" />
        <MessageDialog v-model="showErrorDialog" :message="errorMessage" messageColor="--accent-green" title="Error" titleColor="--accent-red" />
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
