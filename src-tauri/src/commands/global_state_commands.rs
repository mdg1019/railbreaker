use crate::states::global_state::{global_state, GlobalState};

#[tauri::command]
pub fn load_global_state() -> GlobalState {
    global_state().lock().unwrap().clone()
}