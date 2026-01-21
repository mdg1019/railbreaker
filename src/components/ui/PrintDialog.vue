<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import ModalDialog from './ModalDialog.vue'
import RaceClassification from '../racecard/RaceClassification.vue'
import type { Race, Racecard } from '../../models/racecard'

const props = defineProps<{
    modelValue: boolean
    racecard: Racecard | null
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void
    (e: 'print', value: number[]): void
}>()

const selectedRaces = ref<number[]>([])

const races = computed(() => props.racecard?.races ?? [])
const title = computed(() => {
    if (!props.racecard) return 'Print'
    return `Print - ${props.racecard.track} - ${props.racecard.date}`
})

function raceNumberFor(race: Race, idx: number) {
    return race.raceNumber ?? idx + 1
}

function close() {
    emit('update:modelValue', false)
}

function applyDefaults() {
    if (!props.racecard) {
        selectedRaces.value = []
        return
    }
    selectedRaces.value = races.value.map((race, idx) => raceNumberFor(race, idx))
}

function toggleRace(raceNumber: number, checked: boolean) {
    if (checked) {
        if (!selectedRaces.value.includes(raceNumber)) {
            selectedRaces.value = [...selectedRaces.value, raceNumber].sort((a, b) => a - b)
        }
        return
    }
    selectedRaces.value = selectedRaces.value.filter((value) => value !== raceNumber)
}

function confirmPrint() {
    emit('print', [...selectedRaces.value])
    close()
}

watch(
    [() => props.modelValue, () => props.racecard],
    ([isOpen]) => {
        if (isOpen) applyDefaults()
    },
    { immediate: true },
)
</script>

<template>
    <ModalDialog :model-value="modelValue" :title="title" :titleColor="'--accent-green'" @update:modelValue="close">
        <div class="race-list" role="list">
            <label v-for="(race, idx) in races" :key="race.raceNumber ?? idx" class="race-row" role="listitem">
                <input class="race-checkbox" type="checkbox" :checked="selectedRaces.includes(raceNumberFor(race, idx))"
                    @change="toggleRace(raceNumberFor(race, idx), ($event.target as HTMLInputElement).checked)" />
                <div class="race-left">
                    🐎 <span class="race-number">Race {{ raceNumberFor(race, idx) }}:</span>
                </div>
                <div class="race-right">
                    <RaceClassification :race="race" />
                </div>
            </label>
        </div>

        <template #actions>
            <button type="button" @click="close" :style="{ color: 'var(--accent-red)' }">Cancel</button>
            <button
                type="button"
                @click="confirmPrint"
                :disabled="selectedRaces.length === 0"
                :style="selectedRaces.length !== 0 ? { color: 'var(--accent-green)' } : undefined"
            >
                Print
            </button>
        </template>
    </ModalDialog>
</template>

<style scoped lang="scss">
.race-list {
    display: grid;
    grid-template-columns: auto;
    gap: 0.25rem 0;
    justify-content: center;
}

.race-row {
    display: flex;
    cursor: pointer;
    justify-content: flex-start;
}

.race-checkbox {
    align-self: center;
    margin-left: 0.25rem;
}

.race-left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    border-radius: 8px 0 0 8px;
}

.race-right {
    display: block;
    min-width: 0;
    padding: 0.5rem 0.75rem;
    border-radius: 0 8px 8px 0;
    text-align: left;
    overflow: hidden;
}

.race-right>* {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.race-row:hover .race-left,
.race-row:hover .race-right {
    background: var(--modal-action-hover-bg);
}

.race-number {
    color: var(--accent-green);
}

</style>
