use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
struct LayerNeuron<T, const FROM: usize, const INSIDE: usize> {
    // inputs: [Option<*mut Option<T>>; FROM],
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
            // inputs: [None; FROM],
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
            // inputs: [None; FROM],
            weights: [[T::default(); FROM]; INSIDE],
            neurons: [None; INSIDE],
            activation_function,
            error_function,
        }
    }
    fn new_val(val: T, activation_function: fn(T) -> T, error_function: fn(T) -> T) -> Self {
        LayerNeuron {
            // inputs: [None; FROM],
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
    // fn init_in(&mut self, inputs: &'a mut [T; FROM]) {
    //     for i in 0..FROM {
    //         self.inputs[i] = Some(&mut Some(inputs[i]));
    //     }
    // }
    // fn init(&'a mut self, inputs: &'a mut [Option<T>; FROM]) {
    //     for i in 0..FROM {
    //         self.inputs[i] = Some(&mut inputs[i]);
    //     }
    // }
    // fn calculate(&mut self)
    // where
    //     T: Display + Debug + PartialEq,
    // {
    //     for i in 0..FROM {
    //         if let Some(val) = &self.inputs[i] {
    //             for j in 0..INSIDE {
    //                 if let Some(mut current_value) = self.neurons[j] {
    //                     current_value = current_value
    //                         + unsafe { *self.inputs[i].unwrap() }.unwrap() * self.weights[j][i]
    //                 } else {
    //                     let temp = unsafe { *{ { self.inputs[i] }.unwrap() } };
    //                     println!("{}|{}| {:?}", i, j, temp);
    //                     self.neurons[j] =
    //                         Some(unsafe { *self.inputs[i].unwrap() }.unwrap() * self.weights[j][i]);
    //                     // Some(unsafe { *self.inputs[i].unwrap() }.unwrap() * self.weights[j][i])
    //                     // &mut { self.layers[SIZE - 1].neurons }
    //                 }
    //                 assert_ne!(self.neurons[j], None);
    //             }
    //         } else {
    //             // self.inputs[i] = {
    //             //     let zae = [None; FROM];
    //             //     for i in 0..FROM {
    //             //         zae[i] = Some(self.inputs[i])
    //             //     }
    //             //     zae
    //             // }
    //             panic!("Layer not fully initialised!")
    //             // self.init( inputs ) // but from here we don't know what inputs is, so panic!
    //         }
    //     }
    // }
    fn calculate2(&mut self, inputsa: [Option<T>; FROM])
    where
        T: Display + Debug + PartialEq,
    {
        for i in 0..FROM {
            // if let Some(val) = &self.inputsa[i] {
            for j in 0..INSIDE {
                if let Some(mut current_value) = self.neurons[j] {
                    current_value = current_value + inputsa[i].unwrap() * self.weights[j][i]
                } else {
                    self.neurons[j] = Some(inputsa[i].unwrap() * self.weights[j][i]);
                    // Some(unsafe { *self.inputs[i].unwrap() }.unwrap() * self.weights[j][i])
                    // &mut { self.layers[SIZE - 1].neurons }
                }
                // assert_ne!(self.neurons[j], None);
            }
            // } else {
            // self.inputs[i] = {
            //     let zae = [None; FROM];
            //     for i in 0..FROM {
            //         zae[i] = Some(self.inputs[i])
            //     }
            //     zae
            // }
            // panic!("Layer not fully initialised!")
            // self.init( inputs ) // but from here we don't know what inputs is, so panic!
            // }
        }
    }
    fn calculate21(&mut self, inputsa: [T; FROM])
    where
        T: Display + Debug + PartialEq,
    {
        for i in 0..FROM {
            // if let Some(val) = &self.inputs[i] {
            for j in 0..INSIDE {
                if let Some(mut current_value) = self.neurons[j] {
                    current_value = current_value + inputsa[i] * self.weights[j][i]
                } else {
                    self.neurons[j] = Some(inputsa[i] * self.weights[j][i]);
                    // Some(unsafe { *self.inputs[i].unwrap() }.unwrap() * self.weights[j][i])
                    // &mut { self.layers[SIZE - 1].neurons }
                }
                // assert_ne!(self.neurons[j], None);
            }
            // } else {
            // self.inputs[i] = {
            //     let zae = [None; FROM];
            //     for i in 0..FROM {
            //         zae[i] = Some(self.inputs[i])
            //     }
            //     zae
            // }
            // panic!("Layer not fully initialised!")
            // self.init( inputs ) // but from here we don't know what inputs is, so panic!
            // }
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
    inputs: [T; NUM_IN],
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
        SquareNetwork {
            inputs: [T::default(); NUM_IN],
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
            inputs: [T::default(); NUM_IN],
            layers_in: LayerNeuron::new_val(val, function, function),
            layers: [LayerNeuron::new_val(val, function, function); SIZE],
            layers_out: LayerNeuron::new_val(val, function, function),
            outputs: [None; NUM_OUT],
        }
    }
    // fn init(&'a mut self, inputs: &'a mut [T; NUM_IN]) {
    //     self.layers_in.init_in(inputs);
    //     if SIZE > 0 {
    //         self.layers[0].init(&mut { self.layers_in.neurons });
    //         for i in 1..SIZE {
    //             self.layers[i].init(&mut { self.layers[i - 1].neurons });
    //         }
    //         self.layers_out.init(&mut { self.layers[SIZE - 1].neurons });
    //     } else {
    //         self.layers_out.init(&mut { self.layers_in.neurons });
    //     }
    // }

    // fn calculate(&mut self)
    // where
    //     T: Display + Debug,
    // {
    //     println!("------0-------\n{:#?}------0-------\n", &self);
    //     self.layers_in.calculate();
    //     println!("------1-------\n{:#?}------1-------\n", &self);
    //     for i in 0..SIZE {
    //         println!("------2-------\n{:#?}", &self);
    //         self.layers[i].calculate();
    //         println!("{:#?}------2-------\n", &self);
    //     }
    //     self.layers_out.calculate();
    //     println!("------3-------\n{:#?}------3-------\n", &self);
    // }
    fn calculate2(&mut self, inputsz: &'a mut [T; NUM_IN])
    where
        T: Display + Debug,
    {
        // for i in 0..SIZE {
        //     println!("------2-------\n{:#?}", &self);
        //     self.layers[i].calculate2({ self. });
        //     println!("{:#?}------2-------\n", &self);
        // }
        // self.layers_out.calculate2();
        // println!("------3-------\n{:#?}------3-------\n", &self);

        println!("------0-------\n{:#?}------0-------\n", &self);
        self.layers_in.calculate21(*inputsz);
        println!("------1-------\n{:#?}------1-------\n", &self);
        if SIZE > 0 {
            self.layers[0].calculate2({ self.layers_in.neurons });
            for i in 1..SIZE {
                self.layers[i].calculate2({ self.layers[i - 1].neurons });
            }
            self.layers_out
                .calculate2({ self.layers[SIZE - 1].neurons });
        } else {
            self.layers_out.calculate2({ self.layers_in.neurons });
        }
        println!("------7-------\n{:#?}------7-------\n", &self);
    }
}

fn main() {
    println!("Hello, world!");
    let mut inputs = [1., 2.];
    let mut my_net = SquareNetwork::<f64, 1, 3, 2, 1>::new_val(1., lol);
    my_net.inputs = inputs;
    println!("{:#?}", my_net);
    // my_net.init(&mut inputs);
    my_net.calculate2(&mut inputs);
}

fn lol<T>(some: T) -> T
where
    T: Num<T>,
{
    some
}
