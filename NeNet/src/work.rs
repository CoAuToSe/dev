#![no_implicit_prelude]
extern crate core;
use core::{
    cmp::{PartialEq, PartialOrd},
    default::Default,
    fmt::{Debug, Display},
    marker::Copy,
    ops::{Add, Div, Mul, Not, Sub},
    option::Option::{self, None, Some},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
struct LayerNeuron<T, const FROM: usize, const INSIDE: usize> {
    weights: [[T; FROM]; INSIDE],
    bias: [T; INSIDE],
    sum: [Option<T>; INSIDE],
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
            bias: [T::default(); INSIDE],
            sum: [None; INSIDE],
            neurons: [None; INSIDE],
            activation_function: |_| T::default(),
            error_function: |_| T::default(),
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

trait Unit<T>: Num<T> + FromStr + Default {
    fn unit() -> Self;
    fn not_unit() -> Self;
    fn abs(self) -> Self;
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
    fn abs(self) -> Self {
        if self >= T::default() {
            return self;
        } else {
            return T::default() - self;
        }
    }
}

impl<'a, T: Unit<T>, const FROM: usize, const INSIDE: usize> LayerNeuron<T, FROM, INSIDE> {
    fn new(activation_function: fn(T) -> T, error_function: fn(T) -> T) -> Self
    where
        T: Default,
    {
        LayerNeuron {
            weights: [[T::default(); FROM]; INSIDE],
            bias: [T::default(); INSIDE],
            sum: [None; INSIDE],
            neurons: [None; INSIDE],
            activation_function,
            error_function,
        }
    }
    fn fun(val: T) -> [[T; FROM]; INSIDE] {
        let mut begin = [[val; FROM]; INSIDE];
        let mut signi = T::not_unit();
        let mut signj = T::not_unit();
        for j in 0..INSIDE {
            for i in 0..FROM {
                let mut temp = T::unit();
                for _ in 0..i {
                    temp = temp + T::unit()
                }
                signj = signj * T::not_unit();
                if j % FROM == 0 {
                    signi = signi * T::not_unit();
                }
                if i % FROM == 0 {
                    signi = signi * T::not_unit();
                }
                if i + j % FROM == 0 {
                    signi = signi * T::not_unit();
                }
                if j % INSIDE == 1 {
                    signi = signi * T::not_unit();
                }
                if i % INSIDE == 1 {
                    signi = signi * T::not_unit();
                }
                if i + j % INSIDE == 1 {
                    signi = signi * T::not_unit();
                }
                begin[j][i] = begin[j][i] * (temp + T::unit()) * signi * signj;
                // begin[j][i] = begin[j][i] * (temp + T::unit());
            }
        }
        begin
    }
    fn new_val(val: T, activation_function: fn(T) -> T, error_function: fn(T) -> T) -> Self {
        LayerNeuron {
            weights: Self::fun(val),
            bias: [T::unit(); INSIDE],
            sum: [None; INSIDE],
            neurons: [None; INSIDE],
            activation_function,
            error_function,
        }
    }
    fn calculate(&mut self, inputsz: [Option<T>; FROM])
    where
        T: Debug,
    {
        let mut tempz = [T::default(); FROM];
        for i in 0..FROM {
            tempz[i] = inputsz[i].unwrap()
        }
        self.calculate_first_layer(tempz)
    }
    fn calculate_first_layer(&mut self, inputsa: [T; FROM])
    where
        T: Debug,
    {
        for e in &mut self.sum {
            *e = None
        }
        for i in 0..FROM {
            for j in 0..INSIDE {
                if let Some(current_value) = self.sum[j] {
                    // println!(
                    //     "{:?} | {:?} | {:?}",
                    //     current_value, inputsa[i], self.weights[j][i]
                    // );
                    self.sum[j] = Some(current_value + inputsa[i] * self.weights[j][i])
                } else {
                    // println!("NIV | {:?} | {:?}", inputsa[i], self.weights[j][i]);
                    self.sum[j] = Some(inputsa[i] * self.weights[j][i] + self.bias[j]);
                }
            }
        }

        for j in 0..INSIDE {
            self.neurons[j] = Some((self.activation_function)(self.sum[j].unwrap()));
        }
    }
    fn learn(&mut self, supposed_output: [T; INSIDE], inpa: [T; FROM], bias: T) -> [T; FROM]
    where
        T: Default,
    {
        let mut input_error = [T::default(); FROM];
        for i in 0..FROM {
            for j in 0..INSIDE {
                let error = self.neurons[j].unwrap() - supposed_output[j];
                // let error = self.weights[j][i] * inpa[i] - supposed_output[j];
                // self.weights[j][i] = self.weights[j][i] - bias * inpa[i] * error;
                // input_error[i] = input_error[i]
                //     + bias * self.weights[j][i] * error * (self.error_function)(inpa[i]);
                self.weights[j][i] = self.weights[j][i] - bias * inpa[i] * error;
                // self.weights[j][i] = self.weights[j][i]
                //     - bias * inpa[i] * error * (self.error_function)(self.sum[j].unwrap());
                input_error[i] = input_error[i]
                    + self.weights[j][i] * (self.error_function)(self.sum[j].unwrap());
            }
        }
        for j in 0..INSIDE {
            self.bias[j] = self.bias[j] - bias * (self.neurons[j].unwrap() - supposed_output[j]);
            // self.bias[j] = self.bias[j]
            //     - bias
            //         * (self.neurons[j].unwrap() - supposed_output[j])
            //         * (self.error_function)(self.sum[j].unwrap());
            self.neurons[j] = None
        }
        input_error
    }
    fn init_supposed_learn(
        &mut self,
        supposed_output: [T; INSIDE],
        inpa: [T; FROM],
        mut input_error: [T; FROM],
        bias: T,
    ) -> [T; FROM]
    where
        T: Default,
    {
        // let mut input_error = [T::default(); FROM];
        for i in 0..FROM {
            for j in 0..INSIDE {
                let error = self.neurons[j].unwrap() - supposed_output[j];
                // let error = self.weights[j][i] * inpa[i] - supposed_output[j];
                // self.weights[j][i] = self.weights[j][i] - bias * inpa[i] * error;
                // input_error[i] = input_error[i]
                //     + bias * self.weights[j][i] * error * (self.error_function)(inpa[i]);
                self.weights[j][i] = self.weights[j][i] - bias * inpa[i] * error;
                // self.weights[j][i] = self.weights[j][i]
                //     - bias * inpa[i] * error * (self.error_function)(self.sum[j].unwrap());
                input_error[i] = input_error[i]
                    + self.weights[j][i] * (self.error_function)(self.sum[j].unwrap());
            }
        }
        for j in 0..INSIDE {
            self.bias[j] = self.bias[j] - bias * (self.neurons[j].unwrap() - supposed_output[j]);
            // self.bias[j] = self.bias[j]
            //     - bias
            //         * (self.neurons[j].unwrap() - supposed_output[j])
            //         * (self.error_function)(self.sum[j].unwrap());
            self.neurons[j] = None
        }
        input_error
    }
    fn supposed_learn(
        &mut self,
        calculated_output_error: [T; INSIDE],
        input_values: [T; FROM],
        bias: T,
    ) -> [T; FROM]
    where
        T: Default,
    {
        let mut input_error = [T::default(); FROM];
        for i in 0..FROM {
            for j in 0..INSIDE {
                let opti = calculated_output_error[j] * (self.error_function)(self.sum[j].unwrap());
                self.weights[j][i] = self.weights[j][i] - bias * opti * input_values[i];
                input_error[i] = input_error[i] + self.weights[j][i] * opti;
            }
        }
        for j in 0..INSIDE {
            self.bias[j] = self.bias[j]
                - bias * calculated_output_error[j] * (self.error_function)(self.sum[j].unwrap());
            self.neurons[j] = None
        }
        if ERROR_SCALED {
            let mut inside_t = T::default();
            for _ in 0..INSIDE {
                inside_t = inside_t + T::unit()
            }
            for i in 0..FROM {
                input_error[i] = input_error[i] / inside_t;
            }
        }
        input_error
    }
    // fn learn_option(
    //     &mut self,
    //     supposed_output: [Option<T>; INSIDE],
    //     inpa: [T; FROM],
    //     bias: T,
    // ) -> [T; FROM]
    // where
    //     T: Default,
    // {
    //     let mut tempa = [T::default(); INSIDE];
    //     for i in 0..INSIDE {
    //         tempa[i] = supposed_output[i].unwrap()
    //     }
    //     self.learn(tempa, inpa, bias)
    // }
    fn learn_option2(
        &mut self,
        supposed_output: [T; INSIDE],
        inpa: [Option<T>; FROM],
        bias: T,
    ) -> [T; FROM]
    where
        T: Default,
    {
        let mut tempa = [T::default(); FROM];
        for i in 0..FROM {
            tempa[i] = inpa[i].unwrap()
        }
        self.learn(supposed_output, tempa, bias)
    }
    fn supposed_learn_option2(
        &mut self,
        calculated_output_error: [T; INSIDE],
        input_values: [Option<T>; FROM],
        bias: T,
    ) -> [T; FROM]
    where
        T: Default,
    {
        let mut tempa = [T::default(); FROM];
        for i in 0..FROM {
            tempa[i] = input_values[i].unwrap()
        }
        self.supposed_learn(calculated_output_error, tempa, bias)
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
    // outputs: [Option<T>; NUM_OUT],
    outputs: [T; NUM_OUT],
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
    // fn new(function: fn(T) -> T) -> Self
    // where
    //     T: Default,
    // {
    //     // let mut temp = ;
    //     SquareNetwork {
    //         inputs: [T::default(); NUM_IN],
    //         layers_in: LayerNeuron::new(function, function),
    //         layers: [LayerNeuron::new(function, function); SIZE],
    //         layers_out: LayerNeuron::new(function, function),
    //         outputs: [T::default(); NUM_OUT],
    //     }
    // }
    fn new_val(
        val: T,
        function: fn(T) -> T,
        error_function: fn(T) -> T,
        inputs: [T; NUM_IN],
        outputs: [T; NUM_OUT],
    ) -> Self
    where
        T: Default,
    {
        SquareNetwork {
            inputs: inputs,
            layers_in: LayerNeuron::new_val(val, function, error_function),
            layers: [LayerNeuron::new_val(val, function, error_function); SIZE],
            layers_out: LayerNeuron::new_val(val, function, error_function),
            outputs: outputs,
        }
    }
    fn calculate(&mut self) -> [T; NUM_OUT]
    where
        T: Display + Debug + Default,
    {
        self.layers_in.calculate_first_layer(self.inputs);
        // print!("!");
        if SIZE > 0 {
            self.layers[0].calculate(self.layers_in.neurons);
            // print!("!");
            for i in 1..SIZE {
                self.layers[i].calculate(self.layers[i - 1].neurons);
                // print!("!");
            }
            self.layers_out.calculate(self.layers[SIZE - 1].neurons);
            // print!("!");
        } else {
            self.layers_out.calculate(self.layers_in.neurons);
        }
        self.output()
    }
    fn learn(&mut self, bias: T)
    where
        T: Display + Debug + Default,
    {
        if SIZE > 0 {
            let err =
                self.layers_out
                    .learn_option2(self.outputs, self.layers[SIZE - 1].neurons, bias);

            let mut arr;
            if SIZE > 1 {
                arr = self.layers[SIZE - 1].learn_option2(err, self.layers[SIZE - 2].neurons, bias);
                for i in 2..SIZE - 1 {
                    arr = self.layers[SIZE - i].learn_option2(
                        arr,
                        self.layers[SIZE - i - 1].neurons,
                        bias,
                    );
                }
                arr = self.layers[0].learn_option2(arr, self.layers_in.neurons, bias);
            } else {
                arr = self.layers[0].learn_option2(err, self.layers_in.neurons, bias);
            }
            self.layers_in.learn(arr, self.inputs, bias);
        } else {
            extern crate core;
            use core::todo;
            todo!()
        }
    }
    fn supposed_learn(&mut self, bias: T)
    where
        T: Display + Debug + Default,
    {
        if SIZE > 0 {
            let outputs_errors = {
                let mut temp = self.outputs;
                for i in 0..NUM_OUT {
                    temp[i] = self.layers_out.neurons[i].unwrap() - temp[i]
                }
                temp
            };
            let before_last_error = self.layers_out.supposed_learn_option2(
                outputs_errors,
                self.layers[SIZE - 1].neurons,
                bias,
            );

            let mut arr;
            if SIZE > 1 {
                arr = self.layers[SIZE - 1].supposed_learn_option2(
                    before_last_error,
                    self.layers[SIZE - 2].neurons,
                    bias,
                );
                for i in 2..SIZE - 1 {
                    arr = self.layers[SIZE - i].supposed_learn_option2(
                        arr,
                        self.layers[SIZE - i - 1].neurons,
                        bias,
                    );
                }
                arr = self.layers[0].supposed_learn_option2(arr, self.layers_in.neurons, bias);
            } else {
                arr = self.layers[0].supposed_learn_option2(
                    before_last_error,
                    self.layers_in.neurons,
                    bias,
                );
            }
            self.layers_in.supposed_learn(arr, self.inputs, bias);
        } else {
            extern crate core;
            use core::todo;
            todo!()
        }
    }

    fn output(&self) -> [T; NUM_OUT] {
        let mut temp = [T::default(); NUM_OUT];
        for i in 0..NUM_OUT {
            temp[i] = self.layers_out.neurons[i].unwrap()
        }
        temp
    }
    fn input(&mut self, some: [T; NUM_IN]) {
        self.inputs = some
    }
}

const ERROR_SCALED: bool = true;
const PRECISION: usize = 10;
const MAX_STEPS: usize = 10_000;
const BIAS: f64 = 0.01;

fn main() {
    extern crate std;
    use std::{print, println};
    println!("Hello, world!");

    let inpae = [[0., 0.], [1., 0.], [0., 1.], [1., 1.]];
    let oute = [0., 0., 0., 1.];
    // let inputs = [0.5, 0.2];
    let inputs = [0., 0.];
    // let outputs = [0.2, -0.4, 0.5];
    let outputs = [0.];

    // working with a BIAS = 1. // to use is value is clamped
    let mut my_net =
        // SquareNetwork::<f64, 1, 3, 2, 1>::new_val(0.5, furry, futanari, inputs, outputs);

    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, monster, tentacles, inputs, outputs);

    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, shota, oppai, inputs, outputs);

    // working with a BIAS ~= 0.1
    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, netori, tsundere, inputs, outputs);

    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, lol, lol2, inputs, outputs);

    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, furry, futanari, inputs, outputs);

    // may don't work because deriv is non linear/ it is not C^1 or C^infinit
    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, yiff, dick_girl, inputs, outputs);

    // may don't work because some y have to x solutions
    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, gloryhole, milf, inputs, outputs);

    // may don't work because some y have to x solutions
    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, gloryhole, milf, inputs, outputs);

    // working with a low BIAS ~= 0.01
    // let mut my_net = SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, wtf, ok, inputs, outputs);

    // let mut my_net =
        SquareNetwork::<f64, 2, 3, 2, 1>::new_val(0.5, poker_face, troll_face, inputs, outputs);

    my_net.calculate();
    println!("{:#?} {:?}", my_net, my_net.inputs);
    // my_net.inputs = &mut inputs;
    let mut end = false;
    let mut last = [0.; 1];
    let mut temp = 0;
    'a: for loli in 0..MAX_STEPS {
        if loli % oute.len() == 0 {
            temp = 0
        }
        // my_net.calculate();
        // println!("{:#?}", my_net);
        my_net.inputs = inpae[loli % inpae.len()];
        my_net.outputs = [oute[loli % oute.len()]; 1];
        // let some = my_net.output();
        let some = my_net.calculate();
        for i in 0..some.len() {
            if some[i].is_nan() {
                end = true
            }
        }
        if !end {
            for i in 0..some.len() {
                last[i] = some[i]
            }
        }
        // println!("{:#?}", some);
        for i in 0..some.len() {
            if (my_net.outputs[i] - some[i]) * (my_net.outputs[i] - some[i]) <= 0.0001 {
                // println!("{:#?}", (my_net.outputs[i] - some[i]));
                temp += 1;
                println!(
                    "{:?} | {:?} | {:?}",
                    my_net.inputs, my_net.outputs, my_net.layers_out.neurons
                );
            }
        }
        // if temp != 0 {
        //     println!("{}", "lol")
        // }
        if temp == oute.len() || end {
            println!("{:?}|{:?}", last, some);
            println!("{:#?}", loli);
            break 'a;
        }
        if temp == oute.len() {
            println!("{:?}|{:?}", last, some);
            println!("{:#?}", loli);
            end = true;
        } else {
            // my_net.learn(0.1);
            my_net.supposed_learn(BIAS);
        }
        print!(".");
    }
    my_net.calculate();
    println!("{:#?}", my_net.layers_out.neurons);
    println!("{:#?}", "wtf");
    println!("{:?} {:?}", furry(-10.), futanari(-10.));
    println!("{:?} {:?}", furry(-1.), futanari(-1.));
    println!("{:?} {:?}", furry(-0.1), futanari(-0.1));
    println!("{:?} {:?}", furry(-0.), futanari(-0.));
    println!("{:?} {:?}", furry(0.), futanari(0.));
    println!("{:?} {:?}", furry(0.1), futanari(0.1));
    println!("{:?} {:?}", furry(1.), futanari(1.));
    println!("{:?} {:?}", furry(10.), futanari(10.));
    println!("aze");
    println!("{:?} {:?}", espa(4., PRECISION), lena(2., PRECISION));
    println!("{:?}", espa(lena(4., PRECISION), PRECISION));
    println!("{:?}", powra(4., 2.5));
    println!("{:?}", powra(15.7565, 7.12));
    println!("{:?}", powra(20., 3.));
    println!("{:?}", powra(100., 3.5));
    println!("{:?}", espa(1., PRECISION));
    println!("{:?}", mulaa::<f64>(3));
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
    // let lols_some = lol(some);
    // let retunrd = ((T::unit() + T::unit()) * lols_some - T::unit()) * lols_some;
    // println!("lol2 {:?} | {:?} | {:?}", some, lols_some, retunrd);
    let var = clamp(some, T::default(), T::unit());
    let three = T::unit() + T::unit() + T::unit();
    let retunrd = (three + three) * (var - var * var);
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
    let mut temp = T::default();
    if x > T::default() {
        let my_value = x;
        let mut suck = (T::unit(), T::unit(), T::default());
        for i in 0..vara {
            // temp = temp + mula(my_value, i) / mulaa(i);
            temp = temp + suck.0 / suck.1;
            // if i == 0 || i == 1 {
            suck = (
                suck.0 * my_value,
                suck.1 * (suck.2 + T::unit()),
                suck.2 + T::unit(),
            );
        }
        return temp;
    } else {
        let my_value = T::default() - x;
        let mut suck = (T::unit(), T::unit(), T::default());
        for i in 0..vara {
            // temp = temp + mula(my_value, i) / mulaa(i);
            temp = temp + suck.0 / suck.1;
            suck = (
                suck.0 * my_value,
                suck.1 * (suck.2 + T::unit()),
                suck.2 + T::unit(),
            );
        }
        return T::unit() / temp;
    }
}
fn lena<T>(x: T, vara: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::default();
    let my_value = (T::abs(x) - T::unit()) / (T::abs(x) + T::unit());
    let mut suck = (T::unit(), T::default());
    for i in 0..vara {
        // let suck = mula(my_value, i);
        temp = temp + my_value * suck.0 * suck.0 / ((T::unit() + T::unit()) * suck.1 + T::unit());
        suck = (suck.0 * my_value, suck.1 + T::unit());
    }
    return (T::unit() + T::unit()) * temp;
}

