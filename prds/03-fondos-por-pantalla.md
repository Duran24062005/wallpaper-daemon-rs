# PRD 03 — Fondos independientes por pantalla

## Propósito

Asignar y aplicar imágenes distintas a cada monitor conectado, respetando las capacidades reales de cada escritorio.

## Alcance

- Selección independiente por `ScreenId`.
- Contrato de aplicación por pantalla.
- Backends GNOME, KDE, XFCE y Cinnamon cuando la investigación confirme soporte.
- Degradación explícita si un backend solo soporta fondo global.

## Contexto y estado actual

El selector actual recibe una lista global y excluye opcionalmente un único fondo. GNOME mediante `gsettings` no ofrece el mismo modelo por monitor en todos los entornos o versiones.

## Diseño

```rust
pub type ScreenAssignments = HashMap<String, PathBuf>;

pub trait WallpaperBackend {
    fn set_for_screen(
        &self,
        screen: &ScreenInfo,
        path: &Path,
    ) -> Result<(), WallpaperError>;
}
```

El comando conceptual será `wpd set-random --per-screen`. Debe permitir informar asignaciones fallidas sin ocultar cuáles pantallas sí fueron actualizadas.

## Decisiones

- La selección pertenece al core; la forma de aplicar una imagen pertenece al backend.
- No se simulará soporte por monitor donde el escritorio solo acepte una imagen compuesta; esa alternativa se documentará como capacidad distinta.
- Las imágenes se pueden reutilizar entre pantallas solo si la política configurada lo permite; por defecto se intenta evitar duplicados.

## Aceptación

- Una asignación reproducible puede probarse con un backend fake.
- Un monitor no soportado produce capacidad/error identificable.
- KDE, XFCE, Cinnamon y GNOME documentan qué estrategia usan y qué limitaciones tienen.

## Mantenimiento

Revisar `selector`, `ScreenProvider`, cada backend y los comandos CLI. Actualizar la matriz de capacidades cuando cambien escritorios o versiones.
