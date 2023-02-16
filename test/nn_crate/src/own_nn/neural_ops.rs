//! represent all function used to calculate the output of a neuron
#[allow(unused)]
pub struct NeuralOps<T> {
    pub activation: fn(T) -> T,
    pub derivative: fn(T) -> T,
}

pub const SIGMOID: NeuralOps<f64> = NeuralOps {
    activation: |x: f64| 1.0 / (1.0 + std::f64::consts::E.powf(-x)),
    derivative: |x: f64| x * (1.0 - x),
};

pub const TANH: NeuralOps<f64> = NeuralOps {
    activation: |x: f64| 1.0 - 2.0 / (std::f64::consts::E.powf(2.0 * x) + 1.0),
    derivative: |x: f64| 1.0 - x.powi(2),
};

pub const RELU: NeuralOps<f64> = NeuralOps {
    activation: |x: f64| x.max(0.0),
    derivative: |x: f64| {
        if x > 0.0 {
            return 1.0;
        } else {
            return 0.0;
        }
    },
};