fn mula<T>(val: T, recurent: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::unit();
    for _ in 0..recurent {
        temp = temp * val;
    }
    temp
}
fn mulina<T>(val: T, n: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::default();
    for _ in 0..n {
        temp = temp + val;
    }
    temp
}
fn mulaa<T>(recurent: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::unit();
    let mut index = T::unit();
    for _ in 1..recurent {
        index = index + T::unit();
        temp = temp * index;
    }
    temp
}

fn shota<T>(val: T) -> T
where
    T: Unit<T>,
{
    T::unit() / (T::unit() + espa(T::default() - val, PRECISION))
}
fn oppai<T>(val: T) -> T
where
    T: Unit<T> + Default,
{
    let temp = shota(val);
    temp * (T::unit() - temp)
}

fn powra<T>(val: T, pow: T) -> T
where
    T: Unit<T>,
{
    // extern crate std;
    // use std::{print, println};
    // let zaeazraz = lena(T::default()+val,PRECISION);
    // println!("lna:{:?} | {:?} ", zaeazraz, pow*zaeazraz);
    if val > T::default() {
        return T::default() + espa(pow * lena(T::default() + val, PRECISION), PRECISION);
    } else {
        return T::default() - espa(pow * lena(T::default() + val, PRECISION), PRECISION);
    }
}

