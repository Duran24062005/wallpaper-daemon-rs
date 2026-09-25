## Fase 0 — Robustecer el MVP
- [x] **0.1** Crear `WallpaperError` con `thiserror` y quitar todos los `expect()`/`unwrap()`
- [ ] **0.2** Canonicalizar rutas antes de comparar con la URI de `gsettings` (arregla la repetición de fondo)
- [ ] **0.3** Soportar `picture-uri` (tema claro) además de `picture-uri-dark`
- [ ] **0.4** Decodificar URIs (`%20`, etc.)
- [ ] **0.5** Verificar código de salida de `gsettings get/set`
- [ ] **0.6** Escaneo recursivo opcional con `walkdir`
- [ ] **0.7** Configuración persistente en `~/.config/wallpaper-daemon-rs/config.toml` (`serde` + `toml` + `directories`)
- [ ] **0.8** Tests unitarios para `scanner`, `selector`, `wallpapers`

## Fase 1 — Reorganizar el proyecto
- [ ] **1.1** Convertir el `Cargo.toml` raíz en `[workspace]`
- [ ] **1.2** Mover el código actual a `crates/core/` como librería (sin `main.rs`)
- [ ] **1.3** Crear `crates/daemon/` con un `main.rs` mínimo que use `core`
- [ ] **1.4** Extraer la lógica de `gsettings` a `crates/backends/backend-gnome/`
- [ ] **1.5** Definir el `trait WallpaperBackend` en `core` e implementarlo para GNOME

## Fase 2 — Detección de pantallas
- [ ] **2.1** Definir `struct ScreenInfo` y `trait ScreenProvider`
- [ ] **2.2** Implementar `X11ScreenProvider` (xrandr o `x11rb`)
- [ ] **2.3** Implementar `WaylandScreenProvider` (D-Bus del compositor o `wlr-output-management`)
- [ ] **2.4** Comando temporal `wpd list-screens` (o un `main.rs` de prueba) para validar

## Fase 3 — Fondos aleatorios por pantalla
- [ ] **3.1** Cambiar el `Selector` para trabajar sobre `HashMap<ScreenId, PathBuf>`
- [ ] **3.2** Investigar soporte real de fondo-por-monitor en GNOME/Cinnamon (probablemente imagen compuesta)
- [ ] **3.3** Implementar `backend-kde` (scripting D-Bus de Plasma) — nativo por monitor
- [ ] **3.4** Implementar `backend-xfce` (`xfconf-query`) — nativo por monitor
- [ ] **3.5** Implementar `backend-cinnamon`

## Fase 4 — Video como fondo (todas las pantallas)
- [ ] **4.1** Decidir estrategia X11 (`xwinwrap` + mpv) vs. Wayland (`wlr-layer-shell`)
- [ ] **4.2** Levantar `mpv` como proceso hijo con `--input-ipc-server`
- [ ] **4.3** Definir `trait VideoBackend` y su primera implementación
- [ ] **4.4** Verificar decodificación por hardware (`--hwdec=auto`) y medir consumo

## Fase 5 — Video individual por pantalla
- [ ] **5.1** Gestionar N instancias de `mpv`, una por monitor, ancladas a su geometría
- [ ] **5.2** Supervisión de procesos: detectar crash y reiniciar
- [ ] **5.3** Pausar video al bloquear pantalla (D-Bus `org.freedesktop.login1` o `org.gnome.ScreenSaver`)

## Fase 6 — CLI / daemon / API
- [ ] **6.1** Crear `crates/ipc/` con el protocolo de mensajes (versión `v1`)
- [ ] **6.2** Elegir mecanismo: D-Bus (`zbus`) vs. socket Unix
- [ ] **6.3** Implementar el servicio en `daemon` (start/stop/status/next)
- [ ] **6.4** Crear `crates/cli/` (`wpd`) con subcomandos que hablan por IPC
- [ ] **6.5** Archivo `systemd --user` para arranque/gestión del daemon
- [ ] **6.6** Salida en JSON del CLI (`--json`) para que la GUI la consuma

## Fase 7 — GUI en Java
- [ ] **7.1** Elegir JavaFX y estructurar el proyecto (`gui/`)
- [ ] **7.2** Conectar la GUI al daemon vía `ProcessBuilder` + CLI JSON (primera versión)
- [ ] **7.3** Pantalla de selección de carpeta/video y vista de monitores detectados
- [ ] **7.4** Controles start/stop/next y estado del daemon
- [ ] **7.5** Empaquetado con `jlink`/`jpackage`
- [ ] **7.6** Migrar a D-Bus directo si se necesita refresco en tiempo real (opcional)

## Fase 8 — Empaquetado final
- [ ] **8.1** Reglas de `.deb`/`.rpm` en `packaging/`
- [ ] **8.2** Instalar daemon + CLI + GUI + servicio systemd + `.desktop` en un solo paquete
- [ ] **8.3** (Opcional) Pipeline CI/CD en GitHub Actions