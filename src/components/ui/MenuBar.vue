<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { Racecard } from "../../models/racecard";
import type { Racecards } from "../../models/racecards";
import {
    HORSE_SORTING_METHOD_PROGRAM_NUMBER,
    HORSE_SORTING_METHOD_NAME,
    HORSE_SORTING_METHOD_MORNING_LINE,
    HORSE_SORTING_METHOD_CSPM,
    HORSE_SORTING_METHOD_TRIP,
} from "../../constants/horseSortingMethods";
import RaceClassification from "../racecard/RaceClassification.vue";

const props = defineProps<{
    racecards: Racecards;
    racecardLabel: string;
    currentRacecardIndex: number;
    racecard: Racecard | null;
    raceNumber: number;
    raceOptions: number;
    horseSortMethod: string;
}>();

const emit = defineEmits<{
    (e: "select-racecard", index: number): void;
    (e: "select-race", value: number): void;
    (e: "update:horseSortMethod", value: string): void;
}>();

const isRacecardDropdownOpen = ref(false);
const isRaceDropdownOpen = ref(false);
const racecardDropdownRef = ref<HTMLElement | null>(null);
const raceDropdownRef = ref<HTMLElement | null>(null);

const horseSortMethodModel = computed({
    get: () => props.horseSortMethod,
    set: (value: string) => {
        if (!value) {
            return;
        }
        emit("update:horseSortMethod", value);
    },
});

function toggleRacecardDropdown() {
    if (props.racecards.racecardEntries.length === 0) {
        return;
    }
    isRacecardDropdownOpen.value = !isRacecardDropdownOpen.value;
    if (isRacecardDropdownOpen.value) {
        isRaceDropdownOpen.value = false;
    }
}

function toggleRaceDropdown() {
    if (!props.racecard) {
        return;
    }
    isRaceDropdownOpen.value = !isRaceDropdownOpen.value;
    if (isRaceDropdownOpen.value) {
        isRacecardDropdownOpen.value = false;
    }
}

function handleSelectRacecard(index: number) {
    emit("select-racecard", index);
    isRacecardDropdownOpen.value = false;
}

function handleSelectRace(value: number) {
    emit("select-race", value);
    isRaceDropdownOpen.value = false;
}

function handleDropdownOutsideClick(event: MouseEvent) {
    const target = event.target as Node | null;
    if (!target) {
        return;
    }
    if (racecardDropdownRef.value && !racecardDropdownRef.value.contains(target)) {
        isRacecardDropdownOpen.value = false;
    }
    if (raceDropdownRef.value && !raceDropdownRef.value.contains(target)) {
        isRaceDropdownOpen.value = false;
    }
}

function handleDropdownKeydown(event: KeyboardEvent) {
    if (event.key !== "Escape") {
        return;
    }
    if (isRacecardDropdownOpen.value) {
        isRacecardDropdownOpen.value = false;
    }
    if (isRaceDropdownOpen.value) {
        isRaceDropdownOpen.value = false;
    }
}

onMounted(() => {
    window.addEventListener("click", handleDropdownOutsideClick, true);
    window.addEventListener("keydown", handleDropdownKeydown);
});

onUnmounted(() => {
    window.removeEventListener("click", handleDropdownOutsideClick, true);
    window.removeEventListener("keydown", handleDropdownKeydown);
});
</script>

