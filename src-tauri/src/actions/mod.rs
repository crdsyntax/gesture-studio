pub mod commands;

use crate::models::*;
use crate::app::error::AppError;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

pub trait ActionExecutor: Send + Sync {
    fn execute(&self, payload: &str) -> Result<ExecutionResult, AppError>;
    fn action_type(&self) -> ActionType;
}

pub struct OpenAppExecutor;
pub struct ExecuteCommandExecutor;
pub struct OpenUrlExecutor;
pub struct SimulateKeyboardExecutor;

impl ActionExecutor for OpenAppExecutor {
    fn execute(&self, payload: &str) -> Result<ExecutionResult, AppError> {
        info!("Opening application: {}", payload);
        Ok(ExecutionResult {
            action_id: Uuid::nil(),
            success: true,
            message: format!("Opening: {}", payload),
            timestamp: chrono::Utc::now(),
        })
    }

    fn action_type(&self) -> ActionType {
        ActionType::OpenApp
    }
}

impl ActionExecutor for ExecuteCommandExecutor {
    fn execute(&self, payload: &str) -> Result<ExecutionResult, AppError> {
        info!("Executing command: {}", payload);
        Ok(ExecutionResult {
            action_id: Uuid::nil(),
            success: true,
            message: format!("Command executed: {}", payload),
            timestamp: chrono::Utc::now(),
        })
    }

    fn action_type(&self) -> ActionType {
        ActionType::ExecuteCommand
    }
}

impl ActionExecutor for OpenUrlExecutor {
    fn execute(&self, payload: &str) -> Result<ExecutionResult, AppError> {
        info!("Opening URL: {}", payload);
        Ok(ExecutionResult {
            action_id: Uuid::nil(),
            success: true,
            message: format!("URL opened: {}", payload),
            timestamp: chrono::Utc::now(),
        })
    }

    fn action_type(&self) -> ActionType {
        ActionType::OpenUrl
    }
}

pub struct ActionEngine {
    executors: HashMap<ActionType, Box<dyn ActionExecutor>>,
}

impl ActionEngine {
    pub fn new() -> Self {
        let mut executors: HashMap<ActionType, Box<dyn ActionExecutor>> = HashMap::new();
        executors.insert(ActionType::OpenApp, Box::new(OpenAppExecutor));
        executors.insert(ActionType::ExecuteCommand, Box::new(ExecuteCommandExecutor));
        executors.insert(ActionType::OpenUrl, Box::new(OpenUrlExecutor));
        Self { executors }
    }

    pub fn execute(&self, action: &AssignedAction) -> ExecutionResult {
        match self.executors.get(&action.action_type) {
            Some(executor) => match executor.execute(&action.payload) {
                Ok(result) => result,
                Err(e) => {
                    error!("Action execution failed: {}", e);
                    ExecutionResult {
                        action_id: action.action_id,
                        success: false,
                        message: e.to_string(),
                        timestamp: chrono::Utc::now(),
                    }
                }
            },
            None => ExecutionResult {
                action_id: action.action_id,
                success: false,
                message: format!("No executor for action type: {:?}", action.action_type),
                timestamp: chrono::Utc::now(),
            },
        }
    }
}
