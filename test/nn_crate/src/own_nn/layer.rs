use crate::neuron::Neuron;

pub struct Layer<'a> {
    len_in: usize,
    len_out: usize,
    neurons_weights: Box<[Neuron<'a>]>,
}

impl<'a> Layer<'a> {
    fn new(len_in: usize, len_out: usize, neurons_weights: Box<[Neuron<'a>]>) -> Self {
        Layer {
            len_in,
            len_out,
            neurons_weights,
        }
    }

    pub fn back_propagate_gradiant(&mut self) {
        for neurons in self.neurons_weights.as_mut() {
            if let Some(_) = neurons.gradiant {
                neurons.back_propagate_gradiant()
            } else {
                unreachable!("at least one neurone of the layer doesn't have a gradiant")
            }
        }
    }

    pub fn forward(&mut self) {
        for neuron in self.neurons_weights.iter_mut() {
            neuron.sum();
        }

        // while let Some(neuron) = self.neurons_weights.iter_mut().next() {
        //     neuron.sum();
        // }

        // let mut index = 0;
        // while let Some(neuron) = self.neurons_weights.get_mut(index) {
        //     neuron.sum();
        //     index += 1;
        // }
    }
}
