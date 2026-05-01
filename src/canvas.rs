use image::{ImageBuffer, Luma};
use num_traits::{Float, ToPrimitive};

pub type GrayImage16 = ImageBuffer<Luma<u16>, Vec<u16>>;

pub struct PositionCalculator<F> {
    x_min: F,
    _x_max: F,
    _y_min: F,
    y_max: F,
    pixels_per_unit: F,
    width: usize,
    height: usize,
}

impl<F: Float + ToPrimitive> PositionCalculator<F> {
    pub fn new(x_min: F, x_max: F, y_min: F, y_max: F, pixels_per_unit: F) -> Self {
        let width = ((x_max - x_min) * pixels_per_unit)
            .ceil()
            .to_usize()
            .expect("invalid image width");

        let height = ((y_max - y_min) * pixels_per_unit)
            .ceil()
            .to_usize()
            .expect("invalid image height");

        Self {
            x_min,
            _x_max: x_max,
            _y_min: y_min,
            y_max,
            pixels_per_unit,
            width,
            height,
        }
    }

    fn point_to_pixel(&self, point: &[F; 2]) -> Option<usize> {
        let x = point[0];
        let y = point[1];

        if !x.is_finite() || !y.is_finite() {
            return None;
        }

        let col = ((x - self.x_min) * self.pixels_per_unit)
            .floor()
            .to_isize()?;

        let row = ((self.y_max - y) * self.pixels_per_unit)
            .floor()
            .to_isize()?;

        if !(0..self.height as isize).contains(&row) {
            return None;
        }

        if !(0..self.width as isize).contains(&col) {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        Some(row * self.width + col)
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

pub struct HitCanvas<F> {
    positions: PositionCalculator<F>,
    counts: Vec<u32>,
}

impl<F: Float + ToPrimitive> HitCanvas<F> {
    pub fn new(positions: PositionCalculator<F>) -> Self {
        let size = positions.width() * positions.height();

        Self {
            positions,
            counts: vec![0; size],
        }
    }

    pub fn record(&mut self, point: &[F; 2]) {
        if let Some(pixel) = self.positions.point_to_pixel(point) {
            self.counts[pixel] = self.counts[pixel].saturating_add(1);
        }
    }

    fn width(&self) -> usize {
        self.positions.width()
    }

    fn height(&self) -> usize {
        self.positions.height()
    }

    fn max_count(&self) -> u32 {
        self.counts.iter().copied().max().unwrap_or(1)
    }

    fn count_at(&self, row: usize, col: usize) -> u32 {
        self.counts[row * self.width() + col]
    }

    pub fn to_gray_image_16(&self) -> GrayImage16 {
        let width = self.width();
        let height = self.height();
        let max_count = self.max_count();

        let mut img = GrayImage16::new(width as u32, height as u32);

        for row in 0..height {
            for col in 0..width {
                let count = self.count_at(row, col);
                let value = log_scale_count_u16(count, max_count);

                img.put_pixel(col as u32, row as u32, Luma([value]));
            }
        }

        img
    }
}

fn log_scale_count_u16(count: u32, max_count: u32) -> u16 {
    if max_count == 0 {
        return 0;
    }

    let scaled = ((count as f64 + 1.0).ln() / (max_count as f64 + 1.0).ln()) * u16::MAX as f64;

    scaled.round() as u16
}
