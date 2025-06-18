use plotters::{prelude::*, style::Color};

use crate::{DISCOUNT_FACTORS, SUCCESS_PROBABILITIES};

pub fn graphic(results: &Vec<Vec<Vec<f32>>>) {
    let colors = [&BLUE, &RED, &GREEN, &MAGENTA];

    // Aumentar el tamaño de la imagen para gráficos más grandes y legibles
    let root = BitMapBackend::new("analytics/rewards.png", (3200, 2400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let sub_areas = root.split_evenly((2, 2));

    for (success_idx, success_prob_results) in results.iter().enumerate() {
        let sub_area = &sub_areas[success_idx];

        // Preparar datos acumulativos para esta probabilidad de éxito
        let mut all_cumulative_rewards = Vec::new();
        let mut global_min = f32::INFINITY;
        let mut global_max = f32::NEG_INFINITY;
        let mut max_length = 0;

        for rewards in success_prob_results.iter() {
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

        // Calcular un rango Y más granular con espacios más pequeños
        let y_range_size = global_max - global_min;
        let y_padding = y_range_size * 0.1; // 10% de padding
        let y_min = global_min - y_padding;
        let y_max = global_max + y_padding;

        let y_range = y_min..y_max;

        let mut chart = ChartBuilder::on(sub_area)
            .caption(
                &format!(
                    "Probabilidad de Éxito = {:.1}",
                    SUCCESS_PROBABILITIES[success_idx]
                ),
                ("sans-serif", 40), // Título más grande
            )
            .margin(25) // Márgenes más grandes
            .x_label_area_size(80) // Área más grande para etiquetas X
            .y_label_area_size(120) // Área más grande para etiquetas Y
            .build_cartesian_2d(0..max_length as i32, y_range)
            .unwrap();

        chart
            .configure_mesh()
            .y_max_light_lines(20) // Más líneas de cuadrícula para mejor granularidad
            .x_max_light_lines(10)
            .axis_desc_style(("sans-serif", 20)) // Etiquetas de ejes más grandes
            .label_style(("sans-serif", 16)) // Números en los ejes más grandes
            .x_desc("Pasos de Simulación")
            .y_desc("Reward Acumulativo")
            .draw()
            .unwrap();

        for (i, cumulative_rewards) in all_cumulative_rewards.iter().enumerate() {
            chart
                .draw_series(LineSeries::new(
                    (0..cumulative_rewards.len()).map(|x| (x as i32, cumulative_rewards[x])),
                    colors[i].stroke_width(3), // Líneas más gruesas para mejor visibilidad
                ))
                .unwrap()
                .label(&format!(
                    "γ = {} (Final: {:.2})",
                    DISCOUNT_FACTORS[i],
                    cumulative_rewards[cumulative_rewards.len() - 1]
                ))
                .legend(move |(x, y)| Circle::new((x, y), 5, colors[i].filled())); // Círculos más grandes en la leyenda
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.95))
            .border_style(&BLACK)
            .label_font(("sans-serif", 16)) // Fuente más grande para la leyenda
            .margin(15) // Más margen para la leyenda
            .draw()
            .unwrap();
    }

    println!("Imagen comparativa 2x2 guardada: analytics/rewards.png");
}
