use crate::{layer::*, neuron_old::Neuron};

pub struct NeuralNetwork<'a, const LENIN: usize, const LENOUT: usize, N: Neuron, T> {
    pub input: [N; LENIN],
    // pub hidden: Vec<Layer<Hidden<'a, T>>>,
    pub output: [N; LENOUT],
    aze: &'a T,
}

// impl<const LENIN: usize, const LENOUT: usize, N: Neuron> NeuralNetwork<LENIN, LENOUT, N> {
//     pub fn new(
//         input_count: usize,
//         hidden_counts: &[usize],
//         output_count: usize,
//         hidden_activation: fn(f64) -> f64,
//         output_activation: fn(f64) -> f64,
//     ) -> Self {
//         // Input layer
//         let input = NeuralNetwork::<LENIN, LENOUT, N>::make_layer(input_count, 0);
//         let mut prev_count = input_count;

//         // Hidden layers
//         let mut hidden: Vec<Vec<N>> = Vec::with_capacity(hidden_counts.len());
//         for count in hidden_counts.iter() {
//             hidden.push(NeuralNetwork::<LENIN, LENOUT, N>::make_layer(
//                 *count, prev_count,
//             ));
//             prev_count = *count;
//         }

//         // Output layer
//         let output = NeuralNetwork::<LENIN, LENOUT, N>::make_layer(output_count, prev_count);

//         NeuralNetwork {
//             input,
//             hidden,
//             output,
//             hidden_activation,
//             output_activation,
//         }
//     }
//     fn make_layer(count: usize, prev_count: usize) -> Vec<N> {
//         let mut result = Vec::with_capacity(count);
//         for _ in 0..count {
//             result.push(N::new(prev_count));
//         }
//         result
//     }

//     pub fn pulse(&mut self, input: &[f64]) -> Vec<f64> {
//         // Set the output of the input layer
//         for (i, n) in input.iter().zip(self.input.iter_mut()) {
//             n.set_output(*i);
//         }

//         let mut prev_layer = &self.input;

//         // Pulse each hidden layer
//         for layer in self.hidden.iter_mut() {
//             NeuralNetwork::<LENIN, LENOUT, N>::pulse_layer(
//                 layer,
//                 prev_layer,
//                 self.hidden_activation,
//             );
//             prev_layer = layer;
//         }

//         // Pulse the output layer
//         NeuralNetwork::<LENIN, LENOUT, N>::pulse_layer(
//             &mut self.output,
//             prev_layer,
//             self.output_activation,
//         );
//         self.output.iter().map(|n| n.output()).collect()
//     }
//     fn pulse_layer(layer: &mut Vec<N>, prev_layer: &Vec<N>, activation: fn(f64) -> f64) {
//         for neuron in layer.iter_mut() {
//             let mut sum = neuron.bias();
//             for (c, n) in neuron.connections().iter().zip(prev_layer.iter()) {
//                 sum += c.weight() * n.output();
//             }
//             neuron.set_output(activation(sum));
//         }
//     }
// }
