use num_traits::Float;

use crate::attractors::Attractor;

#[derive(Clone, Copy)]
pub struct CliffordAttractor<T> {
    pub point: [T; 2],
    a: T,
    b: T,
    c: T,
    d: T,
}

impl<T> CliffordAttractor<T> {
    pub fn new(point: [T; 2], a: T, b: T, c: T, d: T) -> Self {
        Self { point, a, b, c, d }
    }
}

impl<T: Float> Attractor<T, 2> for CliffordAttractor<T> {
    fn step(&mut self) {
        let x = self.point[0];
        let y = self.point[1];

        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = self.d;

        self.point = [
            (a * y).sin() + c * (a * x).cos(),
            (b * x).sin() + d * (b * y).cos(),
        ];
    }

    fn get(&self) -> [T; 2] {
        self.point
    }
}
