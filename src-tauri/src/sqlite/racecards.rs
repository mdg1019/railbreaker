use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use std::collections::HashMap;
use tauri::State;
use crate::json::to_camel_case_value;
use crate::models::racecard::{Horse, KeyTrainerStat, PastPerformance, Race, Racecard, Workout};
use serde_json::Value;

const RACE_COLUMNS: usize = 43;
const HORSE_COLUMNS: usize = 143;
const PAST_PERFORMANCE_COLUMNS: usize = 101;

fn placeholders(count: usize) -> String {
    std::iter::repeat("?")
        .take(count)
        .collect::<Vec<_>>()
        .join(", ")
}

#[tauri::command]
pub async fn add_racecard(
    pool: State<'_, SqlitePool>,
    racecard: Racecard,
) -> Result<Racecard, String> {
    add_racecard_inner(&pool, racecard)
        .await
        .map_err(|e| format!("Failed to add racecard: {}", e))
}

#[tauri::command]
pub async fn racecard_exists_by_zip_name(
    pool: State<'_, SqlitePool>,
    zip_file_name: String,
) -> Result<bool, String> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT 1 FROM racecards WHERE zip_file_name = ? LIMIT 1;",
    )
    .bind(zip_file_name)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| format!("Failed to check racecard: {}", e))?
    .is_some();

    Ok(exists)
}

#[tauri::command]
pub async fn get_racecard_by_id(
    pool: State<'_, SqlitePool>,
    racecard_id: i64,
) -> Result<Value, String> {
    let racecard = read_racecard_by_id(&pool, racecard_id)
        .await
        .map_err(|e| format!("Failed to load racecard: {}", e))?;
    let value = serde_json::to_value(&racecard)
        .map_err(|e| format!("Failed to serialize racecard: {}", e))?;
    Ok(to_camel_case_value(value))
}

#[tauri::command]
pub async fn get_all_racecards(
    pool: State<'_, SqlitePool>,
) -> Result<Value, String> {
    let rows = sqlx::query("SELECT * FROM racecards ORDER BY track ASC, date DESC;")
        .fetch_all(&*pool)
        .await
        .map_err(|e| format!("Failed to load racecards: {}", e))?;

    let racecards: Vec<Racecard> = rows
        .into_iter()
        .map(|row| Racecard {
            id: row.get("id"),
            zip_file_name: row.get("zip_file_name"),
            track: row.get("track"),
            date: row.get("date"),
            long_date: row.get("long_date"),
            races: Vec::new(),
        })
        .collect();

    let value = serde_json::to_value(&racecards)
        .map_err(|e| format!("Failed to serialize racecards: {}", e))?;
    Ok(to_camel_case_value(value))
}

#[tauri::command]
pub async fn update_note(
    pool: State<'_, SqlitePool>,
    horse_id: i64,
    note: String,
) -> Result<(), String> {
    sqlx::query("UPDATE horses SET note = ? WHERE id = ?;")
        .bind(note)
        .bind(horse_id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("Failed to update note: {}", e))?;

    Ok(())
}

pub async fn read_racecard_by_id(pool: &SqlitePool, racecard_id: i64) -> Result<Racecard, sqlx::Error> {
    let racecard_row = sqlx::query("SELECT * FROM racecards WHERE id = ?;")
        .bind(racecard_id)
        .fetch_one(pool)
        .await?;

    let mut racecard = Racecard {
        id: racecard_row.get("id"),
        zip_file_name: racecard_row.get("zip_file_name"),
        track: racecard_row.get("track"),
        date: racecard_row.get("date"),
        long_date: racecard_row.get("long_date"),
        races: Vec::new(),
    };

    let race_rows = sqlx::query("SELECT * FROM races WHERE racecard_id = ? ORDER BY id;")
        .bind(racecard_id)
        .fetch_all(pool)
        .await?;

    let horse_rows = sqlx::query(
        r#"
        SELECT h.* FROM horses h
        JOIN races r ON r.id = h.race_id
        WHERE r.racecard_id = ?
        ORDER BY h.id;
        "#,
    )
    .bind(racecard_id)
    .fetch_all(pool)
    .await?;

    let workout_rows = sqlx::query(
        r#"
        SELECT w.* FROM workouts w
        JOIN horses h ON h.id = w.horse_id
        JOIN races r ON r.id = h.race_id
        WHERE r.racecard_id = ?
        ORDER BY w.id;
        "#,
    )
    .bind(racecard_id)
    .fetch_all(pool)
    .await?;

    let past_performance_rows = sqlx::query(
        r#"
        SELECT pp.* FROM past_performances pp
        JOIN horses h ON h.id = pp.horse_id
        JOIN races r ON r.id = h.race_id
        WHERE r.racecard_id = ?
        ORDER BY pp.id;
        "#,
    )
    .bind(racecard_id)
    .fetch_all(pool)
    .await?;

    let key_trainer_rows = sqlx::query(
        r#"
        SELECT kts.* FROM key_trainer_stats kts
        JOIN horses h ON h.id = kts.horse_id
        JOIN races r ON r.id = h.race_id
        WHERE r.racecard_id = ?
        ORDER BY kts.id;
        "#,
    )
    .bind(racecard_id)
    .fetch_all(pool)
    .await?;

    let mut workouts_by_horse: HashMap<i64, Vec<Workout>> = HashMap::new();
    for row in workout_rows {
        let workout = workout_from_row(&row);
        workouts_by_horse
            .entry(workout.horse_id)
            .or_default()
            .push(workout);
    }

    let mut past_by_horse: HashMap<i64, Vec<PastPerformance>> = HashMap::new();
    for row in past_performance_rows {
        let past = past_performance_from_row(&row);
        past_by_horse
            .entry(past.horse_id)
            .or_default()
            .push(past);
    }

    let mut kts_by_horse: HashMap<i64, Vec<KeyTrainerStat>> = HashMap::new();
    for row in key_trainer_rows {
        let kts = key_trainer_stat_from_row(&row);
        kts_by_horse
            .entry(kts.horse_id)
            .or_default()
            .push(kts);
    }

    let mut horses_by_race: HashMap<i64, Vec<Horse>> = HashMap::new();
    for row in horse_rows {
        let mut horse = horse_from_row(&row);
        horse.workouts = workouts_by_horse.remove(&horse.id).unwrap_or_default();
        horse.past_performances = past_by_horse.remove(&horse.id).unwrap_or_default();
        horse.key_trainer_stats = kts_by_horse.remove(&horse.id).unwrap_or_default();
        horses_by_race.entry(horse.race_id).or_default().push(horse);
    }

    let mut races = Vec::with_capacity(race_rows.len());
    for row in race_rows {
        let mut race = race_from_row(&row);
        race.horses = horses_by_race.remove(&race.id).unwrap_or_default();
        races.push(race);
    }

    racecard.races = races;
    Ok(racecard)
}

