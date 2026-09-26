# PRD 07 — GUI opcional en Java

## Propósito

Ofrecer una interfaz gráfica para configurar y controlar el daemon sin duplicar la lógica de selección, backends o procesos.

## Alcance

- Aplicación JavaFX.
- Comunicación con `wpd`/IPC local.
- Selección de carpeta, intervalo, imágenes o vídeos.
- Vista de monitores y controles `start`, `stop`, `next`.
- Estado del daemon y backend detectado.

Fuera de alcance: implementar lógica de wallpaper en Java o conexión remota.

## Decisión de integración

La primera versión usará `ProcessBuilder` para invocar `wpd` con salida estructurada, preferiblemente JSON. D-Bus directo puede añadirse después si se requieren eventos en tiempo real.

## Contratos

La GUI dependerá únicamente de comandos y formatos documentados del PRD 06. No leerá archivos internos del daemon ni modificará directamente GSettings, procesos `mpv` o la configuración fuera del CLI.

## Aceptación

- La GUI detecta daemon ausente y muestra una acción útil.
- Las operaciones reflejan errores del daemon sin ocultarlos.
- La pantalla de monitores usa identificadores del sistema, no posiciones inventadas.
- El paquete puede ejecutarse sin que el usuario tenga un JDK de desarrollo.

## Mantenimiento

Revisar `gui/`, contrato del CLI, empaquetado JavaFX y compatibilidad de versiones. Cada cambio del protocolo debe mantener compatibilidad o incrementar la versión.
