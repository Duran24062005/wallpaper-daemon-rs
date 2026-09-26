# PRD 08 — Empaquetado y distribución

## Propósito

Distribuir el daemon, CLI, GUI opcional y archivos de integración como una instalación coherente para Linux.

## Alcance

- Paquetes `.deb` y `.rpm`.
- Binarios daemon y `wpd`.
- Unidad `systemd --user`.
- Archivo `.desktop` para la GUI.
- Configuración, logs y desinstalación.
- Pipeline CI/CD opcional.

## Instalación propuesta

- Binarios en una ruta estándar del sistema o del usuario según el formato.
- Unidad en `systemd/user` y activación explícita o documentada.
- Configuración del usuario fuera del paquete, bajo `~/.config/wallpaper-daemon-rs/`.
- Datos temporales y sockets bajo rutas de runtime del usuario, nunca en el repositorio.

## Requisitos y compatibilidad

Documentar Rust/runtime, `gsettings` para GNOME, `mpv` para vídeo, sesión gráfica y diferencias X11/Wayland. El paquete no debe asumir que todos los backends están disponibles.

## Actualización y eliminación

Las actualizaciones preservan configuración compatible. La desinstalación detiene el servicio, elimina archivos instalados y conserva o elimina configuración del usuario según una opción explícita; nunca borra fondos personales.

## Aceptación

- Instalación limpia deja `wpd`, daemon y servicio utilizables.
- Upgrade no pierde configuración.
- El servicio falla con diagnóstico claro si falta una dependencia.
- `.deb` y `.rpm` se validan en entornos limpios.
- CI ejecuta compilación, tests, lints y validaciones de artefactos.

## Mantenimiento

Revisar `packaging/`, unidad systemd, archivos `.desktop`, scripts de build y README de instalación. Mantener una matriz de distribución, arquitectura y backend soportado.
