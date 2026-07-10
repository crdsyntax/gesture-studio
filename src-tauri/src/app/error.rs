use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Camera error: {0}")]
    Camera(String),

    #[error("Vision engine error: {0}")]
    Vision(String),

    #[error("Gesture recognition error: {0}")]
    Gesture(String),

    #[error("Training error: {0}")]
    Training(String),

    #[error("Action execution error: {0}")]
    Action(String),

    #[error("Storage error: {0}")]
    Storage(#[from] rusqlite::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
