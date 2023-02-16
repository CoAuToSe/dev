use super::Connection;
use connection::Connection as C;
use neuron::{rand_weight, Neuron as N};
// use fast_io::prelude::*;

#[derive(Clone, Debug)]
pub struct Neuron {
    pub bias: f64,
    pub output: f64,
    pub connections: Vec<Connection>,
}

impl CustomIO for Neuron {
    fn save<T: CopyIO>(&self, f: &mut T) -> Result<()> {
        f.write_copy(&self.bias)?;
        self.connections.save(f)
    }
    fn load<T: CopyIO>(f: &mut T) -> Result<Self> {
        Ok(Neuron {
            bias: f.read_copy()?,
            output: 0.0,
            connections: Vec::load(f)?,
        })
    }
}

impl PartialEq for Neuron {
    fn eq(&self, rhs: &Self) -> bool {
        self.bias == rhs.bias && self.connections == rhs.connections
    }
}

impl N for Neuron {
    type C = Connection;

    fn new(prev_count: usize) -> Neuron {
        let mut connections: Vec<Connection> = Vec::with_capacity(prev_count);
        for _ in 0..prev_count {
            connections.push(Connection::new(rand_weight()));
        }
        Neuron {
            bias: rand_weight(),
            output: 0.0,
            connections,
        }
    }

    fn bias(&self) -> f64 {
        self.bias
    }
    fn output(&self) -> f64 {
        self.output
    }
    fn set_output(&mut self, output: f64) {
        self.output = output;
    }
    fn connections(&mut self) -> &mut Vec<Self::C> {
        &mut self.connections
    }
}
