# wallpaper-daemon-rs

Cambiador de fondos de pantalla para **GNOME en Linux**, escrito en **Rust**.

Escanea una carpeta de imágenes, elige una al azar (evitando repetir la actual) y la aplica como fondo de escritorio usando `gsettings`.

> **Estado: MVP v0.1.0.** Hoy es un ejecutable de una sola pasada: cambia el fondo una vez y termina. El modo daemon (ejecución continua, intervalos, comandos `start/stop/next/status`) está en el roadmap.

Este proyecto tiene además un objetivo didáctico: aprender **Rust** (ownership, `Result`/`Option`, módulos, `PathBuf`, procesos) y **cómo un programa se comunica con Linux/GNOME** (procesos hijo, `gsettings`, GSettings/dconf y, más adelante, D-Bus). Por eso se usa la biblioteca estándar siempre que es posible y solo se añade una dependencia externa (`rand`).

---

## Características

- Escaneo de una carpeta de imágenes (`jpg`, `jpeg`, `png`, `webp`; extensión sin distinguir mayúsculas).
- Selección aleatoria de un fondo distinto al actual.
- Lectura del fondo actual y aplicación del nuevo mediante `gsettings`.
- Sin dependencias pesadas: solo `std` + `rand`.

## Requisitos

| Requisito | Detalle |
|---|---|
| Sistema operativo | Linux con escritorio **GNOME** (probado con Ubuntu) |
| `gsettings` | Debe estar en el `PATH` (viene con GNOME) |
| Rust | **1.85 o superior** (el proyecto usa `edition = "2024"`) |
| Sesión gráfica | Necesaria para que `gsettings` pueda modificar el fondo |

Si tu `cargo` es antiguo, instala una versión reciente con [rustup](https://rustup.rs).

## Instalación y uso

```bash
git clone https://github.com/Duran24062005/wallpaper-daemon-rs.git
cd wallpaper-daemon-rs

cargo run --release
```

Ejemplo de salida:

```text
Current wallpaper: file:///home/usuario/wallpaper-daemon-rs/assets/3.jpeg
Selected wallpaper: assets/5.jpeg
```

> **Importante:** por ahora la carpeta de imágenes está fija en `assets` y es una ruta **relativa**, así que el programa debe ejecutarse desde la raíz del repositorio. Para usar tus propias imágenes, colócalas en `assets/` o cambia la ruta en `src/main.rs`.

## Cómo funciona

```text
scanner::scan_images("assets")        →  Vec<PathBuf>
wallpapers::get_current_wallpaper()   →  String (URI actual)
selector::select_random(...)          →  &PathBuf (distinto al actual)
wallpapers::set_wallpaper(...)        →  gsettings set ... picture-uri-dark
```

Internamente ejecuta:

```bash
gsettings get org.gnome.desktop.background picture-uri
gsettings set org.gnome.desktop.background picture-uri-dark file:///ruta/absoluta/imagen.jpg
```

Para el detalle de módulos, flujo de datos y decisiones de diseño consulta [`docs/Architecture.md`](docs/Architecture.md). Para el contexto y roadmap del proyecto, [`docs/context.md`](docs/context.md).

## Estructura del proyecto

```text
wallpaper-daemon-rs/
├── Cargo.toml
├── assets/                  # Imágenes de prueba
├── docs/
│   ├── Architecture.md      # Arquitectura actual y objetivo
│   ├── context.md           # Contexto, decisiones y roadmap
│   └── app-context.md       # Conversación original de diseño (histórico)
├── src/
│   ├── main.rs              # Punto de entrada y orquestación
│   ├── scanner.rs           # Descubrimiento de imágenes
│   ├── selector.rs          # Selección aleatoria
│   ├── wallpapers.rs        # Interacción con GNOME vía gsettings
│   ├── config.rs            # (vacío) configuración persistente
│   ├── scheduler.rs         # (vacío) ejecución periódica
│   └── code_templates/      # Fragmentos de referencia, no se compilan
└── tests/
    ├── scanner_test.rs      # (vacío)
    └── wallpaper_test.rs    # (vacío)
```

## Limitaciones conocidas

- **No es aún un daemon:** hace un solo cambio y termina.
- **Carpeta fija** (`assets`) y relativa al directorio de ejecución.
- **Exclusión del fondo actual poco fiable:** el scanner devuelve rutas relativas (`assets/1.jpeg`) y `gsettings` devuelve una URI absoluta, por lo que la comparación puede no detectar que la imagen elegida ya es la actual.
- **Claves distintas al leer y escribir:** se lee `picture-uri` pero se escribe `picture-uri-dark`. En GNOME con tema oscuro la clave activa es la segunda.
- La URI leída no se decodifica (rutas con espacios u otros caracteres codificados como `%20`).
- El escaneo **no es recursivo**.
- Los errores terminan el programa con `expect` (panic) en lugar de un mensaje amigable.
- Solo GNOME; sin soporte multi-monitor por salida.
- Los módulos `config` y `scheduler` y los archivos de `tests/` están vacíos.

## Roadmap

- [x] Escáner de imágenes
- [x] Selector aleatorio sin repetir el fondo actual
- [x] Lectura/escritura del fondo con `gsettings`
- [ ] Corregir las limitaciones anteriores (rutas canónicas, clave light/dark, decodificación de URI)
- [ ] Tests unitarios e integración
- [ ] Configuración persistente (`~/.config/...`, `serde` + `toml`)
- [ ] Scheduler con intervalo configurable
- [ ] Modo daemon: `start`, `stop`, `next`, `status`, manejo de señales
- [ ] Backend D-Bus en lugar de `gsettings`
- [ ] Multi-monitor y soporte Wayland/GNOME por salida

## Desarrollo

```bash
cargo build      # compilar
cargo run        # ejecutar
cargo test       # ejecutar tests (aún sin casos)
cargo fmt        # formatear
cargo clippy     # lints
```

Los commits siguen un estilo tipo *gitmoji* (`feat: :sparkles: ...`).

## Autor

[Duran24062005](https://github.com/Duran24062005)