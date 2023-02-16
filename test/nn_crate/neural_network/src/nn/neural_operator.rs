use std::f64::consts::E;

pub trait NeuralOperator {
    fn activation(x : f64) -> f64;
    fn derivative(x : f64) -> f64;
}

pub struct Sigmoid();
pub struct Tanh();
pub struct ReLU();

impl NeuralOperator for Sigmoid {
    fn activation(x : f64) -> f64 {
        1.0 / (1.0 + E.powf(-x))
    }
    fn derivative(x : f64) -> f64 {
        x * (1.0 - x)
    }
}
impl NeuralOperator for Tanh {
    fn activation(x : f64) -> f64 {
        1.0 - 2.0 / (E.powf(2.0 * x) + 1.0)
    }
    fn derivative(x : f64) -> f64 {
        1.0 - x.powi(2)
    }
}
impl NeuralOperator for ReLU {
    fn activation(x : f64) -> f64 {
        x.max(0.0)
    }
    fn derivative(x : f64) -> f64 {
        if x > 0.0 {
            return 1.0;
        }
        else {
            return 0.0;
        }
    }
}