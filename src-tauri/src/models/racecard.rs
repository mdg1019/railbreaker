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
    pub age_sex_restrictions: String,
    pub todays_race_classification: String,
    pub purse: Option<u32>,
    pub claiming_price: Option<u32>,
    pub track_record: Option<f64>,
    pub race_conditions: String,
    pub todays_lasix_list: String,
    pub todays_bute_list: String,
    pub todays_coupled_list: String,
    pub todays_mutuel_list: String,
    pub simulcast_host_track_code: String,
    pub simulcast_host_track_race_number: Option<u32>,
    pub all_weather_surface_flag: String,
    pub horses: Vec<Horse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Horse {
    pub post_position: Option<u32>,
    pub entry: String,
    pub claiming_price_of_horse: Option<u32>,
    pub breed_type: String,
    pub todays_nasal_strip_change: Option<u32>,
    pub todays_trainer: String,
    pub trainer_starts: Option<u32>,
    pub trainer_wins: Option<u32>,
    pub trainer_places: Option<u32>,
    pub trainer_shows: Option<u32>,
    pub todays_jockey: String,
    pub apprentice_weight_allowance: Option<f32>,
    pub jockey_starts: Option<u32>,
    pub jockey_wins: Option<u32>,
    pub jockey_places: Option<u32>,
    pub jockey_shows: Option<u32>,
    pub todays_owner: String,
    pub owners_silks: String,
    pub main_track_only_ae_indicator: String,
}