fn min<T>(val1: T, val2: T) -> T
where
    T: Unit<T>,
{
    if val1 <= val2 {
        return val1;
    } else {
        return val2;
    }
}
fn max<T>(val1: T, val2: T) -> T
where
    T: Unit<T>,
{
    if val1 >= val2 {
        return val1;
    } else {
        return val2;
    }
}

fn netori<T>(val: T) -> T
where
    T: Unit<T> + Debug,
{
    let one = T::unit();
    let two = one + one;
    let five = two + two + one;
    let ten = five + five;
    let zade = max(val / ten, val);
    // let zade = max(T::default(), val);
    // println!("{:?}, {:?}", val, zade);
    zade
}
fn tsundere<T>(val: T) -> T
where
    T: Unit<T>,
{
    if T::default() <= val {
        return T::unit();
    } else {
        let one = T::unit();
        let two = one + one;
        let five = two + two + one;
        let ten = five + five;
        return one / ten;
        // return T::default();
    }
}
fn monster<T>(val: T) -> T
where
    T: Unit<T> + Debug,
{
    shota(val) * netori(val)
}
fn tentacles<T>(val: T) -> T
where
    T: Unit<T>,
{
    oppai(val) * tsundere(val)
}

fn furry<T>(val: T) -> T
where
    T: Unit<T>,
{
    (T::unit() + T::unit()) * shota(val) - T::unit()
}

