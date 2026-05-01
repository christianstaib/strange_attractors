use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use clap::Parser;
use indicatif::ProgressBar;
use strange_attractor::attractors::{
    clifford::CliffordAttractor, henon::HenonAttractor, ikeda::IkedaAttractor, Attractor,
};

mod canvas;
use canvas::{HitCanvas, PositionCalculator};

#[derive(Parser, Debug)]
#[command(name = "Strange Attractor")]
#[command(about = "Generate beautiful strange attractor visualizations", long_about = None)]
struct Args {
    /// Type of attractor: clifford, henon, ikeda, or lorenz
    #[arg(value_parser = ["clifford", "henon", "ikeda", "lorenz"])]
    attractor: String,

    /// Pixels per unit (higher = more detail)
    #[arg(short, long, default_value = "10000")]
    pixels_per_unit: f64,

    /// Number of iterations to compute
    #[arg(short, long, default_value = "1000000000")]
    iterations: u64,

    /// Output image path
    #[arg(short, long, default_value = "attractor.png")]
    output: PathBuf,
}

fn calculate_bounds<A: Attractor<f64, 2> + Clone>(mut attractor: A) -> (f64, f64, f64, f64) {
    let duration = Duration::from_secs(1);
    let start = Instant::now();

    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    while start.elapsed() < duration {
        attractor.step();
        let [x, y] = attractor.get();
        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
    }

    let x_diff = x_max - x_min;
    let y_diff = y_max - y_min;

    x_min -= x_diff / 100.0;
    x_max += x_diff / 100.0;
    y_min -= y_diff / 100.0;
    y_max += y_diff / 100.0;

    (x_min, x_max, y_min, y_max)
}

fn main() {
    let args = Args::parse();

    match args.attractor.as_str() {
        "clifford" => run_2d_attractor(
            CliffordAttractor::<f64>::new([1.0, 1.0], -1.7, 1.3, -0.1, -1.2),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
        "henon" => run_2d_attractor(
            HenonAttractor::<f64>::new([0.0, 0.0], 1.4, 0.3),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
        "ikeda" => run_2d_attractor(
            IkedaAttractor::<f64>::new([0.0, 0.0], 0.8),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
        _ => eprintln!("Unknown attractor type: {}", args.attractor),
    }
}

fn run_2d_attractor<A: Attractor<f64, 2> + Clone>(
    attractor: A,
    pixels_per_unit: f64,
    iterations: u64,
    output_path: &Path,
) {
    let (x_min, x_max, y_min, y_max) = calculate_bounds(attractor.clone());

    let mut attractor = attractor;

    let pos_calc = PositionCalculator::new(x_min, x_max, y_min, y_max, pixels_per_unit);
    let mut canvas = HitCanvas::new(pos_calc);

    let steps = 100_000u64;
    let iterations_per_step = iterations / steps;
    let progress = ProgressBar::new(steps);

    for i in 0..iterations {
        attractor.step();

        let point = attractor.get();
        canvas.record(&point);

        if i % iterations_per_step == 0 {
            progress.inc(1);
        }
    }

    progress.finish();

    // save as PNG
    let img = canvas.to_gray_image_16();
    img.save(output_path).unwrap();
    println!("Saved attractor image to {}", output_path.display());
}
