pub mod commands;

use crate::app::error::AppError;
use crate::models::*;
use rusqlite::{params, Connection};
use std::path::Path;
use tracing::info;
use uuid::Uuid;

#[derive(Default)]
pub struct Storage {
    conn: Option<Connection>,
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self, AppError> {
        info!("Opening storage at: {:?}", path);

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        let storage = Self { conn: Some(conn) };
        storage.initialize_tables()?;

        Ok(storage)
    }

    fn connection(&self) -> Result<&Connection, AppError> {
        self.conn
            .as_ref()
            .ok_or_else(|| AppError::Internal("Storage not initialized".to_string()))
    }

    fn initialize_tables(&self) -> Result<(), AppError> {
        let conn = self.connection()?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS gestures (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                gesture_type TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS gesture_samples (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                gesture_id TEXT NOT NULL,
                frame_index INTEGER NOT NULL,
                landmarks TEXT NOT NULL,
                FOREIGN KEY (gesture_id) REFERENCES gestures(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS actions (
                id TEXT PRIMARY KEY,
                gesture_id TEXT NOT NULL,
                action_type TEXT NOT NULL,
                payload TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                FOREIGN KEY (gesture_id) REFERENCES gestures(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )?;

        info!("Database tables initialized");
        Ok(())
    }

    pub fn save_gesture(&self, template: &GestureTemplate) -> Result<(), AppError> {
        let conn = self.connection()?;
        let gesture_type = match template.gesture_type {
            GestureType::Static => "static",
            GestureType::Dynamic => "dynamic",
        };

        conn.execute(
            "INSERT OR REPLACE INTO gestures (id, name, gesture_type, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                template.id.to_string(),
                template.name,
                gesture_type,
                template.created_at.to_rfc3339(),
            ],
        )?;

        for (sample_idx, sequence) in template.samples.iter().enumerate() {
            for (frame_idx, frame) in sequence.frames.iter().enumerate() {
                let landmarks_json = serde_json::to_string(&frame.landmarks)?;
                conn.execute(
                    "INSERT INTO gesture_samples (gesture_id, frame_index, landmarks) VALUES (?1, ?2, ?3)",
                    params![template.id.to_string(), (sample_idx * 1000 + frame_idx) as i32, landmarks_json],
                )?;
            }
        }

        Ok(())
    }

    pub fn load_gestures(&self) -> Result<Vec<GestureTemplate>, AppError> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare("SELECT id, name, gesture_type, created_at FROM gestures")?;

        let templates = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let gesture_type: String = row.get(2)?;
                let created_at: String = row.get(3)?;

                Ok((
                    Uuid::parse_str(&id).unwrap_or_default(),
                    name,
                    gesture_type,
                    chrono::DateTime::parse_from_rfc3339(&created_at)
                        .map(|d| d.with_timezone(&chrono::Utc))
                        .unwrap_or_default(),
                ))
            })?
            .filter_map(|r| r.ok())
            .map(|(id, name, gesture_type, created_at)| {
                let gt = match gesture_type.as_str() {
                    "dynamic" => GestureType::Dynamic,
                    _ => GestureType::Static,
                };

                GestureTemplate {
                    id,
                    name,
                    gesture_type: gt,
                    samples: vec![],
                    created_at,
                }
            })
            .collect();

        Ok(templates)
    }

    pub fn delete_gesture(&self, gesture_id: Uuid) -> Result<(), AppError> {
        let conn = self.connection()?;
        conn.execute(
            "DELETE FROM gestures WHERE id = ?1",
            params![gesture_id.to_string()],
        )?;
        Ok(())
    }

    pub fn count_samples(&self) -> Result<usize, AppError> {
        let conn = self.connection()?;
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM gesture_samples", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn count_actions(&self) -> Result<usize, AppError> {
        let conn = self.connection()?;
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM actions", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn save_action(&self, action: &AssignedAction) -> Result<(), AppError> {
        let conn = self.connection()?;
        let action_type = format!("{:?}", action.action_type);
        conn.execute(
            "INSERT OR REPLACE INTO actions (id, gesture_id, action_type, payload, enabled) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                action.action_id.to_string(),
                action.gesture_id.to_string(),
                action_type,
                action.payload,
                action.enabled as i32,
            ],
        )?;
        Ok(())
    }

    pub fn load_actions(&self) -> Result<Vec<AssignedAction>, AppError> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, gesture_id, action_type, payload, enabled FROM actions",
        )?;

        let actions = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let gesture_id: String = row.get(1)?;
                let action_type: String = row.get(2)?;
                let payload: String = row.get(3)?;
                let enabled: i32 = row.get(4)?;

                Ok((
                    id, gesture_id, action_type, payload, enabled,
                ))
            })?
            .filter_map(|r| r.ok())
            .map(|(id, gesture_id, action_type, payload, enabled)| {
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

                AssignedAction {
                    action_id: Uuid::parse_str(&id).unwrap_or_default(),
                    gesture_id: Uuid::parse_str(&gesture_id).unwrap_or_default(),
                    action_type: at,
                    payload,
                    enabled: enabled != 0,
                }
            })
            .collect();

        Ok(actions)
    }

    pub fn load_actions_for_gesture(
        &self,
        gesture_id: Uuid,
    ) -> Result<Vec<AssignedAction>, AppError> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, gesture_id, action_type, payload, enabled FROM actions WHERE gesture_id = ?1",
        )?;

        let actions = stmt
            .query_map(params![gesture_id.to_string()], |row| {
                let id: String = row.get(0)?;
                let gesture_id: String = row.get(1)?;
                let action_type: String = row.get(2)?;
                let payload: String = row.get(3)?;
                let enabled: i32 = row.get(4)?;

                Ok((
                    id, gesture_id, action_type, payload, enabled,
                ))
            })?
            .filter_map(|r| r.ok())
            .map(|(id, gesture_id, action_type, payload, enabled)| {
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

                AssignedAction {
                    action_id: Uuid::parse_str(&id).unwrap_or_default(),
                    gesture_id: Uuid::parse_str(&gesture_id).unwrap_or_default(),
                    action_type: at,
                    payload,
                    enabled: enabled != 0,
                }
            })
            .collect();

        Ok(actions)
    }

    pub fn delete_action(&self, action_id: Uuid) -> Result<(), AppError> {
        let conn = self.connection()?;
        conn.execute(
            "DELETE FROM actions WHERE id = ?1",
            params![action_id.to_string()],
        )?;
        Ok(())
    }
}
