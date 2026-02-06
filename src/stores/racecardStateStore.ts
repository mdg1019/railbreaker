import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { RacecardState } from "../models/racecardState";
import { Racecard } from "../models/racecard";
import { RacecardEntry, Racecards } from "../models/racecards";

function isRacecardIdxValid(idx: number, racecardState: RacecardState): boolean {
    return (
        idx >= 0 && idx < racecardState.racecards.racecardEntries.length
    );
}

let saveNoteTimeouts = new Map<number, ReturnType<typeof setTimeout>>();

function updateHorseNote(racecard: Racecard, horseId: number, note: string): boolean {
    for (const race of racecard.races ?? []) {
        const horse = race.horses?.find(h => h.id === horseId);
        if (horse) {
            horse.note = note;
            return true;
        }
    }
    return false;
}

function updateHorseScratch(racecard: Racecard, horseId: number, scratched: boolean): boolean {
    for (const race of racecard.races ?? []) {
        const horse = race.horses?.find(h => h.id === horseId);
        if (horse) {
            horse.scratched = scratched;
            return true;
        }
    }
    return false;
}

export const useRacecardStateStore = defineStore("RacecardState", {
    state: () => ({
        racecardState: new RacecardState(),
        currentRaceNumber: 1,
    }),
    getters: {
        getCurrentRacecardIdx(): number {
            return this.racecardState.currentRacecardIdx;
        },
        getCurrentRacecard(): Racecard | null {
            if (isRacecardIdxValid(this.racecardState.currentRacecardIdx, this.racecardState)) {
                return this.racecardState.racecards.racecardEntries[this.racecardState.currentRacecardIdx].racecard;
            }
            return null;
        },
        getRacecards(): Racecards {
            return this.racecardState.racecards;
        },
        getCurrentRaceNumber(): number {
            return this.currentRaceNumber;
        },
    },
    actions: {
        setLastOpenedRace(raceNumber: number): void {
            this.currentRaceNumber = raceNumber;
            if (!isRacecardIdxValid(this.racecardState.currentRacecardIdx, this.racecardState)) {
                return;
            }
            this.racecardState.racecards.racecardEntries[this.racecardState.currentRacecardIdx].last_opened_race = raceNumber;
        },
        setCurrentRacecardIdx(idx: number): void {
            const entries = this.racecardState.racecards.racecardEntries;
            if (entries.length === 0) {
                this.racecardState.currentRacecardIdx = 0;
                this.currentRaceNumber = 1;
                return;
            }

            if (!isRacecardIdxValid(idx, this.racecardState)) {
                return;
            }

            if (isRacecardIdxValid(this.racecardState.currentRacecardIdx, this.racecardState)) {
                entries[this.racecardState.currentRacecardIdx].last_opened_race = this.currentRaceNumber;
            }

            this.racecardState.currentRacecardIdx = idx;
            const entry = entries[idx];
            this.currentRaceNumber = entry.last_opened_race > 0 ? entry.last_opened_race : 1;
        },
        addRacecard(racecard: Racecard): void {
            this.racecardState.racecards.racecardEntries.push(
                new RacecardEntry(racecard),
            );
        },

        setNote(note: string, horseId: number): void {
            const entry = this.racecardState.racecards.racecardEntries[this.racecardState.currentRacecardIdx];
            if (!entry) {
                return;
            }

            updateHorseNote(entry.racecard, horseId, note);

            const currentRacecard = this.getCurrentRacecard;
            if (currentRacecard && currentRacecard !== entry.racecard) {
                updateHorseNote(currentRacecard, horseId, note);
            }

            const existingTimeout = saveNoteTimeouts.get(horseId);
            if (existingTimeout) {
                clearTimeout(existingTimeout);
            }

            const timeout = setTimeout(async () => {
                await invoke("update_note", { note: note, horseId: horseId }).catch((err) => {
                    console.error("Failed to update note", err);
                });
                saveNoteTimeouts.delete(horseId);
            }, 500);

            saveNoteTimeouts.set(horseId, timeout);
        },

        setScratch(horseId: number, scratched: boolean): void {
            const entry = this.racecardState.racecards.racecardEntries[this.racecardState.currentRacecardIdx];
            if (!entry) {
                return;
            }

            updateHorseScratch(entry.racecard, horseId, scratched);

            const currentRacecard = this.getCurrentRacecard;
            if (currentRacecard && currentRacecard !== entry.racecard) {
                updateHorseScratch(currentRacecard, horseId, scratched);
            }

            invoke("set_scratch", { horseId: horseId, scratched: scratched }).catch((err) => {
                console.error("Failed to update scratch status", err);
            });
        },

        deleteRacecardAt(index: number): void {
            if (
                index < 0 ||
                index >= this.racecardState.racecards.racecardEntries.length
            ) {
                return;
            }

            this.racecardState.racecards.racecardEntries.splice(index, 1);
        },
    },
});
