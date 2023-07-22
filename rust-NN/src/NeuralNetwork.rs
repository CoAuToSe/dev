use crate::mods::Neuron::Neuron;

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

impl NeuralNetwork {
    pub fn backpropagate(&mut self, input: &[f64], target: &[f64], learning_rate: f64) {
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
            let next_layer_bis = &self.layers[i + 1].clone();
            for (o, neuron) in outputs_prev_layer.iter().zip(self.layers[i].iter_mut()) {
                let next_layer = next_layer_bis;
                let sum_weights = next_layer
                    .iter()
                    .map(|neuron| neuron.weights.iter().sum::<f64>())
                    .sum::<f64>(); // Use sum here
                let next_delta = next_layer
                    .iter()
                    .map(|neuron| neuron.get_delta())
                    .sum::<f64>();
                let deriv = neuron.derivative(*o);
                let delta = deriv * sum_weights * next_delta;
                neuron.update_delta(delta);
            }
        }

        for (i, layer) in self.layers.iter_mut().enumerate().rev() {
            let input = &all_outputs[i];
            for neuron in layer.iter_mut() {
                let delta = neuron.get_delta();
                neuron.adjust_weights(input, delta, learning_rate);
            }
        }
    }

    pub fn train(
        &mut self,
        dataset: &[Vec<f64>],
        targets: &[Vec<f64>],
        learning_rate: f64,
        epochs: usize,
    ) {
        for _ in 0..epochs {
            for (input, target) in dataset.iter().zip(targets) {
                self.backpropagate(input, target, learning_rate);
            }
        }
    }
}
