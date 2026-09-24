# Contexto del proyecto — wallpaper-daemon-rs

> Documento de referencia para cualquier persona (o asistente de IA) que retome el proyecto. Describe **qué es, por qué existe, en qué estado está y hacia dónde va**.
>
> Última actualización: 2026-09-24 · Versión del código: `0.1.0` (commit `3e4b5d6`; el bucle con scheduler llegó en `3fe3ce6` y la unificación de la clave `picture-uri-dark` en `b8736bd`).

---

## 1. Qué es

Un cambiador de fondos de pantalla para **GNOME/Linux** escrito en **Rust**. Reconstruye desde cero un proyecto previo hecho en Python (que usaba `subprocess` + `gsettings`), pero **no es una traducción línea por línea**: se rehace para entender Rust y Linux al mismo tiempo.

## 2. Objetivos

**Funcional**
- Escanear una carpeta de imágenes, elegir una al azar sin repetir la actual y aplicarla como fondo (**hecho**, con la salvedad de la deuda técnica nº 1).
- Repetir el cambio de forma periódica (**hecho de forma mínima**: bucle con intervalo fijo de 10 s).
- Evolucionar a un **daemon** controlable (`start`, `stop`, `next`, `status`) con carpeta e intervalo configurables.

**De aprendizaje** (igual de importante que el funcional)
- **Rust:** ownership, borrowing, `Result`/`Option`, `?`, structs, enums, traits, módulos, iteradores, `Path`/`PathBuf`, errores propios, concurrencia, testing.
- **Linux/GNOME:** procesos hijo, `gsettings` → GSettings/dconf → GNOME, señales, configuración de usuario, D-Bus (IPC), sesión gráfica, Wayland y monitores.

Cadena conceptual que el proyecto busca hacer explícita:

```text
Rust → std::process::Command → gsettings → GSettings/dconf → GNOME → Escritorio
```

## 3. Principios de diseño

1. **Primero `std`, después crates.** Una dependencia se añade solo cuando aparece una necesidad real y se entiende qué problema resuelve. Hoy la única es `rand`.
2. **Construir en capas.** Primero `gsettings` vía `Command`; luego D-Bus; luego multi-monitor.
3. **Módulos pequeños con una responsabilidad** (scanner, selector, backend de wallpaper, config, scheduler).
4. **Aprender de los errores del compilador:** preferir tipos que expresen fallos (`Result`, `Option`) antes que ocultarlos.
5. **No abstraer prematuramente.** Las estructuras (`Wallpaper`, `WallpaperManager`, trait de backend) llegan cuando el código las pide.

## 4. Estado actual (v0.1.0, en desarrollo)

### Implementado

| Módulo | Estado | Qué hace |
|---|---|---|
| `main.rs` | ✅ | Bucle infinito: escanea `assets`, lee el fondo actual, elige otro al azar, lo imprime, lo aplica y espera 10 s. Se detiene con `Ctrl+C`. |
| `scanner.rs` | ✅ | `scan_images(dir) -> Result<Vec<PathBuf>, _>`. Filtra `jpg/jpeg/png/webp`, no recursivo. |
| `selector.rs` | ✅ | `select_random(images, current) -> Option<&PathBuf>`. Excluye la actual y elige con `rand`. |
| `wallpapers.rs` | ✅ | `get_current_wallpaper()` y `set_wallpaper(path)` con `gsettings`; ambos usan la clave `picture-uri-dark`. |
| `scheduler.rs` | 🟡 | `wait(seconds)`: `thread::sleep` bloqueante. Sin intervalo configurable ni cancelación. |
| `code_templates/` | 📎 | Fragmentos de referencia con `println!` de depuración comentados. No forman parte del árbol de módulos. |

### Pendiente / vacío

| Elemento | Estado |
|---|---|
| `config.rs` | Archivo vacío y sin declarar en `main.rs` |
| `tests/scanner_test.rs` | Vacío |
| `tests/wallpaper_test.rs` | Vacío |
| CLI (`start/stop/next/status`) | No existe |
| Manejo de señales / parada limpia | No existe |

### Dependencias

```toml
[dependencies]
rand = "0.10.3"      # edition = "2024" → requiere Rust ≥ 1.85
```

## 5. Problemas conocidos (deuda técnica)

Ordenados por impacto:

