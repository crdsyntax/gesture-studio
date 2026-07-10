use crate::app::error::AppError;
use crate::models::{HandDetection, HandType, LandmarkPoint};
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandDetectionInput {
    pub hand_id: u32,
    pub confidence: f32,
    pub landmarks: Vec<LandmarkPoint>,
    pub handedness: String,
}

#[tauri::command]
pub fn submit_hand_frame(
    state: State<AppState>,
    detections: Vec<HandDetectionInput>,
) -> Result<(), AppError> {
    if detections.is_empty() {
        let _ = state.event_tx.send(crate::events::AppEvent::HandsLost);
        return Ok(());
    }

    for input in detections {
        let handedness = match input.handedness.to_lowercase().as_str() {
            "left" => HandType::Left,
            _ => HandType::Right,
        };

        let detection = HandDetection {
            hand_id: input.hand_id,
            confidence: input.confidence,
            landmarks: input.landmarks,
            handedness,
        };

        {
            let mut svc = state.service.lock().unwrap();
            svc.last_detection = Some(detection.clone());
        }

        let _ = state
            .event_tx
            .send(crate::events::AppEvent::HandDetected { detection });
    }

    Ok(())
}
