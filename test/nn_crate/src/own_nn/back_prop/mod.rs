mod connection;
mod learning_parameters;
mod neuron;
pub mod prelude;
mod training_result;
pub use self::connection::Connection;
pub use self::learning_parameters::LearningParameters;
pub use self::neuron::Neuron;
pub use self::training_result::TrainingResult;

use crate::neural_network::NeuralNetwork;
use std::f64::INFINITY;
use std::time::{Duration, Instant};
// use fast_io::prelude::*;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug)]
pub struct BackProp {
    net: NeuralNetwork<Neuron>,
    pub parameters: LearningParameters,
    pub hidden_derivative: fn(f64) -> f64,
    pub output_derivative: fn(f64) -> f64,
}

impl BackProp {
    pub fn new(
        input_count: usize,
        hidden_counts: &[usize],
        output_count: usize,
        learning_rate: f64,
        momentum: f64,
        weight_decay: f64,
        hidden_activation: fn(f64) -> f64,
        hidden_derivative: fn(f64) -> f64,
        output_activation: fn(f64) -> f64,
        output_derivative: fn(f64) -> f64,
    ) -> Self {
        BackProp {
            net: NeuralNetwork::new(
                input_count,
                hidden_counts,
                output_count,
                hidden_activation,
                output_activation,
            ),
            parameters: LearningParameters {
                learning_rate,
                momentum,
                weight_decay,
            },
            hidden_derivative,
            output_derivative,
        }
    }

    pub fn pulse(&mut self, input: &[f64]) -> Vec<f64> {
        self.net.pulse(input)
    }

    pub fn back_prop(&mut self, target: &[f64]) {
        // Set the sum of the output layer
        for (n, t) in self.net.output.iter_mut().zip(target.iter()) {
            n.sum = t - n.output;
        }

        let mut layers: Vec<&mut Vec<Neuron>> = self.net.hidden.iter_mut().rev().collect();
        layers.push(&mut self.net.input);
        let mut layers = layers.iter_mut();

        let mut prev_layer = layers.next().unwrap();

        // Pulse output layer
        BackProp::back_prop_layer(
            &self.parameters,
            &mut self.net.output,
            prev_layer,
            self.output_derivative,
        );

        for layer in layers {
            // Pulse the hidden layers
            BackProp::back_prop_layer(
                &self.parameters,
                *prev_layer,
                *layer,
                self.hidden_derivative,
            );

            prev_layer = layer;
        }
    }
    fn back_prop_layer(
        parameters: &LearningParameters,
        layer: &mut Vec<Neuron>,
        prev_layer: &mut Vec<Neuron>,
        derivative: fn(f64) -> f64,
    ) {
        let mut gradient: f64;
        let mut pre_delta: f64;
        let mut delta: f64;

        for neuron in layer.iter_mut() {
            gradient = derivative(neuron.output) * neuron.sum;
            pre_delta = gradient * parameters.learning_rate;
            for (connection, prev_neuron) in
                neuron.connections.iter_mut().zip(prev_layer.iter_mut())
            {
                // Update the weight
                delta = prev_neuron.output * pre_delta;
                connection.weight += delta;
                connection.weight += parameters.momentum * connection.prev_delta;
                connection.weight *= parameters.weight_decay;

                // Add to the sum of the connected neuron
                prev_neuron.sum += gradient * connection.weight;
            }

            // Update the bias of this neuron
            neuron.bias += pre_delta;
            neuron.bias += parameters.momentum * neuron.prev_delta;
            neuron.bias *= parameters.weight_decay;
            neuron.prev_delta = pre_delta;

            // Reset the sum
            neuron.sum = 0.0;
        }
    }
    pub fn test(&mut self, input: &[f64], target: &[f64]) -> f64 {
        self.pulse(input)
            .iter()
            .zip(target.iter())
            .fold(0.0, |acc, (o, t)| acc + (o - t).powi(2))
            / target.len() as f64
    }

    pub fn train(
        &mut self,
        min_error: f64,
        max_epochs: Option<u64>,
        max_duration: Option<Duration>,
        train_inputs: &[&[f64]],
        train_targets: &[&[f64]],
        test_inputs: &[&[f64]],
        test_targets: &[&[f64]],
    ) -> TrainingResult {
        let mut rng = thread_rng();

        let mut m_error = INFINITY;
        let mut error: f64;
        let mut epochs = 0u64;

        let mut indexes: Vec<usize> = (0..train_inputs.len()).collect();

        let start = Instant::now();
        loop {
            // Train
            rng.shuffle(&mut indexes);
            for index in indexes.iter() {
                self.pulse(train_inputs[*index]);
                self.back_prop(train_targets[*index]);
            }

            // Test
            error = 0.0;
            for (input, target) in test_inputs.iter().zip(test_targets.iter()) {
                error += self.test(*input, *target);
            }
            error /= test_inputs.len() as f64;

            epochs += 1u64;

            // Break if one of the criteria in reached
            if error < m_error {
                m_error = error;
                if error <= min_error {
                    break;
                }
            }
            if let Some(e) = max_epochs {
                if e < epochs {
                    break;
                }
            }
            if let Some(d) = max_duration {
                if d > start.elapsed() {
                    break;
                }
            }
        }

        TrainingResult {
            duration: start.elapsed(),
            epochs,
            error,
            min_error: m_error,
        }
    }

    // // Writes the network to the given stream, f, at it's current position
    // pub fn save<T: CopyIO>(&self, f: &mut T) -> Result<()> {
    //     self.net.save(f)?;
    //     f.write_copy(&self.parameters)
    // }
    // pub fn load<T: CopyIO>(
    //     f: &mut T,
    //     hidden_activation: fn(f64) -> f64,
    //     hidden_derivative: fn(f64) -> f64,
    //     output_activation: fn(f64) -> f64,
    //     output_derivative: fn(f64) -> f64,
    // ) -> Result<Self> {
    //     Ok(BackProp {
    //         net: NeuralNetwork::load(f, hidden_activation, output_activation)?,
    //         parameters: f.read_copy()?,
    //         hidden_derivative,
    //         output_derivative,
    //     })
    // }
}
