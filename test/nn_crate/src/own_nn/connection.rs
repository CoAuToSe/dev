use crate::neuron::Neuron;

pub struct Connection<'a> {
    pub weight: f64,
    pub prev_delta: f64,
    pub error: Option<f64>,
    pub gradiant: Option<f64>,
    pub from: &'a mut Neuron<'a>,
}

impl Connection<'_> {
    pub fn reset(&mut self) {
        self.error = None;
        self.gradiant = None;
    }

    pub fn error(&mut self, error_output: f64) {
        if let Some(error) = self.error {
            unreachable!("trying to calculate already calculated connection error")
        } else {
            self.error = Some(self.from.output() * error_output);
        }
    }

    pub fn back_propagate(&mut self) {
        if let Some(error) = self.error {
            if let Some(output_of_input) = self.from.output {
                if let Some(sum_error_input) = self.from.error.as_mut() {
                    *sum_error_input += output_of_input * error
                } else {
                    self.from.error = Some(output_of_input * error)
                }
            } else {
                unreachable!("trying to calculate error a neuron that wasn't activated")
            }
        } else {
            unreachable!("trying to backpropagate with a connection without calculated error")
        }
    }

    pub fn back_propagate_gradiant(&mut self, gradiant_out: f64, derive_output: f64) {
        if let Some(grad) = self.gradiant.as_mut() {
            unreachable!("gradiant already known which mean that it is calculated multiple time or not reset")
        } else {
            self.gradiant = Some(gradiant_out * derive_output * self.from.output());
            if let Some(grad) = self.from.gradiant.as_mut() {
                *grad += gradiant_out * self.weight;
            } else {
                self.from.gradiant = Some(gradiant_out * self.weight);
            }
        }
    }
}