pub async fn add_racecard_inner(pool: &SqlitePool, mut racecard: Racecard) -> Result<Racecard, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        r#"
        INSERT INTO racecards (
            zip_file_name,
            track,
            date,
            long_date
        )
        VALUES (?, ?, ?, ?);
        "#,
    )
    .bind(&racecard.zip_file_name)
    .bind(&racecard.track)
    .bind(&racecard.date)
    .bind(&racecard.long_date)
    .execute(&mut *tx)
    .await?;
    racecard.id = result.last_insert_rowid();

    for race in &mut racecard.races {
        race.racecard_id = racecard.id;
        let race_sql = format!(
            r#"
            INSERT INTO races (
                racecard_id,
                race_number,
                distance,
                surface,
                race_type,
                age_sex_restrictions,
                todays_race_classification,
                purse,
                claiming_price,
                track_record,
                race_conditions,
                todays_lasix_list,
                todays_bute_list,
                todays_coupled_list,
                todays_mutuel_list,
                simulcast_host_track_code,
                simulcast_host_track_race_number,
                all_weather_surface_flag,
                race_conditions_line1,
                race_conditions_line2,
                race_conditions_line3,
                race_conditions_line4,
                race_conditions_line5,
                race_conditions_line6,
                low_claiming_price,
                statebred_flag,
                wager_type_line1,
                wager_type_line2,
                wager_type_line3,
                wager_type_line4,
                wager_type_line5,
                wager_type_line6,
                wager_type_line7,
                wager_type_line8,
                wager_type_line9,
                two_f_bris_pace_par,
                four_f_bris_pace_par,
                six_f_bris_pace_par,
                bris_speed_for_class,
                bris_late_pace_par,
                post_times,
                post_time_pacific_military,
                todays_equibase_abbreviated_race_conditions
            )
            VALUES ({});
            "#,
            placeholders(RACE_COLUMNS)
        );
        let result = sqlx::query(&race_sql)
        .bind(race.racecard_id)
        .bind(race.race_number)
        .bind(race.distance)
        .bind(&race.surface)
        .bind(&race.race_type)
        .bind(&race.age_sex_restrictions)
        .bind(&race.todays_race_classification)
        .bind(race.purse)
        .bind(race.claiming_price)
        .bind(race.track_record)
        .bind(&race.race_conditions)
        .bind(&race.todays_lasix_list)
        .bind(&race.todays_bute_list)
        .bind(&race.todays_coupled_list)
        .bind(&race.todays_mutuel_list)
        .bind(&race.simulcast_host_track_code)
        .bind(race.simulcast_host_track_race_number)
        .bind(&race.all_weather_surface_flag)
        .bind(&race.race_conditions_line1)
        .bind(&race.race_conditions_line2)
        .bind(&race.race_conditions_line3)
        .bind(&race.race_conditions_line4)
        .bind(&race.race_conditions_line5)
        .bind(&race.race_conditions_line6)
        .bind(race.low_claiming_price)
        .bind(&race.statebred_flag)
        .bind(&race.wager_type_line1)
        .bind(&race.wager_type_line2)
        .bind(&race.wager_type_line3)
        .bind(&race.wager_type_line4)
        .bind(&race.wager_type_line5)
        .bind(&race.wager_type_line6)
        .bind(&race.wager_type_line7)
        .bind(&race.wager_type_line8)
        .bind(&race.wager_type_line9)
        .bind(race.two_f_bris_pace_par)
        .bind(race.four_f_bris_pace_par)
        .bind(race.six_f_bris_pace_par)
        .bind(race.bris_speed_for_class)
        .bind(race.bris_late_pace_par)
        .bind(&race.post_times)
        .bind(&race.post_time_pacific_military)
        .bind(&race.todays_equibase_abbreviated_race_conditions)
        .execute(&mut *tx)
        .await?;
        race.id = result.last_insert_rowid();

        for horse in &mut race.horses {
            horse.race_id = race.id;
            let horse_sql = format!(
                r#"
                INSERT INTO horses (
                    race_id,
                    post_position,
                    entry,
                    claiming_price_of_horse,
                    breed_type,
                    todays_nasal_strip_change,
                    todays_trainer,
                    trainer_starts,
                    trainer_wins,
                    trainer_places,
                    trainer_shows,
                    todays_jockey,
                    apprentice_weight_allowance,
                    jockey_starts,
                    jockey_wins,
                    jockey_places,
                    jockey_shows,
                    todays_owner,
                    owners_silks,
                    main_track_only_ae_indicator,
                    program_number,
                    morning_line_odds,
                    horse_name,
                    year_of_birth,
                    horses_foaling_month,
                    sex,
                    horses_color,
                    weight,
                    sire,
                    sires_sire,
                    dam,
                    dams_sire,
                    breeder,
                    state_country_where_bred,
                    program_post_position,
                    todays_medication_new,
                    todays_medication_old,
                    equipment_change,
                    lifetime_record_todays_distance_starts,
                    lifetime_record_todays_distance_wins,
                    lifetime_record_todays_distance_places,
                    lifetime_record_todays_distance_shows,
                    lifetime_record_todays_distance_earnings,
                    lifetime_record_todays_track_starts,
                    lifetime_record_todays_track_wins,
                    lifetime_record_todays_track_places,
                    lifetime_record_todays_track_shows,
                    lifetime_record_todays_track_earnings,
                    lifetime_record_turf_starts,
                    lifetime_record_turf_wins,
                    lifetime_record_turf_places,
                    lifetime_record_turf_shows,
                    lifetime_record_turf_earnings,
                    lifetime_record_wet_starts,
                    lifetime_record_wet_wins,
                    lifetime_record_wet_places,
                    lifetime_record_wet_shows,
                    lifetime_record_wet_earnings,
                    current_year_record_year,
                    current_year_record_starts,
                    current_year_record_wins,
                    current_year_record_places,
                    current_year_record_shows,
                    current_year_record_earnings,
                    previous_year_record_year,
                    previous_year_record_starts,
                    previous_year_record_wins,
                    previous_year_record_places,
                    previous_year_record_shows,
                    previous_year_record_earnings,
                    lifetime_record_starts,
                    lifetime_record_wins,
                    lifetime_record_places,
                    lifetime_record_shows,
                    lifetime_record_earnings,
                    bris_run_style,
                    quirin_speed_points,
                    trainer_jockey_combo_starts,
                    trainer_jockey_combo_wins,
                    trainer_jockey_combo_places,
                    trainer_jockey_combo_shows,
                    trainer_jockey_combo_roi,
                    days_since_last_race,
                    lifetime_all_weather_starts,
                    lifetime_all_weather_wins,
                    lifetime_all_weather_places,
                    lifetime_all_weather_shows,
                    lifetime_all_weather_earnings,
                    best_bris_speed_all_weather_surface,
                    bris_prime_power_rating,
                    trainer_starts_current_year,
                    trainer_wins_current_year,
                    trainer_places_current_year,
                    trainer_shows_current_year,
                    trainer_roi_current_year,
                    trainer_starts_previous_year,
                    trainer_wins_previous_year,
                    trainer_places_previous_year,
                    trainer_shows_previous_year,
                    trainer_roi_previous_year,
                    jockey_starts_current_year,
                    jockey_wins_current_year,
                    jockey_places_current_year,
                    jockey_shows_current_year,
                    jockey_roi_current_year,
                    jockey_starts_previous_year,
                    jockey_wins_previous_year,
                    jockey_places_previous_year,
                    jockey_shows_previous_year,
                    jockey_roi_previous_year,
                    sire_stud_fee,
                    best_bris_speed_fast_track,
                    best_bris_speed_turf,
                    best_bris_speed_off_track,
                    best_bris_speed_distance,
                    auction_price,
                    where_when_sold_at_auction,
                    bris_dirt_pedigree_rating,
                    bris_mud_pedigree_rating,
                    bris_turf_pedigree_rating,
                    bris_distance_pedigree_rating,
                    best_bris_speed_life,
                    best_bris_speed_most_recent_year,
                    best_bris_speed_2nd_most_recent_year,
                    best_bris_speed_todays_track,
                    starts_fast_dirt,
                    wins_fast_dirt,
                    places_fast_dirt,
                    shows_fast_dirt,
                    earnings_fast_dirt,
                    jockey_distance_turf_label,
                    jockey_distance_turf_starts,
                    jockey_distance_turf_wins,
                    jockey_distance_turf_places,
                    jockey_distance_turf_shows,
                    jockey_distance_turf_roi,
                    jockey_distance_turf_earnings,
                    trainer_jockey_combo_starts_meet,
                    trainer_jockey_combo_wins_meet,
                    trainer_jockey_combo_places_meet,
                    trainer_jockey_combo_shows_meet,
                    trainer_jockey_combo_roi_meet,
                    note
                )
                VALUES ({});
                "#,
                placeholders(HORSE_COLUMNS)
            );
            let result = sqlx::query(&horse_sql)
            .bind(horse.race_id)
            .bind(horse.post_position)
            .bind(&horse.entry)
            .bind(horse.claiming_price_of_horse)
            .bind(&horse.breed_type)
            .bind(horse.todays_nasal_strip_change)
            .bind(&horse.todays_trainer)
            .bind(horse.trainer_starts)
            .bind(horse.trainer_wins)
            .bind(horse.trainer_places)
            .bind(horse.trainer_shows)
            .bind(&horse.todays_jockey)
            .bind(horse.apprentice_weight_allowance)
            .bind(horse.jockey_starts)
            .bind(horse.jockey_wins)
            .bind(horse.jockey_places)
            .bind(horse.jockey_shows)
            .bind(&horse.todays_owner)
            .bind(&horse.owners_silks)
            .bind(&horse.main_track_only_ae_indicator)
            .bind(&horse.program_number)
            .bind(horse.morning_line_odds)
            .bind(&horse.horse_name)
            .bind(horse.year_of_birth)
            .bind(horse.horses_foaling_month)
            .bind(&horse.sex)
            .bind(&horse.horses_color)
            .bind(horse.weight)
            .bind(&horse.sire)
            .bind(&horse.sires_sire)
            .bind(&horse.dam)
            .bind(&horse.dams_sire)
            .bind(&horse.breeder)
            .bind(&horse.state_country_where_bred)
            .bind(&horse.program_post_position)
            .bind(horse.todays_medication_new)
            .bind(horse.todays_medication_old)
            .bind(horse.equipment_change)
            .bind(horse.lifetime_record_todays_distance_starts)
            .bind(horse.lifetime_record_todays_distance_wins)
            .bind(horse.lifetime_record_todays_distance_places)
            .bind(horse.lifetime_record_todays_distance_shows)
            .bind(horse.lifetime_record_todays_distance_earnings)
            .bind(horse.lifetime_record_todays_track_starts)
            .bind(horse.lifetime_record_todays_track_wins)
            .bind(horse.lifetime_record_todays_track_places)
            .bind(horse.lifetime_record_todays_track_shows)
            .bind(horse.lifetime_record_todays_track_earnings)
            .bind(horse.lifetime_record_turf_starts)
            .bind(horse.lifetime_record_turf_wins)
            .bind(horse.lifetime_record_turf_places)
            .bind(horse.lifetime_record_turf_shows)
            .bind(horse.lifetime_record_turf_earnings)
            .bind(horse.lifetime_record_wet_starts)
            .bind(horse.lifetime_record_wet_wins)
            .bind(horse.lifetime_record_wet_places)
            .bind(horse.lifetime_record_wet_shows)
            .bind(horse.lifetime_record_wet_earnings)
            .bind(horse.current_year_record_year)
            .bind(horse.current_year_record_starts)
            .bind(horse.current_year_record_wins)
            .bind(horse.current_year_record_places)
            .bind(horse.current_year_record_shows)
            .bind(horse.current_year_record_earnings)
            .bind(horse.previous_year_record_year)
            .bind(horse.previous_year_record_starts)
            .bind(horse.previous_year_record_wins)
            .bind(horse.previous_year_record_places)
            .bind(horse.previous_year_record_shows)
            .bind(horse.previous_year_record_earnings)
            .bind(horse.lifetime_record_starts)
            .bind(horse.lifetime_record_wins)
            .bind(horse.lifetime_record_places)
            .bind(horse.lifetime_record_shows)
            .bind(horse.lifetime_record_earnings)
            .bind(&horse.bris_run_style)
            .bind(horse.quirin_speed_points)
            .bind(horse.trainer_jockey_combo_starts)
            .bind(horse.trainer_jockey_combo_wins)
            .bind(horse.trainer_jockey_combo_places)
            .bind(horse.trainer_jockey_combo_shows)
            .bind(horse.trainer_jockey_combo_roi)
            .bind(horse.days_since_last_race)
            .bind(horse.lifetime_all_weather_starts)
            .bind(horse.lifetime_all_weather_wins)
            .bind(horse.lifetime_all_weather_places)
            .bind(horse.lifetime_all_weather_shows)
            .bind(horse.lifetime_all_weather_earnings)
            .bind(horse.best_bris_speed_all_weather_surface)
            .bind(horse.bris_prime_power_rating)
            .bind(horse.trainer_starts_current_year)
            .bind(horse.trainer_wins_current_year)
            .bind(horse.trainer_places_current_year)
            .bind(horse.trainer_shows_current_year)
            .bind(horse.trainer_roi_current_year)
            .bind(horse.trainer_starts_previous_year)
            .bind(horse.trainer_wins_previous_year)
            .bind(horse.trainer_places_previous_year)
            .bind(horse.trainer_shows_previous_year)
            .bind(horse.trainer_roi_previous_year)
            .bind(horse.jockey_starts_current_year)
            .bind(horse.jockey_wins_current_year)
            .bind(horse.jockey_places_current_year)
            .bind(horse.jockey_shows_current_year)
            .bind(horse.jockey_roi_current_year)
            .bind(horse.jockey_starts_previous_year)
            .bind(horse.jockey_wins_previous_year)
            .bind(horse.jockey_places_previous_year)
            .bind(horse.jockey_shows_previous_year)
            .bind(horse.jockey_roi_previous_year)
            .bind(horse.sire_stud_fee)
            .bind(horse.best_bris_speed_fast_track)
            .bind(horse.best_bris_speed_turf)
            .bind(horse.best_bris_speed_off_track)
            .bind(horse.best_bris_speed_distance)
            .bind(horse.auction_price)
            .bind(&horse.where_when_sold_at_auction)
            .bind(&horse.bris_dirt_pedigree_rating)
            .bind(&horse.bris_mud_pedigree_rating)
            .bind(&horse.bris_turf_pedigree_rating)
            .bind(&horse.bris_distance_pedigree_rating)
            .bind(horse.best_bris_speed_life)
            .bind(horse.best_bris_speed_most_recent_year)
            .bind(horse.best_bris_speed_2nd_most_recent_year)
            .bind(horse.best_bris_speed_todays_track)
            .bind(horse.starts_fast_dirt)
            .bind(horse.wins_fast_dirt)
            .bind(horse.places_fast_dirt)
            .bind(horse.shows_fast_dirt)
            .bind(horse.earnings_fast_dirt)
            .bind(&horse.jockey_distance_turf_label)
            .bind(horse.jockey_distance_turf_starts)
            .bind(horse.jockey_distance_turf_wins)
            .bind(horse.jockey_distance_turf_places)
            .bind(horse.jockey_distance_turf_shows)
            .bind(horse.jockey_distance_turf_roi)
            .bind(horse.jockey_distance_turf_earnings)
            .bind(horse.trainer_jockey_combo_starts_meet)
            .bind(horse.trainer_jockey_combo_wins_meet)
            .bind(horse.trainer_jockey_combo_places_meet)
            .bind(horse.trainer_jockey_combo_shows_meet)
            .bind(horse.trainer_jockey_combo_roi_meet)
            .bind(&horse.note)
            .execute(&mut *tx)
            .await?;
            horse.id = result.last_insert_rowid();

            for workout in &mut horse.workouts {
                workout.horse_id = horse.id;
                let result = sqlx::query(
                    r#"
                    INSERT INTO workouts (
                        horse_id,
                        date,
                        time,
                        track,
                        distance,
                        condition,
                        description,
                        main_inner_track_indicator,
                        workouts_that_day_distance,
                        rank
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
                    "#,
                )
                .bind(workout.horse_id)
                .bind(&workout.date)
                .bind(workout.time)
                .bind(&workout.track)
                .bind(workout.distance)
                .bind(&workout.condition)
                .bind(&workout.description)
                .bind(&workout.main_inner_track_indicator)
                .bind(workout.workouts_that_day_distance)
                .bind(workout.rank)
                .execute(&mut *tx)
                .await?;
                workout.id = result.last_insert_rowid();
            }

            for past_performance in &mut horse.past_performances {
                past_performance.horse_id = horse.id;
                let past_sql = format!(
                    r#"
                    INSERT INTO past_performances (
                        horse_id,
                        race_date,
                        days_since_last_race,
                        track_code,
                        bris_track_code,
                        race_number,
                        track_condition,
                        distance,
                        surface,
                        special_chute_indicator,
                        entrants,
                        post_position,
                        equipment,
                        racename,
                        medication,
                        trip_comment,
                        winners_name,
                        place_name,
                        show_name,
                        winners_weight,
                        place_weight,
                        show_weight,
                        winners_margin,
                        place_margin,
                        show_margin,
                        alternate_comment_line,
                        weight,
                        odds,
                        entry,
                        race_classication,
                        claiming_price,
                        purse,
                        start_call_position,
                        first_call_position,
                        second_call_position,
                        gate_call_position,
                        stretch_call_position,
                        finish_position,
                        money_position,
                        start_call_between_lengths_leader,
                        start_call_between_lengths,
                        first_call_between_lengths_leader,
                        first_call_between_lengths,
                        second_call_between_lengths_leader,
                        second_call_between_lengths,
                        bris_race_shape_1st_call,
                        stretch_call_between_lengths_leader,
                        stretch_call_between_lengths,
                        finish_between_lengths_leader,
                        finish_between_lengths,
                        bris_race_shape_2nd_call,
                        bris_2f_pace,
                        bris_4f_pace,
                        bris_6f_pace,
                        bris_8f_pace,
                        bris_10f_pace,
                        bris_late_pace,
                        bris_speed_rating,
                        speed_rating,
                        track_variant,
                        two_f_fraction,
                        three_f_fraction,
                        four_f_fraction,
                        five_f_fraction,
                        six_f_fraction,
                        seven_f_fraction,
                        eight_f_fraction,
                        ten_f_fraction,
                        twelve_f_fraction,
                        fourteen_f_fraction,
                        sixteen_f_fraction,
                        fraction_1,
                        fraction_2,
                        fraction_3,
                        final_time,
                        claimed_code,
                        trainer,
                        jockey,
                        apprentice_weight_allowance,
                        race_type,
                        age_sex_restrictions,
                        statebred_flag,
                        restricted_qualifier_flag,
                        favorite_indicator,
                        front_bandages_indicator,
                        bris_speed_par_for_race,
                        bar_shoes,
                        company_line_codes,
                        low_claiming_price_of_race,
                        high_claiming_price_of_race,
                        code_for_prior_races,
                        claimed_and_trainer_switches_1,
                        claimed_and_trainer_switches_2,
                        claimed_and_trainer_switches_3,
                        claimed_and_trainer_switches_4,
                        claimed_and_trainer_switches_5,
                        claimed_and_trainer_switches_6,
                        extended_start_comment,
                        sealed_track_indicator,
                        previous_all_weather_surface_indicator,
                        equibase_abbreviated_race_condition
                    )
                    VALUES ({});
                    "#,
                    placeholders(PAST_PERFORMANCE_COLUMNS)
                );
                let result = sqlx::query(&past_sql)
                .bind(past_performance.horse_id)
                .bind(&past_performance.race_date)
                .bind(past_performance.days_since_last_race)
                .bind(&past_performance.track_code)
                .bind(&past_performance.bris_track_code)
                .bind(past_performance.race_number)
                .bind(&past_performance.track_condition)
                .bind(past_performance.distance)
                .bind(&past_performance.surface)
                .bind(&past_performance.special_chute_indicator)
                .bind(past_performance.entrants)
                .bind(past_performance.post_position)
                .bind(&past_performance.equipment)
                .bind(&past_performance.racename)
                .bind(past_performance.medication)
                .bind(&past_performance.trip_comment)
                .bind(&past_performance.winners_name)
                .bind(&past_performance.place_name)
                .bind(&past_performance.show_name)
                .bind(past_performance.winners_weight)
                .bind(past_performance.place_weight)
                .bind(past_performance.show_weight)
                .bind(past_performance.winners_margin)
                .bind(past_performance.place_margin)
                .bind(past_performance.show_margin)
                .bind(&past_performance.alternate_comment_line)
                .bind(past_performance.weight)
                .bind(past_performance.odds)
                .bind(&past_performance.entry)
                .bind(&past_performance.race_classication)
                .bind(past_performance.claiming_price)
                .bind(past_performance.purse)
                .bind(&past_performance.start_call_position)
                .bind(&past_performance.first_call_position)
                .bind(&past_performance.second_call_position)
                .bind(&past_performance.gate_call_position)
                .bind(&past_performance.stretch_call_position)
                .bind(&past_performance.finish_position)
                .bind(&past_performance.money_position)
                .bind(past_performance.start_call_between_lengths_leader)
                .bind(past_performance.start_call_between_lengths)
                .bind(past_performance.first_call_between_lengths_leader)
                .bind(past_performance.first_call_between_lengths)
                .bind(past_performance.second_call_between_lengths_leader)
                .bind(past_performance.second_call_between_lengths)
                .bind(past_performance.bris_race_shape_1st_call)
                .bind(past_performance.stretch_call_between_lengths_leader)
                .bind(past_performance.stretch_call_between_lengths)
                .bind(past_performance.finish_between_lengths_leader)
                .bind(past_performance.finish_between_lengths)
                .bind(past_performance.bris_race_shape_2nd_call)
                .bind(past_performance.bris_2f_pace)
                .bind(past_performance.bris_4f_pace)
                .bind(past_performance.bris_6f_pace)
                .bind(past_performance.bris_8f_pace)
                .bind(past_performance.bris_10f_pace)
                .bind(past_performance.bris_late_pace)
                .bind(past_performance.bris_speed_rating)
                .bind(past_performance.speed_rating)
                .bind(past_performance.track_variant)
                .bind(past_performance.two_f_fraction)
                .bind(past_performance.three_f_fraction)
                .bind(past_performance.four_f_fraction)
                .bind(past_performance.five_f_fraction)
                .bind(past_performance.six_f_fraction)
                .bind(past_performance.seven_f_fraction)
                .bind(past_performance.eight_f_fraction)
                .bind(past_performance.ten_f_fraction)
                .bind(past_performance.twelve_f_fraction)
                .bind(past_performance.fourteen_f_fraction)
                .bind(past_performance.sixteen_f_fraction)
                .bind(past_performance.fraction_1)
                .bind(past_performance.fraction_2)
                .bind(past_performance.fraction_3)
                .bind(past_performance.final_time)
                .bind(&past_performance.claimed_code)
                .bind(&past_performance.trainer)
                .bind(&past_performance.jockey)
                .bind(past_performance.apprentice_weight_allowance)
                .bind(&past_performance.race_type)
                .bind(&past_performance.age_sex_restrictions)
                .bind(&past_performance.statebred_flag)
                .bind(&past_performance.restricted_qualifier_flag)
                .bind(&past_performance.favorite_indicator)
                .bind(&past_performance.front_bandages_indicator)
                .bind(past_performance.bris_speed_par_for_race)
                .bind(&past_performance.bar_shoes)
                .bind(&past_performance.company_line_codes)
                .bind(past_performance.low_claiming_price_of_race)
                .bind(past_performance.high_claiming_price_of_race)
                .bind(&past_performance.code_for_prior_races)
                .bind(&past_performance.claimed_and_trainer_switches_1)
                .bind(&past_performance.claimed_and_trainer_switches_2)
                .bind(&past_performance.claimed_and_trainer_switches_3)
                .bind(&past_performance.claimed_and_trainer_switches_4)
                .bind(&past_performance.claimed_and_trainer_switches_5)
                .bind(&past_performance.claimed_and_trainer_switches_6)
                .bind(&past_performance.extended_start_comment)
                .bind(&past_performance.sealed_track_indicator)
                .bind(&past_performance.previous_all_weather_surface_indicator)
                .bind(&past_performance.equibase_abbreviated_race_condition)
                .execute(&mut *tx)
                .await?;
                past_performance.id = result.last_insert_rowid();
            }

            for key_trainer_stat in &mut horse.key_trainer_stats {
                key_trainer_stat.horse_id = horse.id;
                let result = sqlx::query(
                    r#"
                    INSERT INTO key_trainer_stats (
                        horse_id,
                        category,
                        starts,
                        win_pct,
                        in_the_money_pct,
                        roi
                    )
                    VALUES (?, ?, ?, ?, ?, ?);
                    "#,
                )
                .bind(key_trainer_stat.horse_id)
                .bind(&key_trainer_stat.category)
                .bind(key_trainer_stat.starts)
                .bind(key_trainer_stat.win_pct)
                .bind(key_trainer_stat.in_the_money_pct)
                .bind(key_trainer_stat.roi)
                .execute(&mut *tx)
                .await?;
                key_trainer_stat.id = result.last_insert_rowid();
            }
        }
    }

    tx.commit().await?;
    Ok(racecard)
}

