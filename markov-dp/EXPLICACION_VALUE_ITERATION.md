# Explicación del Value Iteration Implementado

## ¿Para qué sirven las matrices de transición?

Las **matrices de transición** que construiste son la columna vertebral de tu MDP. Te explico su importancia:

### 1. **Modelan la incertidumbre del mundo real**

- Tu robot no siempre se mueve exactamente donde quiere
- Con probabilidad **0.8** va en la dirección deseada
- Con probabilidad **0.1** cada una se desvía a izquierda o derecha
- Esto simula el ruido y la incertidumbre de los sensores y actuadores reales

### 2. **Estructura de las matrices**

```
transition_matrix[acción][estado_origen][estado_destino] = probabilidad
```

- **4 matrices** (una por cada acción: North, South, East, West)
- **48x48 estados** (6 filas × 8 columnas = 48 estados totales)
- Cada celda contiene la **probabilidad** de transición

### 3. **Permiten calcular expectativas**

Las matrices son esenciales para el algoritmo Value Iteration porque permiten calcular:

```
V*(s) = max_a Σ P(s'|s,a) × [R(s') + γ × V*(s')]
```

## Implementación del Value Iteration

### ¿Qué hace el algoritmo?

1. **Inicializa** todos los valores de estado en 0
2. **Itera** actualizando cada valor basándose en:
   - La recompensa inmediata
   - Los valores futuros esperados (descontados)
3. **Converge** cuando los cambios son menores al umbral θ
4. **Extrae** la política óptima eligiendo la mejor acción para cada estado

### Componentes clave implementados:

#### `ValueIteration` struct

```rust
pub struct ValueIteration {
    pub values: Vec<f32>,           // V(s) - valor de cada estado
    pub policy: Vec<usize>,         // π(s) - política óptima
    pub rewards: Vec<f32>,          // R(s) - recompensa de cada estado
    pub discount_factor: f32,       // γ - factor de descuento
    pub theta: f32,                // θ - umbral de convergencia
}
```

#### Función principal `run()`

- Ejecuta las iteraciones hasta convergencia
- Actualiza valores usando la ecuación de Bellman
- Extrae la política óptima al final

### Resultados obtenidos

Con **γ = 0.98** (factor de descuento alto):

- **Converge en 21 iteraciones**
- El estado inicial **S0** tiene valor **5.028**
- La **política óptima** guía al robot desde cualquier punto hacia la meta **M**

## Simulaciones exitosas

### Desde S0 (esquina superior izquierda):

```
S0 → S1 → S6 → S7 → S14 → S21 → S22 → M
7 pasos para llegar a la meta
```

### Desde S12 (izquierda del mapa):

```
S12 → S19 → S20 → S21 → S22 → M
5 pasos para llegar a la meta
```

## Próximos pasos (Fase 2 y 3)

### Fase 2: Simulación y Visualización

1. **✅ Movimiento del robot** - Ya implementado
2. **🔄 Interfaz para cambiar factores de descuento** - Parcialmente implementado
3. **🔄 Visualización de la política** - Pendiente (mostrar flechas en el mapa)

### Fase 3: Análisis y Evaluación

1. **✅ Sistema de métricas** - Ya implementado (pasos, recompensa acumulada)
2. **✅ Evaluación de robustez** - Ya implementado (diferentes γ)
3. **🔄 Graficación de resultados** - Pendiente (exportar datos para graficar)

## Conceptos clave que debes entender

### Factor de descuento (γ)

- **γ = 0.86**: Robot "impaciente", prefiere recompensas inmediatas
- **γ = 0.98**: Robot "paciente", valora recompensas futuras

### Política óptima (π\*)

- Te dice **qué acción tomar** en cada estado
- Ejemplo: En S0, la mejor acción es **East**

### Función de valor (V\*)

- Te dice **qué tan bueno** es estar en un estado
- Estados cerca de la meta tienen valores altos
- Estados cerca de peligros tienen valores bajos

## Archivo implementado

Todo el código del Value Iteration está en:

- `/src/mdp/analytics.rs` - Algoritmo principal
- `/src/mdp/mod.rs` - Integración y simulación
- `/src/map.rs` - Interfaz con el mapa
- `/src/main.rs` - Ejecución y pruebas

¡Tu implementación está funcionando perfectamente y cumple con los requerimientos de la Fase 1!
