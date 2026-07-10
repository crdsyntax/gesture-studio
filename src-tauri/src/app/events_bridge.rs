use crate::events::AppEvent;
use serde::Serialize;
use std::sync::OnceLock;
use tauri::AppHandle;
use tauri::Emitter;
use tracing::error;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn init(handle: AppHandle) {
    let _ = APP_HANDLE.set(handle);
}

pub fn emit<T: Serialize + Clone>(event: &str, payload: T) {
    if let Some(handle) = APP_HANDLE.get() {
        if let Err(e) = handle.emit(event, payload) {
            error!("Failed to emit frontend event '{}': {}", event, e);
        }
    }
}

pub fn forward_event(event: &AppEvent) {
    match event {
        AppEvent::CameraStarted { device } => {
            emit("camera-started", serde_json::json!({ "device": device }));
        }
        AppEvent::CameraStopped => {
            emit("camera-stopped", serde_json::json!({}));
        }
        AppEvent::HandDetected { detection } => {
            emit("hand-detected", detection.clone());
        }
        AppEvent::HandsLost => {
            emit("hands-lost", serde_json::json!({}));
        }
        AppEvent::LandmarksNormalized { landmarks } => {
            emit("landmarks-normalized", landmarks.clone());
        }
        AppEvent::GestureRecognized { gesture } => {
            emit("gesture-recognized", gesture.clone());
        }
        AppEvent::GestureTrainingComplete { gesture_id, template } => {
            emit("training-complete", serde_json::json!({
                "gesture_id": gesture_id,
                "template_name": template.name,
            }));
        }
        AppEvent::ActionExecuted { result } => {
            emit("action-executed", result.clone());
        }
        AppEvent::ActionFailed { action_id, error: msg } => {
            emit("action-failed", serde_json::json!({
                "action_id": action_id,
                "error": msg,
            }));
        }
        AppEvent::Error { message, source } => {
            emit("app-error", serde_json::json!({
                "message": message,
                "source": source,
            }));
        }
        _ => {}
    }
}

pub fn emit_frame(frame_data: &str, width: u32, height: u32) {
    emit("camera-frame", serde_json::json!({
        "data": frame_data,
        "width": width,
        "height": height,
    }));
}
