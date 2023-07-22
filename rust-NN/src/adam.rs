#[derive(Debug, Clone)]
pub struct Neuron {
    pub weights: Vec<f64>,
    pub bias: f64,
    pub delta: f64,
    pub m: Vec<f64>,
    pub v: Vec<f64>,
    pub bias_m: f64,
    pub bias_v: f64,
}

impl Neuron {
    fn new(num_inputs: usize) -> Self {
        let weights = (0..num_inputs)
            .map(|_| rand::random::<f64>() * 2.0 - 1.0)
            .collect();
        let bias = rand::random::<f64>() * 2.0 - 1.0;
        let delta = 0.0;
        let m = vec![0.0; num_inputs];
        let v = vec![0.0; num_inputs];
        let bias_m = 0.0;
        let bias_v = 0.0;
        Neuron {
            weights,
            bias,
            delta,
            m,
            v,
            bias_m,
            bias_v,
        }
    }

    pub fn adjust_weights(
        &mut self,
        input: &[f64],
        delta: f64,
        learning_rate: f64,
        beta1: f64,
        beta2: f64,
        epsilon: f64,
        t: usize,
    ) {
        for i in 0..self.weights.len() {
            self.m[i] = beta1 * self.m[i] + (1.0 - beta1) * input[i] * delta;
            self.v[i] = beta2 * self.v[i] + (1.0 - beta2) * (input[i] * delta).powi(2);

            let m_hat = self.m[i] / (1.0 - beta1.powi(t as i32));
            let v_hat = self.v[i] / (1.0 - beta2.powi(t as i32));

            self.weights[i] -= learning_rate * m_hat / (v_hat.sqrt() + epsilon);
        }
        self.bias -= delta * learning_rate;
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
impl NeuralNetwork {
    fn train(
        &mut self,
        dataset: &[Vec<f64>],
        targets: &[Vec<f64>],
        learning_rate: f64,
        epochs: usize,
        beta1: f64,
        beta2: f64,
        epsilon: f64,
    ) {
        for epoch in 0..epochs {
            for (input, target) in dataset.iter().zip(targets) {
                self.backpropagate(
                    input,
                    target,
                    learning_rate,
                    beta1,
                    beta2,
                    epsilon,
                    epoch + 1,
                );
            }
        }
    }
}

#[test]
fn testing_adam() {
    let dataset: Vec<Vec<f64>> = vec![
        vec![0.0],
        vec![0.1],
        vec![0.2],
        vec![0.3],
        vec![0.4],
        vec![0.5],
    ];
    let targets: Vec<Vec<f64>> = vec![
        vec![0.0],
        vec![0.2],
        vec![0.4],
        vec![0.6],
        vec![0.8],
        vec![1.0],
    ];

    let mut nn = new(&[1, 4, 1]);

    let learning_rate = 0.01;
    let epochs = 1000;
    let beta1 = 0.9;
    let beta2 = 0.999;
    let epsilon = 1e-8;

    println!("{:#?}", nn);
    nn.train(
        &dataset,
        &targets,
        learning_rate,
        epochs,
        beta1,
        beta2,
        epsilon,
    );
    println!("{:#?}", nn);
    println!("{:?}", nn.forward(&[0.5]));
    println!("{:?}", nn.forward(&[0.2]));
    println!("{:?}", nn.forward(&[0.3]));
}

#[test]
fn testing_main_2023_07_13_15_53() {
    let dataset: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets: Vec<Vec<f64>> = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let mut nn = new(&[2, 2, 1]);
    let learning_rate = 0.01;
    let epochs = 5000;
    let beta1 = 0.9;
    let beta2 = 0.999;
    let epsilon = 1e-8;

    println!("{:#?}", nn);

    nn.train(
        &dataset,
        &targets,
        learning_rate,
        epochs,
        beta1,
        beta2,
        epsilon,
    );
    println!("{:#?}", nn);
    println!("{:?}", nn.forward(&[0.0, 0.0]));
    println!("{:?}", nn.forward(&[0.0, 1.0]));
    println!("{:?}", nn.forward(&[1.0, 0.0]));
    println!("{:?}", nn.forward(&[1.0, 1.0]));
}

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    layers: Vec<Vec<Neuron>>,
}

pub fn new(sizes: &[usize]) -> NeuralNetwork {
    let mut layers = Vec::new();
    for i in 1..sizes.len() {
        layers.push((0..sizes[i]).map(|_| Neuron::new(sizes[i - 1])).collect());
    }
    NeuralNetwork { layers }
}

impl NeuralNetwork {
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut input = input.to_vec();
        for layer in &self.layers {
            input = layer.iter().map(|neuron| neuron.compute(&input)).collect();
        }
        input
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
impl NeuralNetwork {
    pub fn backpropagate(
        &mut self,
        input: &[f64],
        target: &[f64],
        learning_rate: f64,
        beta1: f64,
        beta2: f64,
        epsilon: f64,
        t: usize,
    ) {
        let mut output = input.to_vec();
        let mut all_outputs = vec![output.clone()];
        for layer in &mut self.layers {
            output = layer
                .iter_mut()
                .map(|neuron| neuron.compute(&output))
                .collect();
            all_outputs.push(output.clone());
        }

        let last_layer_index = self.layers.len() - 1;
        for (o, t) in output.iter().zip(target) {
            for neuron in self.layers[last_layer_index].iter_mut() {
                let deriv = neuron.derivative(*o);
                let delta = deriv * (t - o);
                neuron.update_delta(delta);
            }
        }

        for i in (0..last_layer_index).rev() {
            let outputs_prev_layer = &all_outputs[i];
            let layer_bis = &self.layers[i].clone();
            let next_layer_bis = &self.layers[i + 1].clone();
            for (neuron_index, (o, neuron)) in outputs_prev_layer
                .iter()
                .zip(self.layers[i].iter_mut())
                .enumerate()
            {
                let mut delta = 0.0;
                for next_neuron in next_layer_bis {
                    let w = next_neuron.weights[neuron_index]; // get the specific weight
                    let next_delta = next_neuron.get_delta();
                    delta += w * next_delta;
                }
                let deriv = neuron.derivative(*o);
                delta *= deriv;
                neuron.update_delta(delta);
            }
        }

        for (i, layer) in self.layers.iter_mut().enumerate().rev() {
            let input = &all_outputs[i];
            for neuron in layer.iter_mut() {
                let delta = neuron.get_delta();
                neuron.adjust_weights(input, delta, learning_rate, beta1, beta2, epsilon, t);
            }
        }
    }
}