<template>
    <div class="menubar">
        <div class="menu-group" ref="racecardDropdownRef">
            <span class="menu-label">Racecard</span>
            <button
                type="button"
                class="menu-trigger"
                :class="{ disabled: racecards.racecardEntries.length === 0 }"
                :disabled="racecards.racecardEntries.length === 0"
                :aria-expanded="isRacecardDropdownOpen"
                @click="toggleRacecardDropdown"
            >
                <span class="menu-trigger-text">{{ racecardLabel }}</span>
            </button>
            <div v-if="isRacecardDropdownOpen" class="menu-list" role="listbox">
                <button
                    v-for="(entry, idx) in racecards.racecardEntries"
                    :key="entry.racecard.id ?? entry.racecard.zip_file_name ?? idx"
                    type="button"
                    class="menu-item"
                    :class="{ active: idx === currentRacecardIndex }"
                    @click="handleSelectRacecard(idx)"
                >
                    {{ entry.racecard.track || entry.racecard.track_code || "Racecard" }} — {{ entry.racecard.long_date || entry.racecard.date || entry.racecard.zip_file_name }}
                </button>
            </div>
        </div>
        <div class="menu-group" ref="raceDropdownRef">
            <span class="menu-label">Race</span>
            <button
                type="button"
                class="menu-trigger"
                :class="{ disabled: !racecard }"
                :disabled="!racecard"
                :aria-expanded="isRaceDropdownOpen"
                @click="toggleRaceDropdown"
            >
                <span class="menu-trigger-text" v-if="racecard">
                    <span class="color-accent-yellow">Race {{ raceNumber }} - </span><RaceClassification class="color-accent-yellow" :race="racecard.races[raceNumber - 1]" />
                </span>
                <span class="menu-trigger-text" v-else>No racecard selected</span>
            </button>
            <div v-if="isRaceDropdownOpen" class="menu-list race-list" role="listbox">
                <button
                    v-for="n in raceOptions"
                    :key="n"
                    type="button"
                    class="menu-item race-item"
                    :class="{ active: n === raceNumber }"
                    @click="handleSelectRace(n)"
                >
                    <span class="race-item-left">Race {{ n }}</span>
                    <span class="race-item-right">
                        <RaceClassification :race="racecard!.races[n - 1]" />
                    </span>
                </button>
            </div>
        </div>
        <div class="menu-group sort-group">
            <span class="menu-label">Sort:</span>
            <div class="sort-options" role="radiogroup" aria-label="Sort horses">
                <label class="sort-option">
                    <input
                        v-model="horseSortMethodModel"
                        type="radio"
                        name="sort-method-inline"
                        :value="HORSE_SORTING_METHOD_PROGRAM_NUMBER"
                    />
                    <span class="sort-text">Program Number</span>
                </label>
                <label class="sort-option">
                    <input
                        v-model="horseSortMethodModel"
                        type="radio"
                        name="sort-method-inline"
                        :value="HORSE_SORTING_METHOD_NAME"
                    />
                    <span class="sort-text">Horse's Name</span>
                </label>
                <label class="sort-option">
                    <input
                        v-model="horseSortMethodModel"
                        type="radio"
                        name="sort-method-inline"
                        :value="HORSE_SORTING_METHOD_MORNING_LINE"
                    />
                    <span class="sort-text">Morning Line</span>
                </label>
                <label class="sort-option">
                    <input
                        v-model="horseSortMethodModel"
                        type="radio"
                        name="sort-method-inline"
                        :value="HORSE_SORTING_METHOD_CSPM"
                    />
                    <span class="sort-text">CSPM Score</span>
                </label>
                <label class="sort-option">
                    <input
                        v-model="horseSortMethodModel"
                        type="radio"
                        name="sort-method-inline"
                        :value="HORSE_SORTING_METHOD_TRIP"
                    />
                    <span class="sort-text">Trip Score</span>
                </label>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.menubar {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.75rem 1rem;
    border: 2px solid var(--accent-green);
    border-radius: 0.75rem;
    background: linear-gradient(135deg, rgba(11, 11, 11, 0.92), rgba(24, 24, 24, 0.92));
    font-family: "MGSans", sans-serif;
    font-size: 1.05rem;
}

.menu-group {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
}

.sort-group {
    gap: 0.4rem;
}

.sort-options {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0.35rem 0.5rem;
    border-radius: 0.5rem;
    border: 2px solid var(--accent-green);
    background: #000;
}

.sort-option {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.95rem;
    color: var(--accent-yellow);
    cursor: pointer;
    user-select: none;
}

.sort-option input {
    width: 1rem;
    height: 1rem;
    accent-color: var(--accent-green);
    cursor: pointer;
}

.sort-text {
    white-space: nowrap;
}

.menu-label {
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-yellow);
    white-space: nowrap;
    font-family: "MGSans", sans-serif;
}

.menu-trigger {
    position: relative;
    min-width: 220px;
    max-width: 420px;
    padding: 0.45rem 2rem 0.45rem 0.65rem;
    border-radius: 0.5rem;
    border: 2px solid var(--accent-green);
    background-color: #000;
    color: #fff;
    font-size: 1rem;
    font-family: "MGSans", sans-serif;
    text-align: left;
    cursor: pointer;
}

.menu-trigger::after {
    content: "";
    position: absolute;
    right: 0.6rem;
    top: 50%;
    width: 1rem;
    height: 1rem;
    transform: translateY(-50%);
    background-color: var(--accent-green);
    mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M5.5 7.5 10 12l4.5-4.5'/%3E%3C/svg%3E") no-repeat center / contain;
    -webkit-mask: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3E%3Cpath d='M5.5 7.5 10 12l4.5-4.5'/%3E%3C/svg%3E") no-repeat center / contain;
}

.menu-trigger:disabled,
.menu-trigger.disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.menu-trigger-text {
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--accent-yellow);
}

.menu-list {
    position: absolute;
    top: calc(100% + 0.4rem);
    left: 0;
    z-index: 20;
    min-width: 100%;
    max-width: 520px;
    max-height: none;
    height: auto;
    overflow: visible;
    background: #000;
    border: 2px solid var(--accent-green);
    border-radius: 0.5rem;
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.45);
    padding: 0.25rem 0;
    box-sizing: border-box;
}

.menu-item {
    width: 100%;
    text-align: left;
    padding: 0.45rem 0.65rem;
    border: none;
    background: transparent;
    color: var(--accent-yellow);
    font-size: 1rem;
    font-family: "MGSans", sans-serif;
    cursor: pointer;
}

.menu-item:hover,
.menu-item:focus {
    background: rgba(74, 222, 128, 0.18);
    outline: none;
}

.menu-item.active {
    background: rgba(74, 222, 128, 0.3);
}

.race-item {
    display: grid;
    grid-template-columns: 8ch 1fr;
    align-items: center;
    column-gap: 0.5rem;
}

.race-item-left {
    text-align: left;
    justify-self: start;
    font-variant-numeric: tabular-nums;
}

.race-item-right {
    text-align: left;
    min-width: 0;
}
</style>
