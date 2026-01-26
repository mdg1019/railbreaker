use crate::analysis::contextual_speed_and_pace_model::{rank_race_auto, RaceRankResult};
use crate::models::racecard::Race;
use serde_json::{Map, Value};

fn to_snake_case_key(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    let mut prev_was_alpha = false;
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            if !out.ends_with('_') {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_was_alpha = true;
        } else if ch.is_ascii_digit() {
            if prev_was_alpha && !out.ends_with('_') && !out.ends_with("line") {
                out.push('_');
            }
            out.push(ch);
            prev_was_alpha = false;
        } else {
            out.push(ch);
            prev_was_alpha = ch.is_ascii_alphabetic();
        }
    }
    out
}

fn to_snake_case_value(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(to_snake_case_value).collect()),
        Value::Object(map) => {
            let mut next = Map::new();
            for (key, value) in map {
                next.insert(to_snake_case_key(&key), to_snake_case_value(value));
            }
            Value::Object(next)
        }
        other => other,
    }
}

#[tauri::command]
pub fn rank_race(
    race: Value,
    racecard_date: Option<String>,
    scratched_horses: Vec<u32>,
) -> Result<RaceRankResult, String> {
    let race = to_snake_case_value(race);
    let race: Race =
        serde_json::from_value(race).map_err(|e| format!("Failed to parse race payload: {}", e))?;
    let date = racecard_date.as_deref();
    Ok(rank_race_auto(&race, date, &scratched_horses))
}
