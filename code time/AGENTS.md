# Code Time

IDE nativo en Rust + GPUI. Desarrollo por fases; ver el plan maestro del proyecto para el roadmap.

## Comandos

- Compilar: `cargo build`
- Verificar: `cargo check`
- Formatear: `cargo fmt`
- Ejecutar: `cargo run` (o `./target/debug/code-time`)

## Notas de build (Linux)

- GPUI 0.2.2 necesita `libxkbcommon-x11` en el enlazado. Si falta el paquete de desarrollo
  (`libxkbcommon-x11-dev`), el proyecto incluye `vendor/lib/libxkbcommon-x11.so` (enlace al
  runtime instalado) y `.cargo/config.toml` añade `-L native=vendor/lib` a rustflags.
  La solución limpia es instalar `libxkbcommon-x11-dev` con el gestor de paquetes.
- `libc` está fijado a `0.2.189` en `Cargo.lock` porque `xattr 0.2.3` (dependencia transitiva)
  es incompatible con `libc >= 0.2.190`.

## Estado

- Fase 1 completada: ventana GPUI, layout (barra superior, explorador, editor con pestañas,
  panel inferior, barra de estado), explorador de archivos funcional y apertura de archivos.
- Los paneles Terminal/Build/Errores están preparados pero su contenido llega en fases
  posteriores; no simulan funcionalidad.
