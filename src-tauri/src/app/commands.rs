use crate::app::error::AppError;
use crate::config::AppConfig;
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStatus {
    pub version: String,
    pub camera_active: bool,
    pub model_loaded: bool,
    pub fps: f64,
    pub latency_ms: f64,
}

#[tauri::command]
pub fn get_app_status(state: State<AppState>) -> Result<AppStatus, AppError> {
    let svc = state.service.lock().unwrap();
    Ok(AppStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
        camera_active: svc.camera.is_running(),
        model_loaded: svc.vision.is_loaded(),
        fps: svc.camera.get_fps() as f64,
        latency_ms: 0.0,
    })
}

#[tauri::command]
pub fn get_config(state: State<AppState>) -> Result<AppConfig, AppError> {
    let config = state.config.read().unwrap();
    Ok(config.clone())
}

#[tauri::command]
pub fn update_config(
    state: State<AppState>,
    config: AppConfig,
) -> Result<(), AppError> {
    config.save();
    {
        let mut current = state.config.write().unwrap();
        *current = config;
    }
    let _ = state.event_tx.send(crate::events::AppEvent::ConfigChanged);
    Ok(())
}
