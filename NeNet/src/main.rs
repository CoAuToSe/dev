use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Copy)]
struct LayerNeuron<T, const FROM: usize, const INSIDE: usize> {
    weights: [[T; FROM]; INSIDE],
    neurons: [Option<T>; INSIDE],
    activation_function: fn(T, T) -> T,
    error_function: fn(T, T) -> T,
}

impl<'a, T: Default + Copy, const FROM: usize, const INSIDE: usize> Default
    for LayerNeuron<T, FROM, INSIDE>
{
    fn default() -> Self {
        LayerNeuron {
            weights: [[T::default(); FROM]; INSIDE],
            neurons: [None; INSIDE],
            activation_function: |_, _| T::default(),
            error_function: |_, _| T::default(),
        }
    }
}

impl<'a, T: Copy, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {
    fn new(activation_function: fn(T, T) -> T, error_function: fn(T, T) -> T) -> Self
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
    fn new_val(val: T, activation_function: fn(T, T) -> T, error_function: fn(T, T) -> T) -> Self {
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
    + PartialOrd<T>
{
}

impl<
        T: Copy
            + Add<T, Output = T>
            + Sub<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + PartialEq<T>
            + PartialOrd<T>,
    > Num<T> for T
{
}

impl<'a, T: Num<T>, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {
    fn calculate(&mut self, inputsa: [Option<T>; FROM], unit: T)
    where
        T: Default,
    {
        let mut temp = [T::default(); FROM];
        for i in 0..FROM {
            temp[i] = inputsa[i].unwrap()
        }
        self.calculate_first_layer(temp, unit)
    }
    fn calculate_first_layer(&mut self, inputsa: [T; FROM], unit: T) {
        for mut e in self.neurons {
            e = None
        }
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
            self.neurons[j] = Some((self.activation_function)(self.neurons[j].unwrap(), unit));
        }
    }
    fn learn(&mut self, error: [T; INSIDE], bias: T, unit: T) -> [T; FROM]
    where
        T: Default,
    {
        let mut input_error = [T::default(); FROM];
        for i in 0..FROM {
            for j in 0..INSIDE {
                self.weights[j][i] = self.weights[j][i]
                    - self.weights[j][i]
                        * bias
                        * (self.error_function)(self.neurons[j].unwrap() - error[j], unit);
                input_error[i] = input_error[i]
                    + self.weights[j][i]
                        * bias
                        * (self.error_function)(self.neurons[j].unwrap() - error[j], unit);
            }
        }
        input_error
    }

    fn learn_option(&mut self, error: [Option<T>; INSIDE], bias: T, unit: T) -> [T; FROM]
    where
        T: Default,
    {
        let mut temp = [T::default(); INSIDE];
        for i in 0..FROM {
            temp[i] = error[i].unwrap()
        }
        self.learn(temp, bias, unit)
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
    // outputs: [Option<T>; NUM_OUT],
    outputs: *mut [T; NUM_OUT],
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
    fn new(function: fn(T, T) -> T) -> Self
    where
        T: Default,
    {
        // let mut temp = ;
        SquareNetwork {
            inputs: &mut [T::default(); NUM_IN],
            layers_in: LayerNeuron::new(function, function),
            layers: [LayerNeuron::new(function, function); SIZE],
            layers_out: LayerNeuron::new(function, function),
            outputs: &mut [T::default(); NUM_OUT],
        }
    }
    fn new_val(
        val: T,
        function: fn(T, T) -> T,
        error_function: fn(T, T) -> T,
        mut inputs: [T; NUM_IN],
        mut outputs: [T; NUM_OUT],
    ) -> Self
    where
        T: Default,
    {
        SquareNetwork {
            inputs: &mut inputs,
            layers_in: LayerNeuron::new_val(val, function, error_function),
            layers: [LayerNeuron::new_val(val, function, error_function); SIZE],
            layers_out: LayerNeuron::new_val(val, function, error_function),
            outputs: &mut outputs,
        }
    }
    fn calculate(&mut self, unit: T)
    where
        T: Display + Debug + Default,
    {
        // println!("------0-------\n{:#?}------0-------\n", &self);
        // self.layers_in.calculate_first_layer(*inputsz);
        self.layers_in
            .calculate_first_layer(unsafe { *self.inputs }, unit);
        if SIZE > 0 {
            self.layers[0].calculate(self.layers_in.neurons, unit);
            for i in 1..SIZE {
                self.layers[i].calculate(self.layers[i - 1].neurons, unit);
            }
            self.layers_out
                .calculate(self.layers[SIZE - 1].neurons, unit);
        } else {
            self.layers_out.calculate(self.layers_in.neurons, unit);
        }
        let mut temp = unsafe { *self.outputs };
        for i in 0..NUM_OUT {
            temp[i] = self.layers_out.neurons[i].unwrap();
        }
        // println!("------7-------\n{:#?}------7-------\n", &self);
    }
    fn learn(&mut self, bias: T, unit: T)
    where
        T: Display + Debug + Default,
    {
        if SIZE > 0 {
            let err = self.layers_out.learn(unsafe { *self.outputs }, bias, unit);
            let mut arr = self.layers[SIZE - 1].learn(err, bias, unit);
            for i in 2..SIZE {
                arr = self.layers[SIZE - i].learn(arr, bias, unit);
            }
            self.layers_in.learn(arr, bias, unit);
        } else {
            self.layers_out.learn(unsafe { *self.outputs }, bias, unit);
            // self.layers_in.learn_option(self.layers_out.neurons, bias);
            todo!()
        }
        // println!("------0-------\n{:#?}------0-------\n", &self);
        // self.layers_in.calculate_first_layer(*inputsz);
        // self.layers_in
        //     .calculate_first_layer(unsafe { *self.inputs });
        // if SIZE > 0 {
        //     self.layers[0].calculate(self.layers_in.neurons);
        //     for i in 1..SIZE {
        //         self.layers[i].calculate(self.layers[i - 1].neurons);
        //     }
        //     self.layers_out.calculate(self.layers[SIZE - 1].neurons);
        // } else {
        //     self.layers_out.calculate(self.layers_in.neurons);
        // }
        // let mut temp = unsafe { *self.outputs };
        // for i in 0..NUM_OUT {
        //     temp[i] = self.layers_out.neurons[i].unwrap();
        // }
        // println!("------7-------\n{:#?}------7-------\n", &self);
    }

    fn expected(&mut self, expected_output: [T; NUM_OUT]) {}

    // fn outputs(&mut self) -> [T; NUM_OUT]
    // //deprecated
    // where
    //     T: Default + Display + Debug,
    // {
    //     let mut to_return = [T::default(); NUM_OUT];
    //     'fora: for i in 0..NUM_OUT {
    //         if let Some(value) = self.outputs[i] {
    //             to_return[i] = value
    //         } else {
    //             self.calculate();
    //             for i in 0..NUM_OUT {
    //                 to_return[i] = self.outputs[i].unwrap()
    //             }
    //         }
    //     }
    //     to_return
    // }
}

