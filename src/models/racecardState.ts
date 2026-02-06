import { Racecards } from "./racecards";

export class RacecardState {
    racecards: Racecards;
    currentRacecardIdx: number;

    constructor() {
        this.racecards = new Racecards();
        this.currentRacecardIdx = 0;
    }
}