fn opt_i64_to_u32(value: Option<i64>) -> Option<u32> {
    value.map(|v| v as u32)
}

fn opt_i64_to_i32(value: Option<i64>) -> Option<i32> {
    value.map(|v| v as i32)
}

fn race_from_row(row: &SqliteRow) -> Race {
    Race {
        id: row.get("id"),
        racecard_id: row.get("racecard_id"),
        race_number: opt_i64_to_u32(row.get::<Option<i64>, _>("race_number")),
        distance: opt_i64_to_i32(row.get::<Option<i64>, _>("distance")),
        surface: row.get("surface"),
        race_type: row.get("race_type"),
        age_sex_restrictions: row.get("age_sex_restrictions"),
        todays_race_classification: row.get("todays_race_classification"),
        purse: opt_i64_to_u32(row.get::<Option<i64>, _>("purse")),
        claiming_price: opt_i64_to_u32(row.get::<Option<i64>, _>("claiming_price")),
        track_record: row.get("track_record"),
        race_conditions: row.get("race_conditions"),
        todays_lasix_list: row.get("todays_lasix_list"),
        todays_bute_list: row.get("todays_bute_list"),
        todays_coupled_list: row.get("todays_coupled_list"),
        todays_mutuel_list: row.get("todays_mutuel_list"),
        simulcast_host_track_code: row.get("simulcast_host_track_code"),
        simulcast_host_track_race_number: opt_i64_to_u32(
            row.get::<Option<i64>, _>("simulcast_host_track_race_number"),
        ),
        all_weather_surface_flag: row.get("all_weather_surface_flag"),
        race_conditions_line1: row.get("race_conditions_line1"),
        race_conditions_line2: row.get("race_conditions_line2"),
        race_conditions_line3: row.get("race_conditions_line3"),
        race_conditions_line4: row.get("race_conditions_line4"),
        race_conditions_line5: row.get("race_conditions_line5"),
        race_conditions_line6: row.get("race_conditions_line6"),
        low_claiming_price: opt_i64_to_u32(row.get::<Option<i64>, _>("low_claiming_price")),
        statebred_flag: row.get("statebred_flag"),
        wager_type_line1: row.get("wager_type_line1"),
        wager_type_line2: row.get("wager_type_line2"),
        wager_type_line3: row.get("wager_type_line3"),
        wager_type_line4: row.get("wager_type_line4"),
        wager_type_line5: row.get("wager_type_line5"),
        wager_type_line6: row.get("wager_type_line6"),
        wager_type_line7: row.get("wager_type_line7"),
        wager_type_line8: row.get("wager_type_line8"),
        wager_type_line9: row.get("wager_type_line9"),
        two_f_bris_pace_par: opt_i64_to_u32(row.get::<Option<i64>, _>("two_f_bris_pace_par")),
        four_f_bris_pace_par: opt_i64_to_u32(row.get::<Option<i64>, _>("four_f_bris_pace_par")),
        six_f_bris_pace_par: opt_i64_to_u32(row.get::<Option<i64>, _>("six_f_bris_pace_par")),
        bris_speed_for_class: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_speed_for_class")),
        bris_late_pace_par: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_late_pace_par")),
        post_times: row.get("post_times"),
        post_time_pacific_military: row.get("post_time_pacific_military"),
        todays_equibase_abbreviated_race_conditions: row.get(
            "todays_equibase_abbreviated_race_conditions",
        ),
        horses: Vec::new(),
    }
}

