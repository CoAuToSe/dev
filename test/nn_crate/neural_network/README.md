# neural_network
This crate is a modular implementation of a neural network. 
It was created with the intention to make the implementation of any learning method for the neural network really simple and without a runtime cost. 
If you want to know how to implement any learning technique you can take a look at the src/back_prop folder where you can find an implementation of back propagation.
If you're just interested in using a neural network to solve problems you can follow the tutorial down below on how to add it to your project and how to use it.

## Add neural_network to your project
Add this line to your dependencies in your Cargo.toml file
```
neural_network = "0.1.3"
```
Then, add this at the beginning of your src/main.rs or src/lib.rs file
```rust
extern crate neural_network;
```
And you're good to go!

# Examples of using back propagation
Here's an example of how to use back propagation to train a neural network to return the sin and the cos of a number. 
You'll also see how to save the network to a file and reload it.
```rust
use neural_network::back_prop::prelude::*;
use std::fs::{File, remove_file};
use rand;

fn main() {
    let file_name = "back_prop_net.nn";
    let mut net = BackProp::new(1, &[50, 50, 50], 2,
                                      0.05, 0.1, 1.0,
                                      Tanh::activation, Tanh::derivative,
                                      Tanh::activation, Tanh::derivative);

    {
        // Generate the training data
        let mut train_inputs: Vec<[f64;1]> = Vec::with_capacity(1000);
        let mut train_targets: Vec<[f64;2]> = Vec::with_capacity(1000);

        for _ in 0..1000 {
            let num: f64 = rand::random();

            train_inputs.push([num]);
            train_targets.push([num.sin(), num.cos()]);
        }

        // Generate the testing data
        let mut test_inputs: Vec<[f64;1]> = Vec::with_capacity(100);
        let mut test_targets: Vec<[f64;2]> = Vec::with_capacity(100);

        for _ in 0..100 {
            let num: f64 = rand::random();
            let sc = num.sin_cos();

            test_inputs.push([num]);
            test_targets.push([sc.0, sc.1]);
        }

        let train_inputs: Vec<&[f64]> = train_inputs.iter().map(|n|n as &[f64]).collect();
        let train_targets: Vec<&[f64]> = train_targets.iter().map(|n|n as &[f64]).collect();
        let test_inputs: Vec<&[f64]> = test_inputs.iter().map(|n|n as &[f64]).collect();
        let test_targets: Vec<&[f64]> = test_targets.iter().map(|n|n as &[f64]).collect();

        let result = net.train(0.001, None, None,
                               &train_inputs, &train_targets,
                               &test_inputs, &test_targets);
        assert!(result.min_error <= 0.001);
    }

    net.save(&mut File::create(file_name).unwrap()).unwrap();
    let loaded = BackProp::load(&mut File::open(file_name).unwrap(), Tanh::activation, Tanh::derivative,
                         Tanh::activation, Tanh::derivative, ).unwrap();

    assert_eq!(net, loaded);
    remove_file(file_name).unwrap();
}
```