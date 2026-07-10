# 11 — Decisiones Arquitectónicas (ADR)

## ADR-001: Eventos mediante crossbeam en lugar de Tauri events

**Problema:** Necesitamos comunicación asíncrona entre módulos Rust sin acoplamiento directo. Tauri proporciona `tauri::EventEmitter` pero requiere acceso a `AppHandle`.

**Alternativas:**
- Tauri events directos (AppHandle global)
- Tokio mpsc
- crossbeam channels

**Solución elegida:** crossbeam channels unbounded.

**Consecuencias:**
- Los módulos no dependen de Tauri, facilitando testing unitario y reuso como library standalone.
- `EventProcessor` es el único consumidor del canal, simplificando sincronización.
- Sin límite de tamaño del canal (riesgo de crecimiento infinito si consumidor es lento).
- `event_tx` clonable distribuido a todos los módulos.

---

## ADR-002: `std::sync::{Mutex, RwLock}` en lugar de parking_lot

**Problema:** `rusqlite::Connection` no implementa `Sync`, por lo que necesita `Mutex`. `parking_lot::Mutex` es más rápido pero no compatible con todas las operaciones.

**Alternativas:**
- parking_lot::Mutex
- std::sync::Mutex
- rwlock dedicado por recurso

**Solución elegida:** `std::sync::Mutex` para Storage, `std::sync::RwLock` para config.

**Consecuencias:**
- API estándar sin dependencia extra.
- `Mutex` es suficiente para Storage dado que la mayoría de operaciones son write.
- `RwLock` permite lecturas concurrentes de config.

---

## ADR-003: Sin ORM. SQL directo con rusqlite

**Problema:** Necesitamos persistencia local con mínimo overhead.

**Alternativas:**
- Diesel ORM
- SeaORM
- SQLx
- rusqlite directo

**Solución elegida:** rusqlite con queries SQL directas sin capa ORM.

**Consecuencias:**
- Cero generación de código y macros complejas.
- Control total sobre queries y optimizaciones (WAL, foreign keys).
- Más boilerplate manual en operaciones CRUD.
- Fácil de migrar a otro motor SQL si fuera necesario.

---

## ADR-004: 11 comandos Tauri planos sin capa REST

**Problema:** El frontend necesita comunicación con el backend. La opción natural en Tauri es IPC via commands.

**Alternativas:**
- Servidor HTTP embebido (actix-web, axum)
- Tauri commands directos
- gRPC

**Solución elegida:** Tauri commands directos sin HTTP intermedio.

**Consecuencias:**
- Menor latencia (sin serialización HTTP).
- Tipado automático entre Rust y TypeScript.
- Sin puertos de red expuestos.
- Acoplamiento a Tauri (no reusable con otros frontends sin adaptador).

---

## ADR-005: GestureType y ActionType con `PartialEq` y `Hash`

**Problema:** El gesture engine necesita comparar y buscar por tipo de gesto/acción eficientemente.

**Alternativas:**
- Strings con match
- Integers discriminantes
- Enums con derive

**Solución elegida:** Enums Rust con derive macros.

**Consecuencias:**
- Type safety en toda la codebase.
- Pattern matching exhaustivo en handlers.
- Serialización serde para IPC y BD.

---

## ADR-006: TOML para config, no JSON

**Problema:** Formato de serialización para configuración.

**Alternativas:**
- JSON (más universal, pero sin comentarios)
- YAML (más complejo)
- TOML (estándar en Rust, soporta comentarios)

**Solución elegida:** TOML via crate `toml`.

**Consecuencias:**
- Formato legible, editable manualmente.
- Integración natural con Rust (estándar en Cargo).
- Sin soporte de arrays anidados complejos, pero suficiente para config plana.
