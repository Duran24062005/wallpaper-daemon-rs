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