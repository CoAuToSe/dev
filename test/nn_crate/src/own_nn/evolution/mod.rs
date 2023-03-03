mod connection;
mod neuron;

use self::connection::Connection;
use self::neuron::Neuron;
use NeuralNetwork;
use rand::{ Rng, thread_rng, ThreadRng, random };
use std::f64::{ INFINITY, NEG_INFINITY };

pub struct Evolver {
    nets: Vec<(NeuralNetwork<Neuron>, f64)>,
    pub mutation_rate: f64,
    pub max_size: usize
}

macro_rules! pick {
    ($a:expr, $b:expr, $rng:ident, $mr:ident, $mid:ident) => {{
        let mut r: f64 = $rng.gen();
        if r < $mid {
            if r < $mr {
                r = $rng.gen::<f64>();
            }
            else {
                r = $a;
            }
        }
        else {
            r = $b;
        }
        r
    }}
}

impl Evolver {
    pub fn new(input_count: usize, hidden_counts: &[usize], output_count: usize,
               max_size: usize, mutation_rate: f64,
               hidden_activation: fn(f64) -> f64, output_activation: fn(f64) -> f64) -> Self {
        Evolver {
            nets: (0usize..max_size)
                .map(|_|(NeuralNetwork::new(input_count, hidden_counts, output_count, hidden_activation, output_activation), 0.0))
                .collect(),
            mutation_rate,
            max_size
        }
    }

    pub fn evolve(&mut self) {
        let mut rng = thread_rng();

        // Get the min and the max rewards
        let best_net: NeuralNetwork<Neuron>;
        let mut min_fit = INFINITY;
        {
            let mut max_index = 0usize;
            let mut max_fit = NEG_INFINITY;
            for (i, net) in self.nets.iter().enumerate() {
                if net.1 < min_fit {
                    min_fit = net.1;
                }
                if net.1 > max_fit {
                    max_fit = net.1;
                    max_index = i;
                }
            }
            best_net = self.nets[max_index].0.clone();
        }

        // Get the sum and add the min_fit to all of the
        let sum = self.nets.iter_mut()
            .fold(0.0, |acc, net| {
                net.1 += min_fit;
                acc + net.1
            });

        self.nets = (0usize..(self.max_size - 1usize)).map(|_| {
            let net_a: &NeuralNetwork<Neuron> = Evolver::pick_network(&self.nets, sum);
            let net_b: &NeuralNetwork<Neuron> = Evolver::pick_network(&self.nets, sum);
            let mid = (1.0 - self.mutation_rate) / 2.0;
            (NeuralNetwork {
                input: Evolver::mate_layers(&net_a.input, &net_b.input, self.mutation_rate, mid, &mut rng),
                hidden: net_a.hidden.iter().zip(net_b.hidden.iter())
                    .map(|(a, b)| Evolver::mate_layers(a, b, self.mutation_rate, mid, &mut rng))
                    .collect(),
                output: Evolver::mate_layers(&net_a.output, &net_b.output, self.mutation_rate, mid, &mut rng),
                hidden_activation: net_a.hidden_activation,
                output_activation: net_a.output_activation
            }, 0.0)
        }).collect();
        self.nets.push((best_net, 0.0));
    }
    fn mate_layers(layer_a: &Vec<Neuron>, layer_b: &Vec<Neuron>, mutation_rate: f64, mid: f64, rng: &mut ThreadRng) -> Vec<Neuron> {
        layer_a.iter().zip(layer_b.iter())
            .map(|(n_a, n_b)|
                Neuron {
                    bias: pick!(n_a.bias, n_b.bias, rng, mutation_rate, mid),
                    output: 0.0,
                    connections: n_a.connections.iter().zip(n_b.connections.iter())
                        .map(|(c_a, c_b)| Connection {
                            weight: pick!(c_a.weight, c_b.weight, rng, mutation_rate, mid)
                        })
                        .collect()
                }
            ).collect()
    }
    fn pick_network<'a>(nets: &'a Vec<(NeuralNetwork<Neuron>, f64)>, sum: f64) -> &'a NeuralNetwork<Neuron> {
        let mut sum = sum;
        let rand: f64 = random();
        for net in nets.iter() {
            sum -= net.1;
            if sum <= rand {
                return &net.0;
            }
        }
        &nets.first().unwrap().0
    }

    pub fn pulse(&mut self, index: usize, input: &[f64]) -> Vec<f64> {
        self.nets[index].0.pulse(input)
    }

    pub fn get_fitness(&self, index: usize) -> f64 {
        self.nets[index].1
    }
    pub fn set_fitness(&mut self, index: usize, x: f64) {
        self.nets[index].1 = x;
    }
    pub fn increment_fitness(&mut self, index: usize, x: f64) {
        self.nets[index].1 += x;
    }

    pub fn len(&self) -> usize {
        self.nets.len()
    }
}