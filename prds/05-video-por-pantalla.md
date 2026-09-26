# PRD 05 — Vídeos independientes por pantalla

## Propósito

Extender el vídeo global para mantener una reproducción y una asignación por monitor.

## Alcance

- Una instancia controlada de `mpv` por salida.
- Mapeo pantalla–vídeo y geometría.
- Supervisión, reinicio y apagado limpio.
- Pausa al bloquear o suspender la sesión.
- Límites configurables de resolución, FPS e instancias.

## Contexto

Es una fase posterior al vídeo global: multiplica procesos y consumo, y depende de que el proveedor de pantallas y el backend gráfico soporten superficies independientes.

## Contrato conceptual

```rust
pub struct ScreenVideo {
    pub screen_id: String,
    pub path: PathBuf,
}

pub trait MultiVideoManager {
    fn apply(&mut self, videos: &[ScreenVideo]) -> Result<(), WallpaperError>;
    fn health(&self) -> Vec<ProcessHealth>;
}
```

## Decisiones

- Cada proceso debe tener un identificador, socket IPC y estado aislados.
- Un crash se reinicia con backoff limitado; los fallos repetidos pasan a estado degradado.
- La pausa se detecta mediante `org.freedesktop.login1` o la interfaz disponible del escritorio.

## Aceptación

- Un monitor puede detenerse sin matar los demás.
- El daemon apaga todos los procesos al salir.
- Se informa qué pantalla falló y por qué.
- Las pruebas usan procesos fake, no requieren reproducir vídeo real.

## Mantenimiento

Revisar supervisor de procesos, proveedores de pantalla, configuración de recursos y documentación de integración con `login1`/ScreenSaver.
