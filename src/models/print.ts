import { Racecard } from './racecard';

export class RaceCardPrintPayload  {
    constructor(
        public raceCard: Racecard,
        public printRaces: number[] = []
    ) {}
}

