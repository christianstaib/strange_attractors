pub mod clifford;
pub mod henon;
pub mod ikeda;

pub trait Attractor<T, const N: usize> {
    fn step(&mut self);

    fn get(&self) -> [T; N];
}
