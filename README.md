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
- Konoha destruida en 360 grados con terreno continuo, crater profundo escalonado, barrios derrumbados, bosque, cordillera, torres y puertas.
- Monte Hokage integrado en una montana distante con cinco rostros tallados y rasgos individuales.
- Personajes modelados con volumen completo, haori posterior y detalles visibles desde diferentes angulos.
- Sello de invocacion refractivo alrededor de la escena principal.

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
| Follaje distante | Grupos de hojas, ruido y luz superior | Reflexion suave |
| Montana distante | Estratos y erosion procedural | Superficie mate |

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
cargo run --release -- --width 960 --height 540 --samples 2 --depth 4 --angle 82 --zoom 0.90 --output renders/final.bmp
```

Este ajuste conserva una relacion 16:9 y es razonable para una computadora con graficos integrados. Para una captura mas limpia se puede subir a `--samples 3`, aceptando un tiempo de render considerablemente mayor.

Ejecutar las pruebas:

```bash
cargo test
```

## Ventana interactiva

Abrir el diorama en una ventana nativa de Windows:

```bash
cargo run --release -- --window --width 400 --height 225
```

Controles:

- `a` / `d` o flechas izquierda/derecha: orbitar alrededor de la escena.
- `w` / `s` o flechas arriba/abajo: subir y bajar la camara.
- `+` / `-`: acercar y alejar la camara.
- `r`: volver a renderizar la vista.
- `Esc`: cerrar la ventana.

La ventana usa una resolucion interactiva y ajustes moderados para responder bien en graficos integrados. El render final puede generarse despues con mayor resolucion, muestras y profundidad.

Mientras una tecla permanece presionada, la ventana reduce temporalmente la resolucion, la profundidad de rayos y las muestras de sombra para mantener el movimiento continuo. Al soltarla, restaura automaticamente una vista detallada con sombras suaves. El BVH se construye una sola vez al abrir la ventana y se reutiliza durante toda la navegacion.

## Camara por consola

```bash
cargo run --release -- --interactive --width 320 --height 180 --samples 1 --depth 2
```

Controles:

- `a` / `d`: rotar la camara.
- `w` / `s`: subir y bajar la camara.
- `+` / `-`: acercar y alejar la camara.
- `r`: volver a renderizar.
- `q`: salir.

Cada vista interactiva se guarda en `renders/interactive.bmp`.

## Animacion

Probar la orbita completa con pocos cuadros:

```bash
cargo run --release -- --width 320 --height 180 --samples 1 --depth 2 --frames 24 --animate
```

Generar los 120 cuadros para el video final:

```bash
cargo run --release -- --width 640 --height 360 --samples 2 --depth 3 --frames 120 --animate
```

Los cuadros se guardan en `frames/` con numeracion consecutiva. Primero conviene revisar la prueba de 24 cuadros; el render final puede tardar bastante en graficos integrados. Luego los cuadros pueden importarse como secuencia de imagenes en el editor de video elegido para producir el archivo que se enlazara en este README.

## Evidencia de raytracing

- **Reflexion:** armas, protectores metalicos y detalles pulidos reflejan el skybox mediante rayos secundarios y Fresnel.
- **Refraccion:** el sello y las corrientes de chakra alteran los rayos que los atraviesan con un indice de refraccion propio.
- **Transparencia:** el humo de invocacion y el polvo dejan ver parcialmente la escena posterior.
- **Sombras:** cada punto consulta visibilidad hacia varias muestras de la luz para producir bordes suaves.
- **Skybox:** el cielo procedural incluye horizonte, sol, resplandor, nubes altas y polvo atmosferico.

## Opciones

```text
--width N       ancho del render
--height N      alto del render
--frame N       cuadro individual de la orbita
--frames N      cantidad de cuadros de la animacion
--animate       renderiza todos los cuadros en frames/
--window        abre una ventana nativa interactiva
--interactive   abre el control de camara por consola
--summary       muestra el resumen de la escena
--angle N       angulo manual de camara en grados
--elevation N   desplazamiento vertical de la camara
--zoom N        acercamiento de camara
--samples N     muestras por eje
--depth N       rebotes maximos de raytracing
--output PATH   archivo BMP o PPM de salida
```

## Video

El video final del diorama se agregara aqui antes de la entrega.
