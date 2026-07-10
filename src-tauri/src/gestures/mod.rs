pub mod commands;

use crate::models::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct GestureEngine {
    templates: HashMap<Uuid, GestureTemplate>,
}

impl GestureEngine {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn recognize_static(&self, landmarks: &NormalizedLandmarks) -> Option<RecognizedGesture> {
        for template in self.templates.values() {
            if template.gesture_type != GestureType::Static {
                continue;
            }

            if let Some(sample) = template.samples.first() {
                if let Some(avg_frame) = sample.frames.first() {
                    let confidence = self.euclidean_distance(landmarks, &avg_frame.landmarks);
                    if confidence > 0.8 {
                        return Some(RecognizedGesture {
                            gesture_id: template.id,
                            gesture_name: template.name.clone(),
                            confidence,
                            gesture_type: GestureType::Static,
                            timestamp: chrono::Utc::now(),
                        });
                    }
                }
            }
        }
        None
    }

    pub fn load_templates(&mut self, templates: Vec<GestureTemplate>) {
        for t in templates {
            self.templates.insert(t.id, t);
        }
    }

    fn euclidean_distance(
        &self,
        a: &NormalizedLandmarks,
        b: &NormalizedLandmarks,
    ) -> f32 {
        let dist: f32 = a
            .landmarks
            .iter()
            .zip(b.landmarks.iter())
            .map(|(p1, p2)| {
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let dz = p1.z - p2.z;
                dx * dx + dy * dy + dz * dz
            })
            .sum();

        1.0 / (1.0 + dist.sqrt())
    }
}
