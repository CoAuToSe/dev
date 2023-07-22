#[derive(Debug, Clone)]
pub struct Neuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub delta: f64, // Nouveau champ pour stocker le delta
}

impl Neuron {
    pub fn new(num_inputs: usize) -> Self {
        let weights = (0..num_inputs)
            .map(|_| rand::random::<f64>() * 2.0 - 1.0)
            .collect();
        let bias = rand::random::<f64>() * 2.0 - 1.0;
        Neuron {
            weights,
            bias,
            delta: 0.0,
        }
    }
    pub fn activate(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
        // if x > 0.0 {
        //     x
        // } else {
        //     0.0
        // }
    }

    pub fn derivative(&self, x: f64) -> f64 {
        let y = self.activate(x);
        y * (1.0 - y)
        // if x > 0.0 {
        //     1.0
        // } else {
        //     0.0
        // }
    }

    // Nouvelle méthode pour mettre à jour le delta
    pub fn update_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    // Nouvelle méthode pour obtenir le delta
    pub fn get_delta(&self) -> f64 {
        self.delta
    }
}

pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

impl Neuron {
    pub fn compute(&self, input: &[f64]) -> f64 {
        assert_eq!(self.weights.len(), input.len());
        self.activate(
            self.weights
                .iter()
                .zip(input)
                .map(|(a, b)| a * b)
                .sum::<f64>()
                + self.bias,
        )
    }
}

impl Neuron {
    pub fn adjust_weights(&mut self, input: &[f64], delta: f64, learning_rate: f64) {
        for i in 0..self.weights.len() {
            self.weights[i] -= input[i] * delta * learning_rate;
        }
        self.bias -= delta * learning_rate;
    }
}
