pub mod clifford;
pub mod henon;
pub mod ikeda;
pub mod lorenz;

pub trait Attractor<T, const N: usize> {
    fn step(&mut self);

    fn get(&self) -> [T; N];
}
