use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Not, Sub},
    str::FromStr,
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

impl<'a, T: Copy, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {}

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

trait Unit<T>: Num<T> + FromStr + Default {
    fn unit() -> Self;
    fn not_unit() -> Self;
}
impl<T: Num<T> + FromStr + Default> Unit<T> for T
where
    <T as FromStr>::Err: Debug,
{
    fn unit() -> Self {
        T::from_str("1").unwrap()
    }
    fn not_unit() -> Self {
        T::default() - T::unit()
    }
}

impl<'a, T: Unit<T>, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {
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
    fn fun(val: T) -> [[T; FROM]; INSIDE] {
        let mut begin = [[val; FROM]; INSIDE];
        let mut sign = T::not_unit();
        for e in &mut begin {
            for i in 0..FROM {
                let mut temp = val / val;
                for _ in 0..i {
                    temp = temp + val / val
                }
                e[i] = e[i] * (temp + val / val) * sign;
            }
            sign = sign * T::not_unit();
        }
        begin
    }
    fn new_val(val: T, activation_function: fn(T) -> T, error_function: fn(T) -> T) -> Self {
        LayerNeuron {
            weights: Self::fun(val),
            neurons: [None; INSIDE],
            activation_function,
            error_function,
        }
    }
    fn calculate(&mut self, inputsa: [Option<T>; FROM]) {
        let mut temp = [T::default(); FROM];
        for i in 0..FROM {
            temp[i] = inputsa[i].unwrap()
        }
        self.calculate_first_layer(temp)
    }
    fn calculate_first_layer(&mut self, inputsa: [T; FROM]) {
        for e in &mut self.neurons {
            *e = None
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
            self.neurons[j] = Some((self.activation_function)(self.neurons[j].unwrap()));
        }
    }
    fn learn(&mut self, supposed_output: [T; INSIDE], bias: T) -> [T; FROM]
    where
        T: Default,
    {
        let mut input_error = [T::default(); FROM];
        for i in 0..FROM {
            for j in 0..INSIDE {
                // self.weights[j][i] = self.weights[j][i]
                //     - bias * (self.activation_function)(self.neurons[j].unwrap(), unit);
                let error = supposed_output[j] - self.neurons[j].unwrap();
                self.weights[j][i] = self.weights[j][i] - bias * self.neurons[j].unwrap() * error;
                input_error[i] = input_error[i]
                    + bias
                        * self.weights[j][i]
                        * error
                        * (self.error_function)(self.neurons[j].unwrap());
            }
        }
        for j in 0..INSIDE {
            self.neurons[j] = None
        }
        input_error
    }

    fn learn_option(&mut self, error: [Option<T>; INSIDE], bias: T) -> [T; FROM]
    where
        T: Default,
    {
        let mut temp = [T::default(); INSIDE];
        for i in 0..FROM {
            temp[i] = error[i].unwrap()
        }
        self.learn(temp, bias)
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
        T: Unit<T>,
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
            outputs: &mut [T::default(); NUM_OUT],
        }
    }
    fn new_val(
        val: T,
        function: fn(T) -> T,
        error_function: fn(T) -> T,
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
    fn calculate(&mut self) -> [T; NUM_OUT]
    where
        T: Display + Debug + Default,
    {
        // println!("------0-------\n{:#?}------0-------\n", &self);
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
        self.output()
        // println!("------7-------\n{:#?}------7-------\n", &self);
    }
    fn learn(&mut self, bias: T)
    where
        T: Display + Debug + Default,
    {
        if SIZE > 0 {
            let err = self.layers_out.learn(unsafe { *self.outputs }, bias);
            let mut arr = self.layers[SIZE - 1].learn(err, bias);
            for i in 2..SIZE {
                arr = self.layers[SIZE - i].learn(arr, bias);
            }
            self.layers_in.learn(arr, bias);
        } else {
            self.layers_out.learn(unsafe { *self.outputs }, bias);
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

    // fn expected(&mut self, expected_output: [T; NUM_OUT]) {}

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
    fn output(&self) -> [T; NUM_OUT] {
        let mut temp = [T::default(); NUM_OUT];
        for i in 0..NUM_OUT {
            temp[i] = self.layers_out.neurons[i].unwrap()
        }
        temp
    }
}

fn main() {
    println!("Hello, world!");
    let mut inputs = [0.5, 0.2];
    let mut outputs = [0.2, 0.4, 0.5];
    let mut my_net =
        SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, shota, futanari, inputs, outputs);
    println!("{:#?}", my_net);
    // my_net.inputs = &mut inputs;
    'a: for loli in 0..10_000 {
        // my_net.calculate();
        // println!("{:#?}", my_net);

        let mut temp = 0;
        // let some = my_net.output();
        let some = my_net.calculate();
        // println!("{:#?}", some);
        for i in 0..3 {
            if (outputs[i] - some[i]) * (outputs[i] - some[i]) <= 0.0001 {
                println!("{:#?}", (outputs[i] - some[i]));
                temp += 1
            }
        }
        if temp != 0 {
            println!("{}", "lol")
        }
        if temp == 3 {
            println!("{:#?}", loli);
            break 'a;
        } else {
            my_net.learn(1.);
        }
    }
    my_net.calculate();
    println!("{:#?}", my_net);
    // inputs = [2., 3.];
    // my_net.calculate();
    // println!("{:#?}", my_net);
    // println!("{:#?}", espa(1. as f64, 10, 1.));
    // println!("{:#?}", espa(100. as f64, 10, 1.));
    // println!("{:?}", f64::unit());
}

fn lol<T>(some: T) -> T
where
    T: Unit<T>,
{
    let var = clamp(some, T::default(), T::unit());
    let retunrd = (T::unit() + T::unit() + T::unit()) * (var * var)
        - (T::unit() + T::unit()) * (var * var * var);
    // println!("lol {:?} | {:?} | {:?}", some, var, retunrd);
    retunrd
}
fn lol2<T>(some: T) -> T
where
    T: Unit<T>,
{
    // let var = clamp(some, T::default(), some / some);
    // let retunrd =
    //     (some / some + some / some + some / some + some / some + some / some + some / some)
    //         * (var - var * var);
    // println!("lol2 {:?}| {:?} | {:?}", some, var, retunrd);
    // retunrd
    let lols_some = lol(some);
    let retunrd = ((T::unit() + T::unit()) * lols_some - T::unit()) * lols_some;
    // println!("lol2 {:?} | {:?} | {:?}", some, lols_some, retunrd);
    retunrd
}

fn clamp<T>(val: T, a: T, b: T) -> T
where
    T: PartialOrd<T>,
{
    if val >= b {
        return b;
    }
    if val <= a {
        return a;
    }

    return val;
}

fn espa<T>(x: T, vara: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::unit();
    for i in 0..vara {
        temp = temp + mula(x, i) / mulaa(i);
    }
    temp
}

fn mula<T>(val: T, recurent: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::unit();
    for _ in 0..recurent + 1 {
        temp = temp * val;
    }
    temp
}
fn mulaa<T>(recurent: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::unit();
    for _ in 0..recurent {
        temp = temp * (temp + T::unit());
    }
    temp
}

static PRECISION: usize = 10;
fn shota<T>(val: T) -> T
where
    T: Unit<T>,
{
    T::unit() / (T::unit() + espa(T::default() - val, PRECISION))
}
fn futanari<T>(val: T) -> T
where
    T: Unit<T> + Default,
{
    let temp = shota(val);
    temp * (T::unit() - temp)
}
