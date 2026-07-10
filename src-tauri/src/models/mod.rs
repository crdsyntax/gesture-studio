use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFrame {
    pub timestamp: DateTime<Utc>,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandDetection {
    pub hand_id: u32,
    pub confidence: f32,
    pub landmarks: Vec<LandmarkPoint>,
    pub handedness: HandType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandmarkPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedLandmarks {
    pub hand_id: u32,
    pub landmarks: Vec<LandmarkPoint>,
    pub wrist: LandmarkPoint,
    pub thumb_tip: LandmarkPoint,
    pub index_tip: LandmarkPoint,
    pub middle_tip: LandmarkPoint,
    pub ring_tip: LandmarkPoint,
    pub pinky_tip: LandmarkPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureFrame {
    pub timestamp: DateTime<Utc>,
    pub landmarks: NormalizedLandmarks,
    pub detection: HandDetection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureSequence {
    pub gesture_id: Uuid,
    pub frames: Vec<GestureFrame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognizedGesture {
    pub gesture_id: Uuid,
    pub gesture_name: String,
    pub confidence: f32,
    pub gesture_type: GestureType,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GestureType {
    Static,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignedAction {
    pub action_id: Uuid,
    pub gesture_id: Uuid,
    pub action_type: ActionType,
    pub payload: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    OpenApp,
    ExecuteCommand,
    OpenUrl,
    ChangeVolume,
    MediaControl,
    LockWorkstation,
    SimulateKeyboard,
    SimulateMouse,
    HttpRequest,
    WebSocket,
    Mqtt,
    PowerShell,
    Bash,
    TauriEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub action_id: Uuid,
    pub success: bool,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureTemplate {
    pub id: Uuid,
    pub name: String,
    pub gesture_type: GestureType,
    pub samples: Vec<GestureSequence>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandType {
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraStatus {
    Idle,
    Starting,
    Running,
    Stopped,
    Error(String),
}
