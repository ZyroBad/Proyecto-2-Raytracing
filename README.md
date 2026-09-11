# Valle del Fin Raytracing

Diorama voxel inspirado en el Valle del Fin de Naruto, renderizado con raytracing en Rust puro, sin librerias externas.

La escena representa dos estatuas enfrentadas sobre un valle rocoso, con rio central, cascada, puente, arboles y cielo de atardecer. Todo el diorama esta construido con cubos texturizados proceduralmente.

## Requerimientos cubiertos

- Diorama con cubos texturizados.
- Raytracing implementado desde cero en Rust.
- Camara orbital con acercamiento y alejamiento durante la animacion.
- Skybox procedural con degradado, sol y nubes.
- Reflexion en agua, cascada y roca oscura.
- Refraccion en agua y cascada.
- Mas de cinco materiales con textura propia y parametros de albedo, specular, transparencia y reflectividad.

## Materiales

| Material | Textura | Albedo | Specular | Transparencia | Reflectividad |
| --- | --- | --- | --- | --- | --- |
| Piedra de estatua | Ruido y vetas | Gris claro | Media | 0.00 | 0.05 |
| Roca oscura | Grietas procedurales | Gris oscuro | Media | 0.00 | 0.12 |
| Pasto | Cuadricula verde | Verde | Baja | 0.00 | 0.00 |
| Agua | Ondas procedurales | Azul | Alta | 0.55 | 0.35 |
| Madera | Vetado procedural | Cafe | Baja | 0.00 | 0.03 |
| Hojas | Ruido vegetal | Verde oscuro | Baja | 0.00 | 0.00 |
| Cascada | Franjas de espuma | Celeste | Alta | 0.35 | 0.22 |
| Espuma | Burbujas procedurales | Blanco celeste | Media | 0.18 | 0.10 |
| Musgo | Manchas organicas | Verde musgo | Baja | 0.00 | 0.00 |
| Sendero | Grava procedural | Cafe claro | Baja | 0.00 | 0.01 |

## Ejecutar

Render rapido de prueba:

```bash
cargo run --release -- --width 160 --height 90 --samples 1 --depth 2 --output renders/test.ppm
```

Render recomendado para imagen final:

```bash
cargo run --release -- --width 640 --height 360 --samples 2 --depth 3 --output renders/valle_del_fin.ppm
```

Render BMP para abrirlo facilmente en Windows:

```bash
cargo run --release -- --width 640 --height 360 --samples 2 --depth 3 --output renders/valle_del_fin.bmp
```

Mover la camara manualmente:

```bash
cargo run --release -- --angle 90 --zoom 1.35 --width 480 --height 270 --samples 2 --depth 3 --output renders/camara_manual.bmp
```

Modo interactivo por consola:

```bash
cargo run --release -- --interactive --width 240 --height 135
```

Controles del modo interactivo:

- `a` / `d`: rotar camara.
- `w` / `s`: ajuste fino del angulo.
- `+` / `-`: acercar y alejar camara.
- `r`: renderizar sin cambiar la camara.
- `q`: salir.

Generar frames para video:

```bash
cargo run --release -- --width 480 --height 270 --samples 1 --depth 3 --frames 120 --animate
```

Ver resumen de escena:

```bash
cargo run --release -- --summary
```

## Opciones

```text
--width N       ancho del render, default 480
--height N      alto del render, default 270
--frame N       frame individual para camara orbital
--frames N      cantidad de frames para animacion
--animate       renderiza todos los frames en frames/
--interactive   modo consola para ajustar camara y renderizar previews
--summary       imprime resumen de escena sin renderizar
--angle N       angulo manual de camara en grados
--zoom N        zoom manual, mayor acerca la camara
--samples N     muestras por eje, 1 rapido, 2 default, 4 fino
--depth N       rebotes maximos de raytracing, default 3
--output PATH   salida PPM para un frame
```

## Video

Agregar aqui el video del diorama cuando este subido al README de GitHub.
