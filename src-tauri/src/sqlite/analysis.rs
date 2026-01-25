use crate::analysis::contextual_speed_and_pace_model::{
    CardAnalysis, HorseRank, RaceRankResult, RepFigs, RunStyle, Shape, SurfaceMode, WorkoutSig,
};
use sqlx::{Row, SqlitePool};
use tauri::State;

#[tauri::command]
pub async fn add_card_analysis(
    pool: State<'_, SqlitePool>,
    racecard_id: i64,
    analysis: CardAnalysis,
) -> Result<(), String> {
    add_card_analysis_inner(&pool, racecard_id, analysis)
        .await
        .map_err(|e| format!("Failed to add card analysis: {}", e))
}

#[tauri::command]
pub async fn get_card_analysis_by_racecard_id(
    pool: State<'_, SqlitePool>,
    racecard_id: i64,
) -> Result<CardAnalysis, String> {
    read_card_analysis_by_racecard_id(&pool, racecard_id)
        .await
        .map_err(|e| format!("Failed to load card analysis: {}", e))
}

pub async fn add_card_analysis_inner(
    pool: &SqlitePool,
    racecard_id: i64,
    analysis: CardAnalysis,
) -> Result<(), sqlx::Error> {
    if racecard_id != analysis.racecard_id {
        return Err(sqlx::Error::Protocol(format!(
            "racecard_id mismatch: arg={}, analysis={}",
            racecard_id, analysis.racecard_id
        )));
    }

    sqlx::query(
        r#"
        INSERT OR REPLACE INTO card_analyses (id, racecard_id, track, date)
        VALUES (?, ?, ?, ?);
        "#,
    )
    .bind(analysis.racecard_id)
    .bind(analysis.racecard_id)
    .bind(&analysis.track)
    .bind(&analysis.date)
    .execute(pool)
    .await?;

    for race in &analysis.races {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO race_rank_results (
                id,
                card_analysis_id,
                race_number,
                surface_mode,
                distance_f,
                shape,
                pace_heat,
                epi
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?);
            "#,
        )
        .bind(race.race_id)
        .bind(analysis.racecard_id)
        .bind(race.race_number.map(|v| v as i64))
        .bind(surface_mode_to_i64(race.surface_mode))
        .bind(race.distance_f)
        .bind(shape_to_i64(race.shape))
        .bind(race.pace_heat as i64)
        .bind(race.epi)
        .execute(pool)
        .await?;

        for horse in &race.horses {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO horse_ranks (
                    id,
                    race_rank_result_id,
                    program_number,
                    horse_name,
                    post_position,
                    run_style,
                    quirin,
                    shape,
                    score,
                    rep_speed,
                    rep_early,
                    rep_late,
                    workout_recent_works,
                    workout_top_rank_works,
                    workout_score
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
                "#,
            )
            .bind(horse.horse_id)
            .bind(horse.race_id)
            .bind(&horse.program_number)
            .bind(&horse.horse_name)
            .bind(horse.post_position.map(|v| v as i64))
            .bind(run_style_to_i64(horse.run_style))
            .bind(horse.quirin.map(|v| v as i64))
            .bind(shape_to_i64(horse.shape))
            .bind(horse.score)
            .bind(horse.rep.rep_speed)
            .bind(horse.rep.rep_early)
            .bind(horse.rep.rep_late)
            .bind(horse.workout.recent_works as i64)
            .bind(horse.workout.top_rank_works as i64)
            .bind(horse.workout.score)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

pub async fn read_card_analysis_by_racecard_id(
    pool: &SqlitePool,
    racecard_id: i64,
) -> Result<CardAnalysis, sqlx::Error> {
    let analysis_row = sqlx::query(
        r#"
        SELECT * FROM card_analyses
        WHERE racecard_id = ?
        ORDER BY id DESC
        LIMIT 1;
        "#,
    )
    .bind(racecard_id)
    .fetch_one(pool)
    .await?;

    let card_analysis_id = analysis_row.get::<i64, _>("id");
    let mut analysis = CardAnalysis {
        racecard_id: analysis_row.get("racecard_id"),
        track: analysis_row.get("track"),
        date: analysis_row.get("date"),
        races: Vec::new(),
    };

    let race_rows = sqlx::query(
        r#"
        SELECT * FROM race_rank_results
        WHERE card_analysis_id = ?
        ORDER BY id;
        "#,
    )
    .bind(card_analysis_id)
    .fetch_all(pool)
    .await?;

    for race_row in race_rows {
        let race_rank_result_id = race_row.get::<i64, _>("id");

        let horse_rows = sqlx::query(
            r#"
            SELECT * FROM horse_ranks
            WHERE race_rank_result_id = ?
            ORDER BY id;
            "#,
        )
        .bind(race_rank_result_id)
        .fetch_all(pool)
        .await?;

        let horses = horse_rows
            .into_iter()
            .map(|horse_row| {
                let run_style = run_style_from_i64(horse_row.get::<i64, _>("run_style"))?;
                let shape = shape_from_i64(horse_row.get::<i64, _>("shape"))?;

                Ok(HorseRank {
                    race_id: race_rank_result_id,
                    horse_id: horse_row.get("id"),
                    program_number: horse_row.get("program_number"),
                    horse_name: horse_row.get("horse_name"),
                    post_position: opt_i64_to_u32(horse_row.get::<Option<i64>, _>("post_position")),
                    run_style,
                    quirin: opt_i64_to_u32(horse_row.get::<Option<i64>, _>("quirin")),
                    shape,
                    score: horse_row.get::<Option<f64>, _>("score"),
                    rep: RepFigs {
                        rep_speed: horse_row.get::<Option<f64>, _>("rep_speed"),
                        rep_early: horse_row.get::<Option<f64>, _>("rep_early"),
                        rep_late: horse_row.get::<Option<f64>, _>("rep_late"),
                    },
                    workout: WorkoutSig {
                        recent_works: i64_to_u32(horse_row.get::<i64, _>("workout_recent_works"))?,
                        top_rank_works: i64_to_u32(horse_row.get::<i64, _>("workout_top_rank_works"))?,
                        score: horse_row.get::<f64, _>("workout_score"),
                    },
                })
            })
            .collect::<Result<Vec<HorseRank>, sqlx::Error>>()?;

        let surface_mode = surface_mode_from_i64(race_row.get::<i64, _>("surface_mode"))?;
        let shape = shape_from_i64(race_row.get::<i64, _>("shape"))?;
        let pace_heat = i64_to_u32(race_row.get::<i64, _>("pace_heat"))?;

        analysis.races.push(RaceRankResult {
            racecard_id: analysis.racecard_id,
            race_id: race_rank_result_id,
            race_number: opt_i64_to_u32(race_row.get::<Option<i64>, _>("race_number")),
            surface_mode,
            distance_f: race_row.get("distance_f"),
            shape,
            pace_heat,
            epi: race_row.get("epi"),
            horses,
        });
    }

    Ok(analysis)
}

fn surface_mode_to_i64(mode: SurfaceMode) -> i64 {
    mode as i64
}

fn surface_mode_from_i64(value: i64) -> Result<SurfaceMode, sqlx::Error> {
    match value {
        0 => Ok(SurfaceMode::Dirt),
        1 => Ok(SurfaceMode::Turf),
        _ => Err(sqlx::Error::Protocol(format!(
            "Unknown surface mode: {}",
            value
        ))),
    }
}

fn shape_to_i64(shape: Shape) -> i64 {
    shape as i64
}

fn shape_from_i64(value: i64) -> Result<Shape, sqlx::Error> {
    match value {
        0 => Ok(Shape::Slow),
        1 => Ok(Shape::Honest),
        2 => Ok(Shape::Fast),
        3 => Ok(Shape::Meltdown),
        _ => Err(sqlx::Error::Protocol(format!("Unknown shape: {}", value))),
    }
}

fn run_style_to_i64(style: RunStyle) -> i64 {
    style as i64
}

fn run_style_from_i64(value: i64) -> Result<RunStyle, sqlx::Error> {
    match value {
        0 => Ok(RunStyle::E),
        1 => Ok(RunStyle::EP),
        2 => Ok(RunStyle::P),
        3 => Ok(RunStyle::S),
        4 => Ok(RunStyle::Unk),
        _ => Err(sqlx::Error::Protocol(format!(
            "Unknown run style: {}",
            value
        ))),
    }
}

fn opt_i64_to_u32(value: Option<i64>) -> Option<u32> {
    value.map(|v| v as u32)
}

fn i64_to_u32(value: i64) -> Result<u32, sqlx::Error> {
    if value < 0 {
        return Err(sqlx::Error::Protocol(format!(
            "Negative value for u32: {}",
            value
        )));
    }
    Ok(value as u32)
}
