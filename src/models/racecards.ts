
import { Racecard } from './racecard';

export class Racecards {
    racecardEntries: RacecardEntry[];

  constructor() {
    this.racecardEntries = [];
  }
}
export class RacecardEntry {
    racecard: Racecard;
    last_opened_race: number;
    constructor(racecard: Racecard) {
        this.racecard = racecard;
        this.last_opened_race = 0;
    }
}