fn horse_from_row(row: &SqliteRow) -> Horse {
    Horse {
        id: row.get("id"),
        race_id: row.get("race_id"),
        post_position: opt_i64_to_u32(row.get::<Option<i64>, _>("post_position")),
        entry: row.get("entry"),
        claiming_price_of_horse: opt_i64_to_u32(row.get::<Option<i64>, _>("claiming_price_of_horse")),
        breed_type: row.get("breed_type"),
        todays_nasal_strip_change: opt_i64_to_u32(row.get::<Option<i64>, _>("todays_nasal_strip_change")),
        todays_trainer: row.get("todays_trainer"),
        trainer_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_starts")),
        trainer_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_wins")),
        trainer_places: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_places")),
        trainer_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_shows")),
        todays_jockey: row.get("todays_jockey"),
        apprentice_weight_allowance: opt_i64_to_u32(row.get::<Option<i64>, _>("apprentice_weight_allowance")),
        jockey_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_starts")),
        jockey_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_wins")),
        jockey_places: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_places")),
        jockey_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_shows")),
        todays_owner: row.get("todays_owner"),
        owners_silks: row.get("owners_silks"),
        main_track_only_ae_indicator: row.get("main_track_only_ae_indicator"),
        program_number: row.get("program_number"),
        morning_line_odds: row.get("morning_line_odds"),
        horse_name: row.get("horse_name"),
        year_of_birth: opt_i64_to_u32(row.get::<Option<i64>, _>("year_of_birth")),
        horses_foaling_month: opt_i64_to_u32(row.get::<Option<i64>, _>("horses_foaling_month")),
        sex: row.get("sex"),
        horses_color: row.get("horses_color"),
        weight: opt_i64_to_u32(row.get::<Option<i64>, _>("weight")),
        sire: row.get("sire"),
        sires_sire: row.get("sires_sire"),
        dam: row.get("dam"),
        dams_sire: row.get("dams_sire"),
        breeder: row.get("breeder"),
        state_country_where_bred: row.get("state_country_where_bred"),
        program_post_position: row.get("program_post_position"),
        todays_medication_new: opt_i64_to_u32(row.get::<Option<i64>, _>("todays_medication_new")),
        todays_medication_old: opt_i64_to_u32(row.get::<Option<i64>, _>("todays_medication_old")),
        equipment_change: opt_i64_to_u32(row.get::<Option<i64>, _>("equipment_change")),
        lifetime_record_todays_distance_starts: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_distance_starts"),
        ),
        lifetime_record_todays_distance_wins: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_distance_wins"),
        ),
        lifetime_record_todays_distance_places: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_distance_places"),
        ),
        lifetime_record_todays_distance_shows: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_distance_shows"),
        ),
        lifetime_record_todays_distance_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_distance_earnings"),
        ),
        lifetime_record_todays_track_starts: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_track_starts"),
        ),
        lifetime_record_todays_track_wins: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_track_wins"),
        ),
        lifetime_record_todays_track_places: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_track_places"),
        ),
        lifetime_record_todays_track_shows: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_track_shows"),
        ),
        lifetime_record_todays_track_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_todays_track_earnings"),
        ),
        lifetime_record_turf_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_turf_starts")),
        lifetime_record_turf_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_turf_wins")),
        lifetime_record_turf_places: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_turf_places")),
        lifetime_record_turf_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_turf_shows")),
        lifetime_record_turf_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_turf_earnings"),
        ),
        lifetime_record_wet_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_wet_starts")),
        lifetime_record_wet_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_wet_wins")),
        lifetime_record_wet_places: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_wet_places")),
        lifetime_record_wet_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_wet_shows")),
        lifetime_record_wet_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_record_wet_earnings"),
        ),
        current_year_record_year: opt_i64_to_u32(row.get::<Option<i64>, _>("current_year_record_year")),
        current_year_record_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("current_year_record_starts")),
        current_year_record_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("current_year_record_wins")),
        current_year_record_places: opt_i64_to_u32(row.get::<Option<i64>, _>("current_year_record_places")),
        current_year_record_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("current_year_record_shows")),
        current_year_record_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("current_year_record_earnings"),
        ),
        previous_year_record_year: opt_i64_to_u32(row.get::<Option<i64>, _>("previous_year_record_year")),
        previous_year_record_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("previous_year_record_starts")),
        previous_year_record_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("previous_year_record_wins")),
        previous_year_record_places: opt_i64_to_u32(row.get::<Option<i64>, _>("previous_year_record_places")),
        previous_year_record_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("previous_year_record_shows")),
        previous_year_record_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("previous_year_record_earnings"),
        ),
        lifetime_record_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_starts")),
        lifetime_record_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_wins")),
        lifetime_record_places: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_places")),
        lifetime_record_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_shows")),
        lifetime_record_earnings: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_record_earnings")),
        bris_run_style: row.get("bris_run_style"),
        quirin_speed_points: opt_i64_to_u32(row.get::<Option<i64>, _>("quirin_speed_points")),
        trainer_jockey_combo_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_jockey_combo_starts")),
        trainer_jockey_combo_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_jockey_combo_wins")),
        trainer_jockey_combo_places: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_jockey_combo_places")),
        trainer_jockey_combo_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_jockey_combo_shows")),
        trainer_jockey_combo_roi: row.get("trainer_jockey_combo_roi"),
        days_since_last_race: opt_i64_to_u32(row.get::<Option<i64>, _>("days_since_last_race")),
        lifetime_all_weather_starts: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_all_weather_starts")),
        lifetime_all_weather_wins: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_all_weather_wins")),
        lifetime_all_weather_places: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_all_weather_places")),
        lifetime_all_weather_shows: opt_i64_to_u32(row.get::<Option<i64>, _>("lifetime_all_weather_shows")),
        lifetime_all_weather_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("lifetime_all_weather_earnings"),
        ),
        best_bris_speed_all_weather_surface: opt_i64_to_u32(
            row.get::<Option<i64>, _>("best_bris_speed_all_weather_surface"),
        ),
        bris_prime_power_rating: row.get("bris_prime_power_rating"),
        trainer_starts_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_starts_current_year")),
        trainer_wins_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_wins_current_year")),
        trainer_places_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_places_current_year")),
        trainer_shows_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_shows_current_year")),
        trainer_roi_current_year: row.get("trainer_roi_current_year"),
        trainer_starts_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_starts_previous_year")),
        trainer_wins_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_wins_previous_year")),
        trainer_places_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_places_previous_year")),
        trainer_shows_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("trainer_shows_previous_year")),
        trainer_roi_previous_year: row.get("trainer_roi_previous_year"),
        jockey_starts_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_starts_current_year")),
        jockey_wins_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_wins_current_year")),
        jockey_places_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_places_current_year")),
        jockey_shows_current_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_shows_current_year")),
        jockey_roi_current_year: row.get("jockey_roi_current_year"),
        jockey_starts_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_starts_previous_year")),
        jockey_wins_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_wins_previous_year")),
        jockey_places_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_places_previous_year")),
        jockey_shows_previous_year: opt_i64_to_u32(row.get::<Option<i64>, _>("jockey_shows_previous_year")),
        jockey_roi_previous_year: row.get("jockey_roi_previous_year"),
        sire_stud_fee: opt_i64_to_u32(row.get::<Option<i64>, _>("sire_stud_fee")),
        best_bris_speed_fast_track: opt_i64_to_u32(row.get::<Option<i64>, _>("best_bris_speed_fast_track")),
        best_bris_speed_turf: opt_i64_to_u32(row.get::<Option<i64>, _>("best_bris_speed_turf")),
        best_bris_speed_off_track: opt_i64_to_u32(row.get::<Option<i64>, _>("best_bris_speed_off_track")),
        best_bris_speed_distance: opt_i64_to_i32(row.get::<Option<i64>, _>("best_bris_speed_distance")),
        auction_price: opt_i64_to_u32(row.get::<Option<i64>, _>("auction_price")),
        where_when_sold_at_auction: row.get("where_when_sold_at_auction"),
        bris_dirt_pedigree_rating: row.get("bris_dirt_pedigree_rating"),
        bris_mud_pedigree_rating: row.get("bris_mud_pedigree_rating"),
        bris_turf_pedigree_rating: row.get("bris_turf_pedigree_rating"),
        bris_distance_pedigree_rating: row.get("bris_distance_pedigree_rating"),
        best_bris_speed_life: opt_i64_to_u32(row.get::<Option<i64>, _>("best_bris_speed_life")),
        best_bris_speed_most_recent_year: opt_i64_to_u32(
            row.get::<Option<i64>, _>("best_bris_speed_most_recent_year"),
        ),
        best_bris_speed_2nd_most_recent_year: opt_i64_to_u32(
            row.get::<Option<i64>, _>("best_bris_speed_2nd_most_recent_year"),
        ),
        best_bris_speed_todays_track: opt_i64_to_u32(
            row.get::<Option<i64>, _>("best_bris_speed_todays_track"),
        ),
        starts_fast_dirt: opt_i64_to_u32(row.get::<Option<i64>, _>("starts_fast_dirt")),
        wins_fast_dirt: opt_i64_to_u32(row.get::<Option<i64>, _>("wins_fast_dirt")),
        places_fast_dirt: opt_i64_to_u32(row.get::<Option<i64>, _>("places_fast_dirt")),
        shows_fast_dirt: opt_i64_to_u32(row.get::<Option<i64>, _>("shows_fast_dirt")),
        earnings_fast_dirt: opt_i64_to_u32(row.get::<Option<i64>, _>("earnings_fast_dirt")),
        jockey_distance_turf_label: row.get("jockey_distance_turf_label"),
        jockey_distance_turf_starts: opt_i64_to_u32(
            row.get::<Option<i64>, _>("jockey_distance_turf_starts"),
        ),
        jockey_distance_turf_wins: opt_i64_to_u32(
            row.get::<Option<i64>, _>("jockey_distance_turf_wins"),
        ),
        jockey_distance_turf_places: opt_i64_to_u32(
            row.get::<Option<i64>, _>("jockey_distance_turf_places"),
        ),
        jockey_distance_turf_shows: opt_i64_to_u32(
            row.get::<Option<i64>, _>("jockey_distance_turf_shows"),
        ),
        jockey_distance_turf_roi: row.get("jockey_distance_turf_roi"),
        jockey_distance_turf_earnings: opt_i64_to_u32(
            row.get::<Option<i64>, _>("jockey_distance_turf_earnings"),
        ),
        trainer_jockey_combo_starts_meet: opt_i64_to_u32(
            row.get::<Option<i64>, _>("trainer_jockey_combo_starts_meet"),
        ),
        trainer_jockey_combo_wins_meet: opt_i64_to_u32(
            row.get::<Option<i64>, _>("trainer_jockey_combo_wins_meet"),
        ),
        trainer_jockey_combo_places_meet: opt_i64_to_u32(
            row.get::<Option<i64>, _>("trainer_jockey_combo_places_meet"),
        ),
        trainer_jockey_combo_shows_meet: opt_i64_to_u32(
            row.get::<Option<i64>, _>("trainer_jockey_combo_shows_meet"),
        ),
        trainer_jockey_combo_roi_meet: row.get("trainer_jockey_combo_roi_meet"),
        note: row.get("note"),
        workouts: Vec::new(),
        past_performances: Vec::new(),
        key_trainer_stats: Vec::new(),
    }
}

