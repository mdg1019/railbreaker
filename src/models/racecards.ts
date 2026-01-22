import { Note } from './note';
import { Racecard } from './racecard';

export class Racecards {
    racecardEntries: RacecardEntry[];
    racecardPaths: string[] = [];


  constructor() {
    this.racecardEntries = [];
    this.racecardPaths = [];
  }

  addRacecard(racecard: Racecard, notes: Note[], path: string): void {
   this.racecardEntries.push(new RacecardEntry(racecard, notes));
   this.racecardPaths.push(path);
  }
  
  deleteRacecardAt(index: number): void {
    if (index < 0 || index >= this.racecardEntries.length) {
      return;
    }

    this.racecardEntries.splice(index, 1);
    this.racecardPaths.splice(index, 1);
  }
}

export class RacecardEntry {
    id: string;
    racecard: Racecard;
    last_opened_race: number;
    notes: Note[];

    constructor(racecard: Racecard, notes: Note[]) {
        this.id = crypto.randomUUID();
        this.racecard = racecard;
        this.last_opened_race = 0;
        this.notes = notes;
    }
}