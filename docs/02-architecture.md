# 02 — Architecture

## Capas

```
┌─────────────────────────────────┐
│         Frontend (React)        │
│  Dashboard / Gestures / Actions │
│  Training / Settings            │
└──────────────┬──────────────────┘
               │ Tauri invoke()
┌──────────────▼──────────────────┐
│     Tauri Command Layer         │
│  app/commands.rs                │
│  camera/commands.rs             │
│  gestures/commands.rs           │
│  actions/commands.rs            │
│  storage/commands.rs            │
└──────────────┬──────────────────┘
┌──────────────▼──────────────────┐
│     Application Layer           │
│  services/GestureService        │
│  orchestrates camera → vision   │
│  → landmarks → gestures → actions│
└──────────────┬──────────────────┘
┌──────────────▼──────────────────┐
│     Domain Modules              │
│  camera/   vision/  landmarks/  │
│  gestures/ trainer/ actions/    │
│  storage/  events/  config/     │
└──────────────┬──────────────────┘
┌──────────────▼──────────────────┐
│     Infrastructure              │
│  SQLite / WMF / ONNX Runtime    │
└─────────────────────────────────┘
```

## Módulos

| Módulo      | Responsabilidad                          | Dependencias              |
| ----------- | ---------------------------------------- | ------------------------- |
| `app`       | State, error handling, Tauri setup       | models, config, events    |
| `camera`    | Captura de vídeo (WMF stub)              | events, models            |
| `vision`    | Inferencia MediaPipe (ONNX stub)         | models                    |
| `landmarks` | Normalización: traslación, escala, smoothing | models                |
| `gestures`  | Reconocimiento (distancia euclidiana)    | models, storage           |
| `trainer`   | Entrenamiento: muestreo, promediado      | models, events            |
| `actions`   | Ejecución de acciones (trait + impls)    | models                    |
| `storage`   | SQLite: gestures, samples, actions, settings | models, rusqlite       |
| `config`    | Config TOML con auto-generación          | serde, toml               |
| `events`    | Sistema de eventos crossbeam             | models                    |
| `services`  | Orquestador de todos los módulos         | todos los anteriores      |
| `utils`     | FrameTimer (FPS/latencia)                | —                         |
| `models`    | Tipos compartidos (DTOs, enums)          | serde, uuid, chrono       |

## Dependencias entre módulos

```
app ──► config, events
        │
camera ──► events, models
vision ──► models
landmarks ──► models
gestures ──► models, storage
trainer ──► models, events
actions ──► models
storage ──► models
services ──► camera, vision, landmarks, gestures, actions, storage, config
```

## Principios Arquitectónicos

- Las capas superiores solo dependen de abstracciones (traits).
- Los comandos Tauri son delgados: validan entrada y delegan.
- No hay lógica de UI en backend ni lógica de BD en frontend.
- Cada módulo se comunica mediante eventos crossbeam desacoplados.
