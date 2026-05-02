use num_traits::Float;

use crate::attractors::Attractor;

#[derive(Clone, Copy)]
pub struct LorenzAttractor<T> {
    pub point: [T; 3],
    sigma: T,
    rho: T,
    beta: T,
}

impl<T> LorenzAttractor<T> {
    pub fn new(point: [T; 3], sigma: T, rho: T, beta: T) -> Self {
        Self {
            point,
            sigma,
            rho,
            beta,
        }
    }
}

impl<T: Float> LorenzAttractor<T> {
    pub fn step(&mut self) {
        let x = self.point[0];
        let y = self.point[1];
        let z = self.point[2];

        let dt = T::from(0.01).unwrap();
        let sigma = self.sigma;
        let rho = self.rho;
        let beta = self.beta;

        let dx = sigma * (y - x);
        let dy = x * (rho - z) - y;
        let dz = x * y - beta * z;

        self.point[0] = x + dx * dt;
        self.point[1] = y + dy * dt;
        self.point[2] = z + dz * dt;
    }

    pub fn get(&self) -> [T; 3] {
        self.point
    }

    pub fn get_2d(&self) -> [T; 2] {
        [self.point[0], self.point[1]]
    }
}

impl<T: Float> Attractor<T, 2> for LorenzAttractor<T> {
    fn step(&mut self) {
        LorenzAttractor::step(self);
    }

    fn get(&self) -> [T; 2] {
        self.get_2d()
    }
}
