use tokio::fs;
use crate::models::racecard::{Racecard, Race, Horse, Workout};
use crate::states::global_state::global_state;
use crate::constants::single_file_indexes::*;

#[tauri::command]
pub async fn process_racecard_file<'a>(path: String) -> Result<Racecard, String> {
    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to racecard file: {}", e))?;

    let lines: Vec<Vec<String>> = contents
        .lines()
        .map(|line| {
            line.split(',')
                .map(|field| field.trim().trim_matches('"').to_string())
                .collect()
        })
        .collect();

    let number_of_columns = lines.first().ok_or("Racecard file is empty")?.len();

    for (i, line) in lines.iter().enumerate() {
        if line.len() != number_of_columns {
            return Err(format!("Inconsistent number of columns at line {}", i + 1));
        }
    }

    let track_code = &lines[0][SF_TRACK];
    let track_name = {
        let gs = global_state().lock().unwrap();
        gs.tracks
            .get(track_code)
            .cloned()
            .unwrap_or_else(|| track_code.clone())
    };

    let mut races = Vec::<Race>::new();

    for line in &lines {
        let race_number = line[SF_RACE_NUMBER].parse::<u32>().ok();

        let race_index = races.iter().position(|r| r.race_number == race_number);

        let race_idx = if let Some(idx) = race_index {
            idx
        } else {
            let race = Race {
                race_number: race_number,
                distance: line[SF_DISTANCE].parse::<i32>().ok(),
                surface: line[SF_SURFACE].clone(),
                race_type: line[SF_RACE_TYPE].clone(),
                age_sex_restrictions: line[SF_AGE_SEX_RESTRICTIONS].clone(),
                todays_race_classification: line[SF_TODAYS_RACE_CLASSIFICATION].clone(),
                purse: line[SF_PURSE].parse::<u32>().ok(),
                claiming_price: line[SF_CLAIMING_PRICE].parse::<u32>().ok(),
                track_record: line[SF_TRACK_RECORD].parse::<f64>().ok(),
                race_conditions: line[SF_RACE_CONDITIONS].clone(),
                todays_lasix_list: line[SF_TODAYS_LASIX_LIST].clone(),
                todays_bute_list: line[SF_TODAYS_BUTE_LIST].clone(),
                todays_coupled_list: line[SF_TODAYS_COUPLED_LIST].clone(),
                todays_mutuel_list: line[SF_TODAYS_MUTUEL_LIST].clone(),
                simulcast_host_track_code: line[SF_SIMULCAST_HOST_TRACK_CODE].clone(),
                simulcast_host_track_race_number: line[SF_SIMULCAST_HOST_TRACK_RACE_NUMBER].parse::<u32>().ok(),
                all_weather_surface_flag: line[SF_TODAYS_ALL_WEATHER_SURFACE_FLAG].clone(),
                horses: Vec::new(),
                race_conditions_line1: line[SF_RACE_CONDITIONS_LINE1].clone(),
                race_conditions_line2: line[SF_RACE_CONDITIONS_LINE2].clone(),
                race_conditions_line3: line[SF_RACE_CONDITIONS_LINE3].clone(),
                race_conditions_line4: line[SF_RACE_CONDITIONS_LINE4].clone(),
                race_conditions_line5: line[SF_RACE_CONDITIONS_LINE5].clone(),
                race_conditions_line6: line[SF_RACE_CONDITIONS_LINE6].clone(),
            };
            races.push(race);
            races.len() - 1
        };

        let mut horse = Horse {
            post_position: line[SF_POST_POSITION].parse::<u32>().ok(),
            entry: line[SF_ENTRY].clone(),
            claiming_price_of_horse: line[SF_CLAIMING_PRICE_OF_HORSE].parse::<u32>().ok(),
            breed_type: line[SF_BREED_TYPE].clone(),
            todays_nasal_strip_change: line[SF_TODAYS_NASAL_STRIP_CHANGE].parse::<u32>().ok(),
            todays_trainer: line[SF_TODAYS_TRAINER].clone(),
            trainer_starts: line[SF_TRAINER_STARTS].parse::<u32>().ok(),
            trainer_wins: line[SF_TRAINER_WINS].parse::<u32>().ok(),
            trainer_places: line[SF_TRAINER_PLACES].parse::<u32>().ok(),
            trainer_shows: line[SF_TRAINER_SHOWS].parse::<u32>().ok(),
            todays_jockey: line[SF_TODAYS_JOCKEY].clone(),
            apprentice_weight_allowance: line[SF_APPRENTICE_WEIGHT_ALLOWANCE].parse::<u32>().ok(),
            jockey_starts: line[SF_JOCKEY_STARTS].parse::<u32>().ok(),
            jockey_wins: line[SF_JOCKEY_WINS].parse::<u32>().ok(),
            jockey_places: line[SF_JOCKEY_PLACES].parse::<u32>().ok(),
            jockey_shows: line[SF_JOCKEY_SHOWS].parse::<u32>().ok(),
            todays_owner: line[SF_TODAYS_OWNER].clone(),
            owners_silks: line[SF_OWNERS_SILKS].clone(),
            main_track_only_ae_indicator: line[SF_MAIN_TRACK_ONLY_AE_INDICATOR].clone(),
            program_number: line[SF_PROGRAM_NUMBER].clone(),
            morning_line_odds: line[SF_MORNING_LINE_ODDS].parse::<f64>().ok(),
            horse_name: line[SF_HORSE_NAME].clone(),
            year_of_birth: line[SF_YEAR_OF_BIRTH].parse::<u32>().ok(),
            horses_foaling_month: line[SF_HORSES_FOALING_MONTH].parse::<u32>().ok(),
            sex: line[SF_SEX].clone(),
            horses_color: line[SF_HORSES_COLOR].clone(),
            weight: line[SF_WEIGHT].parse::<u32>().ok(),
            sire: line[SF_SIRE].clone(),
            sires_sire: line[SF_SIRES_SIRE].clone(),
            dam: line[SF_DAM].clone(),
            dams_sire: line[SF_DAMS_SIRE].clone(),
            breeder: line[SF_BREEDER].clone(),
            state_country_where_bred: line[SF_STATE_COUNTRY_WHERE_BRED].clone(),
            program_post_position: line[SF_PROGRAM_POST_POSITION].clone(),
            todays_medication_new: line[SF_TODAYS_MEDICATION_NEW].parse::<u32>().ok(),
            todays_medication_old: line[SF_TODAYS_MEDICATION_OLD].parse::<u32>().ok(),
            equipment_change: line[SF_EQUIPMENT_CHANGE].parse::<u32>().ok(),
            lifetime_record_todays_distance_starts: line[SF_LIFETIME_RECORD_TODAYS_DISTANCE_STARTS].parse::<u32>().ok(),
            lifetime_record_todays_distance_wins: line[SF_LIFETIME_RECORD_TODAYS_DISTANCE_WINS].parse::<u32>().ok(),
            lifetime_record_todays_distance_places: line[SF_LIFETIME_RECORD_TODAYS_DISTANCE_PLACES].parse::<u32>().ok(),
            lifetime_record_todays_distance_shows: line[SF_LIFETIME_RECORD_TODAYS_DISTANCE_SHOWS].parse::<u32>().ok(),
            lifetime_record_todays_distance_earnings: line[SF_LIFETIME_RECORD_TODAYS_DISTANCE_EARNINGS].parse::<u32>().ok(),
            lifetime_record_todays_track_starts: line[SF_LIFETIME_RECORD_TODAYS_TRACK_STARTS].parse::<u32>().ok(),
            lifetime_record_todays_track_wins: line[SF_LIFETIME_RECORD_TODAYS_TRACK_WINS].parse::<u32>().ok(),
            lifetime_record_todays_track_places: line[SF_LIFETIME_RECORD_TODAYS_TRACK_PLACES].parse::<u32>().ok(),
            lifetime_record_todays_track_shows: line[SF_LIFETIME_RECORD_TODAYS_TRACK_SHOWS].parse::<u32>().ok(),
            lifetime_record_todays_track_earnings: line[SF_LIFETIME_RECORD_TODAYS_TRACK_EARNINGS].parse::<u32>().ok(),
            lifetime_record_turf_starts: line[SF_LIFETIME_RECORD_TURF_STARTS].parse::<u32>().ok(),
            lifetime_record_turf_wins: line[SF_LIFETIME_RECORD_TURF_WINS].parse::<u32>().ok(),
            lifetime_record_turf_places: line[SF_LIFETIME_RECORD_TURF_PLACES].parse::<u32>().ok(),
            lifetime_record_turf_shows: line[SF_LIFETIME_RECORD_TURF_SHOWS].parse::<u32>().ok(),
            lifetime_record_turf_earnings: line[SF_LIFETIME_RECORD_TURF_EARNINGS].parse::<u32>().ok(),
            lifetime_record_wet_starts: line[SF_LIFETIME_RECORD_WET_STARTS].parse::<u32>().ok(),
            lifetime_record_wet_wins: line[SF_LIFETIME_RECORD_WET_WINS].parse::<u32>().ok(),
            lifetime_record_wet_places: line[SF_LIFETIME_RECORD_WET_PLACES].parse::<u32>().ok(),
            lifetime_record_wet_shows: line[SF_LIFETIME_RECORD_WET_SHOWS].parse::<u32>().ok(),
            lifetime_record_wet_earnings: line[SF_LIFETIME_RECORD_WET_EARNINGS].parse::<u32>().ok(),
            current_year_record_year: line[SF_CURRENT_YEAR_RECORD_YEAR].parse::<u32>().ok(),
            current_year_record_starts: line[SF_CURRENT_YEAR_RECORD_STARTS].parse::<u32>().ok(),
            current_year_record_wins: line[SF_CURRENT_YEAR_RECORD_WINS].parse::<u32>().ok(),
            current_year_record_places: line[SF_CURRENT_YEAR_RECORD_PLACES].parse::<u32>().ok(),
            current_year_record_shows: line[SF_CURRENT_YEAR_RECORD_SHOWS].parse::<u32>().ok(),
            current_year_record_earnings: line[SF_CURRENT_YEAR_RECORD_EARNINGS].parse::<u32>().ok(),    
            previous_year_record_year: line[SF_PREVIOUS_YEAR_RECORD_YEAR].parse::<u32>().ok(),
            previous_year_record_starts: line[SF_PREVIOUS_YEAR_RECORD_STARTS].parse::<u32>().ok(),
            previous_year_record_wins: line[SF_PREVIOUS_YEAR_RECORD_WINS].parse::<u32>().ok(),
            previous_year_record_places: line[SF_PREVIOUS_YEAR_RECORD_PLACES].parse::<u32>().ok(),
            previous_year_record_shows: line[SF_PREVIOUS_YEAR_RECORD_SHOWS].parse::<u32>().ok(),
            previous_year_record_earnings: line[SF_PREVIOUS_YEAR_RECORD_EARNINGS].parse::<u32>().ok(),  
            lifetime_record_starts: line[SF_LIFETIME_RECORD_STARTS].parse::<u32>().ok(),    
            lifetime_record_wins: line[SF_LIFETIME_RECORD_WINS].parse::<u32>().ok(),    
            lifetime_record_places: line[SF_LIFETIME_RECORD_PLACES].parse::<u32>().ok(),    
            lifetime_record_shows: line[SF_LIFETIME_RECORD_SHOWS].parse::<u32>().ok(),    
            lifetime_record_earnings: line[SF_LIFETIME_RECORD_EARNINGS].parse::<u32>().ok(),           
            bris_run_style: line[SF_BRIS_RUN_STYLE].clone(),
            quirin_speed_points: line[SF_QUIRIN_STYLE_SPEED_POINTS].parse::<u32>().ok(),
            twof_bris_pace_par: line[SF_2F_BRIS_PACE_PAR].parse::<u32>().ok(),
            fourf_bris_pace_par: line[SF_4F_BRIS_PACE_PAR].parse::<u32>().ok(),
            sixf_bris_pace_par: line[SF_6F_BRIS_PACE_PAR].parse::<u32>().ok(),
            bris_speed_for_class: line[SF_BRIS_SPEED_PAR_FOR_CLASS].parse::<u32>().ok(),
            bris_late_pace_par: line[SF_BRIS_LATE_PACE_PAR].parse::<u32>().ok(),
            trainer_jockey_combination_starts: line[SF_TRAINER_JOCKEY_COMBINATION_STARTS].parse::<u32>().ok(),
            trainer_jockey_combination_wins: line[SF_TRAINER_JOCKEY_COMBINATION_WINS].parse::<u32>().ok(),
            trainer_jockey_combination_places: line[SF_TRAINER_JOCKEY_COMBINATION_PLACES].parse::<u32>().ok(),
            trainer_jockey_combination_shows: line[SF_TRAINER_JOCKEY_COMBINATION_SHOWS].parse::<u32>().ok(),
            trainer_two_dollar_roi: line[SF_TRAINER_TWO_DOLLAR_ROI].parse::<u32>().ok(),
            number_of_days_since_last_race: line[SF_NUMBER_OF_DAYS_SINCE_LAST_RACE].parse::<u32>().ok(),
            workouts: Vec::new(),
        };

        for j in 0..12 {
            let workout = Workout {
                date: line[SF_WORKOUT_DATE + j].clone(),
                time: line[SF_WORKOUT_TIME + j].parse::<f64>().ok(),
                track: line[SF_WORKOUT_TRACK + j].clone(),
                distance: line[SF_WORKOUT_DISTANCE + j].parse::<i32>().ok(),
                condition: line[SF_WORKOUT_CONDITION + j].clone(),
                description: line[SF_WORKOUT_DESCRIPTION + j].clone(),
                main_inner_track_indicator: line[SF_WORKOUT_MAIN_INNER_TRACK_INDICATOR + j].clone(),   
                number_of_workouts_that_day_distance: line[SF_WORKOUT_NUMBER_OF_WORKOUTS_THAT_DAY_DISTANCE + j].parse::<u32>().ok(),
                rank: line[SF_WORKOUT_RANK + j].parse::<u32>().ok(),
            };  

            horse.workouts.push(workout);
        }

        races[race_idx].horses.push(horse);
    }

    let racecard = Racecard {
        track: track_name,
        date: lines[0][SF_RACE_DATE].clone(),
        races: races,
    };

    // println!("{:?}", racecard);
    Ok(racecard)
}