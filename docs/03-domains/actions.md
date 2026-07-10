# Actions Domain

## Objetivo

Ejecutar acciones del sistema cuando un gesto es reconocido.

## Responsabilidades

- Mantener registro de acciones asociadas a gestos.
- Ejecutar acción correspondiente al tipo (abrir app, comando, URL, etc.).
- Reportar resultado de ejecución.
- Proveer trait extensible para nuevos tipos de acción.

## Componentes

| Componente        | Archivo               | Propósito                          |
| ----------------- | --------------------- | ---------------------------------- |
| `ActionEngine`    | `actions/mod.rs`      | Registry de executors + dispatch   |
| `ActionExecutor`  | `actions/mod.rs`      | Trait para implementar acciones    |
| `OpenAppExecutor` | `actions/mod.rs`      | Abre aplicación por nombre/ruta    |
| `ExecuteCommandExecutor` | `actions/mod.rs` | Ejecuta comando shell            |
| `OpenUrlExecutor` | `actions/mod.rs`      | Abre URL en navegador              |
| Commands          | `actions/commands.rs` | `list_actions`, `execute_action`   |

## Flujo

```
GestureRecognized
  → lookup actions by gesture_id
  → for each action:
    → ActionEngine::execute(&action)
    → match action_type:
        OpenApp         → OpenAppExecutor::execute(payload)
        ExecuteCommand  → ExecuteCommandExecutor::execute(payload)
        OpenUrl         → OpenUrlExecutor::execute(payload)
        ...
    → ExecutionResult
```

## Modelos

- `AssignedAction { action_id, gesture_id, action_type, payload, enabled }`
- `ActionType`: enum con 14 variantes (OpenApp, ExecuteCommand, OpenUrl, ChangeVolume, MediaControl, LockWorkstation, SimulateKeyboard, SimulateMouse, HttpRequest, WebSocket, Mqtt, PowerShell, Bash, TauriEvent)
- `ExecutionResult { action_id, success, message, timestamp }`
- `ActionExecutor` trait: `fn execute(&self, payload: &str) -> Result<ExecutionResult, AppError>`

## Executors Implementados

| Executor                | ActionType      | Payload         |
| ----------------------- | --------------- | --------------- |
| `OpenAppExecutor`       | `OpenApp`       | Ruta o nombre   |
| `ExecuteCommandExecutor`| `ExecuteCommand`| Comando shell   |
| `OpenUrlExecutor`       | `OpenUrl`       | URL             |

## Riesgos

- Ejecutar comandos shell tiene implicaciones de seguridad (validar en Rust).
- Sin sandboxing para acciones arbitrarias.
- Sin confirmación de usuario antes de ejecutar acciones críticas.
