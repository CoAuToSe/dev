// // use std::env;
// // use std::error::Error;
// // use std::fs::File;
// // use std::io::copy;
// // use reqwest::blocking::get;

// // fn main() -> Result<(), Box<dyn Error>> {
// //     let args: Vec<String> = env::args().collect();

// //     if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
// //         eprintln!("Usage: {} <model_url> <output_file>", args[0]);
// //         return Ok(());
// //     }

// //     let url = &args[1];
// //     let output_file = &args[2];

// //     println!("Downloading from: {}", url);
// //     let mut response = get(url)?;
// //     let mut file = File::create(output_file)?;
// //     copy(&mut response, &mut file)?;

// //     println!("Model downloaded successfully as {}", output_file);
// //     Ok(())
// // }

// use std::env;
// use std::error::Error;
// use std::fs::File;
// use std::io::copy;
// use reqwest::blocking::get;

// fn main() -> Result<(), Box<dyn Error>> {
//     let args: Vec<String> = env::args().collect();

//     // Show help if requested or if not enough arguments are provided
//     if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
//         eprintln!("Usage: {} <model_url> <output_file>", args[0]);
//         return Ok(());
//     }

//     let url = &args[1];
//     let output_file = &args[2];

//     println!("Downloading model from: {}", url);
//     let mut response = get(url)?;
//     let mut file = File::create(output_file)?;
//     copy(&mut response, &mut file)?;

//     println!("Model downloaded successfully as {}", output_file);
//     Ok(())
// }

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 3 {
        eprintln!("Usage: {} <model_url> <output_file> [auth_token]", args[0]);
        eprintln!("Provide the auth token either as the third parameter or set it in the HF_TOKEN environment variable.");
        return Ok(());
    }

    let url = &args[1];
    let output_file = &args[2];

    // Try to obtain token from command-line argument or environment variable.
    let token = if args.len() > 3 {
        Some(args[3].clone())
    } else {
        env::var("HF_TOKEN").ok().take()
    };

    println!("Downloading model from: {}", url);

    let client = Client::new();
    let mut request = client.get(url);

    if let Some(token) = token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let mut response = request.send()?;

    if !response.status().is_success() {
        eprintln!("Failed to download model: {}", response.status());
        return Ok(());
    }

    let mut file = File::create(output_file)?;
    copy(&mut response, &mut file)?;

    println!("Model downloaded successfully as {}", output_file);
    Ok(())
}
