use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use clap::{Args as ClapArgs, Parser, Subcommand};
use indicatif::ProgressBar;
use strange_attractor::attractors::{
    clifford::CliffordAttractor, henon::HenonAttractor, ikeda::IkedaAttractor,
    lorenz::LorenzAttractor, Attractor,
};

mod canvas;
use canvas::{HitCanvas, PositionCalculator};

#[derive(Parser, Debug)]
#[command(name = "Strange Attractor")]
#[command(about = "Generate beautiful strange attractor visualizations", long_about = None)]
struct Cli {
    /// Pixels per unit (higher = more detail)
    #[arg(short, long, default_value_t = 10000.0, global = true)]
    pixels_per_unit: f64,

    /// Number of iterations to compute
    #[arg(short, long, default_value_t = 1_000_000_000, global = true)]
    iterations: u64,

    /// Output image path
    #[arg(short, long, default_value = "attractor.png", global = true)]
    output: PathBuf,

    #[command(subcommand)]
    attractor: AttractorCommand,
}

#[derive(Subcommand, Debug)]
enum AttractorCommand {
    Clifford(CliffordArgs),
    Henon(HenonArgs),
    Ikeda(IkedaArgs),
    Lorenz(LorenzArgs),
}

#[derive(ClapArgs, Debug)]
struct CliffordArgs {
    #[arg(long, default_value_t = 1.0, allow_negative_numbers = true)]
    x: f64,

    #[arg(long, default_value_t = 1.0, allow_negative_numbers = true)]
    y: f64,

    #[arg(long, default_value_t = -1.7, allow_negative_numbers = true)]
    a: f64,

    #[arg(long, default_value_t = 1.3, allow_negative_numbers = true)]
    b: f64,

    #[arg(long, default_value_t = -0.1, allow_negative_numbers = true)]
    c: f64,

    #[arg(long, default_value_t = -1.2, allow_negative_numbers = true)]
    d: f64,
}

#[derive(ClapArgs, Debug)]
struct HenonArgs {
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    x: f64,

    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    y: f64,

    #[arg(long, default_value_t = 1.4, allow_negative_numbers = true)]
    a: f64,

    #[arg(long, default_value_t = 0.3, allow_negative_numbers = true)]
    b: f64,
}

#[derive(ClapArgs, Debug)]
struct IkedaArgs {
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    x: f64,

    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    y: f64,

    #[arg(long, default_value_t = 0.8, allow_negative_numbers = true)]
    u: f64,
}

#[derive(ClapArgs, Debug)]
struct LorenzArgs {
    #[arg(long, default_value_t = 1.0, allow_negative_numbers = true)]
    x: f64,

    #[arg(long, default_value_t = 1.0, allow_negative_numbers = true)]
    y: f64,

    #[arg(long, default_value_t = 1.0, allow_negative_numbers = true)]
    z: f64,

    #[arg(long, default_value_t = 10.0, allow_negative_numbers = true)]
    sigma: f64,

    #[arg(long, default_value_t = 28.0, allow_negative_numbers = true)]
    rho: f64,

    #[arg(long, default_value_t = 2.6667, allow_negative_numbers = true)]
    beta: f64,
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
    let args = Cli::parse();

    match args.attractor {
        AttractorCommand::Clifford(config) => run_2d_attractor(
            CliffordAttractor::<f64>::new(
                [config.x, config.y],
                config.a,
                config.b,
                config.c,
                config.d,
            ),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
        AttractorCommand::Henon(config) => run_2d_attractor(
            HenonAttractor::<f64>::new([config.x, config.y], config.a, config.b),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
        AttractorCommand::Ikeda(config) => run_2d_attractor(
            IkedaAttractor::<f64>::new([config.x, config.y], config.u),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
        AttractorCommand::Lorenz(config) => run_2d_attractor(
            LorenzAttractor::<f64>::new(
                [config.x, config.y, config.z],
                config.sigma,
                config.rho,
                config.beta,
            ),
            args.pixels_per_unit,
            args.iterations,
            &args.output,
        ),
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

    let progress_update_interval = (iterations / 100_000).max(1);
    let progress = ProgressBar::new(iterations);

    for i in 0..iterations {
        attractor.step();

        let point = attractor.get();
        canvas.record(&point);

        if i % progress_update_interval == 0 {
            progress.set_position(i);
        }
    }

    progress.set_position(iterations);
    progress.finish();

    // save as PNG
    let img = canvas.to_gray_image_16();
    img.save(output_path).unwrap();
    println!("Saved attractor image to {}", output_path.display());
}
