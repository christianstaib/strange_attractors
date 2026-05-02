use num_traits::Float;

use crate::attractors::Attractor;

#[derive(Clone, Copy)]
pub struct HenonAttractor<T> {
    pub point: [T; 2],
    a: T,
    b: T,
}

impl<T> HenonAttractor<T> {
    pub fn new(point: [T; 2], a: T, b: T) -> Self {
        Self { point, a, b }
    }
}

impl<T: Float> Attractor<T, 2> for HenonAttractor<T> {
    fn step(&mut self) {
        let x = self.point[0];
        let y = self.point[1];

        let a = self.a;
        let b = self.b;

        self.point = [T::one() - a * x * x + y, b * x];
    }

    fn get(&self) -> [T; 2] {
        self.point
    }
}
