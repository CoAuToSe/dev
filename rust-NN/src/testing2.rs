extern crate nalgebra as na;
use na::{DMatrix, DVector, Scalar, U1};
use rand::Rng;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: f64) -> f64 {
    x * (1.0 - x)
}

fn mse_loss(y_true: &DVector<f64>, y_pred: &DVector<f64>) -> f64 {
    let diff = y_true - y_pred;
    diff.norm_squared() / (y_true.len() as f64)
}

// use nalgebra::{DMatrix, DVector, U1};
use std::iter::repeat;

fn broadcast_vector_to_matrix_col(vector: &DVector<f64>, num_cols: usize) -> DMatrix<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(vector.len() * num_cols);
    for _ in 0..num_cols {
        data.extend_from_slice(&vector.data.as_vec());
    }
    DMatrix::from_vec(vector.len(), num_cols, data)
}

fn broadcast_vector_to_matrix_row(vector: &DVector<f64>, num_rows: usize) -> DMatrix<f64> {
    let mut data: Vec<f64> = Vec::with_capacity(vector.len() * num_rows);
    for _ in 0..num_rows {
        data.extend_from_slice(&vector.data.as_vec());
    }
    DMatrix::from_vec(num_rows, vector.len(), data)
}

fn reduce_matrix_to_vector_col(matrix: &DMatrix<f64>) -> Option<DVector<f64>> {
    let first_col = matrix.column(0);
    for i in 1..matrix.ncols() {
        if matrix.column(i) != first_col {
            return None;
        }
    }
    Some(DVector::from_vec(
        first_col.iter().cloned().collect::<Vec<f64>>(),
    ))
}

fn reduce_matrix_to_vector_row(matrix: &DMatrix<f64>) -> Option<DVector<f64>> {
    let first_row = matrix.row(0);
    for i in 1..matrix.nrows() {
        if matrix.row(i) != first_row {
            return None;
        }
    }
    Some(DVector::from_vec(
        first_row.iter().cloned().collect::<Vec<f64>>(),
    ))
}

fn train(mut x: DMatrix<f64>, y_true: DVector<f64>, epochs: usize, learning_rate: f64) {
    let mut rng = rand::thread_rng();

    let mut weights_1: DMatrix<f64> = DMatrix::from_fn(3, 4, |_, _| rng.gen());
    let mut bias_1: DVector<f64> = DVector::from_fn(4, |_, _| rng.gen());
    let mut weights_2: DMatrix<f64> = DMatrix::from_fn(4, 1, |_, _| rng.gen());
    let mut bias_2: DVector<f64> = DVector::from_fn(1, |_, _| rng.gen());
    println!("a");
    for epoch in 0..epochs {
        let bias_1_matrix = broadcast_vector_to_matrix_col(&bias_1, x.nrows());
        println!(
            "b {:#?}",
            (
                x.clone(),
                weights_1.clone(),
                bias_1.clone(),
                bias_1_matrix.clone()
            )
        );
        // Forward propagation
        let layer_1_output = (x.clone() * weights_1.clone() + bias_1_matrix).map(sigmoid);

        // let layer_1_output = (x.clone() * weights_1.clone() + bias_1_matrix.clone()).map(sigmoid);
        println!("ba");
        let bias_2_matrix = broadcast_vector_to_matrix_row(&bias_2, layer_1_output.nrows());
        let output =
            (layer_1_output.clone() * weights_2.clone() + bias_2_matrix.clone()).map(sigmoid);

        println!(
            "c {:#?}",
            (
                layer_1_output.clone(),
                weights_2.clone(),
                bias_2_matrix.clone(),
                &output
            )
        );

        // Backward propagation
        let output_error = mse_loss(&y_true, &reduce_matrix_to_vector_col(&output).unwrap());
        let d_output_error = 2.0 * (output.clone() - y_true.clone());

        println!("d");
        println!(
            "layer_1_output.transpose() dimensions: {:?}",
            layer_1_output.transpose().shape()
        );
        println!("d_output_error dimensions: {:?}", d_output_error.shape());
        println!(
            "output.map(sigmoid_derivative) dimensions: {:?}",
            output.map(sigmoid_derivative).shape()
        );

        let temp1 = d_output_error.clone() * output.clone().map(sigmoid_derivative);
        println!("{:#?}", temp1);
        let d_weights_2 = layer_1_output.clone().transpose() * temp1;
        println!("e");
        let d_bias_2 = d_output_error.clone() * output.map(sigmoid_derivative);
        println!("f");

        let d_layer_1_error = d_output_error * weights_2.transpose();
        println!("g");
        let d_weights_1 = x.clone().transpose()
            * (d_layer_1_error.clone() * layer_1_output.clone().map(sigmoid_derivative));
        println!("h");
        let d_bias_1 = d_layer_1_error * layer_1_output.map(sigmoid_derivative);
        println!("i");

        // Update weights and biases
        weights_1 -= learning_rate * d_weights_1;
        bias_1 -= learning_rate * d_bias_1;
        weights_2 -= learning_rate * d_weights_2;
        bias_2 -= learning_rate * d_bias_2;

        if epoch % 100 == 0 {
            println!("Epoch {}, Loss: {}", epoch, output_error);
        }
    }
}

fn main() {
    // For testing
    let inputs = DMatrix::from_row_slice(
        4,
        3,
        &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0],
    );
    let outputs = DVector::from_column_slice(&[0.0, 1.0, 1.0, 0.0]);

    train(inputs, outputs, 1000, 0.1);
}
