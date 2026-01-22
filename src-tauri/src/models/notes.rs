use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub race: u32,
    pub horse: u32,
    pub content: String,
}