fn main() {
    println!("Hello, world!");
    let mut inputs = [0.5, 0.2];
    let mut outputs = [0.5; 3];
    let mut my_net =
        SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, shota, futanari, inputs, outputs);
    // my_net.inputs = &mut inputs;
    for _ in 0..100 {
        my_net.calculate(1.);
        // println!("{:#?}", my_net);
        my_net.learn(1., 1.);
    }
    my_net.calculate(1.);
    println!("{:#?}", my_net);
    // inputs = [2., 3.];
    // my_net.calculate();
    // println!("{:#?}", my_net);
    // println!("{:#?}", espa(1. as f64, 10, 1.));
    // println!("{:#?}", espa(100. as f64, 10, 1.));
}

fn lol<T>(some: T, unit: T) -> T
where
    T: Num<T> + Default + Debug,
{
    let var = clamp(some, T::default(), unit);
    let retunrd = (unit + unit + unit) * (var * var) - (unit + unit) * (var * var * var);
    // println!("lol {:?} | {:?} | {:?}", some, var, retunrd);
    retunrd
}
fn lol2<T>(some: T, unit: T) -> T
where
    T: Num<T> + Default + Debug,
{
    // let var = clamp(some, T::default(), some / some);
    // let retunrd =
    //     (some / some + some / some + some / some + some / some + some / some + some / some)
    //         * (var - var * var);
    // println!("lol2 {:?}| {:?} | {:?}", some, var, retunrd);
    // retunrd
    let lols_some = lol(some, unit);
    let retunrd = ((unit + unit) * lols_some - unit) * lols_some;
    // println!("lol2 {:?} | {:?} | {:?}", some, lols_some, retunrd);
    retunrd
}

fn clamp<T>(val: T, a: T, b: T) -> T
where
    T: Num<T>,
{
    if val >= b {
        return b;
    }
    if val <= a {
        return a;
    }

    return val;
}

fn espa<T>(x: T, vara: usize, unit: T) -> T
where
    T: Num<T> + Default,
{
    let mut temp = unit;
    for i in 0..vara {
        temp = temp + mula(x, i, unit) / mulaa(i, unit);
    }
    temp
}

fn mula<T>(val: T, recurent: usize, unit: T) -> T
where
    T: Num<T>,
{
    let mut temp = unit;
    for _ in 0..recurent + 1 {
        temp = temp * val;
    }
    // println!("mula {:?}", temp);
    temp
}
fn mulaa<T>(recurent: usize, unit: T) -> T
where
    T: Num<T>,
{
    let mut temp = unit;
    for _ in 0..recurent {
        temp = temp * (temp + unit);
    }
    // println!("mulaa {:?}", temp);
    temp
}

static PRECISION: usize = 10;
fn shota<T>(val: T, unit: T) -> T
where
    T: Num<T> + Default,
{
    unit / (unit + espa(T::default() - val, PRECISION, unit))
}
fn futanari<T>(val: T, unit: T) -> T
where
    T: Num<T> + Default,
{
    let temp = shota(val, unit);
    temp * (unit - temp)
}
