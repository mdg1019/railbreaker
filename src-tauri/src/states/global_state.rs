use std::{sync::{Mutex, OnceLock}};
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GlobalState {
    pub tracks: HashMap<String, String>,
    pub current_directory: String,
    pub downloads_directory: String,
    pub racecards_directory: String,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            tracks: HashMap::new(),
            current_directory: String::new(),
            downloads_directory: String::new(),
            racecards_directory: String::new(),
        }
    }
}

static GLOBAL_STATE: OnceLock<Mutex<GlobalState>> = OnceLock::new();

pub fn global_state() -> &'static Mutex<GlobalState> {
    GLOBAL_STATE.get_or_init(|| Mutex::new(GlobalState::default()))
}
