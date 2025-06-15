use plotters::{prelude::*, style::Color};

// la politica me dice que accion tomar en cada estado
// para eso es el algoritmo de valor iterativo -> politica optima

// lanzar random y armar rangos desde 0 a 1 (0.8, 0.1, 0.1)

// la grafica la acumulo por 1000 pasos y el reguard y cada politica es
// la de cada factor de descuento

// grafica los resultados y guardalos en imagen

pub fn graphic(results: &Vec<Vec<f32>>) {
    // Factores de descuento correspondientes a cada simulación
    let discount_factors = [0.1, 0.5, 0.9, 0.99];
    let colors = [&BLUE, &RED, &GREEN, &MAGENTA];

    // Preparar datos acumulativos para todas las simulaciones
    let mut all_cumulative_rewards = Vec::new();
    let mut global_min = f32::INFINITY;
    let mut global_max = f32::NEG_INFINITY;
    let mut max_length = 0;

    for rewards in results.iter() {
        let mut cumulative_rewards = Vec::new();
        let mut cumulative_sum = 0.0;

        for &reward in rewards {
            cumulative_sum += reward;
            cumulative_rewards.push(cumulative_sum);
        }

        // Actualizar rangos globales
        let min_reward = cumulative_rewards
            .iter()
            .fold(f32::INFINITY, |a, &b| a.min(b));
        let max_reward = cumulative_rewards
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        global_min = global_min.min(min_reward);
        global_max = global_max.max(max_reward);
        max_length = max_length.max(cumulative_rewards.len());

        all_cumulative_rewards.push(cumulative_rewards);
    }

    // Crear una sola imagen comparativa
    let filename = "analytics/rewards.png";
    let root = BitMapBackend::new(filename, (1200, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let y_range = (global_min - 10.0)..(global_max + 10.0);

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Comparación de Simulaciones - Rewards Acumulativos",
            ("sans-serif", 50),
        )
        .margin(15)
        .x_label_area_size(50)
        .y_label_area_size(70)
        .build_cartesian_2d(0..max_length as i32, y_range)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Pasos de Simulación")
        .y_desc("Reward Acumulativo")
        .draw()
        .unwrap();

    // Dibujar una serie para cada simulación con diferente color
    for (i, cumulative_rewards) in all_cumulative_rewards.iter().enumerate() {
        chart
            .draw_series(LineSeries::new(
                (0..cumulative_rewards.len()).map(|x| (x as i32, cumulative_rewards[x])),
                colors[i],
            ))
            .unwrap()
            .label(&format!(
                "γ = {} (Final: {:.2})",
                discount_factors[i],
                cumulative_rewards[cumulative_rewards.len() - 1]
            ))
            .legend(move |(x, y)| Circle::new((x, y), 4, colors[i].filled()));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.9))
        .border_style(&BLACK)
        .label_font(("sans-serif", 20))
        .draw()
        .unwrap();

    println!("Imagen comparativa guardada: {}", filename);
}
