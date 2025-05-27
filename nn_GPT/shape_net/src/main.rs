use std::env;
use tch::{nn, Device, Tensor};

#[derive(Debug)]
struct MyNet {
    fc1: nn::Linear,
    fc2: nn::Linear,
}

impl MyNet {
    fn new(vs: &nn::Path, input_dim: i64) -> MyNet {
        let fc1 = nn::linear(vs, input_dim, 128, Default::default());
        let fc2 = nn::linear(vs, 128, 10, Default::default());
        MyNet { fc1, fc2 }
    }

    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.apply(&self.fc1).relu().apply(&self.fc2)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Usage: {} [input_dimension]", args[0]);
        println!("Defaults to 784 if not provided.");
        return Ok(());
    }

    let input_dim: i64 = if args.len() > 1 {
        args[1].parse().unwrap_or(784)
    } else {
        784
    };
    println!("Using input dimension: {}", input_dim);

    let device = Device::Cpu;
    let vs = tch::nn::VarStore::new(device);
    let net = MyNet::new(&vs.root(), input_dim);

    let input = Tensor::rand(&[1, input_dim], (tch::Kind::Float, device));
    let output = net.forward(&input);

    println!("Custom network output: {:?}", output);
    Ok(())
}
