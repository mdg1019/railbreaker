import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { RacecardState } from "../models/racecardState";
import { Racecard, Race } from "../models/racecard";
import { RacecardEntry, Racecards } from "../models/racecards";
import { RaceMeta } from "../models/analysis";

function isRacecardIdxValid(idx: number, racecardState: RacecardState): boolean {
    return (
        idx >= 0 && idx < racecardState.racecards.racecardEntries.length
    );
}

let saveNoteTimeouts = new Map<number, ReturnType<typeof setTimeout>>();
let raceMetaRequestId = 0;

export type TripInfo = {
    scratched: boolean;
    program_number?: string;
    horse_name?: string;
    score?: number;
    days_back_1?: number;
    comment_1?: string;
    days_back_2?: number;
    comment_2?: string;
    days_back_3?: number;
    comment_3?: string;
};

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
        raceMeta: null as RaceMeta | null,
        tripData: [] as TripInfo[],
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
        getRaceMeta(): RaceMeta | null {
            return this.raceMeta;
        },
        getTripData(): TripInfo[] {
            return this.tripData;
        },
    },
    actions: {
        updateTripData(): void {
            const currentRacecard = this.getCurrentRacecard;
            if (!currentRacecard) {
                this.tripData = [];
                return;
            }

            const raceIdx = this.currentRaceNumber - 1;
            const race = currentRacecard.races?.[raceIdx];
            if (!race) {
                this.tripData = [];
                return;
            }

            const trips: Array<TripInfo> = [];
            for (const horse of race.horses || []) {
                if (!horse.trip_handicapping_info) continue;
                const cols = horse.trip_handicapping_info.split(",");
                const tripInfo: TripInfo = {
                    scratched: horse.scratched,
                    program_number: horse.program_number,
                    horse_name: horse.horse_name,
                    score: Number(cols[0]),
                    days_back_1: Number(cols[1]),
                    comment_1: cols[2],
                    days_back_2: Number(cols[3]),
                    comment_2: cols[4],
                    days_back_3: Number(cols[5]),
                    comment_3: cols[6],
                };

                console.log(`Parsed trip info for horse ${horse.horse_name}:`, tripInfo);
                trips.push(tripInfo);
            }

            this.tripData = trips
                .sort((a, b) => {
                    const scoreA = Number.isFinite(a.score) ? (a.score as number) : Number.NEGATIVE_INFINITY;
                    const scoreB = Number.isFinite(b.score) ? (b.score as number) : Number.NEGATIVE_INFINITY;
                    return scoreB - scoreA;
                })
                .map((item) => item);
        },
        async updateRaceMeta(): Promise<void> {
            const currentRacecard = this.getCurrentRacecard;
            if (!currentRacecard) {
                this.raceMeta = null;
                return;
            }

            const raceIdx = this.currentRaceNumber - 1;
            const race = currentRacecard.races?.[raceIdx];
            if (!race) {
                this.raceMeta = null;
                return;
            }

            const requestId = ++raceMetaRequestId;
            try {
                const result = await invoke<any>("rank_race", {
                    race: Race.fromObject(race).toObject(),
                    racecardDate: currentRacecard.date ?? null,
                });
                if (requestId === raceMetaRequestId) {
                    this.raceMeta = RaceMeta.fromObject(result);
                }
            } catch (err) {
                console.error("Failed to rank race", err);
                if (requestId === raceMetaRequestId) {
                    this.raceMeta = null;
                }
            }
        },
        setNextRaceNumber(): void {
            const currentRacecard = this.getCurrentRacecard;
            if (!currentRacecard) {
                return;
            }

            const raceCount = currentRacecard.races?.length ?? 0;
            if (raceCount === 0) {
                return;
            }

            const nextRace = this.currentRaceNumber < raceCount ? this.currentRaceNumber + 1 : 1;
            this.setLastOpenedRace(nextRace);
        },   
        setPrevRaceNumber(): void {
            const currentRacecard = this.getCurrentRacecard;
            if (!currentRacecard) {
                return;
            }

            const raceCount = currentRacecard.races?.length ?? 0;
            if (raceCount === 0) {
                return;
            }

            const prevRace = this.currentRaceNumber > 1 ? this.currentRaceNumber - 1 : raceCount;
            this.setLastOpenedRace(prevRace);
        }, 
        setLastOpenedRace(raceNumber: number): void {
            this.currentRaceNumber = raceNumber;
            if (!isRacecardIdxValid(this.racecardState.currentRacecardIdx, this.racecardState)) {
                return;
            }
            
            this.racecardState.racecards.racecardEntries[this.racecardState.currentRacecardIdx].last_opened_race = raceNumber;
            void this.updateRaceMeta();
            this.updateTripData();
        },
        setCurrentRacecardIdx(idx: number): void {
            const entries = this.racecardState.racecards.racecardEntries;
            if (entries.length === 0) {
                this.racecardState.currentRacecardIdx = 0;
                this.currentRaceNumber = 1;
                this.raceMeta = null;
                this.tripData = [];
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
            void this.updateRaceMeta();
            this.updateTripData();
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

            void this.updateRaceMeta();
            this.updateTripData();
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
