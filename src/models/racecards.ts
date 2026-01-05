import { Racecard } from './racecard';

export class Racecards {
    racecardEntries: RacecardEntry[];


  constructor() {
    this.racecardEntries = [];
  }

  addRacecard(racecard: Racecard): void {
    this.racecardEntries.push(new RacecardEntry(racecard));
  }
  
  deleteRacecardAt(index: number): void {
    if (index < 0 || index >= this.racecardEntries.length) {
      return;
    }

    this.racecardEntries.splice(index, 1);
  }
}

export class RacecardEntry {
    id: string;
    racecard: Racecard;
    last_opened_race: number;

    constructor(racecard: Racecard) {
        this.id = crypto.randomUUID();
        this.racecard = racecard;
        this.last_opened_race = 0;
    }
}