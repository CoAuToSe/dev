use super::connection_old::Connection;
// use fast_io::CustomIO;
use std::fmt::Debug;

pub trait Neuron: Clone + Debug + PartialEq /*+ CustomIO*/ {
    type C: Connection + Sized;
    fn new(prev_layer_size: usize) -> Self;

    fn bias(&self) -> f64;
    fn output(&self) -> f64;
    fn set_output(&mut self, output: f64);
    fn connections(&mut self) -> &mut Vec<Self::C>;
}

pub fn rand_weight() -> f64 {
    use rand::{thread_rng, Rng};
    thread_rng().gen_range(-0.33, 0.33)
}
