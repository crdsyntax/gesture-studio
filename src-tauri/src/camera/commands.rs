use crate::app::error::AppError;
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraDevice {
    pub id: String,
    pub name: String,
}

#[tauri::command]
pub fn list_cameras(_state: State<AppState>) -> Result<Vec<CameraDevice>, AppError> {
    #[cfg(windows)]
    {
        let devices = crate::camera::wmf::WmfCapture::enumerate();
        if !devices.is_empty() {
            return Ok(devices
                .into_iter()
                .map(|(id, name)| CameraDevice { id, name })
                .collect());
        }
    }
    Ok(vec![
        CameraDevice {
            id: "default".to_string(),
            name: "Default Camera".to_string(),
        },
    ])
}

#[tauri::command]
pub fn start_camera(
    state: State<AppState>,
    device_id: String,
) -> Result<(), AppError> {
    let config = state.config.read().unwrap();
    let target_fps = config.camera.fps.max(1);
    let resolved_device = if device_id == "default" || device_id.is_empty() {
        config.camera.preferred_device.clone().unwrap_or_else(|| "default".to_string())
    } else {
        device_id.clone()
    };
    drop(config);

    tracing::info!("Start camera requested: {} (resolved: {})", device_id, resolved_device);
    let mut svc = state.service.lock().unwrap();
    svc.camera
        .start(&resolved_device, target_fps)
        .map_err(|e| AppError::Camera(e))?;
    Ok(())
}

#[tauri::command]
pub fn stop_camera(state: State<AppState>) -> Result<(), AppError> {
    tracing::info!("Stop camera requested");
    let mut svc = state.service.lock().unwrap();
    svc.camera.stop();
    Ok(())
}
