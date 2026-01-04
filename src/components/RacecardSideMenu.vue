<script setup lang="ts">
import { computed } from 'vue'
import type { Racecard } from '../models/racecard'

const props = defineProps<{
  racecard: Racecard
  open: boolean
  selectedRace: number
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'update:selectedRace', value: number): void
}>()

const races = computed(() => props.racecard.races ?? [])

function toggle() {
  emit('update:open', !props.open)
}

function selectRace(raceNumber: number) {
  emit('update:selectedRace', raceNumber)
}
</script>

<template>
  <div class="racecard-menu" aria-label="Racecard menu">
    <aside class="panel" :class="{ open }" aria-label="Races">
      <div class="header">
        <div class="track">{{ racecard.track }}</div>
        <div class="date">{{ racecard.date }}</div>
      </div>

      <nav class="races" aria-label="Race list">
        <button
          class="race"
          type="button"
          v-for="(race, idx) in races"
          :key="race.race_number ?? idx"
          :class="{ selected: (race.race_number ?? idx + 1) === selectedRace }"
          @click="selectRace(race.race_number ?? idx + 1)"
        >
          🐎 Race {{ race.race_number ?? idx + 1 }}
        </button>
      </nav>
    </aside>

    <button class="tab" type="button" :class="{ open }" @click="toggle">
      {{ open ? 'Close' : 'Races' }}
    </button>
  </div>
</template>

<style scoped lang="scss">
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
  font-weight: 700;
}

.date {
  opacity: 0.85;
  margin-top: 0.25rem;
}

.races {
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
  padding: 0.5rem 1rem;
  cursor: pointer;
}

.race:hover {
  background: var(--modal-action-hover-bg);
}

.race.selected {
  font-weight: 700;
}

.tab {
  pointer-events: auto;
  position: fixed;
  top: 50%;
  left: 0;
  height: 40px;
  padding: 0 0.75rem;
  background: color-mix(in srgb, var(--modal-titlebar-bg) 82%, transparent);
  color: var(--modal-fg);
  border: 1px solid var(--modal-border);
  border-left: none;
  border-radius: 0 10px 10px 0;
  transform: translateY(-50%);
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
</style>
