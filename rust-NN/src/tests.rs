#[test]
fn testing_main_2023_07_13_15_53() {
    let dataset: Vec<Vec<f64>> = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
    let targets: Vec<Vec<f64>> = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let mut nn = NeuralNetwork::new(&[2, 2, 1]);
    let learning_rate = 0.01;
    let epochs = 5000;
    let beta1 = 0.9;
    let beta2 = 0.999;
    let epsilon = 1e-8;

    println!("{:#?}", nn);

    nn.train(&dataset, &targets, learning_rate, epochs);

    println!("{:#?}", nn);
    println!("{:?}", nn.forward(&[0.0, 0.0]));
    println!("{:?}", nn.forward(&[0.0, 1.0]));
    println!("{:?}", nn.forward(&[1.0, 0.0]));
    println!("{:?}", nn.forward(&[1.0, 1.0]));
}

#[test]
fn testing_main_2023_07_13_15_55() {
    let dataset: Vec<Vec<f64>> = vec![
        vec![0.0],
        vec![0.1],
        vec![0.2],
        vec![0.3],
        vec![0.4],
        vec![0.5],
    ];
    let targets: Vec<Vec<f64>> = vec![
        vec![0.0],
        vec![0.2],
        vec![0.4],
        vec![0.6],
        vec![0.8],
        vec![1.0],
    ];

    let mut nn = NeuralNetwork::new(&[1, 4, 1]);
    // println!("{:#?}", nn);
    nn.train(&dataset, &targets, 0.1, 1000);
    println!("{:#?}", nn);
    println!("{:?}", nn.forward(&[0.5]));
}
