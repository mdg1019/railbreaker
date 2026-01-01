use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Racecard {
    pub track: String,
    pub date: String,
}