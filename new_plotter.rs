use crate::config_reader::Config;
use crate::config_reader::get_name;
use anyhow::Result;
use opener::open;
use plotters::prelude::*;

pub fn graphit(timepl: Vec<(f64, f64)>, optype: &str, config: Config) -> Result<()> {
    // Infer axis ranges from data
    let x_max = timepl
        .iter()
        .map(|(x, _)| *x)
        .fold(0.0f64, |max, val| max.max(val));

    let y_max = timepl
        .iter()
        .map(|(_, y)| *y)
        .fold(0.0f64, |max, val| max.max(val));

    let mut filename = if config.filename.to_lowercase() == "default"
        || config.filename.to_lowercase() == "none"
    {
        get_name()?
    } else {
        config.filename.clone()
    };

    if !filename.to_lowercase().ends_with(".png") {
        filename.push_str(".png");
    }

    let root = BitMapBackend::new(&filename, (config.width, config.height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("{} over time", optype),
            (
                config.captionfontfamily.as_str(),
                config.captionfontsize,
                &config.captioncolor,
            ),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0 as f64..x_max, 0 as f64..y_max + 1 as f64)?;

    chart
        .configure_mesh()
        .x_desc("Time (minutes)")
        .y_desc(match optype {
            "Ping" => "Ping (ms)",
            "Loss" => "Loss (%)",
            _ => "Value",
        })
        .draw()?;

    chart.draw_series(LineSeries::new(timepl, &config.linecolor))?;
    root.present()?;
    open(&filename)?;

    Ok(())
}
