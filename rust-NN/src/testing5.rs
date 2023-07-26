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

fn train(
    x: Array2<f64>,
    y_true: Array2<f64>,
    layers: Vec<usize>,
    epochs: usize,
    learning_rate: f64,
) {
    let mut rng = rand::thread_rng();
    let distr = Uniform::new(0.0, 1.0);

    // Initialize weights and biases for each layer
    let mut weights: Vec<Array2<f64>> = Vec::new();
    let mut biases: Vec<Array1<f64>> = Vec::new();
    for i in 0..layers.len() - 1 {
        weights.push(
            Array::from_iter((0..layers[i] * layers[i + 1]).map(|_| distr.sample(&mut rng)))
                .into_shape((layers[i], layers[i + 1]))
                .unwrap(),
        );
        biases.push(Array::from_iter(
            (0..layers[i + 1]).map(|_| distr.sample(&mut rng)),
        ));
    }

    // Store the output of each layer during forward pass
    let mut outputs: Vec<Array2<f64>> = Vec::new();

    for epoch in 0..epochs {
        outputs.clear();
        outputs.push(x.clone());

        // Forward pass
        for i in 0..weights.len() {
            let output =
                sigmoid(&(outputs[i].dot(&weights[i]) + &biases[i].clone().insert_axis(Axis(0))));
            outputs.push(output);
        }

        let output_error = mse_loss(&y_true, &outputs.last().unwrap());
        let mut d_output_error = 2.0 * (&y_true - outputs.last().unwrap());

        // Backward pass
        for i in (0..weights.len()).rev() {
            let d_weights = outputs[i]
                .t()
                .dot(&(&d_output_error * sigmoid_derivative(&outputs[i + 1])));
            let d_bias = (&d_output_error * sigmoid_derivative(&outputs[i + 1]))
                .mean_axis(Axis(0))
                .unwrap();

            weights[i] -= &(learning_rate * &d_weights); // Change this line
            biases[i] -= &(learning_rate * &d_bias); // And this line

            if i != 0 {
                d_output_error =
                    (&d_output_error.dot(&weights[i].t())) * sigmoid_derivative(&outputs[i]);
            }
            // if i != 0 {
            //     d_output_error =
            //         (&d_output_error * sigmoid_derivative(&outputs[i + 1])).dot(&weights[i].t());
            // }
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

    train(inputs, outputs, vec![3, 5, 2], 100_000, 0.1);
}
