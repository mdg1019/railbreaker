use serde::{Deserialize, Serialize};
use crate::constants::HORSE_SORTING_METHOD_DEFAULT;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigState {
    pub last_directory: String,
    pub window_x: Option<f64>,
    pub window_y: Option<f64>,
    pub window_width: Option<f64>,
    pub window_height: Option<f64>,
    pub horse_sorting_method: String,
}

impl Default for ConfigState {
    fn default() -> Self {
        Self {
            last_directory: String::new(),
            window_x: None,
            window_y: None,
            window_width: None,
            window_height: None,
            horse_sorting_method: HORSE_SORTING_METHOD_DEFAULT.to_string(),
        }
    }
}