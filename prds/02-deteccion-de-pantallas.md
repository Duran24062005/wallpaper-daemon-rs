# PRD 02 — Detección de pantallas

## Propósito

Permitir que el programa conozca las salidas conectadas antes de asignar fondos por monitor.

## Alcance

Definir `ScreenInfo` y `ScreenProvider`, detectar salidas en X11 y Wayland cuando el entorno lo permita y ofrecer un comando diagnóstico `wpd list-screens`.

Fuera de alcance: cambiar fondos por monitor o reproducir vídeo.

## Contexto

El MVP trata el escritorio como una superficie única y no tiene modelo de monitor. X11 ofrece RandR; Wayland no tiene una API universal y depende del compositor.

## Contratos

```rust
pub struct ScreenInfo {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub primary: bool,
}

pub trait ScreenProvider {
    fn list_screens(&self) -> Result<Vec<ScreenInfo>, WallpaperError>;
}
```

El comando debe devolver identificadores estables durante la sesión y explicar cuando la capacidad no está disponible.

## Implementación propuesta

- X11: preferir una implementación estructurada (`x11rb`) o un adaptador de `xrandr` con parser probado.
- Wayland: seleccionar proveedor por compositor; GNOME/Mutter y KDE requieren integración propia, mientras que compositores wlroots pueden usar `wlr-output-management`.
- No afirmar soporte Wayland genérico si el compositor no expone la información necesaria.

## Pruebas y aceptación

- Parseo de salidas con una, varias y ninguna pantalla.
- Resoluciones, posiciones y monitor primario correctos.
- Errores claros cuando falta `DISPLAY`, `WAYLAND_DISPLAY` o la herramienta/protocolo.
- `list-screens` no modifica el estado del escritorio.

## Mantenimiento

Revisar `crates/core`, proveedores de pantalla y la documentación de compatibilidad por escritorio. Mantener fixtures de salida de herramientas y protocolos.
