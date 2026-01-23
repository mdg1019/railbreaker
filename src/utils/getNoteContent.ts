import type { Note } from "../models/note";

export function getNoteContent(
    notes: Array<Note>,
    raceIndex: number,
    horseIndex: number,
): string {
    return notes.find((n) => n.race === raceIndex + 1 && n.horse === horseIndex + 1)?.content || "";
}
