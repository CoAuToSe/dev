#![allow(non_snake_case)]

#[derive(Debug, Default, Clone, Copy)]
struct Neuron<T> {
    value: T,
    error: T,
    potential_error: Option<T>,
    activation_fonction: Option<fn(T) -> T>,
    derive_activation_fonction: Option<fn(T) -> T>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Axion<T> {
    id: usize,
    value: T,
    error: T,
    potential_error: Option<T>,
}

#[derive(Debug, Clone)]
struct NeuralNetwork<'a, T> {
    input: Option<Layer<'a, T>>,
    hidden_layer: Option<Vec<Layer<'a, T>>>,
    output: Option<Layer<'a, T>>,
}

#[derive(Debug, Default, Clone)]
struct Layer<'a, T> {
    neurons: Vec<Neuron<T>>,
    axions: Vec<Axion<T>>,
    layer_from: Option<&'a Option<Layer<'a, T>>>,
    layer_into: Option<&'a Option<Layer<'a, T>>>,
}

impl<'a, T: Clone + Default + std::fmt::Debug> NeuralNetwork<'a, T> {
    fn init(&mut self, width: usize, height: usize) -> () {
        // let mut inti = vec![];
        // assert!(width >= 1);
        // inti.push(Layer::new(height, Some(&self.input), None));
        // for i in 1..width - 1 {
        // inti.push(Layer::new(height, Some(&self.input), None));}

        let temporaly = vec![Layer::new(height, None, None); width];
        // let tempo = ;
        *self = NeuralNetwork {
            input: None,
            hidden_layer: Some(temporaly),
            output: None,
        };
        let my_workspace = &self.hidden_layer;
        if let Some(zae) = my_workspace {
            println!("{:?}\n", zae);
        }
    }
}

impl<'a, T: Clone + Default> Layer<'a, T> {
    fn new(
        height: usize,
        from: Option<&'a Option<Layer<'a, T>>>,
        into: Option<&'a Option<Layer<'a, T>>>,
    ) -> Self {
        Layer {
            neurons: vec![Neuron::new(T::default()); height],
            axions: vec![Axion::new(T::default()); height],
            layer_from: from,
            layer_into: into,
        }
    }
}

impl<T: Default> Neuron<T> {
    fn new(value: T) -> Self {
        Neuron {
            value: value,
            error: T::default(),
            potential_error: Some(T::default()),
            activation_fonction: None,
            derive_activation_fonction: None,
        }
    }
}

impl<T: Default> Axion<T> {
    fn new(value: T) -> Self {
        Axion {
            id: usize::default(),
            value: T::default(),
            error: T::default(),
            potential_error: Some(T::default()),
        }
    }
}
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
fn main() {
    println!("Hello, world!");
    let mut my_net = NeuralNetwork::<f64> {
        input: None,
        hidden_layer: None,
        output: None,
    };
    my_net.init(3, 3);
    println!("{:#?}", my_net);
    let some = Arc::new(Mutex::new(vec![Duration::as_secs_f64(
        &SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
    )]));
    println!("{:?}", some);

    let azerty = Arc::clone(&some);

    let _vals = std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(5));
        let mut lol = azerty.lock().unwrap();
        println!("{:?}", lol);
        (*lol)[0] = Duration::as_secs_f64(
            &SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
        );
        println!("{:?}", lol[0]);
    });

    let mut wanto_finish = false;
    std::thread::spawn(move || loop {
        if wanto_finish == true {
            break;
        }
        //run the NN

        //if NN is good enought :
        wanto_finish = true;
        println!("Done");
    });
    std::thread::sleep(Duration::from_secs(11))
}
