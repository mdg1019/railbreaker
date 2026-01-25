import { CardAnalysis } from './analysis';
import { Racecard } from './racecard';

export class Racecards {
    racecardEntries: RacecardEntry[];


  constructor() {
    this.racecardEntries = [];
  }

  addRacecard(racecard: Racecard, analysis?: CardAnalysis): void {
   this.racecardEntries.push(new RacecardEntry(racecard, analysis));
  }
  
  deleteRacecardAt(index: number): void {
    if (index < 0 || index >= this.racecardEntries.length) {
      return;
    }

    this.racecardEntries.splice(index, 1);
  }
}
export class RacecardEntry {
    racecard: Racecard;
    last_opened_race: number;
    analysis: CardAnalysis;

    constructor(racecard: Racecard, analysis?: CardAnalysis) {
        this.racecard = racecard;
        this.last_opened_race = 0;
        this.analysis = analysis ?? new CardAnalysis();
    }
}