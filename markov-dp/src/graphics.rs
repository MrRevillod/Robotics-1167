use plotters::{prelude::*, style::Color};

use crate::{DISCOUNT_FACTORS, SUCCESS_PROBABILITIES};

pub fn graphic(results: &Vec<Vec<Vec<f32>>>) {
    // Colores correspondientes a cada factor de descuento
    let colors = [&BLUE, &RED, &GREEN, &MAGENTA];

    // Crear una grilla 2x2 de gráficos
    let root = BitMapBackend::new("analytics/rewards.png", (2400, 1600)).into_drawing_area();
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

        let y_range = (global_min - 10.0)..(global_max + 10.0);

        let mut chart = ChartBuilder::on(sub_area)
            .caption(
                &format!("Success Prob = {:.1}", SUCCESS_PROBABILITIES[success_idx]),
                ("sans-serif", 30),
            )
            .margin(15)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..max_length as i32, y_range)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Pasos de Simulación")
            .y_desc("Reward Acumulativo")
            .draw()
            .unwrap();

        // Dibujar una serie para cada factor de descuento
        for (i, cumulative_rewards) in all_cumulative_rewards.iter().enumerate() {
            chart
                .draw_series(LineSeries::new(
                    (0..cumulative_rewards.len()).map(|x| (x as i32, cumulative_rewards[x])),
                    colors[i],
                ))
                .unwrap()
                .label(&format!(
                    "γ = {} (Final: {:.2})",
                    DISCOUNT_FACTORS[i],
                    cumulative_rewards[cumulative_rewards.len() - 1]
                ))
                .legend(move |(x, y)| Circle::new((x, y), 3, colors[i].filled()));
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.9))
            .border_style(&BLACK)
            .label_font(("sans-serif", 12))
            .draw()
            .unwrap();
    }

    println!("Imagen comparativa 2x2 guardada: analytics/rewards.png");
}
