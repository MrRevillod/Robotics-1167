# Simulación MDP para Navegación de Robot - Proyecto Robótica INFO1167

## Descripción General

Este proyecto implementa un **Proceso de Decisión de Markov (MDP)** para resolver un problema de navegación robótica en un entorno 2D. El robot debe navegar por un mapa de cuadrícula de 6×8 para alcanzar un objetivo (meta) mientras maximiza sus recompensas y evita obstáculos y zonas peligrosas.

### Problema Planteado

Según la especificación del proyecto, se debe implementar el algoritmo **MDP Q(s,a) - Value Iteration** para:

- Construir matrices de transición para cada acción (N, S, E, O)
- Utilizar 4 factores de descuento (λ) → (0.86, 0.90, 0.94, 0.98)
- Probar 4 políticas óptimas con variación de probabilidad de éxito → (0.5, 0.7, 0.8, 0.9)
- Evaluar la robustez de las políticas mediante simulación gráfica durante 1000 pasos

## Arquitectura del Sistema

### 1. Estructura del Mapa (`map.rs`)

El mapa está definido como una cuadrícula de 6×8 con diferentes tipos de estados:

```rust
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatusType {
    Normal,   // Estados normales (S) - Recompensa: -0.1
    Danger,   // Estados peligrosos (P) - Recompensa: -0.5
    Wall,     // Muros/obstáculos (O) - Recompensa: -0.1
    Goal,     // Meta (M) - Recompensa: +10.0
}
```

**Configuración del mapa:**

```rust
let raw_map = vec![
    [ "S0",  "S1",  "P1",  "O1",  "S3",  "O2",  "S4",  "S5"  ],
    [ "O3",  "S6",  "S7",  "S8",  "S9",  "S10", "S11", "O4"  ],
    [ "S12", "P2",  "S14", "O5",  "S15", "P3",  "S17", "S18" ],
    [ "S19", "S20", "S21", "S22", "M",   "S24", "S25", "O6"  ],
    [ "S26", "O7",  "O8",  "S27", "S28", "S29", "P4",  "S31" ],
    [ "S32", "O9",  "S33", "S34", "O10", "S35", "S36", "S37" ],
];
```

### 2. Algoritmo MDP - Value Iteration (`mdp.rs`)

#### Construcción de Matrices de Transición

El sistema construye matrices de transición para cada acción considerando:

- **Probabilidad principal**: 80% de ejecutar la acción deseada
- **Probabilidades laterales**: 10% cada una para desviarse a izquierda/derecha

```rust
pub fn build_transition_matrix_static(map: &Map) -> Vec<Vec<Vec<f32>>> {
    // matrices[action][from][to]
    let mut matrices = vec![
        vec![vec![0.0; N_STATES]; N_STATES], // North
        vec![vec![0.0; N_STATES]; N_STATES], // South
        vec![vec![0.0; N_STATES]; N_STATES], // East
        vec![vec![0.0; N_STATES]; N_STATES], // West
    ];

    // Direcciones con probabilidades estocásticas
    let directions = [
        [(-1, 0), (0, -1), (0, 1)], // North: principal, izq(W), der(E)
        [(1, 0), (0, 1), (0, -1)],  // South: principal, izq(E), der(W)
        [(0, 1), (-1, 0), (1, 0)],  // East: principal, izq(N), der(S)
        [(0, -1), (1, 0), (-1, 0)], // West: principal, izq(S), der(N)
    ];
    // ... construcción de matrices considerando muros y límites
}
```

#### Iteración de Valores (Value Iteration)

La **Value Iteration** es el algoritmo central del MDP que nos permite encontrar la **política óptima** de navegación.

**¿Qué problema resuelve?**

El robot necesita decidir qué acción tomar en cada posición del mapa para llegar a la meta de la manera más eficiente, considerando que:

- Sus movimientos no son 100% precisos (puede desviarse)
- Algunas zonas son más peligrosas que otras
- Debe planificar a largo plazo, no solo el siguiente paso

**¿Cómo funciona conceptualmente?**

El algoritmo simula mentalmente todas las posibles trayectorias que puede tomar el robot y calcula cuál es la mejor estrategia. Es como un jugador de ajedrez que piensa varios movimientos por adelantado, pero considerando probabilidades de éxito/fallo.

