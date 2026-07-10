# Training Domain

## Objetivo

Permitir al usuario crear gestos personalizados mediante captura de muestras y generación de plantillas.

## Responsabilidades

- Gestionar el ciclo de entrenamiento (inicio, muestreo, finalización).
- Capturar N muestras de un gesto.
- Promediar landmarks entre muestras para construir plantilla.
- Almacenar plantilla en SQLite.

## Componentes

| Componente        | Archivo               | Propósito                         |
| ----------------- | --------------------- | --------------------------------- |
| `GestureTrainer`  | `trainer/mod.rs`      | Lógica de entrenamiento           |

## Flujo

```
start_training(gesture_id)
  → GestureTrainingStarted event
  loop N times:
    add_sample(GestureSequence)
    → GestureTrainingSample event
  build_template(name, type)
    → promedio de landmarks por frame
    → GestureTemplate
  save_gesture(template)
    → GestureTrainingComplete event
```

## Modelos

- `GestureTemplate` (compartido con gestures domain)
- `GestureSequence` (compartido)

## Algoritmo de Promediado

Por cada frame index:
- Sumar coordenadas (x, y, z) de cada landmark a través de todas las muestras.
- Dividir por cantidad de muestras.
- Construir `NormalizedLandmarks` promediado.

## Riesgos

- Sin validación de calidad de muestra (ruido, mano parcial).
- Sin vista previa durante captura (frontend futuro).
- El número fijo de muestras puede no ser suficiente para gestos complejos.
