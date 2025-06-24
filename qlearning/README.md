# Diferencias entre Value Iteration y Q-Learning

Este documento explica las diferencias clave entre el enfoque actual del proyecto (Value Iteration) y el objetivo a implementar (Q-Learning).

## 1. Value Iteration (Enfoque Actual)

**Value Iteration**, y su variante directa para calcular Q-values, es un algoritmo de **programación dinámica** que pertenece a los métodos **basados en modelo (Model-Based)**.

- **¿Cómo funciona?**: Calcula la política óptima de forma iterativa utilizando un conocimiento completo del entorno.
- **Requisito Clave**: Necesita conocer el **modelo del MDP**, es decir, las probabilidades de transición (`T(s, a, s')`) y las recompensas (`R(s, a, s')`).
- **Proceso**: Es un proceso **offline**. Primero, se calcula la tabla completa de Q-values (`q_values`) para todos los estados y acciones. Una vez que la tabla converge, el robot simplemente la consulta para decidir la mejor acción en cada estado. No "aprende" mientras se mueve, solo ejecuta la política ya calculada.

En tu código actual en [`mdp-qlearning/src/mdp.rs`](mdp-qlearning/src/mdp.rs), la función `value_iteration` implementa exactamente esto. Construye la `transition_matrix` y la usa para computar todos los `q_values` antes de que el robot comience a moverse.

```rust
// Ejemplo conceptual de la actualización en Value Iteration
// V(s) = max_a Σ_s' T(s, a, s') * [R(s, a, s') + γ * V(s')]
// O para Q-values:
// Q(s, a) = Σ_s' T(s, a, s') * [R(s, a, s') + γ * max_a' Q(s', a')]
```

## 2. Q-Learning (Nuevo Enfoque)

**Q-Learning** es un algoritmo de **aprendizaje por refuerzo (Reinforcement Learning)** que pertenece a los métodos **libres de modelo (Model-Free)**.

- **¿Cómo funciona?**: Aprende la política óptima a través de la **experiencia directa**, interactuando con el entorno mediante prueba y error.
- **Requisito Clave**: **No necesita conocer el modelo del MDP**. No requiere saber de antemano las probabilidades de transición ni las recompensas. Las descubre a medida que actúa.
- **Proceso**: Es un proceso **online**. El robot toma una acción, observa la recompensa y el estado siguiente, y utiliza esa información para actualizar gradualmente su tabla de Q-values. El aprendizaje y la acción ocurren simultáneamente.

La fórmula de actualización de Q-Learning es la siguiente:

```
Q(s, a) ← Q(s, a) + α * [R + γ * max_a' Q(s', a') - Q(s, a)]
```

Donde:

- `α` (alpha) es la **tasa de aprendizaje (learning rate)**, que determina cuánto peso se le da a la nueva información.
- `(s, a)` es el par estado-acción actual.
- `R` es la recompensa recibida.
- `s'` es el nuevo estado.
- `γ` (gamma) es el factor de descuento.

## Tabla Comparativa

| Característica    | Value Iteration (Q-value Iteration)                              | Q-Learning                                                               |
| ----------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------------ |
| **Paradigma**     | Programación Dinámica                                            | Aprendizaje por Refuerzo (Temporal-Difference)                           |
| **Modelo**        | **Basado en Modelo** (Requiere `T(s, a, s')`)                    | **Libre de Modelo** (No necesita el modelo)                              |
| **Proceso**       | **Offline**: Calcula la política óptima antes de actuar.         | **Online**: Aprende mientras interactúa con el entorno.                  |
| **Conocimiento**  | Requiere un mapa completo del mundo y sus reglas.                | Aprende las reglas a través de la experiencia (prueba y error).          |
| **Actualización** | Calcula un valor esperado sobre todas las posibles transiciones. | Actualiza el valor basado en la transición y recompensa observadas.      |
| **Exploración**   | No es necesaria. La optimalidad se calcula analíticamente.       | **Fundamental**. Necesita un balance (ej. Epsilon-Greedy) para explorar. |

## Implicaciones para tu Proyecto

Para adaptar tu proyecto de `mdp-qlearning` a Q-Learning, necesitarás:

1.  **Eliminar la `transition_matrix`**: Ya no es necesaria para el algoritmo de aprendizaje.
2.  **Modificar el bucle de simulación**: En lugar de que el robot solo siga una política pre-calculada, cada paso de la simulación implicará:
    a. El robot elige una acción (considerando exploración vs. explotación).
    b. El robot ejecuta la acción y observa el nuevo estado (`s'`) y la recompensa (`R`).
    c. Usas esta información para actualizar la tabla `q_values` según la fórmula de Q-Learning.
3.  **Introducir nuevos hiperparámetros**: Necesitarás `alpha` (tasa de aprendizaje) y `epsilon` (para la estrategia de exploración Epsilon-Greedy).
