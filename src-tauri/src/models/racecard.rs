use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Racecard {
    pub track: String,
    pub date: String,
    pub races: Vec<Race>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    pub race_number: Option<u32>,
    pub distance: Option<i32>,
    pub surface: String,
    pub race_type: String,
    pub horses: Vec<Horse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Horse {
    pub post_position: Option<u32>,
    pub entry: String,
}