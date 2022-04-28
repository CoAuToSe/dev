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
                    // self.sum[j] = Some(inputsa[i] * self.weights[j][i] + self.bias[j]);
                    self.sum[j] = Some(inputsa[i] * self.weights[j][i]);
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
            // self.bias[j] = self.bias[j] - bias * (self.neurons[j].unwrap() - supposed_output[j]);
            // self.bias[j] = self.bias[j]
            //     - bias
            //         * (self.neurons[j].unwrap() - supposed_output[j])
            //         * (self.error_function)(self.sum[j].unwrap());
            self.neurons[j] = None
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

fn main() {
    extern crate std;
    use std::{print, println};
    println!("Hello, world!");
    let inputs = [0.5, 0.2];
    let outputs = [0.2, -0.4, 0.5];
    let mut my_net =
        // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, shota, oppai, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, netori, tsundere, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, monster, tentacles, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, lol, lol2, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, furry, futanari, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, furry, futanari, inputs, outputs);
    SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, gloryhole, milf, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.1, gloryhole, milf, inputs, outputs);
    // SquareNetwork::<f64, 1, 3, 2, 3>::new_val(0.5, wtf, ok, inputs, outputs);
    my_net.calculate();
    println!("{:#?} {:?}", my_net, inputs);
    // my_net.inputs = &mut inputs;
    let mut end = false;
    'a: for loli in 0..10_000 {
        // my_net.calculate();
        // println!("{:#?}", my_net);

        let mut temp = 0;
        // let some = my_net.output();
        let some = my_net.calculate();
        // println!("{:#?}", some);
        for i in 0..3 {
            if (outputs[i] - some[i]) * (outputs[i] - some[i]) <= 0.0001 {
                // println!("{:#?}", (outputs[i] - some[i]));
                temp += 1
            }
        }
        // if temp != 0 {
        //     println!("{}", "lol")
        // }
        if temp == 3 && end {
            println!("{:#?}", loli);
            break 'a;
        }
        if temp == 3 {
            println!("{:#?}", loli);
            end = true;
        } else {
            my_net.learn(0.1);
        }
        print!(".");
    }
    my_net.calculate();
    println!("{:#?}", my_net);
    println!("{:#?}", "wtf");
    println!("{:?} {:?}", furry(-10.), futanari(-10.));
    println!("{:?} {:?}", furry(-1.), futanari(-1.));
    println!("{:?} {:?}", furry(-0.1), futanari(-0.1));
    println!("{:?} {:?}", furry(-0.), futanari(-0.));
    println!("{:?} {:?}", furry(0.), futanari(0.));
    println!("{:?} {:?}", furry(0.1), futanari(0.1));
    println!("{:?} {:?}", furry(1.), futanari(1.));
    println!("{:?} {:?}", furry(10.), futanari(10.));
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
// P\left(x\right)E\left(x,V_{1}\right)+P\left(-x\right)J\left(x,V_{1}\right)
fn espa<T>(x: T, vara: usize) -> T
where
    T: Unit<T>,
{
    let mut temp = T::unit();
    if x > T::default() {
        for i in 1..vara {
            temp = temp + mula(x, i) / mulaa(i);
        }
        return temp;
    } else {
        for i in 1..vara {
            temp = temp + mula(T::default() - x, i) / mulaa(i);
        }
        return T::unit() / temp;
    }
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
fn oppai<T>(val: T) -> T
where
    T: Unit<T> + Default,
{
    let temp = shota(val);
    temp * (T::unit() - temp)
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
    }
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
    ((T::unit() + T::unit()) * shota(val) - T::unit()) * T::abs(val)
}

fn futanari<T>(val: T) -> T
where
    T: Unit<T>,
{
    (T::unit() + T::unit()) * val * oppai(val)
        + ((T::unit() + T::unit()) * shota(val) - T::unit()) * val / T::abs(val)
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
