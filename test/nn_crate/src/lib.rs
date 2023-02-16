mod lib_old;
mod own_nn;
pub use own_nn::*;

#[cfg(test)]
mod tests {
    use crate::{
        back_prop::{prelude::*, BackProp},
        neural_ops::TANH,
    };
    use rand;
    use std::fs::{remove_file, File};

    #[test]
    fn back_prop_works() {
        // let file_name = "back_prop_net.nn";
        let mut net = BackProp::new(
            1,
            &[50, 50, 50],
            2,
            0.05,
            0.1,
            1.0,
            TANH.activation,
            TANH.derivative,
            TANH.activation,
            TANH.derivative,
        );

        {
            // Generate the training data
            let mut train_inputs: Vec<[f64; 1]> = Vec::with_capacity(1000);
            let mut train_targets: Vec<[f64; 2]> = Vec::with_capacity(1000);

            for _ in 0..1000 {
                let num: f64 = rand::random();

                train_inputs.push([num]);
                train_targets.push([num.sin(), num.cos()]);
            }

            // Generate the testing data
            let mut test_inputs: Vec<[f64; 1]> = Vec::with_capacity(100);
            let mut test_targets: Vec<[f64; 2]> = Vec::with_capacity(100);

            for _ in 0..100 {
                let num: f64 = rand::random();
                let sc = num.sin_cos();

                test_inputs.push([num]);
                test_targets.push([sc.0, sc.1]);
            }

            let train_inputs: Vec<&[f64]> = train_inputs.iter().map(|n| n as &[f64]).collect();
            let train_targets: Vec<&[f64]> = train_targets.iter().map(|n| n as &[f64]).collect();
            let test_inputs: Vec<&[f64]> = test_inputs.iter().map(|n| n as &[f64]).collect();
            let test_targets: Vec<&[f64]> = test_targets.iter().map(|n| n as &[f64]).collect();

            let result = net.train(
                0.001,
                None,
                None,
                &train_inputs,
                &train_targets,
                &test_inputs,
                &test_targets,
            );
            assert!(result.min_error <= 0.001);
        }

        // net.save(&mut File::create(file_name).unwrap()).unwrap();
        // let loaded = BackProp::load(
        //     &mut File::open(file_name).unwrap(),
        //     TANH.activation,
        //     TANH.derivative,
        //     TANH.activation,
        //     TANH.derivative,
        // )
        // .unwrap();

        // assert_eq!(net, loaded);
        // remove_file(file_name).unwrap();
    }
    /*
    #[test]
    fn evolution_works() {

    }
    */
}
