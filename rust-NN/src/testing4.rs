use ndarray::array;
use ndarray::s;
use ndarray::Zip;
use ndarray::{Array, Array1, Array2, Axis};
use rand::distributions::{Distribution, Uniform};
use std::iter::FromIterator;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(x: f64) -> f64 {
    x * (1.0 - x)
}

fn mse_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    let diff = y_true - y_pred;
    diff.iter().map(|&x| x.powi(2)).sum::<f64>() / (diff.shape()[0] * diff.shape()[1]) as f64
}

fn absolute_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    (y_true - y_pred).mapv(f64::abs).mean().unwrap()
}

fn absolute_loss_derivative(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> Array2<f64> {
    let diff = y_true - y_pred;
    diff.mapv(|x| if x > 0.0 { -1.0 } else { 1.0 })
}

fn main() {
    let num_inputs = 3;
    let num_hidden = 5;
    let num_outputs = 2;
    let num_samples = 4;

    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0.0, 1.0);
    let mut weights_1: Array2<f64> =
        Array::from_iter((0..num_inputs * num_hidden).map(|_| distr.sample(&mut rng)))
            .into_shape((num_inputs, num_hidden))
            .unwrap();
    let mut bias_1: Array1<f64> = Array::from_iter((0..num_hidden).map(|_| distr.sample(&mut rng)));
    let mut weights_2: Array2<f64> =
        Array::from_iter((0..num_hidden * num_outputs).map(|_| distr.sample(&mut rng)))
            .into_shape((num_hidden, num_outputs))
            .unwrap();
    let mut bias_2: Array1<f64> =
        Array::from_iter((0..num_outputs).map(|_| distr.sample(&mut rng)));

    let inputs = array![
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0]
    ];
    let outputs = array![[0.0, 1.0], [1.0, 0.0], [1.0, 0.0], [0.0, 1.0]];

    for _epoch in 0..100000 {
        let layer_1_output = (&inputs).dot(&weights_1).mapv(|x| sigmoid(x + bias_1[0]));
        let output = (&layer_1_output)
            .dot(&weights_2)
            .mapv(|x| sigmoid(x + bias_2[0]));

        let output_error = mse_loss(&outputs, &output);
        let d_output_error = 2.0 * (&output - &outputs);

        let d_weights_2 = layer_1_output.t().dot(&d_output_error);
        let d_bias_2 = d_output_error.mean_axis(Axis(0)).unwrap();

        let d_layer_1_error = d_output_error.dot(&weights_2.t());
        let d_weights_1 = inputs.t().dot(&d_layer_1_error);
        let d_bias_1 = d_layer_1_error.mean_axis(Axis(0)).unwrap();

        weights_1 = weights_1 - 0.1 * d_weights_1;
        bias_1 = bias_1 - 0.1 * d_bias_1;
        weights_2 = weights_2 - 0.1 * d_weights_2;
        bias_2 = bias_2 - 0.1 * d_bias_2;

        if _epoch % 1000 == 0 {
            println!("Epoch: {}, Loss: {}", _epoch, output_error);
        }
    }
}