fn workout_from_row(row: &SqliteRow) -> Workout {
    Workout {
        id: row.get("id"),
        horse_id: row.get("horse_id"),
        date: row.get("date"),
        time: row.get("time"),
        track: row.get("track"),
        distance: opt_i64_to_i32(row.get::<Option<i64>, _>("distance")),
        condition: row.get("condition"),
        description: row.get("description"),
        main_inner_track_indicator: row.get("main_inner_track_indicator"),
        workouts_that_day_distance: opt_i64_to_u32(
            row.get::<Option<i64>, _>("workouts_that_day_distance"),
        ),
        rank: opt_i64_to_u32(row.get::<Option<i64>, _>("rank")),
    }
}

fn past_performance_from_row(row: &SqliteRow) -> PastPerformance {
    PastPerformance {
        id: row.get("id"),
        horse_id: row.get("horse_id"),
        race_date: row.get("race_date"),
        days_since_last_race: opt_i64_to_u32(row.get::<Option<i64>, _>("days_since_last_race")),
        track_code: row.get("track_code"),
        bris_track_code: row.get("bris_track_code"),
        race_number: opt_i64_to_u32(row.get::<Option<i64>, _>("race_number")),
        track_condition: row.get("track_condition"),
        distance: opt_i64_to_i32(row.get::<Option<i64>, _>("distance")),
        surface: row.get("surface"),
        special_chute_indicator: row.get("special_chute_indicator"),
        entrants: opt_i64_to_u32(row.get::<Option<i64>, _>("entrants")),
        post_position: opt_i64_to_u32(row.get::<Option<i64>, _>("post_position")),
        equipment: row.get("equipment"),
        racename: row.get("racename"),
        medication: opt_i64_to_u32(row.get::<Option<i64>, _>("medication")),
        trip_comment: row.get("trip_comment"),
        winners_name: row.get("winners_name"),
        place_name: row.get("place_name"),
        show_name: row.get("show_name"),
        winners_weight: opt_i64_to_u32(row.get::<Option<i64>, _>("winners_weight")),
        place_weight: opt_i64_to_u32(row.get::<Option<i64>, _>("place_weight")),
        show_weight: opt_i64_to_u32(row.get::<Option<i64>, _>("show_weight")),
        winners_margin: row.get("winners_margin"),
        place_margin: row.get("place_margin"),
        show_margin: row.get("show_margin"),
        alternate_comment_line: row.get("alternate_comment_line"),
        weight: opt_i64_to_u32(row.get::<Option<i64>, _>("weight")),
        odds: row.get("odds"),
        entry: row.get("entry"),
        race_classication: row.get("race_classication"),
        claiming_price: opt_i64_to_u32(row.get::<Option<i64>, _>("claiming_price")),
        purse: opt_i64_to_u32(row.get::<Option<i64>, _>("purse")),
        start_call_position: row.get("start_call_position"),
        first_call_position: row.get("first_call_position"),
        second_call_position: row.get("second_call_position"),
        gate_call_position: row.get("gate_call_position"),
        stretch_call_position: row.get("stretch_call_position"),
        finish_position: row.get("finish_position"),
        money_position: row.get("money_position"),
        start_call_between_lengths_leader: row.get("start_call_between_lengths_leader"),
        start_call_between_lengths: row.get("start_call_between_lengths"),
        first_call_between_lengths_leader: row.get("first_call_between_lengths_leader"),
        first_call_between_lengths: row.get("first_call_between_lengths"),
        second_call_between_lengths_leader: row.get("second_call_between_lengths_leader"),
        second_call_between_lengths: row.get("second_call_between_lengths"),
        bris_race_shape_1st_call: opt_i64_to_u32(
            row.get::<Option<i64>, _>("bris_race_shape_1st_call"),
        ),
        stretch_call_between_lengths_leader: row.get("stretch_call_between_lengths_leader"),
        stretch_call_between_lengths: row.get("stretch_call_between_lengths"),
        finish_between_lengths_leader: row.get("finish_between_lengths_leader"),
        finish_between_lengths: row.get("finish_between_lengths"),
        bris_race_shape_2nd_call: opt_i64_to_u32(
            row.get::<Option<i64>, _>("bris_race_shape_2nd_call"),
        ),
        bris_2f_pace: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_2f_pace")),
        bris_4f_pace: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_4f_pace")),
        bris_6f_pace: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_6f_pace")),
        bris_8f_pace: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_8f_pace")),
        bris_10f_pace: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_10f_pace")),
        bris_late_pace: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_late_pace")),
        bris_speed_rating: opt_i64_to_u32(row.get::<Option<i64>, _>("bris_speed_rating")),
        speed_rating: opt_i64_to_u32(row.get::<Option<i64>, _>("speed_rating")),
        track_variant: opt_i64_to_i32(row.get::<Option<i64>, _>("track_variant")),
        two_f_fraction: row.get("two_f_fraction"),
        three_f_fraction: row.get("three_f_fraction"),
        four_f_fraction: row.get("four_f_fraction"),
        five_f_fraction: row.get("five_f_fraction"),
        six_f_fraction: row.get("six_f_fraction"),
        seven_f_fraction: row.get("seven_f_fraction"),
        eight_f_fraction: row.get("eight_f_fraction"),
        ten_f_fraction: row.get("ten_f_fraction"),
        twelve_f_fraction: row.get("twelve_f_fraction"),
        fourteen_f_fraction: row.get("fourteen_f_fraction"),
        sixteen_f_fraction: row.get("sixteen_f_fraction"),
        fraction_1: row.get("fraction_1"),
        fraction_2: row.get("fraction_2"),
        fraction_3: row.get("fraction_3"),
        final_time: row.get("final_time"),
        claimed_code: row.get("claimed_code"),
        trainer: row.get("trainer"),
        jockey: row.get("jockey"),
        apprentice_weight_allowance: opt_i64_to_u32(
            row.get::<Option<i64>, _>("apprentice_weight_allowance"),
        ),
        race_type: row.get("race_type"),
        age_sex_restrictions: row.get("age_sex_restrictions"),
        statebred_flag: row.get("statebred_flag"),
        restricted_qualifier_flag: row.get("restricted_qualifier_flag"),
        favorite_indicator: row.get("favorite_indicator"),
        front_bandages_indicator: row.get("front_bandages_indicator"),
        bris_speed_par_for_race: opt_i64_to_u32(
            row.get::<Option<i64>, _>("bris_speed_par_for_race"),
        ),
        bar_shoes: row.get("bar_shoes"),
        company_line_codes: row.get("company_line_codes"),
        low_claiming_price_of_race: opt_i64_to_u32(
            row.get::<Option<i64>, _>("low_claiming_price_of_race"),
        ),
        high_claiming_price_of_race: opt_i64_to_u32(
            row.get::<Option<i64>, _>("high_claiming_price_of_race"),
        ),
        code_for_prior_races: row.get("code_for_prior_races"),
        claimed_and_trainer_switches_1: row.get("claimed_and_trainer_switches_1"),
        claimed_and_trainer_switches_2: row.get("claimed_and_trainer_switches_2"),
        claimed_and_trainer_switches_3: row.get("claimed_and_trainer_switches_3"),
        claimed_and_trainer_switches_4: row.get("claimed_and_trainer_switches_4"),
        claimed_and_trainer_switches_5: row.get("claimed_and_trainer_switches_5"),
        claimed_and_trainer_switches_6: row.get("claimed_and_trainer_switches_6"),
        extended_start_comment: row.get("extended_start_comment"),
        sealed_track_indicator: row.get("sealed_track_indicator"),
        previous_all_weather_surface_indicator: row.get("previous_all_weather_surface_indicator"),
        equibase_abbreviated_race_condition: row.get("equibase_abbreviated_race_condition"),
    }
}

fn key_trainer_stat_from_row(row: &SqliteRow) -> KeyTrainerStat {
    KeyTrainerStat {
        id: row.get("id"),
        horse_id: row.get("horse_id"),
        category: row.get("category"),
        starts: opt_i64_to_u32(row.get::<Option<i64>, _>("starts")),
        win_pct: row.get("win_pct"),
        in_the_money_pct: row.get("in_the_money_pct"),
        roi: row.get("roi"),
    }
}
