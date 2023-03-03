// use fast_io::CustomIO;
use std::fmt::Debug;

pub trait Connection: Clone + Debug + PartialEq /*+ CustomIO*/ {
    fn new(weight: f64) -> Self;

    fn weight(&self) -> f64;
}
