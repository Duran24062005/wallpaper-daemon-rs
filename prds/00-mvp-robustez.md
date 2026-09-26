# PRD 00 — Robustez del MVP

## Propósito

Definir el trabajo necesario para convertir el MVP actual en una base confiable antes de introducir workspace, daemon, múltiples pantallas o vídeo.

## Alcance

- Normalización y comparación correcta de rutas y URI `file://`.
- Soporte para fondos de tema claro y oscuro.
- Errores explícitos y validación de procesos externos.
- Escaneo recursivo opcional.
- Configuración persistente e independiente del directorio de ejecución.
- Pruebas unitarias e integración básica.

Fuera de alcance: daemon, IPC, múltiples monitores, vídeo y GUI.

## Contexto y estado actual

El binario en `src/main.rs` escanea `assets`, consulta `picture-uri-dark`, selecciona una imagen y espera diez segundos. `src/error.rs` ya contiene `WallpaperError` y `src/wallpapers.rs` valida los estados de `gsettings`, pero aún hay acoplamiento a rutas fijas, el parseo de URI no decodifica caracteres escapados y la comparación de rutas puede mezclar rutas relativas y absolutas.

## Funcionalidad propuesta

1. Resolver las imágenes a rutas canónicas o absolutas antes de compararlas con el fondo actual.
2. Centralizar el parseo y serialización de URI en la capa de wallpaper; soportar `%20` y otros escapes válidos.
3. Definir una política explícita para `picture-uri` y `picture-uri-dark`: detectar el esquema activo o actualizar ambas claves de forma consistente.
4. Mantener el bucle vivo ante errores recuperables, registrando el error y continuando según una política documentada.
5. Añadir `Config` con carpeta, intervalo y si el escaneo es recursivo. Cargar y guardar TOML en `~/.config/wallpaper-daemon-rs/config.toml`, respetando rutas XDG cuando estén disponibles.
6. Hacer el escaneo recursivo configurable; conservar el filtro `jpg`, `jpeg`, `png` y `webp`, sin distinguir mayúsculas.

## Contratos e interfaces

```rust
pub struct Config {
    pub wallpaper_directory: PathBuf,
    pub interval_seconds: u64,
    pub recursive: bool,
}

pub fn load() -> Result<Config, ConfigError>;
pub fn save(config: &Config) -> Result<(), ConfigError>;
```

Las funciones de `wallpapers` deben devolver `WallpaperError`, verificar `ExitStatus` en lectura y escritura, y no ocultar errores de URI, UTF-8 o proceso.

## Decisiones

- La configuración pertenece al usuario y no al repositorio; `assets` queda como valor por defecto de desarrollo.
- La canonicalización se hace en un límite bien definido, no dispersa en `main` y `selector`.
- La dependencia de escaneo recursivo se incorpora solo si simplifica claramente la implementación; no se añade un runtime asíncrono.

## Pruebas y aceptación

- Rutas relativas y absolutas que representan el mismo archivo no se seleccionan de nuevo.
- URI con espacios escapados se convierte correctamente a `PathBuf`.
- Se prueban ambas claves de GNOME mediante funciones aislables o un backend simulado.
- Un `gsettings` fallido produce un error descriptivo y no un panic.
- Se prueban configuración por defecto, archivo inválido, directorio vacío, directorio inexistente, extensiones y subdirectorios.
- `cargo test`, `cargo clippy` y `cargo fmt --check` pasan.

## Mantenimiento

Revisar `src/main.rs`, `src/config.rs`, `src/error.rs`, `src/scanner.rs`, `src/selector.rs`, `src/wallpapers.rs`, `tests/scanner_test.rs` y `tests/wallpaper_test.rs`. Actualizar `README.md`, `docs/Architecture.md` y `docs/testing.md` cuando cambien las opciones o los valores por defecto.
