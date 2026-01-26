use crate::analysis::contextual_speed_and_pace_model::{rank_race_auto, RaceRankResult};
use crate::models::racecard::Race;
use serde_json::{Value};

#[tauri::command]
pub fn rank_race(
    race: Value,
    racecard_date: Option<String>,
    scratched_horses: Vec<u32>,
) -> Result<RaceRankResult, String> {
    let race: Race =
        serde_json::from_value(race).map_err(|e| format!("Failed to parse race payload: {}", e))?;
    let date = racecard_date.as_deref();
    Ok(rank_race_auto(&race, date, &scratched_horses))
}
