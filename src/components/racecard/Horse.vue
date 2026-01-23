<script setup lang="ts">
import type { Horse } from "../../models/racecard";
import { Note } from "../../models/note";
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
    notes: Array<Note>,
    note: Note,
    racecardPath: string,
    primePowerComparisons: Array<[number | string, string, string]>;
    print: boolean;
}>(), {
    print: false,
});

const emit = defineEmits<{
    (e: "update:notes", value: Array<Note>): void;
}>();

let saveTimeout: ReturnType<typeof setTimeout> | null = null;

function updateNote(note: Note) {
    if (props.print) {
        return;
    }

    const updatedNotes = props.notes.map((item) => new Note(item.race, item.horse, item.content));
    const idx = updatedNotes.findIndex((n) => n.race === note.race && n.horse === note.horse);

    if (idx >= 0) {
        updatedNotes[idx] = note;
    } else {
        updatedNotes.push(note);
    }

    emit("update:notes", updatedNotes);

    if (saveTimeout) {
        clearTimeout(saveTimeout);
    }

    const notesPath = props.racecardPath.replace(/\.json$/i, ".notes");
    const payload = updatedNotes.map((item) => item.toObject());

    saveTimeout = setTimeout(async () => {
        await invoke("save_notes_file", { path: notesPath, notes: payload }).catch(() => { });
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

                <NoteEditor :note="props.note" :print="props.print" @update:note="updateNote"/>
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
