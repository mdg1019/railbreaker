use sqlx::SqlitePool;
use tauri::State;
use crate::models::racecard::Racecard;

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
