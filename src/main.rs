use std::time::{Duration, Instant};

use indicatif::ProgressBar;
use rand_distr::{Distribution, Normal};
use strange_attractor::attractors::{clifford::CliffordAttractor, Attractor};

mod canvas;
use canvas::{HitCanvas, PositionCalculator};

fn calculate_bounds(attractor: &mut CliffordAttractor<f64>) -> (f64, f64, f64, f64) {
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
    let attractor = CliffordAttractor::<f64>::new([1.0, 1.0], -1.7, 1.3, -0.1, -1.2);
    let (x_min, x_max, y_min, y_max) = calculate_bounds(&mut attractor.clone());

    let mut attractor = attractor;

    let pixels_per_unit = 10_000.0;

    let pos_calc = PositionCalculator::new(x_min, x_max, y_min, y_max, pixels_per_unit);
    let mut canvas = HitCanvas::new(pos_calc);

    let steps = 100_000u64;
    let iterations = 10_000_000_000u64;
    let iterations_per_step = iterations / steps;
    let progress = ProgressBar::new(steps);

    let normal = Normal::new(0.0, 0.05).unwrap();

    for i in 0..iterations {
        attractor.step();

        let point = attractor.get();
        canvas.record(&point);

        if i % iterations_per_step == 0 {
            progress.inc(1);
        }

        if i % 1_000 == 0 {
            attractor.point[0] += normal.sample(&mut rand::rng());
            attractor.point[1] += normal.sample(&mut rand::rng());
        }
    }

    // save as PNG
    let img = canvas.to_gray_image();
    img.save("attractor.png").unwrap();
}
