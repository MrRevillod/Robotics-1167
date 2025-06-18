# Robotics INFO1167 - Proyectos de Simulación

Este repositorio contiene dos proyectos de simulación robótica desarrollados en Rust para el curso INFO1167. Ambos proyectos utilizan el motor gráfico Raylib para visualización y diferentes algoritmos para el comportamiento de los robots.

## 📁 Estructura del Proyecto

```
Robotics-1167/
├── basketbots/        # Simulación 3D de robots jugando baloncesto
├── markov-dp/         # Navegación robótica usando Procesos de Decisión de Markov
├── Cargo.toml         # Configuración del workspace
└── README.md          # Este archivo
```

## 🏀 Basketbots - Simulación 3D de Baloncesto Robótico

### Descripción

Simulación tridimensional donde robots autónomos se mueven en un estadio de baloncesto, interactuando con una pelota y aros. Los robots exhiben comportamiento autónomo con movimientos aleatorios y físicas realistas.

### Características Principales

- **Entorno 3D completo**: Estadio, robots y aros renderizados en 3D
- **Robots autónomos**: Movimiento independiente con ángulos y velocidades aleatorias
- **Física realística**: Simulación de gravedad y colisiones
- **Cámara interactiva**: Control de cámara para observar desde diferentes perspectivas
- **Modelos 3D**: Assets personalizados para robots (azules y rojos) y elementos del juego

### Tecnologías

- **Rust** con edición 2024
- **Raylib 5.5.1** para renderizado 3D
- **Rand 0.8.5** para comportamiento aleatorio

## 🤖 Markov-DP - Navegación Robótica con MDP

### Descripción

Implementación de un Proceso de Decisión de Markov (MDP) para resolver problemas de navegación robótica en un entorno 2D de cuadrícula. El robot debe navegar por un mapa de 6×8 celdas para alcanzar un objetivo mientras maximiza recompensas y evita obstáculos.

### Características Principales

- **Algoritmo MDP**: Implementación de Value Iteration Q(s,a)
- **Mapa estructurado**: Cuadrícula 6×8 con diferentes tipos de estados:
  - Estados normales (S) - Recompensa: -0.1
  - Estados peligrosos (P) - Recompensa: -0.5
  - Muros/obstáculos (O) - Recompensa: -0.1
  - Meta (M) - Recompensa: +10.0
- **Múltiples configuraciones**:
  - 4 factores de descuento (λ): 0.86, 0.90, 0.94, 0.98
  - 4 probabilidades de éxito: 0.5, 0.7, 0.8, 0.9
- **Visualización gráfica**: Simulación interactiva y generación de gráficos analíticos
- **Evaluación de robustez**: Simulación de 1000 pasos para validar políticas

### Algoritmo MDP

- **Matrices de transición** para cada acción (Norte, Sur, Este, Oeste)
- **Probabilidades de acción**: 80% acción deseada, 20% distribuido en otras direcciones
- **Convergencia**: Iteración hasta que los valores convergen

### Tecnologías

- **Rust** con edición 2024
- **Raylib 5.5.1** para visualización 2D
- **Plotters 0.3.7** para generación de gráficos analíticos
- **Rand 0.9.1** para simulación estocástica

Los resultados analíticos se generan en la carpeta `analytics/` con gráficos de recompensas y proyección del rendimiento.
