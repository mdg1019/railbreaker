<script setup lang="ts">
import { computed, ref } from 'vue'
import type { Racecards } from '../models/racecards'
import RaceClassification from './RaceClassification.vue';

const props = defineProps<{
    racecards: Racecards
    currentRacecardIndex: number
    currentRace: number
    open: boolean
}>()

const emit = defineEmits<{
    (e: 'update:open', value: boolean): void
    (e: 'update:selectedRace', value: number): void
    (e: 'update:currentRacecardIndex', value: number): void
}>()

const racecardEntry = computed(() => props.racecards.racecardEntries[props.currentRacecardIndex])

const showDropdown = ref(false)

function toggleDropdown() {
    showDropdown.value = !showDropdown.value
}

function selectRacecardIndex(idx: number) {
    emit('update:currentRacecardIndex', idx)
    showDropdown.value = false
}

function toggle() {
    emit('update:open', !props.open)
}

function selectRace(raceNumber: number) {
    props.racecards.racecardEntries[props.currentRacecardIndex].last_opened_race = raceNumber;
    emit('update:selectedRace', raceNumber)
}
</script>

<template>
    <div class="racecard-menu" aria-label="Racecard menu">
        <aside class="panel" :class="{ open }" aria-label="Races">
            <div class="header">
                <div class="dropdown" @click.stop="toggleDropdown" tabindex="0" @blur="showDropdown = false">
                    <div class="dropdown-selected">
                        <div class="selected-text">
                            <div class="track">{{ racecardEntry?.racecard.track }}</div>
                            <div class="date">{{ racecardEntry?.racecard.date }}</div>
                        </div>
                        <div class="chev">{{ showDropdown ? '▴' : '▾' }}</div>
                    </div>

                    <ul v-if="showDropdown" class="dropdown-list">
                        <li v-for="(entry, idx) in racecards.racecardEntries" :key="entry.id"
                            :class="{ active: idx === currentRacecardIndex }" @click.stop="selectRacecardIndex(idx)">
                            <div class="entry-track">{{ entry.racecard.track }}</div>
                            <div class="entry-date">{{ entry.racecard.date }}</div>
                        </li>
                    </ul>
                </div>
            </div>

            <nav class="races" aria-label="Race list">
                <button class="race" type="button" v-for="(race, idx) in racecardEntry?.racecard.races"
                    :key="race.race_number ?? idx" :class="{ selected: (race.race_number ?? idx + 1) === currentRace }"
                    @click="selectRace(race.race_number ?? idx + 1)">
                    <div class="race-row">
                        <div class="race-left">
                            🐎 <span class="race-number">Race {{ race.race_number ?? idx + 1 }}:</span>
                        </div>
                        <div class="race-right">
                            <RaceClassification :race="race" />
                        </div>
                    </div>
                </button>
            </nav>
        </aside>

        <button class="tab" type="button" :class="{ open }" @click="toggle">
            {{ open ? 'Close' : 'Races' }}
        </button>
    </div>
</template>

<style scoped lang="scss">
.race-number {
    color: var(--accent-green-strong);
}

.racecard-menu {
    position: fixed;
    inset: 0;
    pointer-events: none;
    z-index: 900;
}

.panel {
    pointer-events: auto;
    position: fixed;
    top: 0;
    left: 0;
    height: 100vh;
    width: 280px;
    background: var(--bg);
    color: var(--fg);
    border-right: 1px solid var(--modal-border);
    transform: translateX(-100%);
    transition: transform 0.2s ease;
    display: flex;
    flex-direction: column;
}

.panel.open {
    transform: translateX(0);
}

.header {
    padding: 1rem;
    border-bottom: 1px solid var(--modal-border);
}

.track {
    color: var(--accent-green-strong);
    font-weight: 700;
}

.date {
    color: var(--accent-green);
    opacity: 0.85;
    margin-top: 0.25rem;
}

.races {
    display: grid;
    grid-template-columns: auto 1fr;
    grid-auto-rows: auto;
    gap: 0.25rem 0;
    padding: 0.5rem 0;
    overflow: auto;
}

.race {
    appearance: none;
    background: transparent;
    color: inherit;
    border: 0;
    width: 100%;
    text-align: left;
    padding: 0;
    cursor: pointer;
    display: contents;
}

.race-row {
    display: contents;
}

.race:hover .race-left,
.race:hover .race-right {
    background: var(--modal-action-hover-bg);
}

.race.selected .race-left,
.race.selected .race-right {
    background: var(--background-light);
    border: 1px solid var(--modal-border);
    box-shadow: 0 6px 10px rgba(0, 0, 0, 0.06);
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

.tab {
    pointer-events: auto;
    position: fixed;
    top: 1rem;
    left: 0;
    height: 40px;
    padding: 0 0.75rem;
    background: color-mix(in srgb, var(--modal-titlebar-bg) 82%, transparent);
    color: var(--accent-yellow);
    border: 1px solid var(--modal-border);
    border-left: none;
    border-radius: 0 10px 10px 0;
    transform: none;
    cursor: pointer;
    transition: left 0.2s ease, background 0.15s ease;
}

.tab.open {
    left: 280px;
}

.tab:hover {
    background: var(--modal-action-hover-bg);
}

.tab:focus-visible {
    outline: none;
    border-color: var(--ubuntu-blue);
}

.dropdown {
    display: block;
    width: 100%;
}

.dropdown-selected {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0.75rem;
    border-radius: 8px;
    cursor: pointer;
}

.dropdown-selected:hover {
    background: var(--modal-action-hover-bg);
}

.selected-text .track {
    font-weight: 700;
}

.chev {
    color: var(--accent-green-strong);
    margin-left: 0.5rem;
}

.dropdown-list {
    margin: 0.5rem 0 0 0;
    padding: 0.25rem 0;
    list-style: none;
    max-height: 220px;
    overflow: auto;
    border-radius: 8px;
    background: var(--bg-secondary, rgba(0, 0, 0, 0.03));
    border: 1px solid var(--modal-border);
}

.dropdown-list li {
    padding: 0.5rem 0.75rem;
    display: flex;
    justify-content: space-between;
    gap: 0.5rem;
    cursor: pointer;
}

.dropdown-list li:hover,
.dropdown-list li.active {
    background: var(--background-light);
}

.entry-track {
    font-weight: 600;
}

.entry-date {
    opacity: 0.85;
}
</style>