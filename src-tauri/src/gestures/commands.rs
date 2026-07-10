use crate::app::error::AppError;
use crate::models::*;
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct GestureInfo {
    pub id: Uuid,
    pub name: String,
    pub gesture_type: String,
    pub created_at: String,
}

#[tauri::command]
pub fn list_gestures(state: State<AppState>) -> Result<Vec<GestureInfo>, AppError> {
    let storage = state.storage.lock().unwrap();
    let templates = storage.load_gestures()?;
    Ok(templates
        .into_iter()
        .map(|t| GestureInfo {
            id: t.id,
            name: t.name,
            gesture_type: format!("{:?}", t.gesture_type),
            created_at: t.created_at.to_rfc3339(),
        })
        .collect())
}

#[tauri::command]
pub fn create_gesture(
    state: State<AppState>,
    name: String,
    gesture_type: String,
) -> Result<GestureInfo, AppError> {
    tracing::info!("Creating gesture: {} ({})", name, gesture_type);
    let gt = match gesture_type.to_lowercase().as_str() {
        "dynamic" => GestureType::Dynamic,
        _ => GestureType::Static,
    };

    let template = GestureTemplate {
        id: Uuid::new_v4(),
        name: name.clone(),
        gesture_type: gt,
        samples: vec![],
        created_at: chrono::Utc::now(),
    };

    let storage = state.storage.lock().unwrap();
    storage.save_gesture(&template)?;
    let _ = state.event_tx.send(crate::events::AppEvent::StorageUpdated);

    Ok(GestureInfo {
        id: template.id,
        name,
        gesture_type: format!("{:?}", template.gesture_type),
        created_at: template.created_at.to_rfc3339(),
    })
}

#[tauri::command]
pub fn delete_gesture(
    state: State<AppState>,
    gesture_id: Uuid,
) -> Result<(), AppError> {
    let storage = state.storage.lock().unwrap();
    storage.delete_gesture(gesture_id)?;
    let _ = state.event_tx.send(crate::events::AppEvent::StorageUpdated);
    Ok(())
}
