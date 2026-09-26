# PRD 06 — Daemon, CLI e IPC local

## Propósito

Convertir el bucle actual en un servicio de usuario controlable desde terminal y desde futuros clientes locales.

## Alcance

- Proceso daemon persistente.
- Binario `wpd` con comandos de control y configuración.
- Canal de comunicación local versionado mediante D-Bus o socket Unix.
- Unidad `systemd --user`.
- Logs, estado y recuperación.

Fuera de alcance: HTTP, API REST, exposición remota y autenticación web.

## Estado actual

`main.rs` ejecuta un bucle infinito, espera con `thread::sleep` y no tiene señales, estado ni control externo. No existe CLI ni IPC.

## Comandos previstos

```text
wpd start
wpd stop
wpd restart
wpd status
wpd next
wpd config set-folder <path>
wpd config set-interval <seconds>
wpd list-screens
wpd set-random [--per-screen]
wpd set-video <path> [--screen <id>] [--all-screens]
```

## IPC y servicio

Elegir una sola opción primaria durante la implementación:

- D-Bus con `zbus`, preferible por integración con el escritorio, introspección y futuros clientes Java.
- Socket Unix con mensajes JSON por línea, preferible por simplicidad y depuración.

El protocolo se versionará como `v1`, tendrá operaciones de consulta y órdenes idempotentes cuando sea posible, y nunca escuchará en una interfaz de red.

La unidad `systemd --user` debe definir `ExecStart`, reinicio controlado, dependencia de la sesión gráfica y logs hacia `journald`.

## Estado y errores

`status` debe indicar ejecución, backend activo, última operación, último error, intervalo y archivos en uso. `stop` debe interrumpir la espera y terminar procesos hijos limpiamente. `next` debe despertar el scheduler sin esperar el intervalo.

## Aceptación

- El daemon arranca y se detiene mediante `systemctl --user`.
- El CLI puede consultar estado y enviar órdenes sin compartir memoria con el daemon.
- El daemon sobrevive a errores recuperables y deja trazas útiles.
- Un cliente incompatible con `v1` recibe un error de versión claro.
- No hay busy-waiting en reposo.

## Mantenimiento

Revisar scheduler, ciclo de vida, crates `daemon`, `cli` e `ipc`, unidad systemd, documentación de instalación y permisos del canal local.
