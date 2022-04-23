use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
struct LayerNeuron<T, const FROM: usize, const INSIDE: usize> {
    weights: [[T; FROM]; INSIDE],
    neurons: [Option<T>; INSIDE],
    activation_function: fn(T) -> T,
    error_function: fn(T) -> T,
}

impl<'a, T: Default + Copy, const FROM: usize, const INSIDE: usize> Default
    for LayerNeuron<T, FROM, INSIDE>
{
    fn default() -> Self {
        LayerNeuron {
            weights: [[T::default(); FROM]; INSIDE],
            neurons: [None; INSIDE],
            activation_function: |_| T::default(),
            error_function: |_| T::default(),
        }
    }
}

impl<'a, T: Copy, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {
    fn new(activation_function: fn(T) -> T, error_function: fn(T) -> T) -> Self
    where
        T: Default,
    {
        LayerNeuron {
            weights: [[T::default(); FROM]; INSIDE],
            neurons: [None; INSIDE],
            activation_function,
            error_function,
        }
    }
    fn new_val(val: T, activation_function: fn(T) -> T, error_function: fn(T) -> T) -> Self {
        LayerNeuron {
            weights: [[val; FROM]; INSIDE],
            neurons: [None; INSIDE],
            activation_function,
            error_function,
        }
    }
}

trait Num<T>:
    Copy
    + Add<T, Output = T>
    + Sub<T, Output = T>
    + Mul<T, Output = T>
    + Div<T, Output = T>
    + PartialEq<T>
{
}

impl<
        T: Copy
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + PartialEq<T>,
    > Num<T> for T
{
}

impl<'a, T: Num<T>, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {
    fn calculate(&mut self, inputsa: [Option<T>; FROM]) {
        for i in 0..FROM {
            for j in 0..INSIDE {
                if let Some(current_value) = self.neurons[j] {
                    self.neurons[j] = Some(current_value + inputsa[i].unwrap() * self.weights[j][i])
                } else {
                    self.neurons[j] = Some(inputsa[i].unwrap() * self.weights[j][i]);
                }
            }
        }

        for j in 0..INSIDE {
            self.neurons[j] = Some((self.activation_function)(self.neurons[j].unwrap()));
        }
    }
    fn calculate_first_layer(&mut self, inputsa: [T; FROM]) {
        for i in 0..FROM {
            for j in 0..INSIDE {
                if let Some(current_value) = self.neurons[j] {
                    self.neurons[j] = Some(current_value + inputsa[i] * self.weights[j][i])
                } else {
                    self.neurons[j] = Some(inputsa[i] * self.weights[j][i]);
                }
            }
        }

        for j in 0..INSIDE {
            self.neurons[j] = Some((self.activation_function)(self.neurons[j].unwrap()));
        }
    }
}

#[derive(Debug)]
struct SquareNetwork<
    T,
    const SIZE: usize,
    const HEIGHT: usize,
    const NUM_IN: usize,
    const NUM_OUT: usize,
> {
    inputs: *mut [T; NUM_IN],
    layers_in: LayerNeuron<T, NUM_IN, HEIGHT>,
    layers: [LayerNeuron<T, HEIGHT, HEIGHT>; SIZE],
    layers_out: LayerNeuron<T, HEIGHT, NUM_OUT>,
    outputs: [Option<T>; NUM_OUT],
}

impl<
        'a,
        T: Num<T>,
        const SIZE: usize,
        const HEIGHT: usize,
        const NUM_IN: usize,
        const NUM_OUT: usize,
    > SquareNetwork<T, SIZE, HEIGHT, NUM_IN, NUM_OUT>
{
    fn new(function: fn(T) -> T) -> Self
    where
        T: Default,
    {
        // let mut temp = ;
        SquareNetwork {
            inputs: &mut [T::default(); NUM_IN],
            layers_in: LayerNeuron::new(function, function),
            layers: [LayerNeuron::new(function, function); SIZE],
            layers_out: LayerNeuron::new(function, function),
            outputs: [None; NUM_OUT],
        }
    }
    fn new_val(val: T, function: fn(T) -> T) -> Self
    where
        T: Default,
    {
        SquareNetwork {
            inputs: &mut [T::default(); NUM_IN],
            layers_in: LayerNeuron::new_val(val, function, function),
            layers: [LayerNeuron::new_val(val, function, function); SIZE],
            layers_out: LayerNeuron::new_val(val, function, function),
            outputs: [None; NUM_OUT],
        }
    }
    fn calculate(&mut self)
    where
        T: Display + Debug,
    {
        println!("------0-------\n{:#?}------0-------\n", &self);
        // self.layers_in.calculate_first_layer(*inputsz);
        self.layers_in
            .calculate_first_layer(unsafe { *self.inputs });
        if SIZE > 0 {
            self.layers[0].calculate(self.layers_in.neurons);
            for i in 1..SIZE {
                self.layers[i].calculate(self.layers[i - 1].neurons);
            }
            self.layers_out.calculate(self.layers[SIZE - 1].neurons);
        } else {
            self.layers_out.calculate(self.layers_in.neurons);
        }
        println!("------7-------\n{:#?}------7-------\n", &self);
    }
}

fn main() {
    println!("Hello, world!");
    let mut inputs = [1., 2.];
    let mut my_net = SquareNetwork::<f64, 1, 3, 2, 1>::new_val(1., lol);
    my_net.inputs = &mut inputs;
    my_net.calculate();
    inputs = [2., 3.];
    my_net.calculate();
    println!("{:#?}", my_net);
}

fn lol<T>(some: T) -> T
where
    T: Num<T>,
{
    some + some
}
