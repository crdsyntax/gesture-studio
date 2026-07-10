use crate::models::{HandDetection, NormalizedLandmarks, LandmarkPoint};

pub struct LandmarkNormalizer {
    smoothing_factor: f32,
    previous_landmarks: Vec<Vec<LandmarkPoint>>,
}

impl LandmarkNormalizer {
    pub fn new(smoothing_factor: f32) -> Self {
        Self {
            smoothing_factor,
            previous_landmarks: Vec::new(),
        }
    }

    pub fn normalize(&mut self, detections: &[HandDetection]) -> Vec<NormalizedLandmarks> {
        detections
            .iter()
            .map(|detection| {
                let mut landmarks = detection.landmarks.clone();

                landmarks = self.translate_to_wrist(&landmarks);
                landmarks = self.scale_uniform(&landmarks);
                landmarks = self.smooth(&landmarks, detection.hand_id as usize);

                NormalizedLandmarks {
                    hand_id: detection.hand_id,
                    wrist: landmarks[0].clone(),
                    thumb_tip: landmarks[4].clone(),
                    index_tip: landmarks[8].clone(),
                    middle_tip: landmarks[12].clone(),
                    ring_tip: landmarks[16].clone(),
                    pinky_tip: landmarks[20].clone(),
                    landmarks,
                }
            })
            .collect()
    }

    fn translate_to_wrist(&self, landmarks: &[LandmarkPoint]) -> Vec<LandmarkPoint> {
        let wrist = landmarks.first().map(|w| (w.x, w.y, w.z)).unwrap_or((0.0, 0.0, 0.0));
        landmarks
            .iter()
            .map(|l| LandmarkPoint {
                x: l.x - wrist.0,
                y: l.y - wrist.1,
                z: l.z - wrist.2,
            })
            .collect()
    }

    fn scale_uniform(&self, landmarks: &[LandmarkPoint]) -> Vec<LandmarkPoint> {
        let max_dist = landmarks
            .iter()
            .map(|l| (l.x * l.x + l.y * l.y + l.z * l.z).sqrt())
            .fold(0.0f32, f32::max);

        if max_dist < 0.001 {
            return landmarks.to_vec();
        }

        landmarks
            .iter()
            .map(|l| LandmarkPoint {
                x: l.x / max_dist,
                y: l.y / max_dist,
                z: l.z / max_dist,
            })
            .collect()
    }

    fn smooth(
        &mut self,
        landmarks: &[LandmarkPoint],
        hand_idx: usize,
    ) -> Vec<LandmarkPoint> {
        let alpha = self.smoothing_factor;
        let previous = self
            .previous_landmarks
            .get(hand_idx)
            .cloned()
            .unwrap_or_else(|| landmarks.to_vec());

        let smoothed: Vec<LandmarkPoint> = landmarks
            .iter()
            .zip(previous.iter())
            .map(|(current, prev)| LandmarkPoint {
                x: alpha * current.x + (1.0 - alpha) * prev.x,
                y: alpha * current.y + (1.0 - alpha) * prev.y,
                z: alpha * current.z + (1.0 - alpha) * prev.z,
            })
            .collect();

        if self.previous_landmarks.len() <= hand_idx {
            self.previous_landmarks.resize(hand_idx + 1, Vec::new());
        }
        self.previous_landmarks[hand_idx] = smoothed.clone();
        smoothed
    }
}