fn futanari<T>(val: T) -> T
where
    T: Unit<T>,
{
    (T::unit() + T::unit()) * oppai(val)
}

fn yiff<T>(val: T) -> T
where
    T: Unit<T>,
{
    ((T::unit() + T::unit()) * shota(val) - T::unit()) * val
}

fn dick_girl<T>(val: T) -> T
where
    T: Unit<T>,
{
    (T::unit() + T::unit()) * val * val * oppai(val)
        + ((T::unit() + T::unit()) * shota(val) - T::unit()) * val * (T::unit() + T::unit())
}

fn wtf<T>(val: T) -> T
where
    T: Unit<T>,
{
    val
}
fn ok<T>(val: T) -> T
where
    T: Unit<T>,
{
    T::unit()
}

fn gloryhole<T>(val: T) -> T
where
    T: Unit<T>,
{
    shota(val) * val
}

fn milf<T>(val: T) -> T
where
    T: Unit<T>,
{
    shota(val) + oppai(val) * val
}

fn poker_face<T>(val: T) -> T
where
    T: Unit<T>,
{
    lena(espa(val, PRECISION), PRECISION)
}
fn troll_face<T>(val: T) -> T
where
    T: Unit<T>,
{
    derive(val, poker_face)
}

fn derive<T>(val: T, funca: fn(T) -> T) -> T
where
    T: Unit<T>,
{
    let one = T::unit();
    let two = one + one;
    let five = two + two + one;
    let ten = five + five;
    let hundred = ten * ten;
    let thousand = hundred * ten;
    let billion = thousand * thousand * thousand;

    // if T::abs(val) > T::unit() {
    (funca(val + T::unit() / billion) - funca(val)) / (T::unit() / billion)
    // } else {
    //     (poker_face(val + val) - poker_face(val)) / (val)
    // }
}
#[test]
fn min_test() {
    extern crate core;
    use core::assert_eq;
    assert_eq!(min(f64::unit(), f64::default()), f64::default());
    assert_eq!(min(f64::default(), f64::unit()), f64::default());
}

#[test]
fn max_test() {
    extern crate core;
    use core::assert_eq;
    assert_eq!(max(f64::unit(), f64::default()), f64::unit());
    assert_eq!(max(f64::default(), f64::unit()), f64::unit());
}
#[test]
fn mulina_test() {
    extern crate core;
    use core::assert_eq;
    assert_eq!(mulina(f64::unit(), 2), (f64::unit() + f64::unit()));
    assert_eq!(
        mulina(f64::unit() + f64::unit(), 2),
        (f64::unit() + f64::unit() + (f64::unit() + f64::unit()))
    );
}
#[test]
fn mulaa_test() {
    extern crate core;
    use core::assert_eq;
    assert_eq!(mulaa::<f64>(3), 6.);
    assert_eq!(mulaa::<f64>(4), 24.);
    assert_eq!(mulaa::<f64>(5), 120.);
}
