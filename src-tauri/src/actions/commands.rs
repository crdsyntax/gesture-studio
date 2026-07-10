use crate::app::error::AppError;
use crate::models::*;
use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionInfo {
    pub id: Uuid,
    pub gesture_id: Uuid,
    pub action_type: String,
    pub payload: String,
    pub enabled: bool,
}

#[tauri::command]
pub fn list_actions(state: State<AppState>, gesture_id: Uuid) -> Result<Vec<ActionInfo>, AppError> {
    let storage = state.storage.lock().unwrap();
    let actions = storage.load_actions_for_gesture(gesture_id)?;
    Ok(actions
        .into_iter()
        .map(|a| ActionInfo {
            id: a.action_id,
            gesture_id: a.gesture_id,
            action_type: format!("{:?}", a.action_type),
            payload: a.payload,
            enabled: a.enabled,
        })
        .collect())
}

#[tauri::command]
pub fn create_action(
    state: State<AppState>,
    gesture_id: Uuid,
    action_type: String,
    payload: String,
) -> Result<ActionInfo, AppError> {
    let at = match action_type.as_str() {
        "ExecuteCommand" => ActionType::ExecuteCommand,
        "OpenUrl" => ActionType::OpenUrl,
        "ChangeVolume" => ActionType::ChangeVolume,
        "MediaControl" => ActionType::MediaControl,
        "LockWorkstation" => ActionType::LockWorkstation,
        "SimulateKeyboard" => ActionType::SimulateKeyboard,
        "SimulateMouse" => ActionType::SimulateMouse,
        "HttpRequest" => ActionType::HttpRequest,
        "WebSocket" => ActionType::WebSocket,
        "Mqtt" => ActionType::Mqtt,
        "PowerShell" => ActionType::PowerShell,
        "Bash" => ActionType::Bash,
        "TauriEvent" => ActionType::TauriEvent,
        _ => ActionType::OpenApp,
    };

    let action = AssignedAction {
        action_id: Uuid::new_v4(),
        gesture_id,
        action_type: at,
        payload,
        enabled: true,
    };

    let storage = state.storage.lock().unwrap();
    storage.save_action(&action)?;
    let _ = state.event_tx.send(crate::events::AppEvent::StorageUpdated);

    Ok(ActionInfo {
        id: action.action_id,
        gesture_id: action.gesture_id,
        action_type: format!("{:?}", action.action_type),
        payload: action.payload,
        enabled: action.enabled,
    })
}

#[tauri::command]
pub fn delete_action(
    state: State<AppState>,
    action_id: Uuid,
) -> Result<(), AppError> {
    let storage = state.storage.lock().unwrap();
    storage.delete_action(action_id)?;
    let _ = state.event_tx.send(crate::events::AppEvent::StorageUpdated);
    Ok(())
}

#[tauri::command]
pub fn execute_action(
    state: State<AppState>,
    action_id: Uuid,
) -> Result<ExecutionResult, AppError> {
    let storage = state.storage.lock().unwrap();
    let all_actions = storage.load_actions()?;
    let action = all_actions
        .into_iter()
        .find(|a| a.action_id == action_id)
        .ok_or_else(|| AppError::Action("Action not found".to_string()))?;
    drop(storage);

    let svc = state.service.lock().unwrap();
    let result = svc.actions.execute(&action);
    Ok(result)
}
