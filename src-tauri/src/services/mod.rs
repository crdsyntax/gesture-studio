use crate::actions::ActionEngine;
use crate::camera::CameraEngine;
use crate::events::AppEvent;
use crate::gestures::GestureEngine;
use crate::landmarks::LandmarkNormalizer;
use crate::storage::Storage;
use crate::trainer::GestureTrainer;
use crate::vision::VisionEngine;
use crate::AppConfig;
use crossbeam_channel::Sender;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct GestureService {
    pub camera: CameraEngine,
    pub vision: VisionEngine,
    pub landmarks: LandmarkNormalizer,
    pub gestures: GestureEngine,
    pub actions: ActionEngine,
    pub trainer: GestureTrainer,
    pub event_tx: Sender<AppEvent>,
}

impl GestureService {
    pub fn new(config: &AppConfig, event_tx: Sender<AppEvent>) -> Self {
        info!("Initializing GestureService");
        let mut vision = VisionEngine::new(config.vision.confidence_threshold);
        let _ = vision.load_model(&config.vision.model_path);
        Self {
            camera: CameraEngine::new(event_tx.clone()),
            vision,
            landmarks: LandmarkNormalizer::new(config.recognition.smoothing_factor),
            gestures: GestureEngine::new(),
            actions: ActionEngine::new(),
            trainer: GestureTrainer::new(event_tx.clone()),
            event_tx,
        }
    }

    pub fn load_from_storage(&mut self, storage: &Arc<Mutex<Storage>>) {
        if let Ok(storage) = storage.lock() {
            if let Ok(templates) = storage.load_gestures() {
                info!("Loaded {} gesture templates from storage", templates.len());
                self.gestures.load_templates(templates);
            }
        }
    }
}
