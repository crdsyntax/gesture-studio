pub mod commands;

use crate::models::*;
use crate::events::AppEvent;
use crossbeam_channel::Sender;
use tracing::info;
use uuid::Uuid;

pub struct GestureTrainer {
    event_tx: Sender<AppEvent>,
    current_samples: Vec<GestureSequence>,
}

impl GestureTrainer {
    pub fn new(event_tx: Sender<AppEvent>) -> Self {
        Self {
            event_tx,
            current_samples: Vec::new(),
        }
    }

    pub fn start_training(&mut self, gesture_id: Uuid) {
        self.current_samples.clear();
        let _ = self
            .event_tx
            .send(AppEvent::GestureTrainingStarted { gesture_id });
        info!("Training started for gesture: {}", gesture_id);
    }

    pub fn add_sample(&mut self, sequence: GestureSequence) {
        let idx = self.current_samples.len();
        self.current_samples.push(sequence);
        let _ = self
            .event_tx
            .send(AppEvent::GestureTrainingSample {
                gesture_id: uuid::Uuid::nil(),
                sample_index: idx as u32,
            });
    }

    pub fn build_template(&self, name: String, gesture_type: GestureType) -> GestureTemplate {
        let avg_sequence = self.average_samples();
        GestureTemplate {
            id: Uuid::new_v4(),
            name,
            gesture_type,
            samples: vec![avg_sequence],
            created_at: chrono::Utc::now(),
        }
    }

    pub fn sample_count(&self) -> usize {
        self.current_samples.len()
    }

    fn average_samples(&self) -> GestureSequence {
        if self.current_samples.is_empty() {
            return GestureSequence {
                gesture_id: Uuid::nil(),
                frames: vec![],
            };
        }

        let first = &self.current_samples[0];
        let num_frames = first.frames.len();
        let averaged_frames: Vec<GestureFrame> = (0..num_frames)
            .map(|i| {
                let avg_landmarks = self.average_landmark_at_frame(i);
                GestureFrame {
                    timestamp: chrono::Utc::now(),
                    landmarks: avg_landmarks,
                    detection: first.frames[i].detection.clone(),
                }
            })
            .collect();

        GestureSequence {
            gesture_id: Uuid::nil(),
            frames: averaged_frames,
        }
    }

    fn average_landmark_at_frame(&self, frame_idx: usize) -> NormalizedLandmarks {
        let count = self.current_samples.len() as f32;
        let mut sum_landmarks = Vec::new();

        for (i, sample) in self.current_samples.iter().enumerate() {
            if let Some(frame) = sample.frames.get(frame_idx) {
                if i == 0 {
                    sum_landmarks = frame
                        .landmarks
                        .landmarks
                        .iter()
                        .map(|l| (l.x, l.y, l.z))
                        .collect();
                } else {
                    for (j, l) in frame.landmarks.landmarks.iter().enumerate() {
                        if j < sum_landmarks.len() {
                            sum_landmarks[j].0 += l.x;
                            sum_landmarks[j].1 += l.y;
                            sum_landmarks[j].2 += l.z;
                        }
                    }
                }
            }
        }

        let avg_points: Vec<_> = sum_landmarks
            .iter()
            .map(|(x, y, z)| LandmarkPoint {
                x: x / count,
                y: y / count,
                z: z / count,
            })
            .collect();

        NormalizedLandmarks {
            hand_id: 0,
            landmarks: avg_points.clone(),
            wrist: avg_points.first().cloned().unwrap_or(LandmarkPoint { x: 0.0, y: 0.0, z: 0.0 }),
            thumb_tip: avg_points.get(4).cloned().unwrap_or(LandmarkPoint { x: 0.0, y: 0.0, z: 0.0 }),
            index_tip: avg_points.get(8).cloned().unwrap_or(LandmarkPoint { x: 0.0, y: 0.0, z: 0.0 }),
            middle_tip: avg_points.get(12).cloned().unwrap_or(LandmarkPoint { x: 0.0, y: 0.0, z: 0.0 }),
            ring_tip: avg_points.get(16).cloned().unwrap_or(LandmarkPoint { x: 0.0, y: 0.0, z: 0.0 }),
            pinky_tip: avg_points.get(20).cloned().unwrap_or(LandmarkPoint { x: 0.0, y: 0.0, z: 0.0 }),
        }
    }
}
