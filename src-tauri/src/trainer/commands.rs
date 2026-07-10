use crate::app::error::AppError;
use crate::models::*;
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingStatus {
    pub active: bool,
    pub gesture_id: Option<Uuid>,
    pub sample_count: u32,
    pub max_samples: u32,
}

#[tauri::command]
pub fn start_training(
    state: State<AppState>,
    gesture_id: Uuid,
) -> Result<(), AppError> {
    let mut svc = state.service.lock().unwrap();
    svc.trainer.start_training(gesture_id);
    Ok(())
}

#[tauri::command]
pub fn capture_training_sample(
    state: State<AppState>,
) -> Result<u32, AppError> {
    let mut svc = state.service.lock().unwrap();

    let landmarks = svc
        .last_normalized
        .clone()
        .ok_or_else(|| AppError::Training("No hand detected — show your hand to the camera".to_string()))?;

    let detection = svc.last_detection.clone().unwrap_or(HandDetection {
        hand_id: landmarks.hand_id,
        confidence: 0.0,
        landmarks: landmarks.landmarks.clone(),
        handedness: HandType::Right,
    });

    let sequence = GestureSequence {
        gesture_id: Uuid::nil(),
        frames: vec![GestureFrame {
            timestamp: chrono::Utc::now(),
            landmarks,
            detection,
        }],
    };
    svc.trainer.add_sample(sequence);
    Ok(svc.trainer.sample_count() as u32)
}

#[tauri::command]
pub fn finish_training(
    state: State<AppState>,
    name: String,
    gesture_type: String,
) -> Result<(), AppError> {
    let gt = match gesture_type.to_lowercase().as_str() {
        "dynamic" => GestureType::Dynamic,
        _ => GestureType::Static,
    };

    let template = {
        let svc = state.service.lock().unwrap();
        svc.trainer.build_template(name, gt)
    };

    let storage = state.storage.lock().unwrap();
    storage.save_gesture(&template)?;
    let _ = state.event_tx.send(crate::events::AppEvent::StorageUpdated);
    Ok(())
}
