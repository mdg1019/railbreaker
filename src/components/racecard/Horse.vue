<script setup lang="ts">
import type { Horse } from "../../models/racecard";
import Panel from "../ui/Panel.vue";
import HorseHeaderLeft from "../horse-body/horse-header/HorseHeaderLeft.vue";
import HorseHeaderRight from "../horse-body/horse-header/HorseHeaderRight.vue";
import HorseHeaderCenter from "../horse-body/horse-header/HorseHeaderCenter.vue";
import PastPerfomances from "../horse-body/PastPerfomances.vue";
import Workouts from "../horse-body/Workouts.vue";
import TrainerStats from "../horse-body/TrainerStats.vue";
import TrainerJockey from "../horse-body/TrainerJockey.vue";
import NoteEditor from "../horse-body/NoteEditor.vue";
import { invoke } from "@tauri-apps/api/core";

const props = withDefaults(defineProps<{
    horse: Horse,
    primePowerComparisons: Array<[number | string, string, string]>;
    print: boolean;
}>(), {
    print: false,
});

const emit = defineEmits<{
    (e: "update:note", value: [string, number]): void;
}>();

let saveTimeout: ReturnType<typeof setTimeout> | null = null;

function updateNote([note, horseId]: [string, number]) {
    console.log("Updating note for horse ID:", horseId, "with note:", note);
    if (props.print) {
        return;
    }

    emit("update:note", [note, horseId]);

    if (saveTimeout) {
        clearTimeout(saveTimeout);
    }

    saveTimeout = setTimeout(async () => {
        await invoke("update_note", { note: note, horseId: horseId }).catch(() => { });
    }, 500);
}
</script>

<template>
    <Panel :print="props.print" class="horse-panel">
        <div class="horse">
            <div v-if="props.print" class="horizontal-rule"></div>
            <div class="horse-header">
                <HorseHeaderLeft :horse="props.horse" />  

                <HorseHeaderCenter
                    :horse="props.horse"
                    :primePowerComparisons="props.primePowerComparisons" />

                <HorseHeaderRight :horse="props.horse" />
            </div>

            <div class="horse-body">
                <div v-if="!props.print" class="horizontal-line"></div>
                <PastPerfomances :horse="props.horse" :print="props.print" />
                
                <Workouts :workouts="props.horse.workouts" :print="props.print" />

                <TrainerStats :horse="props.horse" />

                <TrainerJockey :horse="props.horse" />

                <NoteEditor :horse="props.horse" :print="props.print" @update:note="updateNote"/>
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.horse {
    font-family: "MGSansCondensed", sans-serif;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.horse-header {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: center;
}

.horizontal-line {
    height: calc(2px + 0.5rem);
    margin-top: 0.5rem;
    border-bottom: 2px solid var(--ubuntu-blue);  
    width: calc(100% + 2 * 1rem);
    margin: 0 calc(-1 * 1rem);
    margin-bottom: 1rem;
}

.workouts {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 0.25rem;
    align-items: flex-start;
}

.workout-item {
    display: flex;
    align-items: center;
}
</style>
