# 06 — Services / GestureService

## Responsabilidad

Orquestador principal del pipeline de reconocimiento. Coordina la captura de cámara, la inferencia de visión, la normalización de landmarks, el reconocimiento de gestos y la ejecución de acciones.

## Dependencias

| Componente         | Módulo   | Propósito                          |
| ------------------ | -------- | ---------------------------------- |
| `CameraEngine`     | camera   | Captura de frames                  |
| `VisionEngine`     | vision   | Detección de manos                 |
| `LandmarkNormalizer`| landmarks | Normalización de landmarks        |
| `GestureEngine`    | gestures | Reconocimiento de gestos           |
| `ActionEngine`     | actions  | Ejecución de acciones              |

## Construcción

```rust
GestureService::new(config: &AppConfig, event_tx: Sender<AppEvent>)
```

Inicializa todos los sub-motores. Cada uno recibe una copia del canal de eventos (`event_tx.clone()`).

## Métodos Públicos

| Método            | Entrada                     | Salida          |
| ----------------- | --------------------------- | ---------------- |
| `load_from_storage` | `&Arc<Mutex<Storage>>`    | ()              |

Carga las plantillas almacenadas en `GestureEngine` al inicio de la app.

## Flujo Interno

```
Frontend: invoke("start_camera")
  → service.camera.start()
    → thread spawn → emite NewFrame por crossbeam
  → app/EventProcessor recibe NewFrame
    → service.vision.detect()
    → service.landmarks.normalize()
    → service.gestures.recognize()
    → si match → service.actions.execute()
```

El flujo real está orquestado por `EventProcessor` en `app/` basado en eventos, no directamente por este servicio.

## Eventos Generados

Ninguno directamente. Delega la emisión a los sub-motores.

## Riesgos

- El pipeline es secuencial. Un cuello de botella en visión afecta todo el throughput.
- Sin backpressure entre stages.
- Depende de `crossbeam_channel::unbounded` — sin límite de crecimiento.
