# PRD 01 — Workspace y arquitectura core

## Propósito

Separar la lógica de dominio del proceso ejecutable y de las integraciones con escritorios Linux, preparando sustitución de backends y pruebas sin sesión gráfica.

## Alcance

- Convertir el crate actual en un Cargo workspace.
- Extraer una librería `core` y un binario `daemon`.
- Mover la integración GNOME a un backend independiente.
- Definir interfaces para selección, configuración, temporización y wallpaper.

Fuera de alcance: implementar KDE, XFCE, vídeo o IPC completo.

## Estado actual

Existe un único crate con módulos `scanner`, `selector`, `scheduler` y `wallpapers`; `main.rs` orquesta directamente todo y `code_templates` no participa en la compilación.

## Diseño propuesto

```text
crates/core/              dominio, configuración y traits
crates/daemon/            ciclo de vida y proceso principal
crates/backends/gnome/    gsettings/GNOME
```

`core` no debe invocar `gsettings`, leer variables gráficas ni crear procesos hijos. El daemon compone un `WallpaperManager` con un backend concreto.

## Contratos

```rust
pub trait WallpaperBackend {
    fn name(&self) -> &'static str;
    fn current(&self) -> Result<Option<PathBuf>, WallpaperError>;
    fn set_image(&self, path: &Path) -> Result<(), WallpaperError>;
}
```

El trait debe evolucionar sin obligar al core a conocer GNOME. Los errores del dominio se mantienen estables; los detalles de `gsettings` quedan en el backend.

## Decisiones y riesgos

- Migración incremental: primero mover código sin cambiar comportamiento, después sustituir acoplamientos.
- Evitar un workspace artificial que solo reubique errores; la extracción depende de cerrar el PRD 00.
- El backend debe poder sustituirse por un fake en pruebas.

## Aceptación

- El workspace compila desde la raíz.
- El daemon conserva el comportamiento del MVP GNOME.
- El core puede probarse sin `gsettings` ni sesión gráfica.
- Existe documentación de dependencias y responsabilidades por crate.

## Mantenimiento

Revisar `Cargo.toml`, `src/main.rs`, `src/wallpapers.rs`, `src/error.rs` y `docs/Architecture.md`. Cada backend nuevo debe documentar capacidades y limitaciones propias.
