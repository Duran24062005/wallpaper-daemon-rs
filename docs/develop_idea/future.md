# Documento de Diseño: `wallpaper-daemon-rs`

**Repositorio:** https://github.com/Duran24062005/wallpaper-daemon-rs
**Lenguaje núcleo:** Rust
**Estado actual:** MVP funcional solo para GNOME (bucle en primer plano, sin daemon real)
**Última actualización:** Septiembre 2026

---

## 1. Resumen ejecutivo

`wallpaper-daemon-rs` es un gestor de fondos de pantalla para Linux escrito en Rust, pensado para evolucionar en tres capas:

1. **Núcleo (core/daemon):** lógica de escaneo, selección, aplicación de fondos (imagen y video) y detección de monitores/escritorio, expuesta como **CLI** y como **servicio de sistema** (API local vía socket/D-Bus).
2. **Compatibilidad multi-entorno:** GNOME, KDE Plasma, Cinnamon y XFCE, en Wayland y X11, mediante un sistema de *backends* intercambiables.
3. **Interfaz gráfica (GUI):** una aplicación en Java que se instala junto al daemon y actúa como cliente de la API/CLI, permitiendo configuración visual.

El principio rector de todo el diseño es: **bajo consumo de recursos, arquitectura modular por capas, y detección automática del entorno de escritorio** para aplicar la estrategia correcta sin intervención del usuario.

---

## 2. Estado actual del MVP (línea base)

Según el código y README actuales:

| Aspecto | Estado |
|---|---|
| Escaneo de imágenes | ✅ Implementado (`scanner.rs`), no recursivo |
| Selección aleatoria | ✅ Implementado (`selector.rs`), sin repetir el actual (poco fiable) |
| Aplicación de fondo | ✅ Solo GNOME vía `gsettings`, solo clave `picture-uri-dark` |
| Bucle de ejecución | ✅ `thread::sleep` cada 10s, en primer plano |
| Configuración persistente | ❌ No existe (`config.rs` vacío) |
| Manejo de errores | ❌ `expect()` / panic ante cualquier fallo |
| Tests | ❌ Vacíos |
| Multi-monitor | ❌ No soportado |
| Soporte multi-DE | ❌ Solo GNOME |
| Video wallpapers | ❌ No implementado |
| Modo daemon (start/stop/status) | ❌ No implementado |
| CLI/API | ❌ No implementado |
| GUI | ❌ No implementado |

Esta tabla es el punto de partida real; todas las fases siguientes construyen sobre (o reemplazan) estas piezas.

---

## 3. Principios de diseño

Estos principios deben guiar **cada decisión técnica** en las fases siguientes:

1. **Bajo consumo de recursos.** El daemon debe pasar la mayor parte del tiempo dormido/bloqueado (esperando eventos o temporizadores), nunca haciendo *polling* agresivo. Evitar escaneos de disco innecesarios y mantener el uso de RAM en el orden de unos pocos MB en reposo.
2. **Abstracción por entorno de escritorio.** Ningún módulo de lógica de negocio (selección, programación, video) debe saber "cómo" se aplica un fondo en GNOME vs. KDE. Eso vive detrás de un *trait* (interfaz) común.
3. **Fallo controlado, nunca pánico.** Todo error (falta un binario, falla D-Bus, carpeta vacía) debe transformarse en un `Result` con un tipo de error propio (`WallpaperError`) y una estrategia de recuperación o mensaje claro — nunca `expect()`/`unwrap()` en rutas de ejecución normales.
4. **Separación CLI/daemon/API.** El binario que corre en segundo plano (daemon) es distinto del binario que el usuario invoca desde terminal (CLI). Se comunican por IPC (socket Unix o D-Bus), nunca comparten proceso.
5. **La GUI es un cliente más.** La GUI en Java **no debe reimplementar lógica**; solo debe hablar con la API/CLI del daemon, igual que lo haría un script de usuario.
6. **Extensibilidad para video sin romper imágenes.** El sistema de "fondo estático" y "fondo en video" deben compartir la misma capa de programación/selección y solo diferir en el backend de renderizado.

---

## 4. Arquitectura objetivo (visión de alto nivel)

