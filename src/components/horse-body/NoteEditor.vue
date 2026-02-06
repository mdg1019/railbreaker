<script setup lang="ts">
import type { Horse } from "../../models/racecard";
import { useRacecardStateStore } from "../../stores/racecardStateStore";
const props = withDefaults(defineProps<{
    horse: Horse,
    print: boolean;
}>(), {
    print: false,
});

const racecardStateStore = useRacecardStateStore();

function handleInput(event: Event) {
  racecardStateStore.setNote((event.target as HTMLTextAreaElement).value, props.horse.id!);
}
</script>

<template>
    <div v-if="!props.print || props.horse.note !== ''" class="container font-small">
        <div class="label" :class="{ 'is-print': props.print }">Notes:</div>
        <textarea
            v-if="!props.print"
            class="note-textarea font-small"
            rows="5"
            cols="40"
            :value="props.horse.note"
            :readonly="props.print"
            @input="handleInput"
        ></textarea>
        <div v-else class="note-content">
            {{ props.horse.note }}
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

.note-content {
    white-space: pre-wrap;
}
</style>
