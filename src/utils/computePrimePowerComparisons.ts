import type { Racecard } from "../models/racecard";

type PrimePowerComparison = [number | string, string, string];

function ordinal(n: number): string {
    const s = ["th", "st", "nd", "rd"];
    const v = n % 100;
    return n + (s[(v - 20) % 10] || s[v] || s[0]);
}

export function computePrimePowerComparisons(
    racecard: Racecard | null,
    race_number: number
): PrimePowerComparison[] {
    const result: PrimePowerComparison[] = [];
    if (!racecard) {
        return result;
    }

    const raceIdx = race_number - 1;
    const race = racecard.races?.[raceIdx];
    if (!race || !Array.isArray(race.horses)) {
        return result;
    }

    const entries: { post: number | string; rating: number }[] = [];
    race.horses.forEach((h: any, idx: number) => {
        const r = h.bris_prime_power_rating;
        if (r === null || r === undefined) return;
        if (Number(r) === 0) return;
        const post = h.post_position ?? h.program_number ?? idx + 1;
        entries.push({ post, rating: Number(r) });
    });

    if (entries.length === 0) {
        return result;
    }

    entries.sort((a, b) => b.rating - a.rating);

    const N = entries.length;
    const tier = Math.ceil(N / 3);

    entries.forEach((e, i) => {
        const position = i + 1;
        let color = "var(--accent-yellow)";
        if (position <= tier) {
            color = "var(--accent-green)";
        } else if (position > 2 * tier) {
            color = "var(--accent-red)";
        }

        const tuple: PrimePowerComparison = [e.post, ordinal(position), color];
        result.push(tuple);
    });

    return result;
}
