
# Basketbots

Este repositorio es un proyecto de simulación de robótica que utiliza el motor gráfico Raylib para representar un estadio donde robots autónomos interactúan con una pelota y aros de baloncesto. El objetivo principal es simular el movimiento de los robots, el disparo de la pelota y la interacción con el entorno.

## Características

- **Robots Autónomos**: Los robots se mueven de manera autónoma dentro del estadio, siguiendo ángulos y velocidades aleatorias.
- **Entorno 3D**: El estadio, los robots y los aros están renderizados en un entorno tridimensional.
- **Interacción con el Usuario**: La cámara puede ser controlada por el usuario para observar la simulación desde diferentes perspectivas.

## Dependencias

- [Raylib](https://www.raylib.com/): Motor gráfico para renderizado 3D.
- [Rand](https://crates.io/crates/rand): Generación de números aleatorios.

## Cómo Ejecutar

1. Asegúrate de tener Rust instalado en tu sistema.
2. Clona este repositorio y navega al directorio del proyecto.
3. Ejecuta el comando:

   ```sh
   cargo run
   ```