use anyhow::Result;
use opener::open;
use plotters::prelude::*;
use rand::Rng;
//use std::env;
use std::path::Path;

pub fn graphit(timepl: Vec<(f64, f64)>, optype: &str) -> Result<()> {
    // Infer axis ranges from data
    let (x_min, x_max) = timepl
        .iter()
        .map(|(x, _)| *x)
        .fold((f64::MAX, f64::MIN), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let (y_min, y_max) = timepl
        .iter()
        .map(|(_, y)| *y)
        .fold((f64::MAX, f64::MIN), |(min, max), val| {
            (min.min(val), max.max(val))
        });

    let file_path = Path::new("Graph.png");
    let mut title = String::from("Graph");
    if file_path.exists() {
        let mut randoms = rand::rng();
        let rndint: u16 = randoms.random_range(0..1000);
        println!("{}", rndint);
        title.push_str(rndint.to_string().as_str());
        title.push_str(".png");
        println!("{}", title)
    } else {
        title.push_str(".png");
        println!("{}", title);
    }

    let root = BitMapBackend::new(&title, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} over time", optype), ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max + 1 as f64)?;

    chart
        .configure_mesh()
        .x_desc("Time (minutes)")
        .y_desc(match optype {
            "Ping" => "Ping (ms)",
            "Loss" => "Loss (%)",
            _ => "Value",
        })
        .draw()?;

    chart.draw_series(LineSeries::new(timepl, &RED))?;
    open(&title)?;

    Ok(())
}
