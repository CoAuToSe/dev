// use std::env;
// use rust_bert::{
//     pipelines::generation_utils::LanguageGenerator, 
//     models::gpt2::GPT2Generator,
//     RustBertError,
// };

// fn main() -> Result<(), RustBertError> {
//     let args: Vec<String> = env::args().collect();

//     // Show help message if "--help" or "-h" is provided or if not enough arguments are given.
//     if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 2 {
//         println!("Usage: {} <prompt> [max_length]", args[0]);
//         println!("  <prompt>     : The text prompt to generate text from.");
//         println!("  [max_length] : Optional maximum length for generated text (default: 50).");
//         println!("\nExample:");
//         println!("  {} \"Once upon a time\" 60", args[0]);
//         return Ok(());
//     }

//     // Get the prompt and maximum length (if provided)
//     let prompt = &args[1];
//     let max_length = if args.len() > 2 {
//         args[2].parse::<i64>().unwrap_or(50)
//     } else {
//         50
//     };

//     println!("Generating text for prompt: \"{}\" with max_length: {}", prompt, max_length);

//     // Initialize the GPT2 generator with default configuration.
//     let mut generator = GPT2Generator::new(Default::default())?;
//     // Generate text based on the prompt.
//     let output = generator.generate(Some(prompt), Some(max_length));
    
//     // Print the generated text.
//     for sentence in output {
//         println!("{:?}", sentence);
//     }
//     Ok(())
// }


use std::env;
use rust_bert::{
    pipelines::generation_utils::{GenerateOptions, LanguageGenerator},
    models::gpt2::GPT2Generator,
    RustBertError,
};

fn main() -> Result<(), RustBertError> {
    let args: Vec<String> = env::args().collect();

    // Show help message if "--help" or "-h" is provided or if not enough arguments are given.
    if args.iter().any(|a| a == "--help" || a == "-h") || args.len() < 2 {
        println!("Usage: {} <prompt> [max_length]", args[0]);
        println!("  <prompt>     : The text prompt to generate text from.");
        println!("  [max_length] : Optional maximum length for generated text (default: 50).");
        println!("\nExample:");
        println!("  {} \"Once upon a time\" 60", args[0]);
        return Ok(());
    }

    // Get the prompt and maximum length (if provided)
    let prompt = &args[1];
    let max_length = if args.len() > 2 {
        args[2].parse::<i64>().unwrap_or(50)
    } else {
        50
    };

    println!("Generating text for prompt: \"{}\" with max_length: {}", prompt, max_length);

    // Initialize the GPT2 generator with default configuration.
    let mut generator = GPT2Generator::new(Default::default())?;
    
    // The generator expects a slice of string slices, so convert the prompt accordingly.
    let prompt_slice = &[prompt.as_str()];
    let generate_options = GenerateOptions {
        max_length: Some(max_length),
        ..Default::default()
    };

    // Generate text based on the prompt.
    let output = generator.generate(Some(prompt_slice), Some(generate_options));
    
    // Print the generated text.
    for sentence in output {
        println!("{:?}", sentence);
    }
    Ok(())
}
