<script setup lang="ts">
import { computed, ref, watch } from "vue";
import ModalDialog from "./ModalDialog.vue";

type SortOption = {
  value: string;
  label: string;
  description?: string;
};

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    options: SortOption[];
    selected: string;
    titleColor?: string;
    cancelButtonColor?: string;
    applyButtonColor?: string;
    rowColor?: string;
  }>(),
  {
    titleColor: "--accent-yellow",
    cancelButtonColor: "--accent-red",
    applyButtonColor: "--accent-yellow",
    rowColor: "--accent-green",
  },
);

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
  (e: "select", value: string): void;
}>();

const selectedValue = ref(props.selected ?? "");

watch(
  () => props.modelValue,
  (isOpen) => {
    if (isOpen) {
      selectedValue.value = props.selected ?? "";
    }
  },
);

const canApply = computed(() => selectedValue.value.length > 0);

function close() {
  emit("update:modelValue", false);
}

function handleSelect(value: string) {
  selectedValue.value = value;
}

function handleApply() {
  if (!canApply.value) {
    return;
  }
  emit("select", selectedValue.value);
  emit("update:modelValue", false);
}
</script>

<template>
  <ModalDialog
    :model-value="modelValue"
    title="Sort Horses"
    :title-color="titleColor"
    @update:modelValue="emit('update:modelValue', $event)"
  >
    <div class="sort-list">
      <div
        v-for="(option, idx) in options"
        :key="option.value"
        class="sort-row"
        :style="rowColor ? 'background-color: var(' + rowColor + ')' : undefined"
        @click="handleSelect(option.value)"
      >
        <label class="select-option">
          <input
            type="radio"
            name="sort-method"
            class="select-radio"
            :value="option.value"
            :checked="option.value === selectedValue"
            :autofocus="idx === 0"
            @change="handleSelect(option.value)"
          />
        </label>
        <div class="sort-text">
          <div class="label">{{ option.label }}</div>
          <div v-if="option.description" class="description">{{ option.description }}</div>
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
        :disabled="!canApply"
        :style="applyButtonColor ? 'color: var(' + applyButtonColor + ')' : undefined"
        @click="handleApply"
      >
        Apply
      </button>
    </template>
  </ModalDialog>
</template>

<style scoped>
.sort-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  align-items: center;
}

.sort-row {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-start;
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  background: var(--bg-2);
  width: 100%;
  cursor: pointer;
}

.sort-text {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.15rem;
  flex: 1;
}

.label {
  text-align: left;
}

.description {
  font-size: 1.1rem;
  opacity: 0.8;
  text-align: left;
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
  width: 1.4rem;
  height: 1.4rem;
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
