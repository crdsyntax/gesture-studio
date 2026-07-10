use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub camera: CameraConfig,
    pub vision: VisionConfig,
    pub recognition: RecognitionConfig,
    pub storage: StorageConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub preferred_device: Option<String>,
    pub resolution: Resolution,
    pub fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionConfig {
    pub model_path: PathBuf,
    pub confidence_threshold: f32,
    pub max_hands: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionConfig {
    pub static_threshold: f32,
    pub dynamic_threshold: f32,
    pub smoothing_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub db_path: PathBuf,
    pub max_samples_per_gesture: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub target_fps: u32,
    pub buffer_size: usize,
    pub log_level: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let data_dir = directories::ProjectDirs::from("com", "gesturestudio", "GestureStudio")
            .map(|d| d.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("data"));

        Self {
            camera: CameraConfig {
                preferred_device: None,
                resolution: Resolution {
                    width: 640,
                    height: 480,
                },
                fps: 30,
            },
            vision: VisionConfig {
                model_path: data_dir.join("models").join("hand_landmark_full.onnx"),
                confidence_threshold: 0.5,
                max_hands: 2,
            },
            recognition: RecognitionConfig {
                static_threshold: 0.8,
                dynamic_threshold: 0.7,
                smoothing_factor: 0.3,
            },
            storage: StorageConfig {
                db_path: data_dir.join("gesture_studio.db"),
                max_samples_per_gesture: 50,
            },
            performance: PerformanceConfig {
                target_fps: 60,
                buffer_size: 1024,
                log_level: "info".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn storage_path(&self) -> &std::path::Path {
        &self.storage.db_path
    }

    pub fn config_path() -> PathBuf {
        directories::ProjectDirs::from("com", "gesturestudio", "GestureStudio")
            .map(|d| d.config_dir().join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"))
    }

    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            let config = AppConfig::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let config_path = Self::config_path();
        if let Some(parent) = config_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(content) = toml::to_string_pretty(self) {
            let _ = std::fs::write(&config_path, content);
        }
    }
}
