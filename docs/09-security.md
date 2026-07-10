# 09 — Security

## Objetivo

Documentar consideraciones de seguridad en la aplicación.

## Permisos Tauri

En v1, la app solicita permisos de:

| Permiso                 | Propósito                          |
| ----------------------- | ---------------------------------- |
| `tauri-plugin-opener`   | Abrir URL desde Tauri              |
| `tauri-plugin-shell`    | Ejecutar comandos shell            |

## Consideraciones

### Ejecución de Comandos

El comando `ExecuteCommand` (via `actions/`) permite ejecutar comandos shell arbitrarios. Riesgos:

- Sin validación de payload en v1.
- Un usuario malicioso podría asignar una acción con `payload: "rm -rf /"`.
- Mitigación futura: whitelist de comandos permitidos, confirmación UI.

### Ejecución de Acciones Automáticas

Las acciones se disparan automáticamente al reconocer un gesto. Riesgos:

- Un falso positivo podría ejecutar una acción destructiva.
- Sin confirmación por defecto.
- Mitigación futura: toggle "require confirmation for dangerous actions".

### Almacenamiento Local

- SQLite almacenado en `%APPDATA%` sin cifrado.
- Datos sensibles (si los hubiera) estarían en texto plano.
- Mitigación futura: SQLCipher o cifrado a nivel de aplicación.

## Guards

En v1 no hay sistema de autenticación, roles ni guards. Es una app de escritorio monousuario.

## Validaciones Existentes

- Ninguna en comandos v1. Los comandos son stubs que siempre retornan Ok.
- Futuro: validación de entrada en cada comando Tauri antes de delegar a módulos.
