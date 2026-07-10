pub mod commands;
pub mod error;
pub mod events_bridge;

use crate::events::AppEvent;
use crate::services::GestureService;
use crate::storage::Storage;
use std::sync::{Arc, Mutex};
use tracing::{error, info};

pub struct EventProcessor;

impl EventProcessor {
    pub fn spawn(
        rx: crossbeam_channel::Receiver<AppEvent>,
        service: Arc<Mutex<GestureService>>,
        storage: Arc<Mutex<Storage>>,
    ) {
        std::thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                events_bridge::forward_event(&event);
                match &event {
                    AppEvent::Shutdown => {
                        info!("Event processor shutting down");
                        break;
                    }
                    AppEvent::Error { message, source } => {
                        error!(source = %source, "{}", message);
                    }
                    AppEvent::NewFrame { .. } => {
                        // Hand detection runs in the frontend (MediaPipe) and is submitted
                        // via submit_hand_frame IPC to avoid stub vision and queue flooding.
                    }
                    AppEvent::HandDetected { detection } => {
                        let mut svc = service.lock().unwrap();
                        let normalized = svc.landmarks.normalize(&[detection.clone()]);
                        for nl in normalized {
                            svc.last_normalized = Some(nl.clone());
                            let _ = svc.event_tx
                                .send(AppEvent::LandmarksNormalized { landmarks: nl });
                        }
                    }
                    AppEvent::LandmarksNormalized { landmarks } => {
                        let svc = service.lock().unwrap();
                        if let Some(recognized) = svc.gestures.recognize_static(landmarks) {
                            let _ = svc.event_tx
                                .send(AppEvent::GestureRecognized { gesture: recognized });
                        }
                    }
                    AppEvent::GestureRecognized { gesture } => {
                        let event_tx = service.lock().unwrap().event_tx.clone();
                        let actions = storage
                            .lock()
                            .unwrap()
                            .load_actions_for_gesture(gesture.gesture_id)
                            .unwrap_or_default();

                        for action in &actions {
                            if action.enabled {
                                let _ = event_tx
                                    .send(AppEvent::ActionTriggered { action: action.clone() });
                            }
                        }

                        for action in actions {
                            if action.enabled {
                                let svc = service.lock().unwrap();
                                let result = svc.actions.execute(&action);
                                let _ = event_tx.send(AppEvent::ActionExecuted { result });
                            }
                        }
                    }
                    _ => {
                        info!(category = event.category(), "Event: {:?}", event);
                    }
                }
            }
        });
    }
}
