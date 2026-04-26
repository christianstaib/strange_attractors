use num_traits::Float;

use crate::attractors::Attractor;

#[derive(Clone, Copy)]
pub struct IkedaAttractor<T> {
    pub point: [T; 2],
    u: T,
}

impl<T> IkedaAttractor<T> {
    pub fn new(point: [T; 2], u: T) -> Self {
        Self { point, u }
    }
}

impl<T: Float> Attractor<T, 2> for IkedaAttractor<T> {
    fn step(&mut self) {
        let x = self.point[0];
        let y = self.point[1];

        let u = self.u;

        let r_sq = x * x + y * y;
        let t = T::from(0.4).unwrap() - T::from(6.0).unwrap() / (T::one() + r_sq);

        let cos_t = t.cos();
        let sin_t = t.sin();

        self.point = [
            T::one() + u * (x * cos_t - y * sin_t),
            u * (x * sin_t + y * cos_t),
        ];
    }

    fn get(&self) -> [T; 2] {
        self.point
    }
}