```
┌─────────────────────────────────────────────────────────────────┐
│                         GUI (Java)                               │
│         Cliente de configuración — instalado con el daemon       │
└───────────────────────────────┬───────────────────────────────────┘
                                 │ llama a
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                     CLI (wallpaperctl / wpd)                     │
│      wpd start | stop | status | next | set-folder | set-video   │
└───────────────────────────────┬───────────────────────────────────┘
                                 │ IPC (socket unix / D-Bus)
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    DAEMON (wallpaper-daemond)                    │
│  ┌───────────────┐  ┌───────────────┐  ┌────────────────────┐    │
│  │ Config Manager │  │ Scheduler      │  │ Monitor Detector   │    │
│  │ (TOML, ~/.config)│ │ (async/tokio) │  │ (xrandr/wlr-output)│    │
│  └───────────────┘  └───────────────┘  └────────────────────┘    │
│  ┌───────────────┐  ┌───────────────────────────────────────┐    │
│  │ Scanner/Selector│ │ Desktop Environment Backend (trait)   │    │
│  │ (img + video)   │ │  ├─ GNOME (gsettings/dconf)           │    │
│  └───────────────┘  │  ├─ KDE Plasma (dbus/plasma-apply-*)   │    │
│                      │  ├─ Cinnamon (gsettings variant)      │    │
│                      │  └─ XFCE (xfconf-query)               │    │
│                      └───────────────────────────────────────┘    │
│  ┌───────────────────────────────────────────────────────────┐    │
│  │ Video Renderer Backend (mpv/wlr-layer-shell/xwinwrap)       │    │
│  └───────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

**Idea clave:** el daemon central es agnóstico; delega en *backends* seleccionados dinámicamente según:
- El entorno de escritorio detectado (variable `XDG_CURRENT_DESKTOP`, procesos activos, sesión D-Bus).
- El servidor de gráficos (Wayland vs. X11, vía `WAYLAND_DISPLAY` / `DISPLAY`).
- El tipo de fondo pedido (imagen estática vs. video).

---

## 5. Roadmap por fases

### Fase 0 — Cerrar el MVP actual
**Objetivo:** dejar sólido lo que ya existe antes de sumar features nuevas.

- [ ] Reemplazar `expect()`/panics por un enum `WallpaperError` (`thiserror`) y propagación con `Result`.
- [ ] Resolver rutas relativas a absolutas/canónicas antes de comparar con la URI de `gsettings` (arregla la detección de "fondo actual").
- [ ] Soportar tanto `picture-uri` (tema claro) como `picture-uri-dark` (tema oscuro): detectar el tema activo o escribir ambas claves.
- [ ] Decodificar URIs (`%20`, etc.) con un crate como `percent-encoding` o `urlencoding`.
- [ ] Escaneo recursivo opcional (`walkdir`).
- [ ] Verificar código de salida de `gsettings get`/`set`, no asumir éxito.
- [ ] Tests unitarios reales para `scanner.rs`, `selector.rs`, `wallpapers.rs`.
- [ ] Config persistente básica: `~/.config/wallpaper-daemon-rs/config.toml` con carpeta e intervalo (usar `serde` + `toml` + `directories` para resolver rutas XDG).

**Entregable:** un binario que ya no depende de rutas fijas en el código ni truena ante errores comunes.

---

### Fase 1 — Detección de pantallas conectadas
**Objetivo:** que el daemon sepa cuántos monitores hay, sus nombres/identificadores y su geometría.

Consideraciones técnicas:
- **X11:** usar `xrandr --query` (parseo de texto) o el crate `x11rb` para hablar directamente con el protocolo X11/RandR.
- **Wayland:** no hay un estándar único; depende del compositor:
  - GNOME (Mutter) y KDE (KWin) exponen información vía D-Bus (`org.gnome.Mutter.DisplayConfig`, `org.kde.KWin`).
  - Compositores wlroots (usados por algunos setups de Cinnamon/XFCE en Wayland) soportan el protocolo `wlr-output-management`, accesible con crates como `wayland-client` + `wayland-protocols-wlr`.
- Diseñar un `trait ScreenProvider` con un método `list_screens() -> Result<Vec<ScreenInfo>, WallpaperError>`, con implementaciones `X11ScreenProvider` y `WaylandScreenProvider`, seleccionadas en runtime.
- `ScreenInfo` debe incluir: `id`, `nombre` (ej. `eDP-1`, `HDMI-1`), resolución, posición y si es primario.

**Entregable:** comando `wpd list-screens` que imprime los monitores detectados con su identificador.

---

### Fase 2 — Fondos aleatorios independientes por pantalla
**Objetivo:** cada monitor puede tener su propio fondo aleatorio, no el mismo para todos.

Consideraciones técnicas:
- El `Selector` pasa de operar sobre "un fondo global" a operar sobre `HashMap<ScreenId, PathBuf>`.
- La aplicación por pantalla depende del DE:
  - **GNOME/Cinnamon:** usan monitores "virtuales" combinados en una sola imagen compuesta, o `picture-uri` por `monitor` en configuraciones recientes — investigar soporte real vía `gsettings` con `org.gnome.desktop.background` (limitado) o recurrir a herramientas como `feh --bg-fill` por salida en X11.
  - **KDE Plasma:** sí soporta fondos por monitor de forma nativa vía scripting D-Bus de Plasma (`org.kde.plasmashell`, método `evaluateScript` con JS de Plasma).
  - **XFCE:** `xfconf-query` permite establecer una propiedad por monitor/workspace (`/backdrop/screen0/monitor<X>/workspace0/last-image`).
- Definir `trait WallpaperBackend` con `set_wallpaper(screen: &ScreenInfo, path: &Path) -> Result<()>` para que cada DE implemente su propia lógica sin tocar el core.

**Entregable:** `wpd set-random --per-screen` aplica una imagen distinta y aleatoria a cada monitor detectado.

---

### Fase 3 — Video como fondo, replicado en todas las pantallas
**Objetivo:** soportar `.mp4`/`.webm` como fondo animado, igual en todos los monitores.

Consideraciones técnicas:
- Reproducir video como fondo de escritorio requiere una ventana especial "detrás" de los iconos/ventanas normales:
  - **X11:** patrón clásico usado por `xwinwrap` + `mpv` (crear una ventana X11 de tipo `desktop`/root y renderizar mpv dentro).
  - **Wayland:** requiere el protocolo `wlr-layer-shell` (capa `background`), soportado por compositores wlroots. GNOME/KDE en Wayland no exponen esto directamente; ahí puede requerirse una extensión (GNOME Shell extension) o limitarse a X11/XWayland como primer soporte.
  - Proyectos de referencia a estudiar (sin copiar código, solo arquitectura): `mpvpaper` (wlroots) y `linux-wallpaperengine`.
- Integrar `mpv` como proceso hijo controlado por IPC (mpv soporta un socket JSON IPC, `--input-ipc-server`), en vez de reimplementar un reproductor de video en Rust.
- El `trait WallpaperBackend` se extiende (o se crea un `trait VideoBackend` hermano) con `set_video_wallpaper(screens: &[ScreenInfo], path: &Path)`.
- Vigilar consumo de GPU/CPU: decodificación por hardware (`--hwdec=auto` en mpv) es obligatoria para cumplir el principio de bajo consumo.

**Entregable:** `wpd set-video ruta.mp4 --all-screens` reproduce el mismo video de fondo en todos los monitores.

---

### Fase 4 — Videos individuales por pantalla
**Objetivo:** cada monitor reproduce un video distinto.

Consideraciones técnicas:
- Requiere una instancia de `mpv` (o del backend elegido) por pantalla, cada una anclada a la geometría de su monitor.
- Gestionar el ciclo de vida de N procesos hijos: arranque, verificación de salud (¿mpv sigue vivo?), reinicio ante crash, y apagado limpio al detener el daemon.
- Aquí el consumo de recursos es crítico: decodificar varios videos simultáneos es costoso. Se recomienda:
  - Límite configurable de resolución/FPS de reproducción para fondos.
  - Pausar la reproducción cuando la pantalla está bloqueada o en suspensión (detectar vía D-Bus `org.freedesktop.login1` o `org.gnome.ScreenSaver`).

**Entregable:** `wpd set-video --screen HDMI-1 ruta1.mp4 --screen eDP-1 ruta2.mp4`.

---

### Fase 5 — Consolidar como CLI / servicio de sistema (API)
**Objetivo:** exponer todo lo anterior de forma estable, para que la propia CLI y, después, la GUI en Java, lo consuman igual.

Consideraciones técnicas:
- **Modo daemon real:** usar `systemd` *user service* (`~/.config/systemd/user/wallpaper-daemon.service`) para arranque, reinicio automático y logs vía `journald`. Esto resuelve "start/stop/status" sin reinventar gestión de procesos.
- **IPC:**
  - Opción simple y liviana: **socket Unix** con un protocolo propio (ej. JSON por línea) — bajo overhead, fácil de depurar.
  - Opción más "nativa" en Linux de escritorio: **D-Bus** con un *service name* propio (ej. `com.duran24062005.WallpaperDaemon`), usando el crate `zbus`. D-Bus tiene la ventaja de integrarse mejor con otros componentes del sistema (incluida una futura GUI o extensiones de shell) y de ofrecer introspección estándar.
  - Recomendación: **D-Bus como API primaria** (más idiomático en el ecosistema Linux de escritorio y fácil de consumir desde Java vía librerías D-Bus), con el socket Unix como fallback simple para el CLI si se prefiere evitar la dependencia de D-Bus en sistemas mínimos.
- **CLI (`wpd`):** subcomandos claros y estables desde el día uno, pensando que la GUI los reflejará 1:1:
  - `wpd start` / `wpd stop` / `wpd restart` / `wpd status`
  - `wpd next` (fuerza cambio inmediato)
  - `wpd config set-folder <ruta>` / `wpd config set-interval <segs>`
  - `wpd list-screens`
  - `wpd set-random [--per-screen]`
  - `wpd set-video <ruta> [--screen <id>] [--all-screens]`
- **Versionado de API:** desde el inicio, versionar el protocolo D-Bus/socket (ej. `v1`) para poder evolucionar sin romper la GUI.

**Entregable:** el daemon corre como servicio de usuario, y toda interacción (incluida la de la futura GUI) pasa por la misma API documentada.

---

### Fase 6 — GUI en Java
**Objetivo:** una aplicación gráfica, instalada junto al daemon, que permite configurar todo visualmente.

Consideraciones técnicas:
- **Tecnología recomendada:** JavaFX (más moderno que Swing, mejor soporte de theming y layouts responsivos). Empaquetado con `jlink`/`jpackage` para generar un instalable nativo (`.deb`/`.rpm`/AppImage) sin requerir que el usuario tenga JDK instalado.
- **Comunicación con el daemon:**
  - Si la API es D-Bus: usar una librería Java de D-Bus (ej. `dbus-java`) para invocar métodos directamente — es la opción más limpia.
  - Alternativa más simple: la GUI invoca el binario `wpd` como subproceso (`ProcessBuilder`) y parsea su salida (JSON si se implementa `--json` en el CLI). Menos elegante, pero desacopla completamente la GUI de detalles de IPC y es más fácil de mantener al inicio.
  - Recomendación pragmática: **empezar con `ProcessBuilder` + salida JSON del CLI** en una primera versión de la GUI (rápido de construir), y migrar a D-Bus directo si se necesita eventos en tiempo real (ej. refrescar la UI cuando el fondo cambia automáticamente).
- **Funcionalidades mínimas de la GUI:**
  - Selección de carpeta de imágenes y de videos.
  - Vista de monitores detectados (mapa simple tipo "configuración de pantallas") para asignar fondo por pantalla.
  - Control de intervalo de cambio.
  - Botones start/stop/next y estado del daemon.
  - Indicador de entorno de escritorio detectado (informativo, para que el usuario entienda qué backend se está usando).
- **Instalación conjunta:** el paquete del sistema (`.deb`, `.rpm`, o Flatpak) debe instalar tanto el binario del daemon/CLI en Rust como el `.jar`/paquete nativo de la GUI, y registrar el servicio `systemd --user` y un acceso directo de escritorio (`.desktop` file) para la GUI.

**Entregable:** el usuario instala un solo paquete y obtiene el daemon corriendo en background más un ícono de aplicación para configurarlo.

---

## 6. Soporte multi-entorno de escritorio (transversal a todas las fases)

Esto no es una fase aislada: debe diseñarse desde la Fase 0 como una interfaz, aunque solo se implemente GNOME al inicio.

```rust
trait WallpaperBackend {
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;                     // detecta si aplica al sistema actual
    fn set_image(&self, screen: &ScreenInfo, path: &Path) -> Result<(), WallpaperError>;
    fn set_video(&self, screen: &ScreenInfo, path: &Path) -> Result<(), WallpaperError>;
}
```

**Estrategia de detección del entorno:**
1. Leer `XDG_CURRENT_DESKTOP` (valores típicos: `GNOME`, `KDE`, `X-Cinnamon`, `XFCE`).
2. Si es ambiguo, verificar procesos activos relevantes (`gnome-shell`, `plasmashell`, `cinnamon`, `xfce4-session`) como respaldo.
3. Verificar `WAYLAND_DISPLAY` vs `DISPLAY` para saber el protocolo gráfico.
4. Instanciar el `WallpaperBackend` correspondiente; si ninguno aplica, fallback a un backend genérico X11 (ej. `feh`/`xwallpaper`) que funciona en casi cualquier gestor de ventanas ligero.

| Entorno | Mecanismo de fondo estático | Mecanismo de fondo por monitor | Notas |
|---|---|---|---|
| GNOME | `gsettings`/`dconf` (`org.gnome.desktop.background`) | Limitado; investigar soporte nativo por monitor | Ya implementado parcialmente |
| KDE Plasma | Scripting D-Bus de Plasma (`plasmashell`, `evaluateScript`) | Nativo, vía el mismo scripting | Buen soporte multi-monitor |
| Cinnamon | Similar a GNOME (`gsettings` con esquema propio de Cinnamon) | Limitado, similar a GNOME | Comparte base con GNOME (fork de GNOME Shell) |
| XFCE | `xfconf-query` (canal `xfce4-desktop`) | Nativo, por monitor/workspace | Buen soporte multi-monitor histórico |

---

## 7. Rendimiento: lineamientos concretos

- El daemon en reposo (sin cambios pendientes) debe estar bloqueado en un `sleep`/`await` de un temporizador o esperando en el socket IPC — **0% de CPU en reposo**, no *busy-waiting*.
- Evitar reescanear la carpeta de imágenes en cada ciclo si no ha cambiado: usar *watch* de sistema de archivos (`notify` crate) en vez de reescaneo por *polling*.
- Para fondos de video: exigir decodificación acelerada por hardware y considerar pausar la reproducción cuando la pantalla está bloqueada/apagada.
- Preferir `async`/`tokio` (o `std::thread` bien acotado) según la complejidad real que se necesite — no adoptar un runtime async completo solo por moda si el daemon sigue siendo mayormente secuencial; evaluarlo cuando se introduzca IPC + múltiples procesos de video concurrentes (Fase 4 en adelante), donde sí aporta valor real.
- Medir memoria y CPU con `valgrind`/`heaptrack` o simplemente `ps`/`top` en cada fase, y fijar un presupuesto (ej. "el daemon sin video activo no debe superar X MB de RSS").

---

## 8. Riesgos y puntos abiertos

- **Wayland + video de fondo** es el punto más incierto: no todos los compositores (especialmente GNOME/Mutter) exponen una forma estándar de poner una ventana "detrás" del escritorio. Puede requerir una extensión de GNOME Shell como componente auxiliar opcional.
- **Fondo por monitor en GNOME/Cinnamon** tiene soporte nativo limitado comparado con KDE/XFCE; puede requerir una imagen compuesta generada en memoria (uniendo N imágenes en un solo lienzo del tamaño total del escritorio) como solución alternativa.
- **Empaquetado multiplataforma** (Rust + Java + systemd) implica coordinar tres ecosistemas de build; conviene definir temprano un pipeline de CI/CD (GitHub Actions) que compile ambos componentes y arme el paquete final.
- **Alcance del "video individual por pantalla"** puede ser costoso en hardware modesto; documentar requisitos mínimos recomendados.

---

## 9. Próximos pasos inmediatos sugeridos

1. Cerrar Fase 0 (robustez del MVP) — es la base para todo lo demás.
2. Diseñar y mergear el `trait WallpaperBackend` / `ScreenProvider` aunque solo tengan una implementación (GNOME) por ahora, para no tener que refactorizar todo después.
3. Elegir explícitamente el mecanismo IPC (D-Bus vs. socket Unix) antes de escribir la CLI definitiva, ya que condiciona el diseño de la GUI en Java.
4. Actualizar `docs/Architecture.md` y `docs/context.md` del repo para reflejar este documento como fuente de verdad del diseño.

---

## 10. Glosario rápido

- **Backend:** implementación concreta de una interfaz genérica para un entorno de escritorio específico.
- **DE (Desktop Environment):** entorno de escritorio (GNOME, KDE Plasma, Cinnamon, XFCE).
- **IPC:** mecanismo de comunicación entre procesos (aquí, entre CLI/GUI y el daemon).
- **Layer shell:** protocolo de Wayland para anclar ventanas a capas específicas (fondo, panel, overlay).
- **Daemon:** proceso que corre en segundo plano de forma persistente.


- [Claude Chat](https://claude.ai/chat/7edfdb63-8432-4eb8-bda8-b1944c7dee60)