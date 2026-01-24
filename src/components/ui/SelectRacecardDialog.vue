<script setup lang="ts">
import { computed, ref, watch } from "vue";
import ModalDialog from "./ModalDialog.vue";

type RacecardRow = {
  id?: number | null;
  track: string;
  date: string;
};

const props = defineProps<{
  modelValue: boolean;
  racecards: RacecardRow[];
  selectedRacecardId?: number | null;
  titleColor?: string;
  buttonColor?: string;
  rowColor?: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "open", value: number | null): void;
}>();

const selectedId = ref<number | null>(props.selectedRacecardId ?? null);

watch(
  () => props.modelValue,
  (isOpen) => {
    if (isOpen) {
      selectedId.value = props.selectedRacecardId ?? null;
    }
  },
);

const canOpen = computed(() => selectedId.value !== null);

function close() {
  emit("update:modelValue", false);
}

function handleSelect(id: number | null) {
  selectedId.value = id;
}

function handleOpen() {
  if (!canOpen.value) {
    return;
  }
  emit("open", selectedId.value);
  emit("update:modelValue", false);
}
</script>

<template>
  <ModalDialog
    :model-value="modelValue"
    title="Select Racecard"
    :title-color="titleColor"
    @update:modelValue="emit('update:modelValue', $event)"
  >
    <div class="racecard-list">
      <div
        v-for="(racecard, idx) in racecards"
        :key="racecard.id ?? idx"
        class="racecard-row"
        :style="rowColor ? 'background-color: var(' + rowColor + ')' : undefined"
      >
        <button
          type="button"
          class="select-button"
          :class="{ selected: racecard.id === selectedId }"
          :style="buttonColor ? 'background-color: var(' + buttonColor + ')' : undefined"
          @click="handleSelect(racecard.id ?? null)"
        >
          {{ racecard.id === selectedId ? "Selected" : "Select" }}
        </button>
        <div class="racecard-text">
          <span class="track">{{ racecard.track }}</span>
          <span class="date">{{ racecard.date }}</span>
        </div>
      </div>
    </div>

    <template #actions>
      <button
        type="button"
        class="action-button"
        :style="buttonColor ? 'background-color: var(' + buttonColor + ')' : undefined"
        @click="close"
      >
        Cancel
      </button>
      <button
        type="button"
        class="action-button"
        :disabled="!canOpen"
        :style="buttonColor ? 'background-color: var(' + buttonColor + ')' : undefined"
        @click="handleOpen"
      >
        Open
      </button>
    </template>
  </ModalDialog>
</template>

<style scoped>
.racecard-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.racecard-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  background: var(--bg-2);
}

.select-button,
.action-button {
  border: none;
  background: var(--accent);
  color: var(--fg);
  padding: 0.35rem 0.75rem;
  border-radius: 0.4rem;
  cursor: pointer;
  font-weight: 600;
}

.select-button.selected {
  opacity: 0.9;
  box-shadow: inset 0 0 0 2px var(--fg);
}

.racecard-text {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.track {
  font-weight: 600;
}

.date {
  opacity: 0.8;
  font-size: 0.9rem;
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
