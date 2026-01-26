import { CardAnalysis, HorseRank, RaceRankResult } from "../models/analysis";

export type Confidence = "StrongSingle" | "Playable" | "Competitive" | "WideOpen" | "Unscorable";

export type Shape = 'Slow' | 'Honest' | 'Fast' | 'Meltdown';

export interface WinBetSuggestion {
  program_number: string;
  horse_name: string;
  min_odds?: number | null; // optional tote gate later
  reason: string;
}

export interface RaceMeta {
  race_number?: number | null;
  shape: Shape;
  epi: number;
  top_score: number | null;
  second_score: number | null;
  gap_1_2: number | null;
  spread_top_to_4: number | null;
  confidence: Confidence;
  win_bet: WinBetSuggestion | null;
}


function sortedScores(race: RaceRankResult): number[] {
  return race.horses
    .map(h => h.score)
    .filter((s): s is number => typeof s === "number" && Number.isFinite(s))
    .sort((a, b) => b - a);
}

function topTwoScores(race: RaceRankResult): { top: number | null; second: number | null } {
  const scores = sortedScores(race);
  return {
    top: scores.length >= 1 ? scores[0] : null,
    second: scores.length >= 2 ? scores[1] : null,
  };
}

export function topNHorsesByScore(race: RaceRankResult, n: number): HorseRank[] {
  return [...race.horses]
    .filter(h => typeof h.score === "number" && Number.isFinite(h.score))
    .sort((a, b) => (b.score! - a.score!))
    .slice(0, n);
}

export function classifyRace(race: RaceRankResult): Confidence {
  const { top, second } = topTwoScores(race);
  if (top === null || second === null) return "Unscorable";

  const top4 = sortedScores(race).slice(0, 4);
  const gap = top - second;
  const spread = top4.length >= 4 ? (top4[0] - top4[3]) : null;

  if (gap >= 2.0 && (spread ?? 0) >= 3.0) return "StrongSingle";
  if (gap >= 1.5) return "Playable";
  if (gap >= 0.5 || (spread !== null && spread >= 1.5)) return "Competitive";
  return "WideOpen";
}

export function winBetSuggestion(
  race: RaceRankResult,
  opts: { minTopScore?: number; minGap?: number } = {}
): WinBetSuggestion | null {
  const minTopScore = opts.minTopScore ?? 2.0;
  const minGap = opts.minGap ?? 1.5;

  const top2 = topNHorsesByScore(race, 2);
  if (top2.length < 2) return null;

  const top = top2[0];
  const second = top2[1];

  const topScore = top.score!;
  const gap = top.score! - second.score!;

  if (topScore >= minTopScore && gap >= minGap) {
    return {
      program_number: top.programNumber,
      horse_name: top.horseName,
      min_odds: null,
      reason: `Top score ${topScore.toFixed(2)} with strong separation (gap ${gap.toFixed(2)}).`,
    };
  }

  return null;
}

export function deriveRaceMeta(race: RaceRankResult): RaceMeta {
  const { top, second } = topTwoScores(race);
  const top4 = sortedScores(race).slice(0, 4);

  const gap_1_2 = top !== null && second !== null ? (top - second) : null;
  const spread_top_to_4 = top4.length >= 4 ? (top4[0] - top4[3]) : null;

  const confidence = classifyRace(race);
  const win_bet = winBetSuggestion(race);

  return {
    race_number: race.raceNumber ?? null,
    shape: race.shape,
    epi: race.epi,
    top_score: top,
    second_score: second,
    gap_1_2,
    spread_top_to_4,
    confidence,
    win_bet,
  };
}

export function deriveCardMeta(card: CardAnalysis): RaceMeta[] {
  return card.races.map(deriveRaceMeta);
}