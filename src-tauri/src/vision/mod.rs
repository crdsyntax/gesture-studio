pub mod commands;

use crate::app::error::AppError;
use crate::models::{HandDetection, HandType, LandmarkPoint};
use std::time::Instant;
use tracing::info;

pub struct VisionEngine {
    model_loaded: bool,
    confidence_threshold: f32,
    start_time: Instant,
}

impl VisionEngine {
    pub fn new(confidence_threshold: f32) -> Self {
        Self {
            model_loaded: false,
            confidence_threshold,
            start_time: Instant::now(),
        }
    }

    pub fn load_model(&mut self, model_path: &std::path::Path) -> Result<(), AppError> {
        info!("Loading model from: {:?}", model_path);
        self.model_loaded = true;
        Ok(())
    }

    pub fn detect_hands(&self, _frame: &[u8]) -> Result<Vec<HandDetection>, AppError> {
        if !self.model_loaded {
            return Err(AppError::Vision("Model not loaded".to_string()));
        }

        let elapsed = self.start_time.elapsed().as_secs_f64();

        let landmarks = synthetic_hand_landmarks(elapsed);
        let confidence: f32 = 0.85 + (elapsed * 0.3).sin() as f32 * 0.1;

        if confidence < self.confidence_threshold {
            return Ok(vec![]);
        }

        Ok(vec![HandDetection {
            hand_id: 0,
            confidence,
            landmarks,
            handedness: if (elapsed * 0.5).sin() > 0.0 {
                HandType::Right
            } else {
                HandType::Left
            },
        }])
    }

    pub fn is_loaded(&self) -> bool {
        self.model_loaded
    }
}

fn synthetic_hand_landmarks(time: f64) -> Vec<LandmarkPoint> {
    let hand_x = (time * 0.4).sin() as f32 * 0.2;
    let hand_y = 0.2 + (time * 0.3).cos() as f32 * 0.15;
    let spread = 0.5 + (time * 0.2).sin() as f32 * 0.3;
    let curl = (time * 0.5).sin() as f32 * 0.3 + 0.5;

    vec![
        LandmarkPoint { x: hand_x, y: hand_y, z: 0.0 },
        LandmarkPoint { x: -0.08 * spread + hand_x, y: 0.08 + hand_y, z: 0.02 },
        LandmarkPoint { x: -0.12 * spread + hand_x, y: 0.14 + hand_y, z: 0.03 },
        LandmarkPoint { x: -0.16 * spread + hand_x, y: 0.20 + hand_y, z: 0.02 },
        LandmarkPoint { x: -0.20 * spread + hand_x, y: 0.26 + hand_y, z: 0.01 },
        LandmarkPoint { x: 0.04 + hand_x, y: 0.12 + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.05 + hand_x, y: 0.22 + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.04 + hand_x, y: 0.32 + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.03 + hand_x, y: 0.40 * curl + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.0 + hand_x, y: 0.14 + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.0 + hand_x, y: 0.26 + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.0 + hand_x, y: 0.38 + hand_y, z: 0.0 },
        LandmarkPoint { x: 0.0 + hand_x, y: 0.48 * curl + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.04 + hand_x, y: 0.12 + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.05 + hand_x, y: 0.22 + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.04 + hand_x, y: 0.32 + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.05 + hand_x, y: 0.38 * curl + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.07 + hand_x, y: 0.10 + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.08 + hand_x, y: 0.18 + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.07 + hand_x, y: 0.26 + hand_y, z: 0.0 },
        LandmarkPoint { x: -0.08 + hand_x, y: 0.30 * curl + hand_y, z: 0.0 },
    ]
}
