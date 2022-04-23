#[allow(unused_imports)]
use std::thread;

// #[derive(Eq, PartialEq)]
struct Neurone<T: Default + std::fmt::Display + Copy + From<u8>> {
    id: usize,
    connections: Vec<Connection>,
    fonction: Fonc<T>,
    error: Option<T>,
}

struct Fonc<T: Default + std::fmt::Display + Copy> {
    main: fn(T) -> T,
    derive: fn(T) -> T,
}

#[derive(Eq, PartialEq)]
struct Connection {
    id: usize,
    neuro_in: usize,
    neuro_out: usize,
}

impl<T: Default + std::fmt::Display + Copy + From<u8>> Neurone<T> {
    fn new() -> Self {
        Neurone {
            id: 0,
            connections: vec![],
            fonction: Fonc {
                main: relu,
                derive: relu_derive,
            },
            error: None,
        }
    }
}

// fn first_char_debug(to_for: String) -> String {
//     let mut temp = vec![];
//     for e in to_for.chars() {
//         println!("{}",e);
//         temp.push(e);
//     }
//     return String::from(temp[0])
// }

fn first_char_for<T: std::fmt::Display>(to_for: T) -> String {
    String::from(vec![format!("{}", to_for).chars().next().unwrap()][0])
}

fn relu<T: Default + std::fmt::Display + Copy + From<u8>>(x: T) -> T {
    if first_char_for(x) != first_char_for(-1) {
        return x;
    }
    Default::default()
}

fn relu_derive<T: Default + std::fmt::Display + Copy + From<u8>>(x: T) -> T {
    let temp = first_char_for(x);
    if temp != first_char_for(-1) && temp != first_char_for(0) {
        return T::from(1 as u8);
    }
    Default::default()
}

// fn relu_derive2<T: Default + std::fmt::Display + Copy>(x: T) -> T {
//     if first_char_for(x) != first_char_for(-1) {
//         return x/x;
//     } return Default::default();
// }

fn main() {
    println!("Hello, world!");
    println!("{:?}\n", first_char_for(format!("{}", -1.5)));
    println!("{:?}\n", first_char_for(format!("{}", 1.5)));
    println!("{:?}\n", first_char_for(format!("{}", 0)));
    println!("{:?}\n", first_char_for(format!("{}", "adzzegr")));
    println!(
        "{:?}\n",
        first_char_for(format!("{}", String::from(";lkre")))
    );
    println!("{:?}", relu(45));
    println!("{:?}", relu(1));
    println!("{:?}", relu(0));
    println!("{:?}", relu(-1));
    println!("{:?}", relu_derive(45));
    println!("{:?}", relu_derive(1));
    println!("{:?}", relu_derive(0));
    println!("{:?}", relu_derive(-1));
}
