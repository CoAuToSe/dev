use ndarray::{Array, Array1, Array2, Axis};
use rand::distributions::{Distribution, Uniform};

// Activation functions and their derivatives
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

// Loss functions and their derivatives
fn mse_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    let diff = y_true - y_pred;
    diff.iter().map(|x| x.powi(2)).sum::<f64>() / (diff.shape()[0] * diff.shape()[1]) as f64
}

fn mse_loss_derivative(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> Array2<f64> {
    2.0 * (y_pred - y_true)
}

fn mae_loss(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> f64 {
    (y_true - y_pred).mapv(f64::abs).mean().unwrap_or(0.0)
}

fn mae_loss_derivative(y_true: &Array2<f64>, y_pred: &Array2<f64>) -> Array2<f64> {
    let diff = y_true - y_pred;
    diff.mapv(|x| if x > 0.0 { -1.0 } else { 1.0 })
}

#[derive(Clone)]
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

trait Optimizer {
    fn update(&mut self, weights: &mut Array2<f64>, gradients: &Array2<f64>);
}

struct OptimizerGD {
    learning_rate: f64,
}

impl Optimizer for OptimizerGD {
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
        let m = Array2::<f64>::zeros(weight_dims);
        let v = Array2::<f64>::zeros(weight_dims);
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
}

impl Optimizer for OptimizerAdam {
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
        let m = Array2::<f64>::zeros(weight_dims);
        let v = Array2::<f64>::zeros(weight_dims);
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
}

impl Optimizer for OptimizerNadam {
    fn update(&mut self, weights: &mut Array2<f64>, gradients: &Array2<f64>) {
        println!("Update function:");
        println!("weights shape: {:?}", weights.dim());
        println!("gradients shape: {:?}", gradients.dim());

        let mut gradients_corrected = gradients.clone();

        if weights.dim().1 != gradients.dim().0 {
            gradients_corrected = gradients_corrected.t().to_owned();
        }

        self.t += 1;
        self.m = self.beta_1 * &self.m + (1.0 - self.beta_1) * &gradients_corrected;
        self.v =
            self.beta_2 * &self.v + (1.0 - self.beta_2) * gradients_corrected.mapv(|x| x.powi(2));

        let m_hat = &self.m / (1.0 - self.beta_1.powi(self.t as i32));
        let v_hat = &self.v / (1.0 - self.beta_2.powi(self.t as i32));

        let nesterov_momentum = self.beta_1 * m_hat
            + ((1.0 - self.beta_1) / (1.0 - self.beta_1.powi(self.t as i32)))
                * &gradients_corrected;

        println!("nesterov_momentum shape: {:?}", nesterov_momentum.dim());
        println!("v_hat shape: {:?}", v_hat.dim());

        let denominator = &v_hat.mapv(|x| x.sqrt()) + self.epsilon;
        println!("denominator shape: {:?}", denominator.dim());

        let update_val = &nesterov_momentum / denominator * self.learning_rate;
        println!("update_val shape: {:?}", update_val.dim());

        *weights -= &update_val;
    }
}

struct Layer {
    weights: Array2<f64>,
    bias: Array2<f64>,
    z: Array2<f64>,
    output: Array2<f64>, // Ajouté pour stocker la sortie après activation
    activation: ActivationFunction,
}

impl Layer {
    fn new(input_size: usize, output_size: usize, activation: ActivationFunction) -> Self {
        let between = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let weights =
            Array2::from_shape_fn((input_size, output_size), |_| between.sample(&mut rng));
        let bias = Array2::from_shape_fn((1, output_size), |_| between.sample(&mut rng));

        let z = Array2::zeros((output_size, 1));
        let output = Array2::zeros((output_size, 1)); // Initialiser le champ `output`
        Layer {
            weights,
            bias,
            z,
            output,
            activation,
        }
    }

    fn forward(&mut self, x: &Array2<f64>) -> Array2<f64> {
        self.z =
            x.dot(&self.weights) + &self.bias.broadcast((x.nrows(), self.bias.ncols())).unwrap();

        self.output = self.activation.function(&self.z);
        self.output.clone()
    }

    fn backward(&self, x: &Array2<f64>, grad: &Array2<f64>) -> (Array2<f64>, Array2<f64>) {
        let act_grad = match self.activation {
            ActivationFunction::Sigmoid => sigmoid_derivative(&self.z),
            ActivationFunction::Relu => relu_derivative(&self.z),
        };

        let m = grad * &act_grad;
        let dw = x.t().dot(&m);
        let db = m.sum_axis(Axis(0)).insert_axis(Axis(1));

        (dw, db)
    }
}

struct Network {
    layers: Vec<Layer>,
}

impl Network {
    fn new(layer_sizes: Vec<usize>, activations: Vec<ActivationFunction>) -> Self {
        let mut layers = Vec::new();
        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(
                layer_sizes[i],
                layer_sizes[i + 1],
                activations[i].clone(),
            ));
        }
        Network { layers }
    }

    fn forward(&mut self, x: &Array2<f64>) -> Array2<f64> {
        self.layers
            .iter_mut()
            .fold(x.clone(), |acc, layer| layer.forward(&acc))
    }

    fn backward(&mut self, x: &Array2<f64>, grad: &Array2<f64>) -> Vec<(Array2<f64>, Array2<f64>)> {
        let mut grads = Vec::new();
        let mut current_grad = grad.clone();

        // Stockez les sorties de chaque couche dans un vecteur pendant la propagation avant
        let mut outputs: Vec<Array2<f64>> = Vec::new();
        outputs.push(x.clone());
        for layer in &self.layers {
            outputs.push(layer.output.clone());
        }
        outputs.pop(); // Retirez la dernière sortie car elle n'est pas nécessaire pour la rétropropagation

        for (idx, layer) in self.layers.iter().rev().enumerate() {
            let (dw, db) = layer.backward(&outputs[self.layers.len() - 1 - idx], &current_grad);
            grads.push((dw, db));
            current_grad = current_grad.dot(&layer.weights.t());
        }

        grads.reverse(); // Inversez les gradients pour qu'ils correspondent à l'ordre des couches
        grads
    }
}

fn train(
    net: &mut Network,
    opt: &mut dyn Optimizer,
    loss_func: LossFunction,
    x_train: &Array2<f64>,
    y_train: &Array2<f64>,
    epochs: usize,
) {
    for epoch in 1..=epochs {
        let y_pred = net.forward(&x_train);
        let loss = loss_func.loss(y_train, &y_pred);
        println!("Epoch: {}, Loss: {}", epoch, loss);

        let loss_grad = loss_func.derivative(y_train, &y_pred);
        let grads = net.backward(&x_train, &loss_grad);

        for (layer, &(ref dw, ref db)) in net.layers.iter_mut().zip(&grads) {
            opt.update(&mut layer.weights, dw);
            opt.update(&mut layer.bias, db);
        }
    }
}

fn main() {
    // Mock data for demonstration; replace with real data as needed
    let x_train = Array::zeros((1000, 10));
    let y_train = Array::zeros((1000, 1));

    let mut net = Network::new(
        vec![10, 10, 1],
        vec![ActivationFunction::Sigmoid, ActivationFunction::Sigmoid],
    );

    let mut opt: Box<dyn Optimizer> =
        Box::new(OptimizerNadam::new(0.01, 0.9, 0.999, 1e-8, (10, 1)));

    train(
        &mut net,
        &mut *opt,
        LossFunction::MSE,
        &x_train,
        &y_train,
        10000,
    );
}
