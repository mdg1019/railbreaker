use std::{sync::{Mutex, OnceLock}};
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GlobalState {
    pub tracks: HashMap<String, String>,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            tracks: HashMap::new(),
        }
    }
}

static GLOBAL_STATE: OnceLock<Mutex<GlobalState>> = OnceLock::new();

pub fn global_state() -> &'static Mutex<GlobalState> {
    GLOBAL_STATE.get_or_init(|| Mutex::new(GlobalState::default()))
}
