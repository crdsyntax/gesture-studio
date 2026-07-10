# Vision Domain

## Objetivo

Ejecutar inferencia del modelo MediaPipe Hands sobre frames de cámara para detectar manos y obtener 21 landmarks por mano.

## Responsabilidades

- Cargar modelo ONNX.
- Ejecutar inferencia por frame.
- Devolver detecciones con confianza.
- Soportar múltiples manos simultáneas.

## Componentes

| Componente      | Archivo              | Propósito                         |
| --------------- | -------------------- | --------------------------------- |
| `VisionEngine`  | `vision/mod.rs`      | Carga de modelo, inferencia stub  |

## Flujo

```
load_model(path) → model_loaded = true
detect_hands(frame) → Vec<HandDetection>
```

## Modelos

- `HandDetection { hand_id, confidence, landmarks, handedness }`
- `LandmarkPoint { x, y, z }`
- `HandType { Left, Right }`

## Configuración

```toml
[vision]
model_path = "data/models/hand_landmark_full.onnx"
confidence_threshold = 0.5
max_hands = 2
```

## Riesgos

- El modelo ONNX (MediaPipe) debe descargarse por separado (~10 MB).
- Sin inferencia real implementada en Sprint 1 (stub).
- ONNX Runtime no incluido como dependencia aún (futuro).