**¿Qué hace el algoritmo paso a paso?**

```rust
pub fn value_iteration(&mut self, discount_factor: f32) {
    // Crear una tabla para almacenar qué tan buena es cada acción en cada posición
    let mut q = vec![vec![0.0_f32; 4]; N_STATES];

    let t = self.transition_matrix.clone(); // Probabilidades de movimiento

    // Repetir el cálculo muchas veces hasta que se estabilice
    for _ in 0..1000 {
        for s in 0..N_STATES { // Para cada posición del mapa (48 posiciones)
            for a in 0..4 { // Para cada dirección: Norte, Sur, Este, Oeste
                let mut valor_esperado = 0_f32;

                // Considerar todos los lugares donde podría terminar el robot
                for posicion_destino in 0..N_STATES {
                    // ¿Qué tan probable es llegar a esta posición?
                    let probabilidad = t[a][s][posicion_destino];

                    // ¿Qué recompensa obtendré al llegar ahí?
                    let recompensa_inmediata = self.map.states[
                        posicion_destino / N_COLS
                    ][posicion_destino % N_COLS].reward;

                    // ¿Cuál es el mejor valor que puedo obtener desde ahí?
                    let mejor_valor_futuro = q[posicion_destino].clone()
                        .into_iter().reduce(f32::max).unwrap_or(0.0);

                    // Combinar todo: Probabilidad × (Recompensa + Valor futuro)
                    valor_esperado += probabilidad * (
                        recompensa_inmediata + discount_factor * mejor_valor_futuro
                    );
                }

                // Guardar qué tan buena es esta acción en esta posición
                q[s][a] = valor_esperado;
            }
        }
    }

    self.q_values = q; // Guardar la tabla final
}
```

**¿Qué logra este proceso?**

1. **Tabla de decisiones**: Al final, tenemos una tabla que dice "en la posición X, la mejor acción es Y"

2. **Aprendizaje por simulación**: El robot "practica" mentalmente miles de escenarios sin moverse físicamente

3. **Considera incertidumbre**: Toma en cuenta que puede fallar el 20% de las veces

4. **Planificación inteligente**: No solo busca el camino más corto, sino el más seguro considerando recompensas

**¿Por qué usamos factores de descuento?**

El `discount_factor` controla la "paciencia" del robot:

- **0.86**: "Prefiero llegar rápido, aunque el camino sea más arriesgado"
- **0.90**: "Balance entre velocidad y seguridad"
- **0.94**: "Prefiero un camino más seguro aunque tome más tiempo"
- **0.98**: "Estoy dispuesto a tomar rutas muy largas si eso garantiza éxito"

**¿Cómo se obtiene la política final?**

```rust
pub fn get_max_policy(&mut self) -> Vec<usize> {
    let mut politica_optima = vec![0; N_STATES];

    for (posicion, valores_acciones) in self.q_values.iter().enumerate() {
        // Para cada posición, encontrar la mejor acción
        let mejor_valor = valores_acciones.iter().reduce(f32::max).unwrap();
        let mejor_accion = valores_acciones.iter()
            .position(|x| *x == *mejor_valor).unwrap();
        politica_optima[posicion] = mejor_accion;
    }

    politica_optima
}
```

**Resultado práctico:**

El algoritmo nos da una **estrategia completa** que dice exactamente qué hacer en cada situación:

- "Si estoy en la posición (2,3), ir al Norte"
- "Si estoy en la posición (4,1), ir al Este"
- Y así para las 48 posiciones del mapa

Esta estrategia es **óptima** porque maximiza las recompensas esperadas considerando tanto las recompensas inmediatas como las futuras.

### 3. Robot Autónomo (`robot.rs`)

El robot implementa un comportamiento estocástico basado en las probabilidades de éxito configuradas:

```rust
fn calc_next_action(&self, next_action: usize) -> (f32, f32) {
    let north = (0.0, -TILE_SIZE);
    let south = (0.0, TILE_SIZE);
    let east = (TILE_SIZE, 0.0);
    let west = (-TILE_SIZE, 0.0);

    let combinations = [
        [north, east, west],
        [south, east, west],
        [east, north, south],
        [west, south, north],
    ];

    let possible_actions = combinations[next_action];
    let success_prob = SUCCESS_PROBABILITIES[self.success_prob] as f32;

    let choice = rand::random::<f32>();

    if choice <= success_prob {
        possible_actions[0]
    } else {
        match rand::random_bool(0.5) {
            true => possible_actions[1],
            false => possible_actions[2],
        }
    }
```

