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

fn relu(x: &Array2<f64>) -> Array2<f64> {
    x.mapv(|a| a.max(0.0))
}

fn relu_derivative(x: &Array2<f64>) -> Array2<f64> {
    x.mapv(|a| if a > 0.0 { 1.0 } else { 0.0 })
}

fn mse_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    let diff = y_true - y_pred;
    diff.iter().map(|x| x.powi(2)).sum::<f64>() / (diff.shape()[0] * diff.shape()[1]) as f64
}

fn mse_loss_derivative(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> Array2<f64> {
    2.0 * (y_pred - y_true)
}

fn mae_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    (y_true - y_pred).mapv(f64::abs).mean().unwrap()
}

fn mae_loss_derivative(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> Array2<f64> {
    let diff = y_true - y_pred;
    diff.mapv(|x| if x > 0.0 { -1.0 } else { 1.0 })
}

enum LossFunction {
    MSE,
    MAE,
}

impl LossFunction {
    fn loss(&self, y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
        match self {
            LossFunction::MSE => mse_loss(y_true, y_pred),
            LossFunction::MAE => mae_loss(y_true, y_pred),
        }
    }

    fn derivative(&self, y_true: &Array2<f64>, y_pred: &Array2<f64>) -> Array2<f64> {
        match self {
            LossFunction::MSE => mse_loss_derivative(y_true, y_pred),
            LossFunction::MAE => mae_loss_derivative(y_true, y_pred),
        }
    }
}
enum ActivationFunction {
    Sigmoid,
    Relu,
}

impl ActivationFunction {
    fn function(&self, x: &Array2<f64>) -> Array2<f64> {
        match self {
            ActivationFunction::Sigmoid => sigmoid(x),
            ActivationFunction::Relu => relu(x),
        }
    }

    fn derivative(&self, x: &Array2<f64>) -> Array2<f64> {
        match self {
            ActivationFunction::Sigmoid => sigmoid_derivative(x),
            ActivationFunction::Relu => relu_derivative(x),
        }
    }
}

macro_rules! printol {
    ($($arg:tt)*) => {
        use std::io::Write;
        let mut stdout = std::io::stdout();
        write!(stdout, "\r{}", format_args!($($arg)*)).unwrap();
        stdout.flush().unwrap();
    };
}
fn train(
    x: Array2<f64>,
    y_true: Array2<f64>,
    epochs: usize,
    learning_rate: f64,
    layers: &[usize],
    loss_fn: LossFunction,
    activation_fn: ActivationFunction,
    target_accuracy: f64,
) {
    let num_samples = x.shape()[0];
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0.0, 1.0);

    let mut weights: Vec<Array2<f64>> = Vec::new();
    let mut biases: Vec<Array1<f64>> = Vec::new();
    for i in 0..(layers.len() - 1) {
        let weights_i =
            Array::from_iter((0..layers[i] * layers[i + 1]).map(|_| distr.sample(&mut rng)))
                .into_shape((layers[i], layers[i + 1]))
                .unwrap();
        let bias_i = Array::from_iter((0..layers[i + 1]).map(|_| distr.sample(&mut rng)));

        weights.push(weights_i);
        biases.push(bias_i);
    }

    for epoch in 0..epochs {
        let mut outputs: Vec<Array2<f64>> = Vec::new();
        outputs.push(x.clone());

        // Forward pass
        for i in 0..weights.len() {
            let output_i = activation_fn
                .function(&(outputs[i].dot(&weights[i]) + &biases[i].clone().insert_axis(Axis(0))));
            outputs.push(output_i);
        }
        let output_error = loss_fn.loss(&y_true, &outputs.last().unwrap());
        // Stop training when the target accuracy is reached
        if output_error < target_accuracy {
            println!(
                "Target accuracy reached at epoch {}: Loss: {}",
                epoch, output_error
            );
            break;
        }

        // Compute error
        let y_pred = outputs[outputs.len() - 1].clone();
        let output_error = loss_fn.loss(&y_true, &y_pred);

        // Backward pass
        let mut d_output_error = loss_fn.derivative(&y_true, &y_pred);
        for i in (0..weights.len()).rev() {
            let d_weights = outputs[i]
                .t()
                .dot(&(&d_output_error * activation_fn.derivative(&outputs[i + 1])));
            let d_bias = (&d_output_error * activation_fn.derivative(&outputs[i + 1]))
                .mean_axis(Axis(0))
                .unwrap();

            weights[i] -= &(learning_rate * d_weights);
            biases[i] -= &(learning_rate * d_bias);

            if i != 0 {
                d_output_error =
                    d_output_error.dot(&weights[i].t()) * activation_fn.derivative(&outputs[i]);
            }
        }

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

    train(
        inputs,
        outputs,
        100_000,
        0.1,
        &[3, 5, 5, 2],
        LossFunction::MAE,
        ActivationFunction::Sigmoid,
        0.001,
    );
}
