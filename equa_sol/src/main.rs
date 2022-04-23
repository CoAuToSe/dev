const ERROR_TYPING: &str = "wrong type";
macro_rules! input {
    ($type:ty) => {{
        use std::io::Write;
        loop {
            print!("{}:", stringify!($type));
            std::io::stdout().flush().unwrap();
            let mut some_var_for_macro = String::default();
            std::io::stdin().read_line(&mut some_var_for_macro).unwrap();
            match some_var_for_macro.trim().parse::<$type>() {
                Ok(n) => break n,
                Err(_) => eprintln!("{}", ERROR_TYPING),
            }
        }
    }};
    ($type:ty, $len:ident) => {
        input!($type, $len, 172)
    };
    ($len:ident, $type:ty) => {
        input!($type, $len, 172)
    };
    ($type:ty, $len:ident, 172) => {
        use std::io::Write;
        let mut into_macro;
        let valeur = loop {
            print!("{}:", stringify!($type));
            std::io::stdout().flush().unwrap();
            let mut some_var_for_macro = String::default();
            into_macro = std::io::stdin().read_line(&mut some_var_for_macro).unwrap();
            match some_var_for_macro.trim().parse::<$type>() {
                Ok(n) => break n,
                Err(_) => eprintln!("{}", ERROR_TYPING),
            }
        };
        $len = into_macro;
        valeur
    };
    ($type:ty, $default_value:literal) => {{
        use std::io::Write;
        // loop {
        print!("{}:", stringify!($type));
        std::io::stdout().flush().unwrap();
        let mut some_var_for_macro = String::default();
        std::io::stdin().read_line(&mut some_var_for_macro).unwrap();
        match some_var_for_macro.trim().parse::<$type>() {
            Ok(n) => n,
            Err(_) => $default_value,
        }
        // }
    }};
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Testing {
    some1: String,
    some2: usize,
}
// Testing { some1:String::from("lol"), some2:0 }

use std::path::Component;

impl std::str::FromStr for Testing {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            // .split("Testing")
            .split(',')
            // .split(',')
            .collect();
        println!("{:?}", coords);
        let x_fromstr = coords[0].parse::<String>().unwrap();
        let y_fromstr = coords[1].parse::<usize>()?;

        Ok(Testing {
            some1: x_fromstr,
            some2: y_fromstr,
        })
    }
}

// impl std::str::FromStr for Component {
//     type Err = std::num::ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let coords: Vec<&str> = s
//             .trim_matches(|p| p == '(' || p == ')')
//             // .split("Testing")
//             .split(',')
//             // .split(',')
//             .collect();
//         println!("{:?}", coords);
//         let x_fromstr = coords[0].parse::<String>().unwrap();
//         let y_fromstr = coords[1].parse::<usize>()?;

//         Ok(Testing {
//             some1: x_fromstr,
//             some2: y_fromstr,
//         })
//     }
// }

enum System<T> {
    Some(T),
    System(Vec<System<T>>),
    None,
}

#[derive(Debug, PartialEq, Eq)]
enum ValueCarac {
    Som(usize),
    Somi(isize),
}
#[derive(Debug, PartialEq, Eq)]
enum ValueType {
    Som(usize),
    Somi(isize),
}

impl std::ops::Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self
    }
}
impl std::ops::Add for ValueCarac {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self
    }
}

#[derive(Debug)]
struct Equation<T, const U: usize, const Y: usize> {
    equialities: Vec<Element<T, Y>>,
}
#[derive(Debug)]
struct Element<G, const T: usize> {
    composers: Vec<Composent<G>>,
}

#[derive(Debug)]
struct Composent<T> {
    value_type: ValueType,
    value_carac: T,
}

impl<T, const U: usize, const Y: usize> Equation<T, U, Y> {
    fn new() -> Self {
        Equation {
            equialities: vec![],
        }
    }
}
impl<T, const U: usize> Element<T, U> {
    fn new() -> Self {
        Element { composers: vec![] }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Composent<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.value_type == other.value_type {
            Self {
                value_type: self.value_type,
                value_carac: self.value_carac + other.value_carac,
            }
        } else {
            panic!()
        }
    }
}

