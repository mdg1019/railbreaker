export type SurfaceMode = 'Dirt' | 'Turf';
export type Shape = 'Slow' | 'Honest' | 'Fast' | 'Meltdown';
export type RunStyle = 'E' | 'EP' | 'P' | 'S' | 'Unk';

export class RepFigs {
  repSpeed: number | null;
  repEarly: number | null;
  repLate: number | null;

  constructor(data: any = {}) {
    this.repSpeed = data.repSpeed ?? null;
    this.repEarly = data.repEarly ?? null;
    this.repLate = data.repLate ?? null;
  }

  static fromObject(obj: any): RepFigs {
    return new RepFigs({
      repSpeed: obj.rep_speed ?? obj.repSpeed ?? null,
      repEarly: obj.rep_early ?? obj.repEarly ?? null,
      repLate: obj.rep_late ?? obj.repLate ?? null,
    });
  }

  toObject(): any {
    return {
      rep_speed: this.repSpeed,
      rep_early: this.repEarly,
      rep_late: this.repLate,
    };
  }
}

export class WorkoutSig {
  recentWorks: number;
  topRankWorks: number;
  score: number;

  constructor(data: any = {}) {
    this.recentWorks = data.recentWorks ?? 0;
    this.topRankWorks = data.topRankWorks ?? 0;
    this.score = data.score ?? 0;
  }

  static fromObject(obj: any): WorkoutSig {
    return new WorkoutSig({
      recentWorks: obj.recent_works ?? obj.recentWorks ?? obj.workout_recent_works ?? obj.workoutRecentWorks ?? 0,
      topRankWorks: obj.top_rank_works ?? obj.topRankWorks ?? obj.workout_top_rank_works ?? obj.workoutTopRankWorks ?? 0,
      score: obj.workout_score ?? obj.workoutScore ?? obj.score ?? 0,
    });
  }

  toObject(): any {
    return {
      recent_works: this.recentWorks,
      top_rank_works: this.topRankWorks,
      score: this.score,
    };
  }
}

export class HorseRank {
  raceId: number | null;
  horseId: number | null;
  programNumber: string;
  horseName: string;
  postPosition: number | null;
  runStyle: RunStyle;
  quirin: number | null;
  shape: Shape;
  score: number | null;
  rep: RepFigs;
  workout: WorkoutSig;

  constructor(data: any = {}) {
    this.raceId = data.raceId ?? null;
    this.horseId = data.horseId ?? null;
    this.programNumber = data.programNumber ?? '';
    this.horseName = data.horseName ?? '';
    this.postPosition = data.postPosition ?? null;
    this.runStyle = data.runStyle ?? 'Unk';
    this.quirin = data.quirin ?? null;
    this.shape = data.shape ?? 'Honest';
    this.score = data.score ?? null;
    this.rep = data.rep ?? new RepFigs();
    this.workout = data.workout ?? new WorkoutSig();
  }

  static fromObject(obj: any): HorseRank {
    const repObj = obj.rep ?? {
      rep_speed: obj.rep_speed ?? obj.repSpeed ?? null,
      rep_early: obj.rep_early ?? obj.repEarly ?? null,
      rep_late: obj.rep_late ?? obj.repLate ?? null,
    };

    const workoutObj = obj.workout ?? {
      recent_works: obj.recent_works ?? obj.recentWorks ?? obj.workout_recent_works ?? obj.workoutRecentWorks ?? 0,
      top_rank_works: obj.top_rank_works ?? obj.topRankWorks ?? obj.workout_top_rank_works ?? obj.workoutTopRankWorks ?? 0,
      score: obj.workout_score ?? obj.workoutScore ?? 0,
    };

    return new HorseRank({
      raceId: obj.race_id ?? obj.raceId ?? null,
      horseId: obj.horse_id ?? obj.horseId ?? null,
      programNumber: obj.program_number ?? obj.programNumber ?? '',
      horseName: obj.horse_name ?? obj.horseName ?? '',
      postPosition: obj.post_position ?? obj.postPosition ?? null,
      runStyle: obj.run_style ?? obj.runStyle ?? 'Unk',
      quirin: obj.quirin ?? null,
      shape: obj.shape ?? 'Honest',
      score: obj.score ?? null,
      rep: RepFigs.fromObject(repObj),
      workout: WorkoutSig.fromObject(workoutObj),
    });
  }

  toObject(): any {
    return {
      race_id: this.raceId,
      horse_id: this.horseId,
      program_number: this.programNumber,
      horse_name: this.horseName,
      post_position: this.postPosition,
      run_style: this.runStyle,
      quirin: this.quirin,
      shape: this.shape,
      score: this.score,
      rep: this.rep.toObject(),
      workout: this.workout.toObject(),
    };
  }
}

export class RaceRankResult {
  racecardId: number | null;
  raceId: number | null;
  raceNumber: number | null;
  surfaceMode: SurfaceMode;
  distanceF: number;
  shape: Shape;
  paceHeat: number;
  epi: number;
  horses: HorseRank[];

  constructor(data: any = {}) {
    this.racecardId = data.racecardId ?? null;
    this.raceId = data.raceId ?? null;
    this.raceNumber = data.raceNumber ?? null;
    this.surfaceMode = data.surfaceMode ?? 'Dirt';
    this.distanceF = data.distanceF ?? 0;
    this.shape = data.shape ?? 'Honest';
    this.paceHeat = data.paceHeat ?? 0;
    this.epi = data.epi ?? 0;
    this.horses = data.horses ?? [];
  }

  static fromObject(obj: any): RaceRankResult {
    return new RaceRankResult({
      racecardId: obj.racecard_id ?? obj.racecardId ?? null,
      raceId: obj.race_id ?? obj.raceId ?? null,
      raceNumber: obj.race_number ?? obj.raceNumber ?? null,
      surfaceMode: obj.surface_mode ?? obj.surfaceMode ?? 'Dirt',
      distanceF: obj.distance_f ?? obj.distanceF ?? 0,
      shape: obj.shape ?? 'Honest',
      paceHeat: obj.pace_heat ?? obj.paceHeat ?? 0,
      epi: obj.epi ?? 0,
      horses: obj.horses?.map((h: any) => HorseRank.fromObject(h)) ?? [],
    });
  }

  toObject(): any {
    return {
      racecard_id: this.racecardId,
      race_id: this.raceId,
      race_number: this.raceNumber,
      surface_mode: this.surfaceMode,
      distance_f: this.distanceF,
      shape: this.shape,
      pace_heat: this.paceHeat,
      epi: this.epi,
      horses: this.horses.map(h => h.toObject()),
    };
  }
}

export class CardAnalysis {
  racecardId: number | null;
  track: string;
  date: string;
  races: RaceRankResult[];

  constructor(data: any = {}) {
    this.racecardId = data.racecardId ?? null;
    this.track = data.track ?? '';
    this.date = data.date ?? '';
    this.races = data.races ?? [];
  }

  static fromObject(obj: any): CardAnalysis {
    return new CardAnalysis({
      racecardId: obj.racecard_id ?? obj.racecardId ?? null,
      track: obj.track ?? '',
      date: obj.date ?? '',
      races: obj.races?.map((r: any) => RaceRankResult.fromObject(r)) ?? [],
    });
  }

  toObject(): any {
    return {
      racecard_id: this.racecardId,
      track: this.track,
      date: this.date,
      races: this.races.map(r => r.toObject()),
    };
  }
}
