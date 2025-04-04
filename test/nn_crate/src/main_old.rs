extern crate neural_network;
extern crate nn_crate;
extern crate time;

use nn_crate::{HaltCondition, LearningMode, NN};

fn main() {
    // create examples of the xor function
    let examples = [
        (vec![0f64, 0f64], vec![0f64]),
        (vec![0f64, 1f64], vec![1f64]),
        (vec![1f64, 0f64], vec![1f64]),
        (vec![1f64, 1f64], vec![0f64]),
    ];

    // create a new neural network
    let mut net1 = NN::new(&[2, 3, 3, 1]);

    // train the network
    net1.train(&examples)
        .log_interval(Some(1000))
        .halt_condition(HaltCondition::MSE(0.01))
        .learning_mode(LearningMode::Incremental)
        .momentum(0.1)
        .go();

    // make sure json encoding/decoding works as expected
    let json = net1.to_json();
    let net2 = NN::from_json(&json);

    // test the trained network
    for &(ref inputs, ref outputs) in examples.iter() {
        let results = net2.run(inputs);
        let (result, key) = (results[0].round(), outputs[0]);
        assert!(result == key);
        println!("{result} {key}");
    }
}
