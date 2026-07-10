# Camera Domain

## Objetivo

Capturar frames de vídeo desde una cámara usando Windows Media Foundation, con soporte para selección de dispositivo, resolución y FPS configurables.

## Responsabilidades

- Detectar dispositivos de cámara disponibles.
- Iniciar/detener captura.
- Notificar eventos del ciclo de vida (started, stopped, disconnected).
- Emitir frames raw para procesamiento downstream.

## Componentes

| Componente      | Archivo               | Propósito                              |
| --------------- | --------------------- | -------------------------------------- |
| `CameraEngine`  | `camera/mod.rs`       | Estado, thread de captura, start/stop  |
| Commands        | `camera/commands.rs`  | `list_cameras`, `start_camera`, `stop_camera` |

## Flujo

```
list_cameras → Vec<CameraDevice>
start_camera(device_id) → thread spawn → CameraStarted event
  loop: capture frame → NewFrame event
stop_camera → flag atomic → thread join → CameraStopped event
```

## Modelos

- `CameraDevice { id, name }`
- `CameraFrame { timestamp, data, width, height }`
- `CameraStatus { Idle, Starting, Running, Stopped, Error }`

## Eventos

| Evento              | Payload       | Disparo                |
| ------------------- | ------------- | ---------------------- |
| `CameraStarted`     | device        | start ok               |
| `CameraStopped`     | —             | stop o disconexión     |
| `CameraDisconnected`| reason        | pérdida de dispositivo |
| `NewFrame`          | CameraFrame   | cada frame capturado   |

## Configuración

```toml
[camera]
preferred_device = ""
resolution = { width = 640, height = 480 }
fps = 30
```

## Riesgos

- Windows Media Foundation requiere inicialización COM por thread.
- La reconexión automática no está implementada (futuro).
- Sin soporte para múltiples cámaras simultáneas (v1).
