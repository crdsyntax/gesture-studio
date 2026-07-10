# 01 — Overview

## Objetivo

Aplicación de escritorio en Rust que detecta movimientos y gestos de la mano mediante cámara, permite entrenar gestos personalizados, almacenarlos y asociarlos a acciones ejecutables en Windows.

## Arquitectura

```
Frontend (React + MUI)
       |
   Tauri IPC (invoke)
       |
  Rust Backend (Tauri Commands)
       |
   Application Layer
       |
   Gesture Engine
       |
   Vision Engine (MediaPipe ONNX)
       |
   Camera Engine (WMF)
```

## Tecnologías

| Capa       | Tecnología                        |
| ---------- | --------------------------------- |
| Frontend   | React 19, TypeScript, MUI 6, Vite 6, Zustand, TanStack Router |
| IPC        | Tauri 2 (invoke command bridge)   |
| Backend    | Rust 1.96, Tokio, Crossbeam       |
| Visión     | MediaPipe Hands (modelo ONNX)     |
| Inferencia | ONNX Runtime (futuro)             |
| Cámara     | Windows Media Foundation          |
| Persistencia| SQLite (rusqlite, bundled)       |
| Logging    | tracing + tracing-subscriber      |
| Config     | TOML + serde                      |

## Convenciones

- Sin capa HTTP ni REST. Todo el frontend se comunica mediante comandos Tauri tipados.
- Sin ORM. Las consultas SQL son directas con rusqlite.
- Sin normalización entre motores de base de datos. Cada engine mantiene su estructura nativa.
- Commits atómicos. Un solo cambio por capa.
- Tipado estricto en todos los límites del sistema (IPC, comandos, DTOs).

## Flujo General

```
1. Usuario conecta cámara
2. CameraEngine captura frames → evento NewFrame
3. VisionEngine ejecuta inferencia MediaPipe → HandDetection
4. LandmarkNormalizer normaliza landmarks
5. GestureEngine compara contra plantillas almacenadas
6. Si coincide → RecognizedGesture → ActionEngine ejecuta acción asociada
7. Resultado → evento ActionExecuted → Frontend actualiza UI
```
