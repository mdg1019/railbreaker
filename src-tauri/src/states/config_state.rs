use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigState {
    pub last_directory: String,
    pub window_x: Option<f64>,
    pub window_y: Option<f64>,
    pub window_width: Option<f64>,
    pub window_height: Option<f64>,
}

impl Default for ConfigState {
    fn default() -> Self {
        Self {
            last_directory: String::new(),
            window_x: None,
            window_y: None,
            window_width: None,
            window_height: None,
        }
    }
}