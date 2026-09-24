# wallpaper-daemon-rs

Cambiador de fondos de pantalla para **GNOME en Linux**, escrito en **Rust**.

Escanea una carpeta de imágenes, elige una al azar (intentando no repetir la actual) y la aplica como fondo de escritorio usando `gsettings`. El proceso se repite en bucle cada **10 segundos**.

> **Estado: MVP en desarrollo (Cargo `0.1.0`).** Ya se ejecuta de forma continua: cambia el fondo, espera y vuelve a empezar hasta que lo detengas con `Ctrl+C`. Todavía **no es un daemon completo**: el intervalo y la carpeta están fijos en el código y no existen los comandos `start/stop/next/status` ni la configuración persistente (ver [Roadmap](#roadmap)).

Este proyecto tiene además un objetivo didáctico: aprender **Rust** (ownership, `Result`/`Option`, módulos, `PathBuf`, procesos) y **cómo un programa se comunica con Linux/GNOME** (procesos hijo, `gsettings`, GSettings/dconf y, más adelante, D-Bus). Por eso se usa la biblioteca estándar siempre que es posible y solo se añade una dependencia externa (`rand`).

---

## Características

- Escaneo de una carpeta de imágenes (`jpg`, `jpeg`, `png`, `webp`; extensión sin distinguir mayúsculas).
- Selección aleatoria de un fondo distinto al actual.
- Lectura del fondo actual y aplicación del nuevo mediante `gsettings` (clave `picture-uri-dark`).
- Ejecución continua: un cambio de fondo cada 10 segundos (`thread::sleep`).
- La carpeta se vuelve a escanear en cada ciclo, así que las imágenes que añadas a `assets/` se detectan sin reiniciar el programa.
- Sin dependencias pesadas: solo `std` + `rand`.

## Requisitos

| Requisito | Detalle |
|---|---|
| Sistema operativo | Linux con escritorio **GNOME** (probado con Ubuntu) |
| `gsettings` | Debe estar en el `PATH` (viene con GNOME) |
| Rust | **1.85 o superior** (el proyecto usa `edition = "2024"`) |
| Sesión gráfica | Necesaria para que `gsettings` pueda modificar el fondo |
| Tema | **Oscuro** (ver [Limitaciones conocidas](#limitaciones-conocidas)) |

Si tu `cargo` es antiguo, instala una versión reciente con [rustup](https://rustup.rs).

## Instalación y uso

```bash
git clone https://github.com/Duran24062005/wallpaper-daemon-rs.git
cd wallpaper-daemon-rs

cargo run --release
```

El programa se queda en ejecución y cambia el fondo cada 10 segundos. Para detenerlo, pulsa `Ctrl+C`.

Ejemplo de salida (un bloque por ciclo):

```text
Changing wallpaper...
Current wallpaper: file:///home/usuario/wallpaper-daemon-rs/assets/3.jpeg
Selected wallpaper: assets/5.jpeg
Changing wallpaper...
Current wallpaper: file:///home/usuario/wallpaper-daemon-rs/assets/5.jpeg
Selected wallpaper: assets/2.jpeg
...
```

> **Importante:** por ahora la carpeta de imágenes está fija en `assets` y es una ruta **relativa**, así que el programa debe ejecutarse desde la raíz del repositorio. Para usar tus propias imágenes, colócalas en `assets/` o cambia la ruta en `src/main.rs`. El intervalo se cambia en la misma función `main` (`scheduler::wait(10)`, en segundos).

## Cómo funciona

```text
loop {
    scanner::scan_images("assets")        →  Vec<PathBuf>
    wallpapers::get_current_wallpaper()   →  String (URI actual)
    selector::select_random(...)          →  &PathBuf (distinto al actual)
    wallpapers::set_wallpaper(...)        →  gsettings set ... picture-uri-dark
    scheduler::wait(10)                   →  thread::sleep de 10 s
}
```

Internamente ejecuta:

```bash
gsettings get org.gnome.desktop.background picture-uri-dark
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
│   ├── main.rs              # Punto de entrada: bucle principal y orquestación
│   ├── scanner.rs           # Descubrimiento de imágenes
│   ├── selector.rs          # Selección aleatoria
│   ├── wallpapers.rs        # Interacción con GNOME vía gsettings
│   ├── scheduler.rs         # Espera entre ciclos (thread::sleep)
│   ├── config.rs            # (vacío, sin declarar) configuración persistente
│   └── code_templates/      # Fragmentos de referencia, no se compilan
└── tests/
    ├── scanner_test.rs      # (vacío)
    └── wallpaper_test.rs    # (vacío)
```

## Limitaciones conocidas

- **No es aún un daemon completo:** el bucle corre en primer plano y solo se detiene con `Ctrl+C`; no hay `start/stop/next/status`, ni manejo de señales, ni ejecución en segundo plano.
- **Intervalo y carpeta fijos** en el código (10 s y `assets`); la carpeta es relativa al directorio de ejecución.
- **Exclusión del fondo actual poco fiable:** el scanner devuelve rutas relativas (`assets/1.jpeg`) y `gsettings` devuelve una URI absoluta, por lo que la comparación puede no detectar que la imagen elegida ya es la actual y a veces se repite el mismo fondo.
- **Solo se maneja `picture-uri-dark`** (tanto al leer como al escribir). Con el tema claro de GNOME la clave activa es `picture-uri`, por lo que el cambio no se vería.
- La URI leída no se decodifica (rutas con espacios u otros caracteres codificados como `%20`).
- El escaneo **no es recursivo**.
- **Cualquier error detiene el bucle:** los errores terminan el programa con `expect` (panic) en lugar de mostrar un mensaje amigable o reintentar. Por ejemplo, un fallo puntual de `gsettings`, o una carpeta sin imágenes alternativas.
- `gsettings get` no comprueba el código de salida.
- Solo GNOME; sin soporte multi-monitor por salida.
- El módulo `config` y los archivos de `tests/` están vacíos.

## Roadmap

- [x] Escáner de imágenes
- [x] Selector aleatorio sin repetir el fondo actual
- [x] Lectura/escritura del fondo con `gsettings`
- [x] Lectura y escritura sobre la misma clave (`picture-uri-dark`)
- [x] Bucle de ejecución continua con intervalo fijo (`thread::sleep`)
- [ ] Corregir las limitaciones restantes (rutas canónicas, soporte de tema claro, decodificación de URI, validar `gsettings get`)
- [ ] Manejo de errores propio (`WallpaperError`) en lugar de `expect`
- [ ] Tests unitarios e integración
- [ ] Configuración persistente (`~/.config/...`, `serde` + `toml`): carpeta e intervalo configurables
- [ ] Modo daemon: `start`, `stop`, `next`, `status`, manejo de señales
- [ ] Backend D-Bus en lugar de `gsettings`
- [ ] Multi-monitor y soporte Wayland/GNOME por salida

## Desarrollo

```bash
cargo build      # compilar
cargo run        # ejecutar (bucle infinito; Ctrl+C para salir)
cargo test       # ejecutar tests (aún sin casos)
cargo fmt        # formatear
cargo clippy     # lints
```

Los commits siguen un estilo tipo *gitmoji* (`feat: :sparkles: ...`).

## Autor

- [Duran24062005](https://github.com/Duran24062005)
- [ChatGPT Chat](https://chatgpt.com/c/6ab4463f-4d5c-83e9-a69f-f83a61e7322e)