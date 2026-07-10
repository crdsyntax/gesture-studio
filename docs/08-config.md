# 08 — Config

## Objetivo

Configuración global del proyecto almacenada en archivo TOML.

## Ubicaciones

| Runtime       | Ruta                                                                |
| ------------- | ------------------------------------------------------------------- |
| Desarrollo    | `config.toml` en el directorio del proyecto                         |
| Producción    | `%APPDATA%/GestureStudio/config/config.toml` (vía `directories` crate) |

## Archivo

Se auto-genera con valores por defecto si no existe. Formato:

```toml
[camera]
preferred_device = ""
resolution = { width = 640, height = 480 }
fps = 30

[vision]
model_path = "data/models/hand_landmark_full.onnx"
confidence_threshold = 0.5
max_hands = 2

[recognition]
static_threshold = 0.8
dynamic_threshold = 0.7
smoothing_factor = 0.3

[storage]
db_path = "C:/Users/<user>/AppData/Roaming/GestureStudio/data/gesture_studio.db"
max_samples_per_gesture = 50

[performance]
target_fps = 60
buffer_size = 1024
log_level = "info"
```

## Estructura Rust

```rust
struct AppConfig {
    camera: CameraConfig,
    vision: VisionConfig,
    recognition: RecognitionConfig,
    storage: StorageConfig,
    performance: PerformanceConfig,
}
```

### Sub-configs

| Config                | Campos                                          |
| --------------------- | ----------------------------------------------- |
| `CameraConfig`        | `preferred_device: Option<String>`, `resolution: Resolution`, `fps: u32` |
| `VisionConfig`        | `model_path: PathBuf`, `confidence_threshold: f32`, `max_hands: u32` |
| `RecognitionConfig`   | `static_threshold: f32`, `dynamic_threshold: f32`, `smoothing_factor: f32` |
| `StorageConfig`       | `db_path: PathBuf`, `max_samples_per_gesture: u32` |
| `PerformanceConfig`   | `target_fps: u32`, `buffer_size: usize`, `log_level: String` |

## Logging

Controlado por variable de entorno `RUST_LOG`. Por defecto: `gesture_studio=info`.

## Ciclo de Vida

```
App startup:
  config = AppConfig::load()  // read file or create default

En runtime:
  Frontend → invoke("update_config", config)
    → AppState.config.write() → EventProcessor recibe ConfigChanged

ConfigChanged:
  → los módulos releen config de AppState según necesidad
```

## Variables de Entorno

| Variable      | Efecto                             |
| ------------- | ---------------------------------- |
| `RUST_LOG`    | Nivel de logging (trace/debug/info/warn/error) |
| `GESTURE_STUDIO_CONFIG` | (futuro) Ruta alternativa de config |