trait Usable {}
impl<T> Usable for T {}

fn main() {
    // use std::io::Write;
    println!("Hello, world!");
    // let mut line = String::new();
    // print!("lol;");
    // std::io::stdout().flush().unwrap();
    // println!("Enter your name :");
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
    // println!("Hello , {}", line);
    // println!("no of bytes read , {}", b1);
    // let someaz = input!(Testing);
    // println!("{:?}", someaz);
    // let nani;
    // let zar = input!(usize, nani);
    // println!("{} {}", nani, zar);
    println!("bonjour");
    // let _state = System::Some(Equation::new());
    // let _state2 = System::System(vec![System::Some(Equation::new())]);
    let mut mines_equations: Vec<Equation<usize, 1, 1>> = vec![];
    // let possibilites: Vec<(&str, Box<dyn Fn() -> ()>)> = vec![
    //     ("voire les equations actuelle", Box::new(show_equa)),
    //     ("ajouter une égalité", ajout_equa()),
    //     ("sauvegarder les égalités dans un fichier"),
    //     ("ajouter des équations à partir d'un fichier"),
    // ];
    'main_loop: loop {
        println!("\nchoisi ce que tu veux faire:");
        println!("1. voire les equations actuelle");
        println!("2. ajouter une égalité");
        println!("3. éditer une égalité (choisir qu'elle équation édit)");
        println!("4. sauvegarder les égalités dans un fichier (WIP)");
        println!("5. ajouter des équations à partir d'un fichier (WIP)");
        println!("172. quitter");
        let choice = input!(u8);
        match choice {
            1 => show_equa_vec(&mines_equations),
            2 => add_equa(&mut mines_equations, Equation::new()), // from raw text
            3 => {
                println!("entre le numéro de l'équation que tu veux édit(défaut = 0)");
                let equa_choice = input!(usize, 0);
                'equa_loop: loop {
                    println!(
                        "\nchoisi ce que tu veux faire avec l'équation {}:",
                        equa_choice
                    );
                    println!("1. voire l'équation {}", equa_choice);
                    println!("2. ajouter un composent à l'équation {}", equa_choice);
                    println!("3. changer l'égalité à éditer");
                    println!("72. retour");
                    println!("172. quitter");

                    let choice = input!(u8);
                    match choice {
                        1 => show_equa(&mines_equations[equa_choice]),
                        2 => add_comp_equa(&mut mines_equations[equa_choice], Element::new()), // from Composent.s one at a time
                        72 => break 'equa_loop,
                        172 => break 'main_loop,
                        _ => (),
                    }
                }
            }
            172 => break 'main_loop,
            _ => (),
        }
    }
}

fn show_equa_vec<T: std::fmt::Debug, const U: usize, const Y: usize>(
    some: &Vec<Equation<T, U, Y>>,
) {
    for e in some {
        show_equa(e)
    }
}
fn show_equa<T: std::fmt::Debug, const U: usize, const Y: usize>(some: &Equation<T, U, Y>) {
    println!("{:?}", some)
}

fn add_equa<T, const U: usize, const Y: usize>(
    some: &mut Vec<Equation<T, U, Y>>,
    to_add: Equation<T, U, Y>,
) {
    some.push(to_add)
}
fn add_comp_equa<T, const U: usize, const Y: usize>(
    some: &mut Equation<T, U, Y>,
    to_add: Element<T, Y>,
) {
    some.equialities.push(to_add)
}
// fn show_equa(to_show: impl std::fmt::Debug) {
//     println!("{:?}", to_show)
// }

// fn ajout_equa<T>(to_push: core::option::Option<T>, to_push_into: Vec<T>) {
//     if let to_push = core::option::Option::Some(value) {
//         to_push_into.push(value)
//     }
// }
