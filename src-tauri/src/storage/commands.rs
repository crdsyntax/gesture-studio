use crate::app::error::AppError;
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub gesture_count: usize,
    pub sample_count: usize,
    pub action_count: usize,
}

#[tauri::command]
pub fn get_stats(state: State<AppState>) -> Result<StorageStats, AppError> {
    let storage = state.storage.lock().unwrap();
    let gesture_count = storage.load_gestures()?.len();
    let sample_count = storage.count_samples()?;
    let action_count = storage.count_actions()?;

    Ok(StorageStats {
        gesture_count,
        sample_count,
        action_count,
    })
}
