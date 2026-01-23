<script setup lang="ts">
import { Note } from "../../models/note";

const props = withDefaults(defineProps<{
    note: Note,
    print: boolean;
}>(), {
    print: false,
});

const emit = defineEmits<{
  (e: "update:note", value: Note): void;
}>();

function handleInput(event: Event) {
  emit("update:note", new Note(
    props.note.race,
    props.note.horse,
    (event.target as HTMLTextAreaElement).value
  ));
}
</script>

<template>
    <div v-if="!props.print || props.note.content !== ''" class="container font-small">
        <div class="label" :class="{ 'is-print': props.print }">Notes:</div>
        <textarea
            v-if="!props.print"
            class="note-textarea font-small"
            rows="5"
            cols="40"
            :value="props.note.content"
            :readonly="props.print"
            @input="handleInput"
        ></textarea>
        <div v-else>
            {{ props.note.content }}
        </div>
    </div>
</template>

<style lang="scss" scoped>
.container {
    font-family: "MGSansCondensed";
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    margin-top: 1rem;
    width: 100%;
    box-sizing: border-box;
}

.label {
    color: var(--accent-yellow);
    margin-top: 0.75rem;
}

.label.is-print {
    margin-top: 0;
}

.note-textarea {
    flex: 1 1 auto;
    width: 100%;
    min-width: 0;
    padding: 0.5rem;
    background-color: var(--bg);
    color: var(--accent-green);
}
</style>
