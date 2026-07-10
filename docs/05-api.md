# 05 — API (Tauri Commands)

## Objetivo

Bridge IPC entre frontend React y backend Rust. Todos los comandos son invocados mediante `invoke()` de Tauri.

## Comandos

### get_app_status

| Propiedad      | Tipo    | Descripción              |
| -------------- | ------- | ------------------------ |
| Ruta           | `get_app_status` | —                |
| Entrada        | —       | —                        |
| Salida         | `AppStatus` | estado actual del sistema |

**DTO Salida:**
```rust
struct AppStatus {
    version: String,      // env!("CARGO_PKG_VERSION")
    camera_active: bool,
    model_loaded: bool,
    fps: f64,
    latency_ms: f64,
}
```

**Errores:** Ninguno.

---

### get_config

| Propiedad      | Tipo    | Descripción              |
| -------------- | ------- | ------------------------ |
| Ruta           | `get_config` | —                  |
| Entrada        | —       | —                        |
| Salida         | `AppConfig` | configuración completa |

**Errores:** Ninguno.

---

### update_config

| Propiedad      | Tipo    | Descripción                    |
| -------------- | ------- | ------------------------------ |
| Ruta           | `update_config` | —                        |
| Entrada        | `config: AppConfig` | —                    |
| Salida         | `()`    | —                              |

**Efectos:** Emite evento `ConfigChanged`. Persiste en disco.

**Errores:** Ninguno.

---

### list_cameras

| Propiedad      | Tipo    | Descripción                       |
| -------------- | ------- | --------------------------------- |
| Ruta           | `list_cameras` | —                           |
| Entrada        | —       | —                                 |
| Salida         | `Vec<CameraDevice>` | lista de cámaras detectadas |

**DTO Salida:**
```rust
struct CameraDevice { id: String, name: String }
```

**Nota:** En v1 retorna una cámara dummy "Default Camera".

---

### start_camera

| Propiedad      | Tipo    | Descripción                    |
| -------------- | ------- | ------------------------------ |
| Ruta           | `start_camera` | —                        |
| Entrada        | `device_id: String` | —                    |
| Salida         | `()`    | —                              |

**Errores:** `AppError::Camera(String)` si no puede iniciar.

---

### stop_camera

| Propiedad      | Tipo    | Descripción                    |
| -------------- | ------- | ------------------------------ |
| Ruta           | `stop_camera` | —                         |
| Entrada        | —       | —                              |
| Salida         | `()`    | —                              |

**Errores:** `AppError::Camera(String)` si no está corriendo.

---

### list_gestures

| Propiedad      | Tipo    | Descripción                       |
| -------------- | ------- | --------------------------------- |
| Ruta           | `list_gestures` | —                          |
| Entrada        | —       | —                                 |
| Salida         | `Vec<GestureInfo>` | gestos almacenados           |

**DTO Salida:**
```rust
struct GestureInfo {
    id: String (UUID),
    name: String,
    gesture_type: "static" | "dynamic",
    created_at: String (RFC 3339),
}
```

---

### create_gesture

| Propiedad      | Tipo    | Descripción                    |
| -------------- | ------- | ------------------------------ |
| Ruta           | `create_gesture` | —                       |
| Entrada        | `name: String, gesture_type: String` | — |
| Salida         | `GestureInfo` | gesto creado              |

**Errores:** `AppError::Gesture(String)` si falla validación.

---

### delete_gesture

| Propiedad      | Tipo    | Descripción                    |
| -------------- | ------- | ------------------------------ |
| Ruta           | `delete_gesture` | —                       |
| Entrada        | `gesture_id: Uuid` | —                    |
| Salida         | `()`    | —                              |

**Efectos:** Elimina gesto y sus samples/acciones en cascada.

---

### list_actions

| Propiedad      | Tipo    | Descripción                       |
| -------------- | ------- | --------------------------------- |
| Ruta           | `list_actions` | —                          |
| Entrada        | `gesture_id: Uuid` | filtrar por gesto        |
| Salida         | `Vec<ActionInfo>` | acciones asociadas        |

**DTO Salida:**
```rust
struct ActionInfo {
    id: String (UUID),
    gesture_id: String (UUID),
    action_type: String,
    payload: String,
    enabled: bool,
}
```

---

### execute_action

| Propiedad      | Tipo    | Descripción                    |
| -------------- | ------- | ------------------------------ |
| Ruta           | `execute_action` | —                      |
| Entrada        | `action_id: Uuid` | —                    |
| Salida         | `ExecutionResult` | resultado de ejecución |

**DTO Salida:**
```rust
struct ExecutionResult {
    action_id: String (UUID),
    success: bool,
    message: String,
    timestamp: String (RFC 3339),
}
```

---

### get_stats

| Propiedad      | Tipo    | Descripción                       |
| -------------- | ------- | --------------------------------- |
| Ruta           | `get_stats` | —                           |
| Entrada        | —       | —                                 |
| Salida         | `StorageStats` | estadísticas de almacenamiento |

**DTO Salida:**
```rust
struct StorageStats {
    gesture_count: usize,
    sample_count: usize,
    action_count: usize,
}
```

---

## Tipos Compartidos

| Tipo               | Uso                             |
| ------------------ | ------------------------------- |
| `AppConfig`        | config/commands.rs              |
| `AppStatus`        | app/commands.rs                 |
| `CameraDevice`     | camera/commands.rs              |
| `GestureInfo`      | gestures/commands.rs            |
| `ActionInfo`       | actions/commands.rs             |
| `ExecutionResult`  | models/mod.rs                   |
| `StorageStats`     | storage/commands.rs             |

## Errores Globales

Todos los comandos retornan `AppError`, serializado como string via `impl serde::Serialize for AppError`.

| Variante          | Causa típica                  |
| ----------------- | ----------------------------- |
| `Camera(String)`  | Fallo de cámara               |
| `Vision(String)`  | Fallo de inferencia           |
| `Gesture(String)` | Fallo de reconocimiento       |
| `Training(String)`| Fallo de entrenamiento        |
| `Action(String)`  | Fallo de ejecución            |
| `Storage`         | Fallo SQLite                  |
| `Config(String)`  | Config inválida               |
| `Io`              | Error de E/S                  |
| `Serialization`   | Error serde                   |
| `Internal(String)`| Error inesperado              |
