use crate::models::racecard::{Horse, PastPerformance, Race, Racecard};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SurfaceMode {
    Dirt,
    Turf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Shape {
    Slow,
    Honest,
    Fast,
    Meltdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunStyle {
    E,
    EP,
    P,
    S,
    Unk,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepFigs {
    pub rep_speed: Option<f64>,
    pub rep_early: Option<f64>,
    pub rep_late: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSig {
    pub recent_works: u32,
    pub top_rank_works: u32,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorseRank {
    pub race_id: i64,
    pub horse_id: i64,
    pub program_number: String,
    pub horse_name: String,
    pub post_position: Option<u32>,
    pub run_style: RunStyle,
    pub quirin: Option<u32>,
    pub shape: Shape,
    pub score: Option<f64>,
    pub rep: RepFigs,
    pub workout: WorkoutSig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceRankResult {
    pub racecard_id: i64,
    pub race_id: i64,
    pub race_number: Option<u32>,
    pub surface_mode: SurfaceMode,
    pub distance_f: f64,
    pub shape: Shape,
    pub pace_heat: u32,
    pub epi: f64,
    pub horses: Vec<HorseRank>,
}

fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}

fn yards_to_furlongs(yards: Option<i32>) -> f64 {
    match yards {
        Some(y) => (y as f64) / 220.0,
        None => 0.0,
    }
}

fn parse_run_style(raw: &str) -> RunStyle {
    let s = raw.trim().to_uppercase();
    if s == "E" || s.starts_with("E/") {
        RunStyle::E
    } else if s == "EP" || s == "E/P" || s.starts_with("EP") {
        RunStyle::EP
    } else if s == "P" {
        RunStyle::P
    } else if s == "S" || s == "C" || s == "CLO" {
        RunStyle::S
    } else {
        RunStyle::Unk
    }
}

fn weighted_avg(values: &[Option<f64>], weights: &[f64]) -> Option<f64> {
    let mut num = 0.0;
    let mut den = 0.0;

    for i in 0..values.len().min(weights.len()) {
        if let Some(v) = values[i] {
            if v.is_finite() {
                num += v * weights[i];
                den += weights[i];
            }
        }
    }

    if den > 0.0 { Some(num / den) } else { None }
}

fn parse_mmddyyyy(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%m/%d/%Y").ok()
}

fn surface_matches(mode: SurfaceMode, pp_surface: &str) -> bool {
    let s = pp_surface.trim().to_uppercase();
    match mode {
        SurfaceMode::Dirt => s == "D",
        SurfaceMode::Turf => s == "T",
    }
}

fn pick_early_fig(pp: &PastPerformance, dist_f: f64) -> Option<f64> {
    if dist_f <= 5.5 {
        pp.bris_2f_pace.map(|x| x as f64).or(pp.bris_4f_pace.map(|x| x as f64))
    } else if dist_f <= 7.5 {
        pp.bris_4f_pace
            .map(|x| x as f64)
            .or(pp.bris_2f_pace.map(|x| x as f64))
            .or(pp.bris_6f_pace.map(|x| x as f64))
    } else {
        pp.bris_6f_pace.map(|x| x as f64).or(pp.bris_4f_pace.map(|x| x as f64))
    }
}

pub fn representative_figures(h: &Horse, race: &Race, mode: SurfaceMode) -> RepFigs {
    let dist_f = yards_to_furlongs(race.distance);
    let weights: [f64; 5] = [0.55, 0.30, 0.15, 0.08, 0.05];


    let usable: Vec<&PastPerformance> = h
        .past_performances
        .iter()
        .filter(|pp| !pp.race_date.trim().is_empty())
        .collect();

    let same_surface: Vec<&PastPerformance> = usable
        .iter()
        .copied()
        .filter(|pp| surface_matches(mode, &pp.surface))
        .collect();

    let pool: Vec<&PastPerformance> = if same_surface.len() >= 2 {
        same_surface
    } else {
        usable
    }
    .into_iter()
    .take(5)
    .collect();

    let speeds: Vec<Option<f64>> = pool.iter().map(|pp| pp.bris_speed_rating.map(|x| x as f64)).collect();
    let earlies: Vec<Option<f64>> = pool.iter().map(|pp| pick_early_fig(pp, dist_f)).collect();
    let lates: Vec<Option<f64>> = pool.iter().map(|pp| pp.bris_late_pace.map(|x| x as f64)).collect();

    RepFigs {
        rep_speed: weighted_avg(&speeds, &weights),
        rep_early: weighted_avg(&earlies, &weights),
        rep_late: weighted_avg(&lates, &weights),
    }
}

#[derive(Debug, Clone, Copy)]
struct Pars {
    speed: f64,
    early: f64,
    late: f64,
}

fn get_pars_for_race(race: &Race, dist_f: f64) -> Option<Pars> {
    let early = if dist_f <= 7.5 {
        race.four_f_bris_pace_par.map(|x| x as f64)
    } else {
        race.six_f_bris_pace_par.map(|x| x as f64)
    };

    let speed = race.bris_speed_for_class.map(|x| x as f64);
    let late = race.bris_late_pace_par.map(|x| x as f64);

    match (speed, early, late) {
        (Some(s), Some(e), Some(l)) => Some(Pars { speed: s, early: e, late: l }),
        _ => None,
    }
}

pub fn race_shape_dirt(race: &Race) -> (Shape, u32, f64) {
    let dist_f = yards_to_furlongs(race.distance);
    let pars = get_pars_for_race(race, dist_f);

    let horses = &race.horses;
    let field_size = horses.len().max(1) as f64;

    let pace_heat = horses
        .iter()
        .filter(|h| {
            let rs = parse_run_style(&h.bris_run_style);
            let q = h.quirin_speed_points.unwrap_or(0);
            rs == RunStyle::E || q >= 6
        })
        .count() as u32;

    let mut sum_early = 0.0;
    let mut n = 0.0;
    for h in horses {
        let rep = representative_figures(h, race, SurfaceMode::Dirt);
        if let Some(e) = rep.rep_early {
            sum_early += e;
            n += 1.0;
        }
    }
    let avg_early = if n > 0.0 { sum_early / n } else { 0.0 };
    let early_vs_par = pars.map(|p| avg_early - p.early).unwrap_or(0.0);

    let epi = clamp((pace_heat as f64 / field_size) * 0.70 + clamp((early_vs_par + 6.0) / 12.0, 0.0, 1.0) * 0.30, 0.0, 1.0);

    let shape = if epi >= 0.88 {
        Shape::Meltdown
    } else if epi >= 0.75 {
        Shape::Fast
    } else if epi <= 0.25 {
        Shape::Slow
    } else {
        Shape::Honest
    };

    (shape, pace_heat, epi)
}

pub fn race_shape_turf(race: &Race) -> (Shape, u32, f64) {
    let dist_f = yards_to_furlongs(race.distance);
    let pars = get_pars_for_race(race, dist_f);

    let horses = &race.horses;
    let field_size = horses.len().max(1) as f64;

    let pace_heat = horses
        .iter()
        .filter(|h| {
            let rs = parse_run_style(&h.bris_run_style);
            let q = h.quirin_speed_points.unwrap_or(0);
            rs == RunStyle::E || (rs == RunStyle::EP && q >= 5) || q >= 7
        })
        .count() as u32;

    let mut sum_early = 0.0;
    let mut n = 0.0;
    for h in horses {
        let rep = representative_figures(h, race, SurfaceMode::Turf);
        if let Some(e) = rep.rep_early {
            sum_early += e;
            n += 1.0;
        }
    }
    let avg_early = if n > 0.0 { sum_early / n } else { 0.0 };
    let early_vs_par = pars.map(|p| avg_early - p.early).unwrap_or(0.0);

    let epi = clamp((pace_heat as f64 / field_size) * 0.60 + clamp((early_vs_par + 4.0) / 10.0, 0.0, 1.0) * 0.40, 0.0, 1.0);

    let shape = if epi >= 0.82 {
        Shape::Meltdown
    } else if epi >= 0.68 {
        Shape::Fast
    } else if epi <= 0.22 {
        Shape::Slow
    } else {
        Shape::Honest
    };

    (shape, pace_heat, epi)
}

pub fn score_horse_dirt(h: &Horse, race: &Race, shape: Shape) -> (Option<f64>, RepFigs) {
    let dist_f = yards_to_furlongs(race.distance);
    let pars = match get_pars_for_race(race, dist_f) {
        Some(p) => p,
        None => return (None, RepFigs { rep_speed: None, rep_early: None, rep_late: None }),
    };

    let rep = representative_figures(h, race, SurfaceMode::Dirt);
    let (rs, re, rl) = match (rep.rep_speed, rep.rep_early, rep.rep_late) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => return (None, rep),
    };

    let spd = rs - pars.speed;
    let early = re - pars.early;
    let late = rl - pars.late;

    let (mut w_s, mut w_e, mut w_l) = (0.45, 0.30, 0.25);
    match shape {
        Shape::Slow => { w_s = 0.45; w_e = 0.40; w_l = 0.15; }
        Shape::Fast => { w_s = 0.45; w_e = 0.25; w_l = 0.30; }
        Shape::Meltdown => { w_s = 0.40; w_e = 0.15; w_l = 0.45; }
        Shape::Honest => {}
    }

    let mut score = w_s * spd + w_e * early + w_l * late;

    if let Some(pp) = h.bris_prime_power_rating {
        score += (pp - 100.0) / 10.0;
    }

    if let Some(lay) = h.days_since_last_race {
        if lay > 60 {
            score -= ((lay - 60) as f64 / 60.0).min(1.5);
        }
    }

    if let Some(tj) = h.trainer_jockey_combo_roi_meet {
        score += clamp(tj / 4.0, -0.5, 0.5);
    }

    (Some(score), rep)
}

pub fn score_horse_turf(h: &Horse, race: &Race, shape: Shape) -> (Option<f64>, RepFigs) {
    let dist_f = yards_to_furlongs(race.distance);
    let pars = match get_pars_for_race(race, dist_f) {
        Some(p) => p,
        None => return (None, RepFigs { rep_speed: None, rep_early: None, rep_late: None }),
    };

    let rep = representative_figures(h, race, SurfaceMode::Turf);
    let (rs, re, rl) = match (rep.rep_speed, rep.rep_early, rep.rep_late) {
        (Some(a), Some(b), Some(c)) => (a, b, c),
        _ => return (None, rep),
    };

    let spd = rs - pars.speed;
    let early = re - pars.early;
    let late = rl - pars.late;

    let (mut w_s, mut w_e, mut w_l) = (0.40, 0.20, 0.40);
    match shape {
        Shape::Slow => { w_s = 0.45; w_e = 0.25; w_l = 0.30; }
        Shape::Fast => { w_s = 0.40; w_e = 0.15; w_l = 0.45; }
        Shape::Meltdown => { w_s = 0.35; w_e = 0.10; w_l = 0.55; }
        Shape::Honest => {}
    }

    let mut score = w_s * spd + w_e * early + w_l * late;

    if let Some(pp) = h.bris_prime_power_rating {
        score += (pp - 100.0) / 12.0;
    }

    let turf_ped = h
        .bris_turf_pedigree_rating
        .replace('*', "")
        .trim()
        .parse::<f64>()
        .ok();
    if let Some(tp) = turf_ped {
        score += clamp((tp - 100.0) / 30.0, -0.5, 0.8);
    }

    (Some(score), rep)
}

pub fn workout_signal(h: &Horse, race_date_mmddyyyy: Option<&str>, days_window: i64) -> WorkoutSig {
    let race_day = race_date_mmddyyyy.and_then(parse_mmddyyyy);

    let mut recent_works = 0u32;
    let mut top_rank_works = 0u32;

    for w in &h.workouts {
        if let (Some(rd), Some(wd)) = (race_day, parse_mmddyyyy(&w.date)) {
            let age = (rd - wd).num_days();
            if age < 0 || age > days_window {
                continue;
            }
        }

        let (rank, n) = (w.rank, w.workouts_that_day_distance);
        if let (Some(r), Some(total)) = (rank, n) {
            if total > 0 {
                recent_works += 1;
                let pct = (r as f64) / (total as f64);
                if pct <= 0.10 {
                    top_rank_works += 1;
                }
            }
        }
    }

    let score = clamp((recent_works as f64) * 0.10 + (top_rank_works as f64) * 0.60, 0.0, 3.0);

    WorkoutSig { recent_works, top_rank_works, score }
}

pub fn rank_race_dirt(race: &Race, racecard_date: Option<&str>) -> RaceRankResult {
    let dist_f = yards_to_furlongs(race.distance);
    let (shape, pace_heat, epi) = race_shape_dirt(race);

    let mut horses: Vec<HorseRank> = race
        .horses
        .iter()
        .map(|h| {
            let run_style = parse_run_style(&h.bris_run_style);
            let (score, rep) = score_horse_dirt(h, race, shape);
            let workout = workout_signal(h, racecard_date, 21);

            HorseRank {
                race_id: race.id,
                horse_id: h.id,
                program_number: h.program_number.clone(),
                horse_name: h.horse_name.clone(),
                post_position: h.post_position,
                run_style,
                quirin: h.quirin_speed_points,
                shape,
                score,
                rep,
                workout,
            }
        })
        .collect();

    horses.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    RaceRankResult {
        racecard_id: race.racecard_id,
        race_id: race.id,
        race_number: race.race_number,
        surface_mode: SurfaceMode::Dirt,
        distance_f: dist_f,
        shape,
        pace_heat,
        epi,
        horses,
    }
}

pub fn rank_race_turf(race: &Race, racecard_date: Option<&str>) -> RaceRankResult {
    let dist_f = yards_to_furlongs(race.distance);
    let (shape, pace_heat, epi) = race_shape_turf(race);

    let mut horses: Vec<HorseRank> = race
        .horses
        .iter()
        .map(|h| {
            let run_style = parse_run_style(&h.bris_run_style);
            let (score, rep) = score_horse_turf(h, race, shape);
            let workout = workout_signal(h, racecard_date, 21);

            HorseRank {
                race_id: race.id,
                horse_id: h.id,
                program_number: h.program_number.clone(),
                horse_name: h.horse_name.clone(),
                post_position: h.post_position,
                run_style,
                quirin: h.quirin_speed_points,
                shape,
                score,
                rep,
                workout,
            }
        })
        .collect();

    horses.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    RaceRankResult {
        racecard_id: race.racecard_id,
        race_id: race.id,
        race_number: race.race_number,
        surface_mode: SurfaceMode::Turf,
        distance_f: dist_f,
        shape,
        pace_heat,
        epi,
        horses,
    }
}

pub fn rank_race_auto(race: &Race, racecard_date: Option<&str>) -> RaceRankResult {
    let surf = race.surface.trim().to_uppercase();
    if surf == "T" {
        rank_race_turf(race, racecard_date)
    } else {
        rank_race_dirt(race, racecard_date)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAnalysis {
    pub racecard_id: i64,
    pub track: String,
    pub date: String,
    pub races: Vec<RaceRankResult>,
}

pub fn analyze_racecard(card: &Racecard) -> CardAnalysis {
    let races = card
        .races
        .iter()
        .map(|r| rank_race_auto(r, Some(&card.date)))
        .collect();

    CardAnalysis {
        racecard_id: card.id,
        track: card.track.clone(),
        date: card.date.clone(),
        races,
    }
}
