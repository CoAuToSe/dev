struct Array {
    data: Vec<f64>,
}

impl Array {
    fn new(data: Vec<f64>) -> Self {
        Array { data }
    }

    fn add(&self, other: &Array) -> Array {
        let result = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a + b)
            .collect::<Vec<f64>>();
        Array::new(result)
    }

    fn sub(&self, other: &Array) -> Array {
        let result = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a - b)
            .collect::<Vec<f64>>();
        Array::new(result)
    }

    fn mul(&self, other: &Array) -> Array {
        let result = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a * b)
            .collect::<Vec<f64>>();
        Array::new(result)
    }

    fn div(&self, other: &Array) -> Array {
        let result = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a / b)
            .collect::<Vec<f64>>();
        Array::new(result)
    }

    fn pow(&self, exponent: f64) -> Array {
        let result = self
            .data
            .iter()
            .map(|a| a.powf(exponent))
            .collect::<Vec<f64>>();
        Array::new(result)
    }

    fn neg(&self) -> Array {
        let result = self.data.iter().map(|a| -a).collect::<Vec<f64>>();
        Array::new(result)
    }

    fn sum(&self) -> f64 {
        self.data.iter().sum()
    }

    fn mean(&self) -> f64 {
        self.sum() / self.data.len() as f64
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
        let result = self.data.iter().map(|a| a.sqrt()).collect::<Vec<f64>>();
        Array::new(result)
    }

    fn sin(&self) -> Array {
        let result = self.data.iter().map(|a| a.sin()).collect::<Vec<f64>>();
        Array::new(result)
    }

    fn exp(&self) -> Array {
        let result = self.data.iter().map(|a| a.exp()).collect::<Vec<f64>>();
        Array::new(result)
    }

    fn log(&self) -> Array {
        let result = self.data.iter().map(|a| a.ln()).collect::<Vec<f64>>();
        Array::new(result)
    }

    fn dot(&self, other: &Array) -> f64 {
        self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum()
    }
}

fn main() {
    let array1 = Array::new(vec![1.0, 2.0, 3.0]);
    let array2 = Array::new(vec![4.0, 5.0, 6.0]);

    let addition_result = array1.add(&array2);
    let subtraction_result = array1.sub(&array2);
    let multiplication_result = array1.mul(&array2);
    let division_result = array1.div(&array2);
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

    println!("Addition: {:?}", addition_result.data);
    println!("Subtraction: {:?}", subtraction_result.data);
    println!("Multiplication: {:?}", multiplication_result.data);
    println!("Division: {:?}", division_result.data);
    println!("Exponentiation: {:?}", exponentiation_result.data);
    println!("Negation: {:?}", negation_result.data);
    println!("Sum: {}", sum_result);
    println!("Mean: {}", mean_result);
    println!("Product: {}", product_result);
    println!("Minimum: {}", min_result);
    println!("Maximum: {}", max_result);
    println!("Square Root: {:?}", sqrt_result.data);
    println!("Sine: {:?}", sin_result.data);
    println!("Exponential: {:?}", exp_result.data);
    println!("Logarithm: {:?}", log_result.data);
    println!("Dot Product: {}", dot_product_result);
}
