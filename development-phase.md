## Task list — Fase 0 (lo único que aplica ahora mismo)

Todo lo demás del roadmap (multi-monitor, multi-DE, video, CLI/API, GUI) **todavía no tiene ni una línea de código** en el repo, así que no tiene sentido listarlo como "tareas concretas" todavía — sigue siendo diseño. Lo que sí existe y necesita trabajo inmediato es esto:

- [ ] **0.1** — En `main.rs` / `wallpapers.rs`: reemplazar los `expect()` (el README confirma que "cualquier error detiene el bucle") por un `WallpaperError` propio con `thiserror`
- [ ] **0.2** — En `wallpapers.rs`: arreglar la comparación de fondo actual — `scanner.rs` devuelve rutas relativas (`assets/1.jpeg`) pero `gsettings get` devuelve URI absoluta; hay que canonicalizar antes de comparar
- [ ] **0.3** — En `wallpapers.rs`: leer/escribir también `picture-uri` (tema claro), no solo `picture-uri-dark`
- [ ] **0.4** — En `wallpapers.rs`: decodificar la URI leída (`%20` etc.)
- [ ] **0.5** — En `wallpapers.rs`: comprobar el código de salida de `gsettings get`, que hoy no se valida
- [ ] **0.6** — En `scanner.rs`: hacerlo recursivo (opcional, con `walkdir`)
- [ ] **0.7** — Implementar `config.rs`, que hoy está vacío: struct `Config` + `serde`/`toml`, resolviendo ruta con `directories` (`~/.config/wallpaper-daemon-rs/config.toml`) para reemplazar la carpeta fija `assets` y el intervalo fijo `10` en `main.rs`
- [ ] **0.8** — Escribir los tests que hoy están vacíos: `scanner_test.rs` y `wallpaper_test.rs`
- [ ] **0.9** — Decidir qué hacer con `src/code_templates/` (¿se borra, se documenta como referencia, o se integra?) — hoy es código muerto que no compila

## Después de cerrar Fase 0

Recién ahí tiene sentido pasar a la reorganización en workspace (`crates/core`, `crates/backends/...`) que vimos antes, porque hoy el proyecto es un único binario plano y cualquier refactor a multi-crate sin antes limpiar `config.rs`/errores solo movería el desorden de lugar.
