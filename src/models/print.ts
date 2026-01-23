import { Note } from './note';
import { Racecard } from './racecard';

export class RaceCardPrintPayload  {
    constructor(
        public raceCard: Racecard,
        public notes: Array<Note> = [],
        public printRaces: number[] = []
    ) {}
}

