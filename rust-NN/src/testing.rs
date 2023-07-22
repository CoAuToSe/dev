extern crate ndarray;
use ndarray::prelude::*;
use ndarray::Array;

extern crate ndarray_rand;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn sigmoid(x: &f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: &f64) -> f64 {
    let y = sigmoid(x);
    y * (1.0 - y)
}

fn mse_loss(y_true: ScalarOrVec, y_pred: ScalarOrVec) -> f64 {
    let diff = y_true - y_pred;
    match diff {
        ScalarOrVec::Scalar(val) => val.powi(2),
        ScalarOrVec::Vec(val) => {
            let sum: f64 = val.iter().map(|&x| x.powi(2)).sum();
            sum / (val.len() as f64)
        }
    }
}

fn main() {
    let x = array![
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0]
    ];
    let y_true = array![0.0, 1.0, 1.0, 0.0];

    let weights_1 = Array::random((3, 4), Uniform::new(0.0, 1.0));
    let bias_1 = Array::random(4, Uniform::new(0.0, 1.0));

    let weights_2 = Array::random((4, 1), Uniform::new(0.0, 1.0));
    let bias_2 = Array::random(1, Uniform::new(0.0, 1.0));

    // Forward propagation
    let layer_1_output = (x.dot(&weights_1) + &bias_1).mapv(sigmoid);
    let output = (layer_1_output.dot(&weights_2) + &bias_2).mapv(sigmoid);

    // Calculation of errors
    let output_error = mse_loss(y_true.clone(), output.clone());
    let d_output_error = 2.0 * (&output - &y_true);

    // Calculate gradients here and then use them to update weights and biases

    // The remaining part of the backward propagation and the update of the weights is not included here.
    // This is due to the complexity of those operations in Rust without a higher-level library for matrix operations.
}

use ndarray::s;
use ndarray::Array;
use ndarray::Array2;
use ndarray::Axis;
use std::f64;

struct Network {
    weights_1: ScalarOrVec,
    bias_1: ScalarOrVec,
    weights_2: ScalarOrVec,
    bias_2: ScalarOrVec,
}