### 4. Sistema de Simulación (`core.rs`)

#### Simulación Masiva de Datos

El núcleo del sistema ejecuta simulaciones exhaustivas:

```rust
pub fn run_simulation() -> Vec<Vec<Vec<f32>>> {
    let mut results = vec![vec![vec![]; 4]; 4]; // [success_prob][discount_factor]

    // Para cada combinación de parámetros
    for success_prob in 0..4 {
        for discount_factor in 0..4 {
            let mut simulation_steps = 0;
            let mut rewards = vec![];

            while simulation_steps < 1000 {
                robot.update(&mdps[discount_factor].get_max_policy(), &map);
                simulation_steps += 1;

                let robot_pos = robot.get_matricial_position();
                rewards.push(map.states[robot_pos[0]][robot_pos[1]].reward);

                // Reiniciar robot al alcanzar la meta
                if robot.get_position() == map.get_goal_position() {
                    let new_position = map.get_random_valid_position();
                    robot.set_position(new_position);
                }
            }
            results[success_prob][discount_factor] = rewards;
        }
    }
    results
}
```

## Parámetros de Configuración

### Constantes del Sistema

```rust
pub const N_ROWS: usize = 6;
pub const N_COLS: usize = 8;
pub const N_STATES: usize = N_ROWS * N_COLS; // 48 estados
pub const PROBABILITIES: [f32; 3] = [0.8, 0.1, 0.1]; // Principal, Izq, Der

// Factores de descuento (λ)
pub const DISCOUNT_FACTORS: [f32; 4] = [0.86, 0.90, 0.94, 0.98];

// Probabilidades de éxito del robot
pub const SUCCESS_PROBABILITIES: [f32; 4] = [0.5, 0.7, 0.8, 0.9];
```

## Visualización y Análisis (`graphics.rs`)

El sistema genera gráficos de recompensas acumulativas para analizar el desempeño:

```rust
pub fn graphic(results: &Vec<Vec<Vec<f32>>>) {
    // Crea 4 subgráficos (uno por probabilidad de éxito)
    // Cada gráfico muestra 4 curvas (una por factor de descuento)

    for (success_idx, success_prob_results) in results.iter().enumerate() {
        let mut cumulative_rewards = Vec::new();
        let mut cumulative_sum = 0.0;

        for &reward in rewards {
            cumulative_sum += reward;
            cumulative_rewards.push(cumulative_sum);
        }
        // ... renderizado con plotters
    }
}
```

## Funcionalidades Principales

### ✅ Implementadas según especificación:

1. **Matrices de Transición**: Construidas para las 4 acciones (N,S,E,O) considerando probabilidades estocásticas
2. **4 Factores de Descuento**: λ = [0.86, 0.90, 0.94, 0.98]
3. **4 Probabilidades de Éxito**: [50%, 70%, 80%, 90%]
4. **Políticas Óptimas**: Calculadas mediante Value Iteration para cada configuración
5. **Simulación de 1000 pasos**: Evaluación robusta de cada política
6. **Visualización gráfica**: Generación automática de gráficos de recompensas acumulativas

### 🎮 Funcionalidades adicionales:

- **Visualización interactiva en tiempo real** usando Raylib
- **Reinicio automático** del robot al alcanzar la meta
- **Validación de movimientos** (límites del mapa y obstáculos)
- **Posicionamiento aleatorio** inicial del robot

## Resultados y Análisis

Los resultados de la simulación se visualizan en `rewards.png`, mostrando:

- **4 gráficos** (uno por probabilidad de éxito del robot)
- **4 curvas por gráfico** (una por factor de descuento)
- **Recompensas acumulativas** durante 1000 pasos de simulación

Esto permite evaluar cómo diferentes factores de descuento y probabilidades de éxito afectan el desempeño del robot en el entorno, cumpliendo con los objetivos del módulo de MDP en robótica.

```

```
