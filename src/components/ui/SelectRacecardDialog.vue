<script setup lang="ts">
import { computed, ref, watch } from "vue";
import ModalDialog from "./ModalDialog.vue";

type RacecardRow = {
  id?: number | null;
  track: string;
  date: string;
};

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    racecards: RacecardRow[];
    selectedRacecardId?: number | null;
    titleColor?: string;
    cancelButtonColor?: string;
    openButtonColor?: string;
    rowColor?: string;
  }>(),
  {
    titleColor: "--accent-yellow",
    cancelButtonColor: "--accent-red",
    openButtonColor: "--accent-yellow",
    rowColor: "--accent-green",
  },
);

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
        <label class="select-option">
          <input
            type="radio"
            name="racecard-select"
            class="select-radio"
            :value="racecard.id ?? null"
            :checked="racecard.id === selectedId"
            @change="handleSelect(racecard.id ?? null)"
          />
        </label>
        <div class="racecard-text">
          <div class="track">{{ racecard.track }}</div>
          <div class="date">{{ racecard.date }}</div>
        </div>
      </div>
    </div>

    <template #actions>
      <button
        type="button"
        class="action-button"
        :style="cancelButtonColor ? 'color: var(' + cancelButtonColor + ')' : undefined"
        @click="close"
      >
        Cancel
      </button>
      <button
        type="button"
        class="action-button"
        :disabled="!canOpen"
        :style="openButtonColor ? 'color: var(' + openButtonColor + ')' : undefined"
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
  align-items: center;
}

.racecard-row {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-start;
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  background: var(--bg-2);
  width: 100%;
}

.racecard-text {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  flex: 1;
}

.track {
  text-align: left;
}

.date {
  text-align: right;
}

.action-button {
  border: none;
  background: var(--accent);
  color: var(--fg);
  padding: 0.35rem 0.75rem;
  border-radius: 0.4rem;
  cursor: pointer;
  font-weight: 600;
}

.select-option {
  display: inline-flex;
  align-items: center;
}

.select-radio {
  width: 1.1rem;
  height: 1.1rem;
  accent-color: var(--fg);
  cursor: pointer;
}

.select-radio:checked {
  accent-color: var(--fg);
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
