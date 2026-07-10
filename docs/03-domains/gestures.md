# Gestures Domain

## Objetivo

Reconocer gestos de la mano comparando landmarks normalizados contra plantillas almacenadas.

## Responsabilidades

- Almacenar plantillas de gestos (estáticos y dinámicos).
- Reconocer gestos estáticos mediante distancia euclidiana.
- Reconocer gestos dinámicos mediante DTW (futuro).
- Exponer comandos CRUD para gestión de gestos.

## Componentes

| Componente       | Archivo                | Propósito                          |
| ---------------- | ---------------------- | ---------------------------------- |
| `GestureEngine`  | `gestures/mod.rs`      | Algoritmo de reconocimiento        |
| Commands         | `gestures/commands.rs` | `list_gestures`, `create_gesture`, `delete_gesture` |

## Flujo

```
NormalizedLandmarks → GestureEngine::recognize_static()
  → comparar contra cada plantilla estática
  → distancia euclidiana promedio
  → si confianza > threshold → RecognizedGesture
```

## Modelos

- `GestureTemplate { id, name, gesture_type, samples, created_at }`
- `GestureType { Static, Dynamic }`
- `GestureSequence { gesture_id, frames }`
- `GestureFrame { timestamp, landmarks, detection }`
- `RecognizedGesture { gesture_id, name, confidence, type, timestamp }`

## Algoritmo

Distancia euclidiana entre landmarks:

```rust
dist = Σ((x1-x2)² + (y1-y2)² + (z1-z2)²)
confidence = 1 / (1 + sqrt(dist))
```

## Configuración

```toml
[recognition]
static_threshold = 0.8
dynamic_threshold = 0.7
smoothing_factor = 0.3
```

## Riesgos

- Sin soporte para gestos dinámicos (DTW) en v1.
- La distancia euclidiana simple puede ser imprecisa para gestos complejos.
- Depende de la calidad de la normalización de landmarks.
