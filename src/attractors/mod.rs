pub mod clifford;

pub trait Attractor<T, const N: usize> {
    fn step(&mut self);

    fn get(&self) -> [T; N];
}
