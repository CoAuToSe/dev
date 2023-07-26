use ndarray::{array, s, Array, Array1, Array2, Axis};
use rand::distributions::{Distribution, Uniform};
use std::f64;
use std::iter::FromIterator;

fn sigmoid(x: &Array2<f64>) -> Array2<f64> {
    let one = Array2::<f64>::ones(x.raw_dim());
    &one / (&one + (-x).mapv(f64::exp))
}

fn sigmoid_derivative(x: &Array2<f64>) -> Array2<f64> {
    x * &(Array2::<f64>::ones(x.raw_dim()) - x)
}

fn mse_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    let diff = y_true - y_pred;
    diff.iter().map(|x| x.powi(2)).sum::<f64>() / (diff.shape()[0] * diff.shape()[1]) as f64
}

fn train(x: Array2<f64>, y_true: Array2<f64>, epochs: usize, learning_rate: f64) {
    let num_inputs = 3;
    let num_hidden_1 = 5;
    let num_hidden_2 = 4;
    let num_outputs = 2;

    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0.0, 1.0);

    let mut weights_1: Array2<f64> =
        Array::from_iter((0..num_inputs * num_hidden_1).map(|_| distr.sample(&mut rng)))
            .into_shape((num_inputs, num_hidden_1))
            .unwrap();
    let mut bias_1: Array1<f64> =
        Array::from_iter((0..num_hidden_1).map(|_| distr.sample(&mut rng)));

    let mut weights_2: Array2<f64> =
        Array::from_iter((0..num_hidden_1 * num_hidden_2).map(|_| distr.sample(&mut rng)))
            .into_shape((num_hidden_1, num_hidden_2))
            .unwrap();
    let mut bias_2: Array1<f64> =
        Array::from_iter((0..num_hidden_2).map(|_| distr.sample(&mut rng)));

    let mut weights_3: Array2<f64> =
        Array::from_iter((0..num_hidden_2 * num_outputs).map(|_| distr.sample(&mut rng)))
            .into_shape((num_hidden_2, num_outputs))
            .unwrap();
    let mut bias_3: Array1<f64> =
        Array::from_iter((0..num_outputs).map(|_| distr.sample(&mut rng)));

    for epoch in 0..epochs {
        let layer_1_output = sigmoid(&(x.dot(&weights_1) + &bias_1.clone().insert_axis(Axis(0))));
        let layer_2_output =
            sigmoid(&(layer_1_output.dot(&weights_2) + &bias_2.clone().insert_axis(Axis(0))));
        let output =
            sigmoid(&(layer_2_output.dot(&weights_3) + &bias_3.clone().insert_axis(Axis(0))));

        let output_error = mse_loss(&y_true, &output);
        let d_output_error = 2.0 * (&output - &y_true);

        let d_weights_3 = layer_2_output
            .t()
            .dot(&(&d_output_error * sigmoid_derivative(&output)));
        let d_bias_3 = (&d_output_error * sigmoid_derivative(&output))
            .mean_axis(Axis(0))
            .unwrap();

        let d_layer_2_error = (&d_output_error * sigmoid_derivative(&output)).dot(&weights_3.t());
        let d_weights_2 = layer_1_output
            .t()
            .dot(&(&d_layer_2_error * sigmoid_derivative(&layer_2_output)));
        let d_bias_2 = (&d_layer_2_error * sigmoid_derivative(&layer_2_output))
            .mean_axis(Axis(0))
            .unwrap();

        let d_layer_1_error =
            (&d_layer_2_error * sigmoid_derivative(&layer_2_output)).dot(&weights_2.t());
        let d_weights_1 = x
            .t()
            .dot(&(&d_layer_1_error * sigmoid_derivative(&layer_1_output)));
        let d_bias_1 = (&d_layer_1_error * sigmoid_derivative(&layer_1_output))
            .mean_axis(Axis(0))
            .unwrap();

        weights_1 = weights_1 - learning_rate * d_weights_1;
        bias_1 = bias_1 - learning_rate * d_bias_1;
        weights_2 = weights_2 - learning_rate * d_weights_2;
        bias_2 = bias_2 - learning_rate * d_bias_2;
        weights_3 = weights_3 - learning_rate * d_weights_3;
        bias_3 = bias_3 - learning_rate * d_bias_3;

        if epoch % 100 == 0 {
            println!("Epoch {}, Loss: {} ;", epoch, output_error);
        }
    }
}

fn main() {
    let inputs = array![
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0]
    ];
    let outputs = array![[0.0, 1.0], [1.0, 0.0], [1.0, 0.0], [0.0, 1.0]];

    train(inputs, outputs, 100_000, 0.1);
}
