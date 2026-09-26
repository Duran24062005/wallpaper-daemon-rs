# PRD 04 — Vídeo global como fondo

## Propósito

Reproducir un vídeo como fondo de escritorio en todas las pantallas usando un reproductor externo controlado por el programa.

## Alcance

- Integración con `mpv` como proceso hijo.
- Control por socket IPC JSON de `mpv`.
- Estrategias separadas para X11, Wayland/wlroots y escritorios sin superficie de fondo compatible.
- Decodificación por hardware y límites de consumo.

Fuera de alcance: vídeo distinto por monitor y GUI.

## Contexto

`mpv` evita reimplementar decodificación. X11 puede usar una ventana de fondo con técnicas tipo `xwinwrap`; Wayland requiere `wlr-layer-shell` y GNOME/KDE pueden necesitar extensiones o quedar fuera de la primera versión.

## Contrato

```rust
pub trait VideoBackend {
    fn set_video(&mut self, path: &Path, screens: &[ScreenInfo]) -> Result<(), WallpaperError>;
    fn stop(&mut self) -> Result<(), WallpaperError>;
}
```

El proceso debe arrancar con IPC privado, `--hwdec=auto` cuando sea compatible y apagarse con una señal/control limpio.

## Riesgos y aceptación

- Rechazar entornos donde no se pueda colocar una superficie detrás de las ventanas.
- Detectar proceso muerto y devolver estado no saludable.
- Validar ruta, extensión y existencia del archivo.
- Medir CPU, GPU y memoria con vídeo activo y detenido.

## Mantenimiento

Documentar versiones mínimas de `mpv`, flags por plataforma y permisos de sockets. No mezclar la lógica de reproducción con `WallpaperBackend` sin definir compatibilidad.
