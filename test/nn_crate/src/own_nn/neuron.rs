use crate::{
    connection::{self, Connection},
    neural_ops::NeuralOps,
};

pub struct Neuron<'a> {
    pub prev_delta: f64,
    pub sum: Option<f64>,
    pub error: Option<f64>,
    pub output: Option<f64>,
    pub gradiant: Option<f64>,
    pub active: &'a NeuralOps<f64>,
    pub connections: Box<[Connection<'a>]>,
}

impl Neuron<'_> {
    pub fn reset(&mut self) {
        self.sum = None;
        self.output = None;
        self.error = None;
        self.gradiant = None;
    }

    pub fn sum(&mut self) -> f64 {
        if let Some(sum) = self.sum {
            sum
        } else {
            let mut sum = 0.;
            for connection in self.connections.as_mut() {
                sum += connection.weight * connection.from.output();
            }
            self.sum = Some(sum);
            sum
        }
    }

    pub fn output(&mut self) -> f64 {
        if let Some(output) = self.output {
            output
        } else {
            let output = (self.active.activation)(self.sum());
            self.output = Some(output);
            output
        }
    }

    pub fn known_error_with_grad(
        &mut self,
        value_wanted: f64,
        loss: fn(f64, f64) -> f64,
        deriv_loss: fn(f64, f64) -> f64,
    ) {
        self.error = Some(loss(value_wanted, self.output()));
        self.gradiant = Some(deriv_loss(value_wanted, self.output()))
    }

    pub fn error(&mut self) -> f64 {
        if let Some(error) = self.error {
            error
        } else {
            unreachable!("wants to get the error of a neuron without its error known")
        }
    }

    pub fn back_propagate_gradiant(&mut self) {
        if let Some(neuron_error) = self.error {
            for connection in self.connections.as_mut() {
                connection.error(neuron_error)
            }
        } else {
            unreachable!("trying to back propagate the error of a neuron without a known error")
        }
    }

    // fn learn(&mut self, current_error: Option<f64>) {
    //     let neuron_error;
    //     if let Some(error) = current_error {
    //         // if the error is not recurrent
    //         neuron_error = error;
    //     } else {
    //         // if the error is only propagate
    //         neuron_error = {
    //             if let Some(calculated_error) = self.error {
    //                 calculated_error
    //             } else {
    //                 0.0
    //             }
    //         }
    //     }
    //     while let Some(_) = self.connections.into_iter().next() {
    //         unreachable!("should never happened: some Connection still have some error in them");
    //     }

    //     for connection in self.connections.as_mut() {
    //         let neuron_error = (self.active.derivative)(neuron_error);
    //     }
    // }
}