1. **Rutas relativas vs. absolutas.** `scan_images("assets")` devuelve `assets/x.jpeg` (relativa) y el fondo actual es una URI absoluta; al compararlas en `select_random` nunca coinciden, así que el fondo actual **no siempre se excluye** y puede repetirse (más visible ahora que el bucle cambia el fondo cada 10 s). *Solución:* canonicalizar en el scanner o comparar con `canonicalize()`.
2. **Errores con `expect` dentro de un bucle infinito.** Cualquier fallo (p. ej. `gsettings` falla, la carpeta queda sin alternativas) provoca un panic y **mata todo el programa** en lugar de saltar al siguiente ciclo. Falta un tipo de error propio (`WallpaperError`) y mensajes claros.
3. **Solo tema oscuro.** Desde `b8736bd` `get` y `set` usan la misma clave (`picture-uri-dark`), pero `picture-uri` (tema claro) no se toca. *Solución:* decidir la estrategia (escribir ambas claves o detectar `color-scheme`).
4. **URI sin decodificar.** El valor devuelto por `gsettings` puede llevar caracteres codificados (`%20`); se compara como texto plano.
5. **`gsettings get` no valida el estado de salida.** Solo se comprueba `set`; en `get` se ignora `output.status`.
6. **Directorio (`"assets"`) e intervalo (`10`) fijos** en `main.rs`; el directorio además depende del directorio de ejecución.
7. **Scheduler bloqueante y sin señales.** `thread::sleep` no se puede interrumpir, así que `stop`/`next` exigirán rediseñarlo (canales, `Condvar` o señales).
8. **Sin tests.** Los archivos de `tests/` están vacíos.
9. **`main.rs` sin formato uniforme** (espacios en blanco al final de línea y líneas en blanco con indentación); pasar `cargo fmt` y `cargo clippy`.

## 6. Roadmap por fases

Basado en el plan original (`docs/app-context.md`), con el estado real:

| Fase | Contenido | Estado |
|---|---|---|
| 1 — Rust básico | Cargo, módulos, tipos, funciones, ownership | ✅ |
| 2 — Sistema de archivos | `Path`/`PathBuf`, `fs`, errores, `Result`/`Option` | ✅ (scanner no recursivo) |
| 3 — Procesos | `Command`, stdout/stderr, exit codes, `gsettings` | ✅ (falta validar `get`) |
| 4 — Wallpaper backend | get/set, light/dark, abstracción del backend | 🟡 get/set hechos (solo `picture-uri-dark`); tema claro y trait pendientes |
| 5 — Arquitectura | scanner, selector, scheduler, backend, configuración | 🟡 scanner, selector y scheduler mínimo hechos; config vacío |
| 6 — Daemon | Ejecución continua, señales, `start/stop/status` | 🟡 ejecución continua con `loop` hecha; señales y comandos pendientes |
| 7 — IPC | D-Bus, sesión gráfica, comunicación con GNOME | ⬜ |
| 8 — Multi-monitor | Detectar salidas, estado por monitor | ⬜ |
| 9 — Rust avanzado | Traits, async/concurrencia, testing, errores propios | ⬜ |

## 7. Próximos pasos sugeridos

1. Corregir la deuda técnica 1, 3 y 4 de la sección 5 (rutas canónicas, tema claro, decodificación de URI).
2. Introducir un tipo de error propio y sustituir los `expect` por propagación con `?` hasta `main`; dentro del bucle, registrar el error y continuar en lugar de terminar.
3. Añadir el primer test unitario en `scanner` (con `std::env::temp_dir()` para no depender de `assets/`).
4. Implementar `config.rs` (directorio, intervalo) y declararlo en `main.rs`; pasar esos valores a `scan_images` y `scheduler::wait`.
5. Manejar señales (`SIGINT`/`SIGTERM`) para una salida limpia y evaluar canales/`Condvar` para poder interrumpir la espera.
6. Extraer un trait de backend (`GsettingsBackend`) cuando se prepare la migración a D-Bus.

## 8. Convenciones

- **Idioma:** documentación en español; código, identificadores y mensajes de commit en inglés.
- **Commits:** estilo gitmoji, p. ej. `feat: :sparkles: the images scanner has been added`.
- **Formato y calidad:** `cargo fmt` y `cargo clippy` antes de subir cambios.
- **Fragmentos de estudio:** el código de referencia o experimental va en `src/code_templates/` (fuera del árbol de módulos).
- **Rutas:** usar `Path`/`PathBuf`, nunca `String`, para representar rutas dentro del programa.

## 9. Glosario rápido

