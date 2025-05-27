use std::env;
use tch::{nn, Device, Kind, Tensor, nn::OptimizerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Usage: {} [epochs] [learning_rate] [output_file]", args[0]);
        println!("Defaults: epochs = 1000, learning_rate = 0.01, output_file = \"linear_model.pt\"");
        return Ok(());
    }

    // Default parameters
    let mut epochs = 1000;
    let mut learning_rate = 0.01;
    let mut output_file = String::from("linear_model.pt");

    if args.len() > 1 {
        if let Ok(e) = args[1].parse::<u32>() {
            epochs = e;
        }
    }
    if args.len() > 2 {
        if let Ok(lr) = args[2].parse::<f64>() {
            learning_rate = lr;
        }
    }
    if args.len() > 3 {
        output_file = args[3].clone();
    }

    println!(
        "Training for {} epochs with learning rate {}. Saving to {}.",
        epochs, learning_rate, output_file
    );

    let device = Device::Cpu;
    let vs = tch::nn::VarStore::new(device);
    let net = tch::nn::linear(&vs.root(), 1, 1, Default::default());

    // Dummy data: training on y = 2x + 3
    let xs = Tensor::of_slice(&[1.0, 2.0, 3.0, 4.0]).to_device(device).view([-1, 1]);
    let ys = Tensor::of_slice(&[5.0, 7.0, 9.0, 11.0]).to_device(device).view([-1, 1]);

    let mut opt = tch::nn::Sgd::default().build(&vs, learning_rate)?;

    for epoch in 1..=epochs {
        let pred = xs.apply(&net);
        let loss = (pred - &ys).pow(2).mean(Kind::Float);
        opt.backward_step(&loss);
        if epoch % (epochs / 10).max(1) == 0 {
            println!("Epoch: {} Loss: {:.4}", epoch, f64::from(&loss));
        }
    }

    vs.save(&output_file)?;
    println!("Training complete. Model saved as {}", output_file);
    Ok(())
}
