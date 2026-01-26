export type SurfaceMode = 'Dirt' | 'Turf';
export type Shape = 'Slow' | 'Honest' | 'Fast' | 'Meltdown';
export type RunStyle = 'E' | 'EP' | 'P' | 'S' | 'Unk';

export class RepFigs {
  rep_speed: number | null;
  rep_early: number | null;
  rep_late: number | null;

  constructor(data: any = {}) {
    this.rep_speed = data.rep_speed ?? null;
    this.rep_early = data.rep_early ?? null;
    this.rep_late = data.rep_late ?? null;
  }

  static fromObject(obj: any): RepFigs {
    return new RepFigs(obj);
  }

  toObject(): any {
    return {
      rep_speed: this.rep_speed,
      rep_early: this.rep_early,
      rep_late: this.rep_late,
    };
  }
}

export class WorkoutSig {
  recent_works: number;
  top_rank_works: number;
  score: number;

  constructor(data: any = {}) {
    this.recent_works = data.recent_works ?? 0;
    this.top_rank_works = data.top_rank_works ?? 0;
    this.score = data.score ?? 0;
  }

  static fromObject(obj: any): WorkoutSig {
    return new WorkoutSig(obj);
  }

  toObject(): any {
    return {
      recent_works: this.recent_works,
      top_rank_works: this.top_rank_works,
      score: this.score,
    };
  }
}

export class HorseRank {
  program_number: string;
  horse_name: string;
  post_position: number | null;
  run_style: RunStyle;
  quirin: number | null;
  shape: Shape;
  score: number | null;
  rep: RepFigs;
  workout: WorkoutSig;

  constructor(data: any = {}) {
    this.program_number = data.program_number ?? '';
    this.horse_name = data.horse_name ?? '';
    this.post_position = data.post_position ?? null;
    this.run_style = data.run_style ?? 'Unk';
    this.quirin = data.quirin ?? null;
    this.shape = data.shape ?? 'Honest';
    this.score = data.score ?? null;
    this.rep = data.rep instanceof RepFigs ? data.rep : RepFigs.fromObject(data.rep ?? {});
    this.workout = data.workout instanceof WorkoutSig ? data.workout : WorkoutSig.fromObject(data.workout ?? {});
  }

  static fromObject(obj: any): HorseRank {
    const repObj = obj.rep ?? {
      rep_speed: obj.rep_speed ?? null,
      rep_early: obj.rep_early ?? null,
      rep_late: obj.rep_late ?? null,
    };

    const workoutObj = obj.workout ?? {
      recent_works: obj.recent_works ?? 0,
      top_rank_works: obj.top_rank_works ?? 0,
      score: 0,
    };

    return new HorseRank({
      program_number: obj.program_number ?? '',
      horse_name: obj.horse_name ?? '',
      post_position: obj.post_position ?? null,
      run_style: obj.run_style ?? 'Unk',
      quirin: obj.quirin ?? null,
      shape: obj.shape ?? 'Honest',
      score: obj.score ?? null,
      rep: RepFigs.fromObject(repObj),
      workout: WorkoutSig.fromObject(workoutObj),
    });
  }

  toObject(): any {
    return {
      program_number: this.program_number,
      horse_name: this.horse_name,
      post_position: this.post_position,
      run_style: this.run_style,
      quirin: this.quirin,
      shape: this.shape,
      score: this.score,
      rep: this.rep.toObject(),
      workout: this.workout.toObject(),
    };
  }
}

export class RaceRankResult {
  race_number: number | null;
  surface_mode: SurfaceMode;
  distance_f: number;
  shape: Shape;
  pace_heat: number;
  epi: number;
  horses: HorseRank[];

  constructor(data: any = {}) {
    this.race_number = data.race_number ?? null;
    this.surface_mode = data.surface_mode ?? 'Dirt';
    this.distance_f = data.distance_f ?? 0;
    this.shape = data.shape ?? 'Honest';
    this.pace_heat = data.pace_heat ?? 0;
    this.epi = data.epi ?? 0;
    this.horses = (data.horses ?? []).map((h: any) =>
      h instanceof HorseRank ? h : HorseRank.fromObject(h)
    );
  }

  static fromObject(obj: any): RaceRankResult {
    return new RaceRankResult({
      race_number: obj.race_number ?? null,
      surface_mode: obj.surface_mode ?? 'Dirt',
      distance_f: obj.distance_f ?? 0,
      shape: obj.shape ?? 'Honest',
      pace_heat: obj.pace_heat ?? 0,
      epi: obj.epi ?? 0,
      horses: obj.horses?.map((h: any) => HorseRank.fromObject(h)) ?? [],
    });
  }

  toObject(): any {
    return {
      race_number: this.race_number,
      surface_mode: this.surface_mode,
      distance_f: this.distance_f,
      shape: this.shape,
      pace_heat: this.pace_heat,
      epi: this.epi,
      horses: this.horses.map(h => h.toObject()),
    };
  }
}

export class CardAnalysis {
  racecard_id: number | null;
  track: string;
  date: string;
  races: RaceRankResult[];

  constructor(data: any = {}) {
    this.racecard_id = data.racecard_id ?? null;
    this.track = data.track ?? '';
    this.date = data.date ?? '';
    this.races = (data.races ?? []).map((r: any) =>
      r instanceof RaceRankResult ? r : RaceRankResult.fromObject(r)
    );
  }

  static fromObject(obj: any): CardAnalysis {
    return new CardAnalysis({
      racecard_id: obj.racecard_id ?? null,
      track: obj.track ?? '',
      date: obj.date ?? '',
      races: obj.races?.map((r: any) => RaceRankResult.fromObject(r)) ?? [],
    });
  }

  toObject(): any {
    return {
      racecard_id: this.racecard_id,
      track: this.track,
      date: this.date,
      races: this.races.map(r => r.toObject()),
    };
  }
}
