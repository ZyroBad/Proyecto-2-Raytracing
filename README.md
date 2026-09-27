# Proyecto 2: Diorama con Raytracing

Diorama voxel inspirado en la llegada de Naruto en Modo Sabio durante la invasion de Pain. El proyecto esta escrito en Rust puro, sin librerias externas, e implementa un raytracer desde cero.

La escena muestra Konoha destruida, el crater de la aldea, Monte Hokage y las invocaciones Gamabunta, Gamaken y Gamahiro. Gamakichi se encuentra sobre Gamabunta y Naruto esta sobre Gamakichi con su traje de Modo Sabio.

## Caracteristicas

- Diorama construido con cubos texturizados proceduralmente.
- Camara orbital de 360 grados con acercamiento y alejamiento.
- BVH para acelerar la interseccion de miles de cubos.
- Renderizado paralelo usando los nucleos disponibles del procesador.
- Sombras suaves, iluminacion difusa, brillo especular y niebla atmosferica.
- Reflexion en armas, protectores y superficies metalicas.
- Refraccion y transparencia en chakra, humo y nubes de invocacion.
- Skybox procedural con horizonte, sol y nubes.
- Konoha destruida en 360 grados y Monte Hokage ampliado.
- Personajes modelados con volumen completo y detalles visibles desde diferentes angulos.

## Personajes

- **Gamabunta:** sapo gigante rojizo con escamas, cresta, haori y vientre segmentado.
- **Gamaken:** sapo magenta con patron moteado, haori y sasumata metalico.
- **Gamahiro:** sapo celeste con escamas acuaticas, mascara ocular y dos espadas.
- **Gamakichi:** sapo naranja juvenil con escamas pequenas y chaleco azul.
- **Naruto:** Modo Sabio con capa roja, traje naranja, pergamino, ojos dorados y protector metalico.

## Materiales principales

Cada material posee textura procedural y parametros independientes de albedo, brillo especular, transparencia, reflectividad e indice de refraccion.

| Material | Textura | Efecto destacado |
| --- | --- | --- |
| Piel de Gamabunta | Escamas grandes, manchas y poros | Reflexion suave |
| Piel de Gamaken | Escamas magenta y moteado | Brillo humedo |
| Piel de Gamahiro | Escamas, bandas y ruido acuatico | Reflexion suave |
| Piel de Gamakichi | Escamas naranjas pequenas y pecas | Brillo humedo |
| Vientre | Poros y pliegues horizontales | Difusion mate |
| Tela y capa | Tejido, dobleces y variaciones de color | Reflexion minima |
| Metal | Rayones direccionales | Reflexion alta |
| Chakra | Pulso cromatico procedural | Transparencia y refraccion |
| Nubes y humo | Ruido de baja frecuencia | Transparencia |
| Roca y crater | Grietas, grava y erosion | Superficie rugosa |
| Madera y cuerda | Vetas y patron trenzado | Reflexion baja |

## Ejecutar

Resumen de la escena:

```bash
cargo run --release -- --summary
```

Render rapido de prueba:

```bash
cargo run --release -- --width 320 --height 180 --samples 1 --depth 2 --output renders/preview.bmp
```

Render recomendado:

```bash
cargo run --release -- --width 800 --height 450 --samples 2 --depth 3 --angle 90 --zoom 0.96 --output renders/final.bmp
```

Ejecutar las pruebas:

```bash
cargo test
```

## Camara interactiva

```bash
cargo run --release -- --interactive --width 320 --height 180 --samples 1 --depth 2
```

Controles:

- `a` / `d`: rotar la camara.
- `w` / `s`: ajuste fino del angulo orbital.
- `+` / `-`: acercar y alejar la camara.
- `r`: volver a renderizar.
- `q`: salir.

Cada vista interactiva se guarda en `renders/interactive.bmp`.

## Animacion

Generar 120 cuadros de una orbita completa:

```bash
cargo run --release -- --width 480 --height 270 --samples 1 --depth 3 --frames 120 --animate
```

Los cuadros se guardan en `frames/` y pueden grabarse o convertirse en video para incluirlo en este README.

## Opciones

```text
--width N       ancho del render
--height N      alto del render
--frame N       cuadro individual de la orbita
--frames N      cantidad de cuadros de la animacion
--animate       renderiza todos los cuadros en frames/
--interactive   abre el control de camara por consola
--summary       muestra el resumen de la escena
--angle N       angulo manual de camara en grados
--zoom N        acercamiento de camara
--samples N     muestras por eje
--depth N       rebotes maximos de raytracing
--output PATH   archivo BMP o PPM de salida
```

## Video

El video final del diorama se agregara aqui antes de la entrega.
