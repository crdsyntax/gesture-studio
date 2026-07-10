# 10 — Flows

## Flujo 1: Reconocimiento de Gesto (completo)

```
Usuario: conecta cámara en UI
  → Frontend: invoke("start_camera", { device_id })
    → camera::commands::start_camera()
      → CameraEngine::start()
        → thread spawn → loop de captura
        → por cada frame: event_tx.send(NewFrame)

EventProcessor recibe NewFrame
  → VisionEngine::detect_hands(frame)
    → si detecta: event_tx.send(HandDetected)
    → si no: event_tx.send(HandsLost)

EventProcessor recibe HandDetected
  → LandmarkNormalizer::normalize(detection)
    → traslación → escala → smoothing
    → event_tx.send(LandmarksNormalized)

EventProcessor recibe LandmarksNormalized
  → GestureEngine::recognize_static(landmarks)
    → comparar contra plantillas en memoria
    → si confianza > threshold:
      → RecognizedGesture { gesture_id, confidence }
      → event_tx.send(GestureRecognized)

EventProcessor recibe GestureRecognized
  → ActionEngine::lookup(gesture_id)
  → ActionEngine::execute(action)
    → match action_type:
        OpenApp  → OpenAppExecutor::execute(payload)
        ExecuteCommand → ExecuteCommandExecutor::execute(payload)
        OpenUrl  → OpenUrlExecutor::execute(payload)
    → event_tx.send(ActionExecuted { result })

EventProcessor recibe ActionExecuted
  → forward a frontend via listener Tauri
  → UI actualiza log/notificación
```

## Flujo 2: Entrenamiento de Gesto

```
Usuario: crea gesto "Peace Sign", tipo Static
  → Frontend: invoke("create_gesture", { name: "Peace Sign", gesture_type: "static" })
    → gestures::commands::create_gesture()
    → retorna GestureInfo { id, name, type, created_at }

Usuario: hace clic en "Entrenar"
  → Frontend: inicia modo training (evento GestureTrainingStarted)
  → Usuario pone mano en posición
  → por cada muestra:
    → se capturan ~60 frames (2 seg a 30 fps)
    → GestureSequence → GestureTrainingSample event
  → cuando tiene N muestras (>3):
    → GestureTrainer::build_template()
      → promedia landmarks por frame_index
      → construye GestureTemplate
    → Storage::save_gesture(&template)
    → GestureTrainingComplete event
    → Frontend muestra resultado

Usuario: asigna acción al gesto
  → Frontend: invoke("...") (futuro comando assign_action)
  → acción queda en BD lista para ejecutarse
```

## Flujo 3: Inicio de Aplicación

```
App startup:
  1. tracing_subscriber init
  2. AppConfig::load() → leer/crear config.toml
  3. Crear (event_tx, event_rx) unbounded channel
  4. Storage::open(&config.storage_path) → SQLite init + tablas
  5. AppState {
       config: Arc<RwLock<AppConfig>>,
       event_tx,
       storage: Arc<Mutex<Storage>>,
     }
  6. EventProcessor::spawn(event_rx) → hilo consumidor
  7. Tauri builder:
       - plugins: opener, shell
       - manage(app_state)
       - register 11 commands
  8. Tauri run → abre ventana React
```

## Flujo 4: Cambio de Configuración

```
Usuario: cambia threshold de reconocimiento en Settings
  → Frontend: invoke("update_config", { recognition: { static_threshold: 0.9 } })
    → app::commands::update_config()
    → write new config a Arc<RwLock<AppConfig>>
    → event_tx.send(ConfigChanged)
    → EventProcessor recibe ConfigChanged
      → (futuro: notificar módulos que relean)
    → Tauri persiste config a disco
    → Frontend: respuesta Ok → UI confirma
```
