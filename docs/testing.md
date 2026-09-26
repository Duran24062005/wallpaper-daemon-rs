# Pruebas

El proyecto separa las pruebas unitarias de las pruebas de integración.

## Ejecutar las pruebas

Desde la raíz del proyecto:

```bash
cargo test
```

También se puede comprobar el formato y los lints con:

```bash
cargo fmt --check
cargo clippy
```

## Cobertura actual

- `tests/scanner_test.rs` verifica que solo se incluyan extensiones de imagen soportadas y que un directorio inexistente produzca un error.
- `tests/wallpaper_test.rs` verifica que una URI `file://` siga siendo válida aunque el archivo configurado haya sido eliminado. Este caso evita confundir un error de ruta con un fallo de `gsettings`.
- `tests/wallpaper_test.rs` verifica que se rechacen valores que no sean URI `file://`.

Estas pruebas no modifican el wallpaper del escritorio ni necesitan una sesión GNOME: prueban la lógica de parsing y escaneo de forma aislada. La ejecución real de `gsettings` requiere una sesión gráfica y debe validarse manualmente con `cargo run`.
