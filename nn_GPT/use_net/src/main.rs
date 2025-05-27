// use std::env;
// use tch::{nn, Device, Tensor};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();

//     if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
//         eprintln!("Usage: {} <model_file> <input_value>", args[0]);
//         return Ok(());
//     }

//     let model_file = &args[1];
//     let input_value: f64 = args[2].parse().unwrap_or(10.0);

//     let device = Device::Cpu;
//     let vs = tch::nn::VarStore::new(device);
//     let net = tch::nn::linear(&vs.root(), 1, 1, Default::default());

//     vs.load(model_file)?;

//     let input = tch::Tensor::of_slice(&[input_value]).to_device(device).view([-1, 1]);
//     let output = input.apply(&net);

//     println!("Inference result for input {}: {:?}", input_value, output);
//     Ok(())
// }

use std::env;
use tch::{CModule, IValue, Tensor};

/// Dummy tokenizer: for each word, returns (word length mod 10) as the token id.
/// Pads or truncates the token sequence to a fixed length.
fn dummy_tokenize(sentence: &str, seq_len: usize) -> Vec<i64> {
    let mut tokens: Vec<i64> = sentence
        .split_whitespace()
        .map(|w| (w.len() as i64) % 10)
        .collect();
    // Adjust to the fixed sequence length: truncate or pad with 0.
    if tokens.len() > seq_len {
        tokens.truncate(seq_len);
    } else {
        while tokens.len() < seq_len {
            tokens.push(0);
        }
    }
    tokens
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Show help if requested or if not enough arguments are provided
    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
        eprintln!("Usage: {} <model_file> <input_sentence>", args[0]);
        return Ok(());
    }

    let model_file = &args[1];
    let input_sentence = &args[2];

    println!("Loading model from: {}", model_file);
    let model = CModule::load(model_file)?;

    // Define the fixed sequence length expected by the model (for example, 10 tokens)
    let seq_len = 10;
    let tokens = dummy_tokenize(input_sentence, seq_len);
    println!("Tokenized input: {:?}", tokens);

    // Convert the token IDs into a tensor of shape [1, seq_len]
    let input_tensor = Tensor::of_slice(&tokens).unsqueeze(0);

    // Run inference using the TorchScript model
    let output = model.forward_is(&[IValue::Tensor(input_tensor)])?;
    println!("Model output: {:?}", output);
    Ok(())
}
