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

macro_rules! printol {
    ($($arg:tt)*) => {
        use std::io::Write;
        let mut stdout = std::io::stdout();
        write!(stdout, "\r{}", format_args!($($arg)*)).unwrap();
        stdout.flush().unwrap();
    };
}
fn train(x: Array2<f64>, y_true: Array2<f64>, epochs: usize, learning_rate: f64, layers: &[usize]) {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0.0, 1.0);

    let mut weights: Vec<Array2<f64>> = Vec::new();
    let mut biases: Vec<Array1<f64>> = Vec::new();

    for i in 0..(layers.len() - 1) {
        let w: Array2<f64> =
            Array::from_iter((0..layers[i] * layers[i + 1]).map(|_| distr.sample(&mut rng)))
                .into_shape((layers[i], layers[i + 1]))
                .unwrap();
        let b: Array1<f64> = Array::from_iter((0..layers[i + 1]).map(|_| distr.sample(&mut rng)));
        weights.push(w);
        biases.push(b);
    }

    for epoch in 0..epochs {
        let mut layer_outputs: Vec<Array2<f64>> = Vec::new();

        // Forward propagation
        let mut layer_input = x.clone();
        for i in 0..(layers.len() - 1) {
            let layer_output =
                sigmoid(&(layer_input.dot(&weights[i]) + &biases[i].clone().insert_axis(Axis(0))));
            layer_outputs.push(layer_output.clone());
            layer_input = layer_output;
        }

        let output = layer_outputs.last().unwrap().clone();
        let output_error = mse_loss(&y_true, &output);

        if epoch % 100 == 0 {
            printol!("Epoch {}, Loss: {} ;", epoch, output_error);
        }

        // Backward propagation
        let mut prev_error = 2.0 * (&output - &y_true);
        for i in (0..(layers.len() - 1)).rev() {
            let d_error = &prev_error * sigmoid_derivative(&layer_outputs[i]);
            let d_weights = layer_outputs
                .get(i.wrapping_sub(1))
                .unwrap_or(&x)
                .t()
                .dot(&d_error);
            let d_biases = d_error.mean_axis(Axis(0)).unwrap();

            weights[i] -= &(learning_rate * d_weights);
            biases[i] -= &(learning_rate * d_biases);

            prev_error = d_error.dot(&weights[i].t());
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

    let layers = &[3, 5, 5, 2]; // You can set as many layers as you want here.

    train(inputs, outputs, 100_000, 0.1, layers);
}