| Término | Significado |
|---|---|
| **GSettings / dconf** | Sistema de configuración de GNOME; `gsettings` es su CLI. |
| **`picture-uri`** | Clave con la URI del fondo para tema claro (no usada actualmente). |
| **`picture-uri-dark`** | Clave con la URI del fondo para tema oscuro (la que usa el programa). |
| **`Path` / `PathBuf`** | Ruta prestada (referencia) / ruta con memoria propia. |
| **D-Bus** | Sistema de IPC de Linux; objetivo de la fase 7. |
| **Daemon** | Proceso en segundo plano que se ejecuta de forma continua. |

## 10. Documentos relacionados

- [`README.md`](../README.md) — presentación, requisitos y uso.
- [`Architecture.md`](Architecture.md) — módulos, flujo de datos y arquitectura objetivo.
- [`app-context.md`](app-context.md) — conversación original de diseño y plan de aprendizaje. Se conserva como **referencia histórica**; su código de ejemplo es anterior a la implementación actual.

> **Nota sobre versiones anteriores de este archivo:** el `context.md` previo era una transcripción de una versión distinta (`wallpaper-rs`, con `clap`, `--dir` y `--interval`). Ese diseño **no corresponde** al código actual del repositorio; `clap` y `WallpaperError` siguen siendo ideas pendientes, no funcionalidades existentes. El scheduler con `thread::sleep` sí existe ya, pero en su versión mínima (intervalo fijo de 10 s).



---

<br>
<br>
<br>
<br>

---



Vamos a crear el proyecto real, con una V1 funcional que puedas compilar y probar en tu Ubuntu. Te explico los pasos y te dejo el código ya armado.Cargo instalado. Ahora creo el proyecto:Perfecto. Ahora escribo el `Cargo.toml` y los módulos de la V1 (scanner, wallpaper, main con scheduler simple):Ahora compilo y corro los tests para verificar que todo funciona de verdad:Compiló sin errores. Ahora corro los tests y una prueba funcional del comando (sin GNOME real, pero probando la lógica de escaneo y el `--help`):Todo funciona. Empaqueto el proyecto (sin `target/` ni `.git/`) para que lo descargues:## Cómo ponerlo en marcha en tu Ubuntu

**1. Descarga y descomprime**
```bash
tar xzf wallpaper-rs.tar.gz
cd wallpaper-rs
```

**2. Verifica que tienes cargo** (si no lo tienes: `sudo apt install cargo`, o mejor `rustup` desde rustup.rs).

**3. Compila y corre**
```bash
cargo build --release
./target/release/wallpaper-rs --dir ~/Pictures --interval 30
```
- `--interval 0` (el default) cambia el fondo una sola vez y termina — ideal para probar.
- `--interval 30` deja el proceso corriendo y cambia el fondo cada 30 minutos.

## Qué hay dentro (y por qué)

- **`src/scanner.rs`** — recorre el directorio con `std::fs::read_dir`, filtra por extensión (`jpg/jpeg/png/webp`) y devuelve `Vec<PathBuf>`. Trae un test unitario (`cargo test`) para que veas cómo se prueban módulos en Rust.
- **`src/wallpaper.rs`** — ejecuta `gsettings set org.gnome.desktop.background picture-uri(-dark) file://...` con `std::process::Command`, con su propio tipo de error (`WallpaperError`) que implementa `Display` y `std::error::Error`, tal como se hace idiomáticamente en Rust.
- **`src/main.rs`** — usa `clap` (derive) para el CLI, `rand` para elegir imagen evitando repetir la anterior, y un loop con `thread::sleep` como scheduler mínimo.

Nota: tuve que fijar `clap = "=4.4.18"` en el `Cargo.toml` porque las versiones más nuevas de clap_derive requieren edition2024, que pide un cargo más reciente del que hay en este sandbox. En tu máquina, si tienes un cargo actualizado (rustup), puedes aflojar esa restricción a `clap = { version = "4", features = ["derive"] }` sin problema.

**Ya compilé y corrí los tests aquí mismo** (`cargo build` y `cargo test` pasaron) para asegurarme de que no hay errores de sintaxis ni de tipos antes de dártelo.

## Siguientes pasos (siguiendo tu roadmap)
- **V2**: mover `dir`/`interval` a un `config.rs` con `serde` + `toml`, guardado en `~/.config/wallpaper-rs/config.toml`.
- **V3**: reemplazar `gsettings` por llamadas D-Bus directas (crate `zbus`) al servicio de GNOME.
- **history.rs**: hoy la "no repetición" es solo respecto a la última imagen; para historial persistente, guarda las últimas N rutas usadas en un archivo.

¿Quieres que sigamos con la V2 (configuración persistente) ahora, o prefieres primero probar esta V1 en tu máquina?