use tokio::fs;
use crate::models::racecard::{Racecard, Race, Horse};
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
            };
            races.push(race);
            races.len() - 1
        };

        let horse = Horse {
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
            apprentice_weight_allowance: line[SF_APPRENTICE_WEIGHT_ALLOWANCE].parse::<f32>().ok(),
            jockey_starts: line[SF_JOCKEY_STARTS].parse::<u32>().ok(),
            jockey_wins: line[SF_JOCKEY_WINS].parse::<u32>().ok(),
            jockey_places: line[SF_JOCKEY_PLACES].parse::<u32>().ok(),
            jockey_shows: line[SF_JOCKEY_SHOWS].parse::<u32>().ok(),
            todays_owner: line[SF_TODAYS_OWNER].clone(),
            owners_silks: line[SF_OWNERS_SILKS].clone(),
            main_track_only_ae_indicator: line[SF_MAIN_TRACK_ONLY_AE_INDICATOR].clone(),
        };

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