impl Network {
    fn train(
        &mut self,
        x: ScalarOrVec,
        y_true: ScalarOrVec,
        epochs: usize,
        learning_rate: ScalarOrVec,
    ) {
        for epoch in 0..epochs {
            // Forward propagation
            let layer_1_output = sigmoid(&x * &self.weights_1.into_vec() + self.bias_1.clone());
            let output = sigmoid(
                dot_product(&layer_1_output.into_vec(), &self.weights_2.into_vec())
                    + self.bias_2.clone(),
            );

            // Backward propagation
            let output_error = mse_loss(y_true.clone(), output.clone());
            let d_output_error = 2.0 * (output - y_true.clone());

            let d_weights_2 = dot_product(
                &layer_1_output.into_vec().transpose(),
                &(d_output_error.clone() * sigmoid_derivative(output.clone())),
            );
            let d_bias_2 = d_output_error.clone() * sigmoid_derivative(output.clone());

            let d_layer_1_error = dot_product(
                &(d_output_error.clone() * sigmoid_derivative(output.clone())),
                &self.weights_2.into_vec().transpose(),
            );
            let d_weights_1 = dot_product(
                &x.transpose(),
                &(d_layer_1_error.clone() * sigmoid_derivative(layer_1_output.into_vec())),
            );
            let d_bias_1 = d_layer_1_error * sigmoid_derivative(layer_1_output.into_vec());

            // Update weights and biases
            self.weights_1 = ScalarOrVec::from_vec(
                self.weights_1
                    .into_vec()
                    .sub_vec(&d_weights_1.mul_scalar(learning_rate)),
            );
            self.bias_1 = self.bias_1 - d_bias_1 * learning_rate;
            self.weights_2 = ScalarOrVec::from_vec(
                self.weights_2
                    .into_vec()
                    .sub_vec(&d_weights_2.mul_scalar(learning_rate)),
            );
            self.bias_2 = self.bias_2 - d_bias_2 * learning_rate;

            if epoch % 100 == 0 {
                println!("Epoch {}, Loss: {}", epoch, output_error.into_scalar());
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ScalarOrVec {
    Scalar(f64),
    Vec(Vec<f64>),
}

fn dot_product(v1: impl Into<ScalarOrVec>, v2: impl Into<ScalarOrVec>) -> Option<f64> {
    let v1_sv = v1.into();
    let v2_sv = v2.into();
    match (v1_sv, v2_sv) {
        (ScalarOrVec::Vec(v1), ScalarOrVec::Vec(v2)) => {
            if v1.len() != v2.len() {
                return None;
            }
            Some(v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum())
        }
        (ScalarOrVec::Scalar(s), ScalarOrVec::Vec(v))
        | (ScalarOrVec::Vec(v), ScalarOrVec::Scalar(s)) => Some(v.iter().map(|a| a * s).sum()),
        _ => None,
    }
}
impl From<f64> for ScalarOrVec {
    fn from(item: f64) -> Self {
        ScalarOrVec::Scalar(item)
    }
}

impl From<Vec<f64>> for ScalarOrVec {
    fn from(item: Vec<f64>) -> Self {
        ScalarOrVec::Vec(item)
    }
}
impl Into<f64> for ScalarOrVec {
    fn into(self) -> f64 {
        match self {
            ScalarOrVec::Scalar(val) => val,
            _ => panic!("Impossible de convertir un Vec<f64> en f64"),
        }
    }
}

impl Into<Vec<f64>> for ScalarOrVec {
    fn into(self) -> Vec<f64> {
        match self {
            ScalarOrVec::Vec(val) => val,
            _ => panic!("Impossible de convertir un f64 en Vec<f64>"),
        }
    }
}
#[test]
fn testing_testing_2023_07_13_17_51() {
    let v1 = ScalarOrVec::Vec(vec![1.0, 2.0, 3.0]);
    let v2 = ScalarOrVec::Vec(vec![4.0, 5.0, 6.0]);
    match dot_product(v1, v2) {
        Some(result) => println!("Dot product: {}", result),
        None => println!("Vectors are not the same length or invalid inputs!"),
    }

    let s1 = ScalarOrVec::Scalar(2.0);
    let v3 = ScalarOrVec::Vec(vec![1.0, 2.0, 3.0]);
    match dot_product(s1, v3) {
        Some(result) => println!("Dot product: {}", result),
        None => println!("Vectors are not the same length or invalid inputs!"),
    }
}

use std::ops::Add;

impl Add for ScalarOrVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Scalar(lhs_val + rhs_val)
            }
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                ScalarOrVec::Vec(rhs_val.iter().map(|x| x + lhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Vec(lhs_val.iter().map(|x| x + rhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                assert_eq!(
                    lhs_val.len(),
                    rhs_val.len(),
                    "Les deux vecteurs doivent avoir la même longueur"
                );
                ScalarOrVec::Vec(
                    lhs_val
                        .iter()
                        .zip(rhs_val.iter())
                        .map(|(x, y)| x + y)
                        .collect(),
                )
            }
        }
    }
}
use std::ops::Sub;

impl Sub for ScalarOrVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Scalar(lhs_val - rhs_val)
            }
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                ScalarOrVec::Vec(rhs_val.iter().map(|x| x - lhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Vec(lhs_val.iter().map(|x| x - rhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                assert_eq!(
                    lhs_val.len(),
                    rhs_val.len(),
                    "Les deux vecteurs doivent avoir la même longueur"
                );
                ScalarOrVec::Vec(
                    lhs_val
                        .iter()
                        .zip(rhs_val.iter())
                        .map(|(x, y)| x - y)
                        .collect(),
                )
            }
        }
    }
}

use std::ops::Mul;

impl Mul for ScalarOrVec {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Scalar(lhs_val * rhs_val)
            }
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                ScalarOrVec::Vec(rhs_val.iter().map(|x| x * lhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Vec(lhs_val.iter().map(|x| x * rhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                assert_eq!(
                    lhs_val.len(),
                    rhs_val.len(),
                    "Les deux vecteurs doivent avoir la même longueur"
                );
                ScalarOrVec::Vec(
                    lhs_val
                        .iter()
                        .zip(rhs_val.iter())
                        .map(|(x, y)| x * y)
                        .collect(),
                )
            }
        }
    }
}

use std::ops::Div;

impl Div for ScalarOrVec {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Scalar(lhs_val / rhs_val)
            }
            (ScalarOrVec::Scalar(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                ScalarOrVec::Vec(rhs_val.iter().map(|x| x / lhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Scalar(rhs_val)) => {
                ScalarOrVec::Vec(lhs_val.iter().map(|x| x / rhs_val).collect())
            }
            (ScalarOrVec::Vec(lhs_val), ScalarOrVec::Vec(rhs_val)) => {
                assert_eq!(
                    lhs_val.len(),
                    rhs_val.len(),
                    "Les deux vecteurs doivent avoir la même longueur"
                );
                ScalarOrVec::Vec(
                    lhs_val
                        .iter()
                        .zip(rhs_val.iter())
                        .map(|(x, y)| x / y)
                        .collect(),
                )
            }
        }
    }
}

impl<'a, 'b> Add<&'b ScalarOrVec> for &'a ScalarOrVec {
    type Output = ScalarOrVec;

    fn add(self, other: &'b ScalarOrVec) -> ScalarOrVec {
        match (self, other) {
            (ScalarOrVec::Scalar(scalar), ScalarOrVec::Scalar(scalar2)) => {
                ScalarOrVec::Scalar(scalar + scalar2)
            }
            (ScalarOrVec::Scalar(scalar), ScalarOrVec::Vec(vec))
            | (ScalarOrVec::Vec(vec), ScalarOrVec::Scalar(scalar)) => {
                ScalarOrVec::Vec(vec.iter().map(|x| x + scalar).collect())
            }
            (ScalarOrVec::Vec(vec1), ScalarOrVec::Vec(vec2)) => {
                assert_eq!(
                    vec1.len(),
                    vec2.len(),
                    "Vectors must be the same length for element-wise addition"
                );
                ScalarOrVec::Vec(vec1.iter().zip(vec2.iter()).map(|(x, y)| x + y).collect())
            }
        }
    }
}
impl<'a, 'b> Add<&'b ScalarOrVec> for &'a mut ScalarOrVec {
    type Output = ScalarOrVec;

    fn add(self, other: &'b ScalarOrVec) -> ScalarOrVec {
        match (self, other) {
            (ScalarOrVec::Scalar(scalar), ScalarOrVec::Scalar(scalar2)) => {
                ScalarOrVec::Scalar(*scalar + *scalar2)
            }
            (ScalarOrVec::Scalar(scalar), ScalarOrVec::Vec(vec)) => {
                ScalarOrVec::Vec(vec.iter().map(|x| x + *scalar).collect())
            }
            (ScalarOrVec::Vec(vec), ScalarOrVec::Scalar(scalar)) => {
                ScalarOrVec::Vec(vec.iter().map(|x| x + scalar).collect())
            }
            (ScalarOrVec::Vec(vec1), ScalarOrVec::Vec(vec2)) => {
                assert_eq!(
                    vec1.len(),
                    vec2.len(),
                    "Vectors must be the same length for element-wise addition"
                );
                ScalarOrVec::Vec(vec1.iter().zip(vec2.iter()).map(|(x, y)| x + y).collect())
            }
        }
    }
}
