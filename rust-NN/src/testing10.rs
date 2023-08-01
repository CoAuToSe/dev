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
struct OptimizerGD {
    learning_rate: f64,
}

impl OptimizerGD {
    fn new(learning_rate: f64) -> Self {
        OptimizerGD { learning_rate }
    }

    fn update(&mut self, weights: &mut Array2<f64>, gradients: &Array2<f64>) {
        *weights -= &(self.learning_rate * gradients);
    }
}

struct OptimizerAdam {
    learning_rate: f64,
    beta_1: f64,
    beta_2: f64,
    epsilon: f64,
    t: usize,
    m: Array2<f64>,
    v: Array2<f64>,
}

impl OptimizerAdam {
    fn new(
        learning_rate: f64,
        beta_1: f64,
        beta_2: f64,
        epsilon: f64,
        weight_dims: (usize, usize),
    ) -> Self {
        let m = Array::zeros(weight_dims);
        let v = Array::zeros(weight_dims);
        OptimizerAdam {
            learning_rate,
            beta_1,
            beta_2,
            epsilon,
            t: 0,
            m,
            v,
        }
    }

    fn update(&mut self, weights: &mut Array2<f64>, gradients: &Array2<f64>) {
        self.t += 1;
        self.m = self.beta_1 * &self.m + (1.0 - self.beta_1) * gradients;
        self.v = self.beta_2 * &self.v + (1.0 - self.beta_2) * gradients.mapv(|x| x.powi(2));

        let m_hat = &self.m / (1.0 - self.beta_1.powi(self.t as i32));
        let v_hat = &self.v / (1.0 - self.beta_2.powi(self.t as i32));

        *weights -= &((&m_hat / (&v_hat.mapv(|x| x.sqrt()) + self.epsilon)) * self.learning_rate);
    }
}
struct OptimizerNadam {
    learning_rate: f64,
    beta_1: f64,
    beta_2: f64,
    epsilon: f64,
    t: usize,
    m: Array2<f64>,
    v: Array2<f64>,
}

impl OptimizerNadam {
    fn new(
        learning_rate: f64,
        beta_1: f64,
        beta_2: f64,
        epsilon: f64,
        weight_dims: (usize, usize),
    ) -> Self {
        let m = Array::zeros(weight_dims);
        let v = Array::zeros(weight_dims);
        OptimizerNadam {
            learning_rate,
            beta_1,
            beta_2,
            epsilon,
            t: 0,
            m,
            v,
        }
    }

    fn update(&mut self, weights: &mut Array2<f64>, gradients: &Array2<f64>) {
        self.t += 1;
        self.m = self.beta_1 * &self.m + (1.0 - self.beta_1) * gradients;
        self.v = self.beta_2 * &self.v + (1.0 - self.beta_2) * gradients.mapv(|x| x.powi(2));

        let m_hat = &self.m / (1.0 - self.beta_1.powi(self.t as i32));
        let v_hat = &self.v / (1.0 - self.beta_2.powi(self.t as i32));

        let nesterov_momentum = self.beta_1 * m_hat
            + ((1.0 - self.beta_1) / (1.0 - self.beta_1.powi(self.t as i32))) * gradients;

        *weights -= &((&nesterov_momentum / (&v_hat.mapv(|x| x.sqrt()) + self.epsilon))
            * self.learning_rate);
    }
}

enum Optimizer {
    GD(OptimizerGD),
    Adam(OptimizerAdam),
    Nadam(OptimizerNadam),
}

impl Optimizer {
    fn update(&mut self, weights: &mut Array2<f64>, gradients: &Array2<f64>) {
        match self {
            Optimizer::GD(opt) => opt.update(weights, gradients),
            Optimizer::Adam(opt) => opt.update(weights, gradients),
            Optimizer::Nadam(opt) => opt.update(weights, gradients),
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
struct Network {
    weights: Array2<f64>,
    bias: Array2<f64>,
    activation: ActivationFunction,
}

impl Network {
    fn new(weight_dims: (usize, usize), bias_dim: usize, activation: ActivationFunction) -> Self {
        let between = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let weights = Array2::from_shape_vec(
            weight_dims,
            between
                .sample_iter(&mut rng)
                .take(weight_dims.0 * weight_dims.1)
                .collect(),
        )
        .unwrap();
        let bias = Array2::from_shape_vec(
            (bias_dim, 1),
            between.sample_iter(&mut rng).take(bias_dim).collect(),
        )
        .unwrap();
        Network {
            weights,
            bias,
            activation,
        }
    }

    fn forward(&self, x: &Array2<f64>) -> Array2<f64> {
        let z = x.dot(&self.weights) + &self.bias;
        self.activation.function(&z)
    }

    fn backward(&self, x: &Array2<f64>, grad: &Array2<f64>) -> (Array2<f64>, Array2<f64>) {
        let m = x.t().dot(grad);
        let n = grad
            .sum_axis(Axis(0))
            .into_shape((self.bias.len(), 1))
            .unwrap();
        (m, n)
    }
}
fn train(
    net: &mut Network,
    opt: &mut Optimizer,
    loss_func: LossFunction,
    x_train: &Array2<f64>,
    y_train: &Array2<f64>,
    epochs: usize,
) {
    for epoch in 1..=epochs {
        let y_pred = net.forward(x_train);
        let loss = loss_func.loss(y_train, &y_pred);
        printol!("Epoch: {}, Loss: {}", epoch, loss);

        let loss_grad = loss_func.derivative(y_train, &y_pred);
        let act_grad = net.activation.derivative(&y_pred);
        let grad = &loss_grad * &act_grad;
        let (dw, db) = net.backward(x_train, &grad);

        opt.update(&mut net.weights, &dw);
        opt.update(&mut net.bias, &db);
    }
}

fn main() {
    // Préparation des données
    let x_train = Array::zeros((1000, 10)); // Exemple de données, à remplacer par vos propres données
    let y_train = Array::zeros((1000, 1)); // Exemple de données, à remplacer par vos propres données

    let mut net = Network::new((10, 1), 1, ActivationFunction::Sigmoid);

    // let mut opt = Optimizer::GD(OptimizerGD::new(0.01));
    // let mut opt = Optimizer::Adam(OptimizerAdam::new(0.01, 0.9, 0.999, 1e-8, (10, 1)));
    let mut opt = Optimizer::Nadam(OptimizerNadam::new(0.01, 0.9, 0.999, 1e-8, (10, 1)));

    let loss_func = LossFunction::MSE;

    train(&mut net, &mut opt, loss_func, &x_train, &y_train, 10000);
}
