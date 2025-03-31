# setup.ps1
# This script creates a Cargo workspace named "my_workspace" with four member packages:
# download_net, train_net, use_net, and shape_net.
# Each package gets its own Cargo.toml and src/main.rs file with example code.
# Run this script in PowerShell.

# Define workspace directory name
$workspaceName = "nn_GPT"

# Create the workspace root directory if it doesn't exist
if (!(Test-Path $workspaceName)) {
    New-Item -ItemType Directory -Path $workspaceName | Out-Null
}

# Create the workspace Cargo.toml
$workspaceToml = @'
[workspace]
members = [
    "download_net",
    "train_net",
    "use_net",
    "shape_net",
]
'@
Set-Content -Path "$workspaceName\Cargo.toml" -Value $workspaceToml

# Define member packages
$members = @("download_net", "train_net", "use_net", "shape_net")

foreach ($member in $members) {
    $memberPath = "$workspaceName\$member"
    if (!(Test-Path $memberPath)) {
         New-Item -ItemType Directory -Path $memberPath | Out-Null
    }

    # Set dependency: download_net uses reqwest; others use tch.
    if ($member -eq "download_net") {
        $dep = 'reqwest = "0.11"'
    } else {
        $dep = 'tch = "0.5"'
    }

    # Create member Cargo.toml
    $cargoToml = @"
[package]
name = "$member"
version = "0.1.0"
edition = "2021"

[dependencies]
$dep
"@
    Set-Content -Path "$memberPath\Cargo.toml" -Value $cargoToml

    # Create the src directory for the package
    $srcPath = "$memberPath\src"
    if (!(Test-Path $srcPath)) {
         New-Item -ItemType Directory -Path $srcPath | Out-Null
    }

    # Prepare the main.rs content based on the member name
    switch ($member) {
        "download_net" {
            $mainRs = @'
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
        eprintln!("Usage: {} <model_url> <output_file>", args[0]);
        return Ok(());
    }

    let url = &args[1];
    let output_file = &args[2];

    println!("Downloading from: {}", url);
    let mut response = get(url)?;
    let mut file = File::create(output_file)?;
    copy(&mut response, &mut file)?;

    println!("Model downloaded successfully as {}", output_file);
    Ok(())
}
'@
        }
        "train_net" {
            $mainRs = @'
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
'@
        }
        "use_net" {
            $mainRs = @'
use std::env;
use tch::{nn, Device, Tensor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
        eprintln!("Usage: {} <model_file> <input_value>", args[0]);
        return Ok(());
    }

    let model_file = &args[1];
    let input_value: f64 = args[2].parse().unwrap_or(10.0);

    let device = Device::Cpu;
    let vs = tch::nn::VarStore::new(device);
    let net = tch::nn::linear(&vs.root(), 1, 1, Default::default());

    vs.load(model_file)?;

    let input = tch::Tensor::of_slice(&[input_value]).to_device(device).view([-1, 1]);
    let output = input.apply(&net);

    println!("Inference result for input {}: {:?}", input_value, output);
    Ok(())
}
'@
        }
        "shape_net" {
            $mainRs = @'
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
'@
        }
        default {
            $mainRs = "// main.rs placeholder"
        }
    }

    # Write the main.rs file for the package
    Set-Content -Path "$srcPath\main.rs" -Value $mainRs
}

Write-Host "Workspace and packages generated successfully in the '$workspaceName' directory."
