# 12 — Glossary

## Términos

| Término              | Definición |
| -------------------- | ---------- |
| **Gesture**          | Secuencia de posiciones de la mano que representa una intención. Puede ser estático (una pose) o dinámico (un movimiento). |
| **Static Gesture**   | Gesto que se reconoce por una única pose de la mano (ej. puño, peace sign, palm). Se compara contra landmarks normalizados de un solo frame. |
| **Dynamic Gesture**  | Gesto que involucra movimiento en el tiempo (ej. swipe, circle). Se reconoce mediante DTW comparando secuencias de frames. (No implementado en v1). |
| **Landmark**         | Punto anatómico en la mano. MediaPipe Hands detecta 21 landmarks por mano (muñeca, cada falange, punta de cada dedo). |
| **Normalization**    | Proceso de transformar landmarks raw a un espacio invariante: traslación al wrist, escalado por distancia muñeca-dedo medio, smoothing temporal. |
| **Plantilla**        | Representación de referencia de un gesto, obtenida promediando múltiples muestras de entrenamiento. |
| **Muestra**          | Una grabación del gesto: secuencia de GestureFrames capturada durante entrenamiento. |
| **Frame**            | Una sola imagen de cámara con sus landmarks asociados. |
| **FPS**              | Frames Per Second. Tasa de captura/procesamiento objetivo. |
| **Latency**          | Tiempo entre que ocurre un gesto y se ejecuta la acción asociada. Objetivo < 50 ms. |
| **Action**           | Operación ejecutable asociada a un gesto: abrir app, ejecutar comando, abrir URL, etc. |
| **Action Executor**  | Implementación del trait `ActionExecutor` que sabe ejecutar un tipo específico de acción. |
| **WMF**              | Windows Media Foundation. Framework de Windows para captura y reproducción multimedia. |
| **ONNX**             | Open Neural Network Exchange. Formato para modelos de ML interoperables. |
| **MediaPipe Hands**  | Modelo de Google para detección de manos y landmarks. |
| **Crossbeam**        | Librería Rust para canales concurrentes sin dependencia de Tokio. |
| **Tauri**            | Framework para construir aplicaciones de escritorio con frontend web y backend Rust. |
| **IPC**              | Inter-Process Communication. Mecanismo de comunicación entre proceso Rust y webview. |
| **DTO**              | Data Transfer Object. Tipo plano serializable usado en fronteras del sistema (IPC, BD). |
| **EventProcessor**   | Hilo dedicado que consume eventos del canal crossbeam y los rutea a los módulos correspondientes. |
| **GestureService**   | Orquestador principal del pipeline de reconocimiento. |
| **AppState**         | Estado global de la aplicación compartido entre comandos Tauri: config, storage, event_tx. |
| **AppError**         | Error unificado con 10 variantes que serializa a string para IPC. |
| **Smoothing**        | Filtro exponencial sobre landmarks consecutivos para reducir jitter. Factor configurable (0.0–1.0). |
| **DTW**              | Dynamic Time Warping. Algoritmo para comparar secuencias de diferente longitud. (Futuro, para gestos dinámicos). |
