use crate::models::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    // Camera events
    CameraStarted { device: String },
    CameraStopped,
    CameraDisconnected { reason: String },
    NewFrame { frame: CameraFrame },

    // Detection events
    HandDetected { detection: HandDetection },
    HandsLost,
    LandmarksNormalized { landmarks: NormalizedLandmarks },

    // Recognition events
    GestureRecognized { gesture: RecognizedGesture },
    GestureTrainingStarted { gesture_id: Uuid },
    GestureTrainingSample { gesture_id: Uuid, sample_index: u32 },
    GestureTrainingComplete { gesture_id: Uuid, template: GestureTemplate },

    // Action events
    ActionTriggered { action: AssignedAction },
    ActionExecuted { result: ExecutionResult },
    ActionFailed { action_id: Uuid, error: String },

    // System events
    ConfigChanged,
    StorageUpdated,
    Error { message: String, source: String },
    Shutdown,
}

impl AppEvent {
    pub fn timestamp(&self) -> DateTime<Utc> {
        Utc::now()
    }

    pub fn category(&self) -> &'static str {
        match self {
            AppEvent::CameraStarted { .. } => "camera",
            AppEvent::CameraStopped => "camera",
            AppEvent::CameraDisconnected { .. } => "camera",
            AppEvent::NewFrame { .. } => "camera",
            AppEvent::HandDetected { .. } => "detection",
            AppEvent::HandsLost => "detection",
            AppEvent::LandmarksNormalized { .. } => "detection",
            AppEvent::GestureRecognized { .. } => "recognition",
            AppEvent::GestureTrainingStarted { .. } => "training",
            AppEvent::GestureTrainingSample { .. } => "training",
            AppEvent::GestureTrainingComplete { .. } => "training",
            AppEvent::ActionTriggered { .. } => "action",
            AppEvent::ActionExecuted { .. } => "action",
            AppEvent::ActionFailed { .. } => "action",
            AppEvent::ConfigChanged => "system",
            AppEvent::StorageUpdated => "system",
            AppEvent::Error { .. } => "system",
            AppEvent::Shutdown => "system",
        }
    }
}
