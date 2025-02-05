use std::ops::{Add, Div, Mul, Sub};

// #[derive(Debug, Clone)]
// struct Array {
//     shape: Vec<usize>,
//     data: Vec<f64>,
// }

pub struct Array<T, const SIZE: usize> {
    shape: [usize; SIZE],
    data: Vec<T>,
}
/*
impl Array {
    fn new(shape: Vec<usize>, data: Vec<f64>) -> Array {
        Array { shape, data }
    }

    // fn get_index(&self, indices: &[usize]) -> usize {
    //     let mut flat_index = 0;
    //     let mut stride = 1;
    //     for &i in indices.iter().rev() {
    //         flat_index += i * stride;
    //         stride *= self.shape[self.shape.len() - 1 - flat_index];
    //     }
    //     flat_index
    // }
    fn get_index(&self, indices: &[usize]) -> usize {
        let mut flat_index = 0;
        let mut stride = 1;
        for i in (0..indices.len()).rev() {
            if let Some(&shape_i) = self.shape.get(i) {
                if indices[i] < shape_i {
                    flat_index += indices[i] * stride;
                    stride *= shape_i;
                } else {
                    // Handle the case where an index is out of bounds
                    panic!("Index out of bounds");
                }
            } else {
                // Handle the case where the index is larger than the shape length
                panic!("Invalid index");
            }
        }
        flat_index
    }

    fn get(&self, indices: &[usize]) -> f64 {
        let index = self.get_index(indices);
        self.data[index]
    }

    fn broadcast_shape(&self, other_shape: &[usize]) -> Vec<usize> {
        let mut result_shape = vec![];
        for (s1, s2) in self.shape.iter().rev().zip(other_shape.iter().rev()) {
            if s1 == s2 || *s1 == 1 || *s2 == 1 {
                result_shape.push(std::cmp::max(*s1, *s2));
            } else {
                panic!("Incompatible shapes for broadcasting");
            }
        }
        result_shape.reverse();
        result_shape
    }

    fn broadcast_to(&self, target_shape: &[usize]) -> Array {
        if self.shape == target_shape {
            return self.clone();
        }
        let new_data = (0..self.size(target_shape))
            .map(|i| {
                let indices = self.indices_from_flat_index(i, target_shape);
                self.get(&indices)
            })
            .collect();
        Array::new(target_shape.to_vec(), new_data)
    }

    fn indices_from_flat_index(&self, flat_index: usize, shape: &[usize]) -> Vec<usize> {
        let mut indices = vec![];
        let mut stride = 1;
        for &s in shape.iter().rev() {
            indices.push((flat_index / stride) % s);
            stride *= s;
        }
        indices.reverse();
        indices
    }

    fn size(&self, shape: &[usize]) -> usize {
        shape.iter().product()
    }

    fn dot(&self, other: &Array) -> Array {
        if self.shape.len() == 2 && other.shape.len() == 2 {
            if self.shape[1] == other.shape[0] {
                let mut result_data = vec![];
                for i in 0..self.shape[0] {
                    let mut row = vec![];
                    for j in 0..other.shape[1] {
                        let element: f64 = (0..self.shape[1])
                            .map(|k| self.get(&[i, k]) * other.get(&[k, j]))
                            .sum();
                        row.push(element);
                    }
                    result_data.extend(row);
                }
                Array::new(vec![self.shape[0], other.shape[1]], result_data)
            } else {
                panic!("Incompatible shapes for dot product");
            }
        } else {
            panic!("Both arrays must be 2D for dot product");
        }
    }
}

impl Add<&Array> for &Array {
    type Output = Array;

    fn add(self, other: &Array) -> Array {
        if self.shape != other.shape {
            let new_shape = self.broadcast_shape(&other.shape);
            let array1 = self.broadcast_to(&new_shape);
            let array2 = other.broadcast_to(&new_shape);
            let new_data = array1
                .data
                .iter()
                .zip(array2.data.iter())
                .map(|(a, b)| a + b)
                .collect();
            Array::new(array1.shape, new_data)
        } else {
            let new_data = self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect();
            Array::new(self.shape.clone(), new_data)
        }
    }
}

impl Sub<&Array> for &Array {
    type Output = Array;

    fn sub(self, other: &Array) -> Array {
        if self.shape != other.shape {
            let new_shape = self.broadcast_shape(&other.shape);
            let array1 = self.broadcast_to(&new_shape);
            let array2 = other.broadcast_to(&new_shape);
            let new_data = array1
                .data
                .iter()
                .zip(array2.data.iter())
                .map(|(a, b)| a - b)
                .collect();
            Array::new(array1.shape, new_data)
        } else {
            let new_data = self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect();
            Array::new(self.shape.clone(), new_data)
        }
    }
}

impl Mul<&Array> for &Array {
    type Output = Array;

    fn mul(self, other: &Array) -> Array {
        if self.shape != other.shape {
            let new_shape = self.broadcast_shape(&other.shape);
            let array1 = self.broadcast_to(&new_shape);
            let array2 = other.broadcast_to(&new_shape);
            let new_data = array1
                .data
                .iter()
                .zip(array2.data.iter())
                .map(|(a, b)| a * b)
                .collect();
            Array::new(array1.shape, new_data)
        } else {
            let new_data = self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .collect();
            Array::new(self.shape.clone(), new_data)
        }
    }
}

impl Div<&Array> for &Array {
    type Output = Array;

    fn div(self, other: &Array) -> Array {
        if self.shape != other.shape {
            let new_shape = self.broadcast_shape(&other.shape);
            let array1 = self.broadcast_to(&new_shape);
            let array2 = other.broadcast_to(&new_shape);
            let new_data = array1
                .data
                .iter()
                .zip(array2.data.iter())
                .map(|(a, b)| a / b)
                .collect();
            Array::new(array1.shape, new_data)
        } else {
            let new_data = self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a / b)
                .collect();
            Array::new(self.shape.clone(), new_data)
        }
    }
}

impl Array {
    fn pow(&self, exponent: f64) -> Array {
        let new_data = self.data.iter().map(|a| a.powf(exponent)).collect();
        Array::new(self.shape.clone(), new_data)
    }

    fn neg(&self) -> Array {
        let new_data = self.data.iter().map(|a| -a).collect();
        Array::new(self.shape.clone(), new_data)
    }

    fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    fn mean(&self) -> f64 {
        self.data.iter().sum::<f64>() / self.data.len() as f64
    }

    fn product(&self) -> f64 {
        self.data.iter().product()
    }

    fn min(&self) -> f64 {
        *self
            .data
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    fn max(&self) -> f64 {
        *self
            .data
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    fn sqrt(&self) -> Array {
        let new_data = self.data.iter().map(|a| a.sqrt()).collect();
        Array::new(self.shape.clone(), new_data)
    }

    fn sin(&self) -> Array {
        let new_data = self.data.iter().map(|a| a.sin()).collect();
        Array::new(self.shape.clone(), new_data)
    }

    fn exp(&self) -> Array {
        let new_data = self.data.iter().map(|a| a.exp()).collect();
        Array::new(self.shape.clone(), new_data)
    }

    fn log(&self) -> Array {
        let new_data = self.data.iter().map(|a| a.ln()).collect();
        Array::new(self.shape.clone(), new_data)
    }
}
*/
fn main() {
    let array_shape1: Vec<usize> = vec![2, 3];
    let array_shape2: Vec<usize> = vec![1, 3];

    let array_data1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let array_data2: Vec<f64> = vec![7.0, 8.0, 9.0];

    let array1 = Array::new(array_shape1.clone(), array_data1.clone());
    let array2 = Array::new(array_shape2.clone(), array_data2.clone());

    // Your Array operations
    let addition_result = &array1 + &array2;
    let subtraction_result = &array1 - &array2;
    let multiplication_result = &array1 * &array2;
    let division_result = &array1 / &array2;
    let exponentiation_result = array1.pow(2.0);
    let negation_result = array1.neg();
    let sum_result = array1.sum();
    let mean_result = array1.mean();
    let product_result = array1.product();
    let min_result = array1.min();
    let max_result = array1.max();
    let sqrt_result = array1.sqrt();
    let sin_result = array1.sin();
    let exp_result = array1.exp();
    let log_result = array1.log();
    let dot_product_result = array1.dot(&array2);

    // Compare results
    println!("Addition (Array): {:?}", addition_result.data);
    println!("Subtraction (Array): {:?}", subtraction_result.data);
    println!("Multiplication (Array): {:?}", multiplication_result.data);
    println!("Division (Array): {:?}", division_result.data);
    println!("Exponentiation (Array): {:?}", exponentiation_result.data);
    println!("Negation (Array): {:?}", negation_result.data);
    println!("Sum (Array): {}", sum_result);
    println!("Mean (Array): {}", mean_result);
    println!("Product (Array): {}", product_result);
    println!("Min (Array): {}", min_result);
    println!("Max (Array): {}", max_result);
    println!("Sqrt (Array): {:?}", sqrt_result.data);
    println!("Sin (Array): {:?}", sin_result.data);
    println!("Exp (Array): {:?}", exp_result.data);
    println!("Log (Array): {:?}", log_result.data);
    println!("Dot Product (Array): {:?}", dot_product_result.data);
}
