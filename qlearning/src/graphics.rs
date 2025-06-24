use crate::SUCCESS_PROBABILITIES;
use plotters::prelude::*;

const COLORS: [RGBColor; 3] = [RED, BLUE, GREEN];

pub fn downsample<T: Copy + Into<f64>>(data: &[T], stride: usize) -> Vec<(usize, f64)> {
    data.iter()
        .enumerate()
        .filter(|(i, _)| i % stride == 0)
        .map(|(i, &v)| (i, v.into()))
        .collect()
}

pub fn plot_rewards_and_steps(
    rewards: &[Vec<f64>],
    steps: &[Vec<usize>],
) -> Result<(), Box<dyn std::error::Error>> {
    plot_line_chart(
        "Recompenza por episodios",
        "Episode",
        "Total Reward",
        "rewards.png",
        rewards,
    )?;

    let steps_f64: Vec<Vec<f64>> = steps
        .iter()
        .map(|series| series.iter().map(|&v| v as f64).collect())
        .collect();

    plot_line_chart(
        "Pasos hasta la meta por episodios",
        "Episode",
        "Steps",
        "steps.png",
        &steps_f64,
    )?;

    Ok(())
}

fn plot_line_chart<T: Copy + Into<f64>>(
    title: &str,
    x_label: &str,
    y_label: &str,
    filename: &str,
    data_series: &[Vec<T>],
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("plots/{}", filename);
    let root = BitMapBackend::new(&filename, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_x = data_series.iter().map(|s| s.len()).max().unwrap_or(0);

    let min_y = data_series
        .iter()
        .flat_map(|s| s.iter().map(|&v| v.into()))
        .fold(f64::INFINITY, f64::min);
    let max_y = data_series
        .iter()
        .flat_map(|s| s.iter().map(|&v| v.into()))
        .fold(f64::NEG_INFINITY, f64::max);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..max_x, min_y..max_y)?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .draw()?;

    for (i, series) in data_series.iter().enumerate() {
        let color = COLORS[i % COLORS.len()].to_rgba();
        let label = format!("P = {:.1}", SUCCESS_PROBABILITIES[i]);

        let downsampled = downsample(series, 50);

        chart
            .draw_series(LineSeries::new(downsampled, &color))?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
