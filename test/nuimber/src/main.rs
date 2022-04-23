
// enum Value {
//     Null,
//     Bool(bool),
//     Number(Number),
//     String(String),
//     Array(Vec<Value>),
//     Object(Map<String, Value>),
// }


// // #[must_use = "closures are lazy and do nothing unless called"]
// // pub trait Fn<Args>: FnMut<Args> {
// //     pub extern "rust-call" fn call(&self, args: Args) -> Self::Output;
// // }
// use std::ops::*;

// struct Calledthing {
//     lol: usize,
// }

// // trait Fn<Args>: FnMut<Args> for Called_thing {

// // }

// impl Fn<Args> for Calledthing {
//     // type Output = Self;
//     fn call(&self) -> Self {
//         Self {lol: self.lol + 1}
//     }
// }

// fn main() {
//     let sometimes = Calledthing { lol : 12};
//     println!("Hello, world! {:?}", sometimes());
// }
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![recursion_limit = "7172"]
// #![type_length_limit = "7172"]
// #![allow(non_camel_case_types)]
// #![allow(unused_must_use)]
// #![allow(unused_variables)]

#[derive(Clone, Debug, Eq, PartialEq,)]
struct Constant<Typ> {
    value: ListOrNot<Typ>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Interval<Typ> {
    min: Num<Typ>,
    max: Num<Typ>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Ensemble<Typ> {
    domaine: ListOrNot<Interval<Typ>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Variable<Typ> {
    name: String,
    value: ListOrNot<Ensemble<Typ>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
enum Num<T> {
    Varia(Box<Variable<T>>),
    Const(Constant<T>)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct System<Typ> {
    list_rules: ListOrNot<Typ>
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Component {
    // group_var: Vec<Num<T>>,
    // group_ops: Vec<String>
    group_caracteres: Vec<String>
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Function<'a, T> {
    name: String,
    vars: Vec<String>,
    f: fn(Vec<Num<T>>) -> Num<T>,
    the_func: Option<&'a fn(Vec<Num<T>>) -> Num<T>>,
    writable_equivalence: Vec<Component>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Equality {
    list_component: Vec<Vec<Component>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Inequality {
    list_component: Vec<Vec<Component>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Equivalence {
    Equa(ListOrNot<Equality>),
    Inqua(ListOrNot<Inequality>)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ListOrNot<T> {
    List(Vec<T>),
    One(T),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum VariableType {}

// impl<T:Default> Function<T> {
//     fn fonction(&self,params: Vec<Num<T>>) -> T {
//         Default::default()
//     }
// }

impl<'a, T: From<T>> Function<'a, T> {
    fn new(name: String, vars: Vec<String>, writable_equivalence: Vec<Component>) -> Function<'a, T> {
        Function {
            name,
            vars,
            f: |_| -> Num<T> {
                panic!("the function didn't initialize so it's doingnot what u want");
                // Num::Const(Constant {value: ListOrNot::One( <T>::from(0) ) })
                // if self.the_func == None {
                //     self.the_func = Some( |varsiant: Vec<Num<T>>| -> T { Default::default() } );
                // };
                // self.the_func.unwrap()
            },
            the_func: None,
            writable_equivalence,
        }
    }
    
    fn init(&mut self, arg: &'a fn(Vec<Num<T>>)->Num<T> ) -> fn(Vec<Num<T>>)->Num<T> {
        match &self.the_func {
            Some(v)=> **v,
            None => {
                let v = arg;
                self.the_func = Some(v);
                self.f = *v;
                *v
            }
        }
    }
}

fn main() -> () {
    println!("Hello, world!");
    let my_const: Constant<f64> = Constant { value: ListOrNot::One(1.0) };
    println!("\na constant: \n{:#?}", my_const);
    let my_var: Variable<f64> = Variable { 
        name: String::from("x"),
        value: ListOrNot::One( 
            Ensemble { 
                domaine: ListOrNot::One( 
                    Interval{ 
                        min: Num::Const(
                            Constant { 
                                value: ListOrNot::One(1.0) 
                            }
                        ),
                        max: Num::Const(
                            Constant { 
                                value: ListOrNot::One(1.0) 
                            }
                        )
                    }
                )
            }
        )
    };
    println!("\na var: \n{:#?}", my_var);
    // let my_function: Function<f64> = Function {
    //     name: String::from("f"),
    //     vars: vec!["x".to_string()],
    //     f: || {
            
    //     }
    //     the_func: None,
    //     writable_equivalence: vec![
    //         Component {
    //             // group_var: vec![Num::Const(my_const), Num::Varia(std::boxed::Box::new(my_var))],
    //             // group_ops: vec![String::from("+")] 
    //             group_caracteres: vec!["lol".to_string()]
    //         }
    //     ],
    // };
    let mut my_function = Function::new(
        "lol".to_string(), 
        vec!["x"].into_iter().map(|x| {x.to_string()}).collect::<Vec<_>>(), 
        vec![]
    );
    let ump: fn(Vec<Num<u64>>) -> Num<u64> = |para: Vec<Num<u64>>| -> Num<u64> {
        Num::Const(Constant {value: ListOrNot::One(0 as u64) }) as Num<u64>
    };
    let umput: fn(Vec<Num<u64>>) -> Num<u64> = |para: Vec<Num<u64>>| -> Num<u64> {
        assert!(para.len() > 0);
        if let Num::Const(Constant {value: ListOrNot::One(to_chage) }) = para[0] {
            Num::Const(Constant {value: ListOrNot::One((to_chage+ 1) as u64) }) as Num<u64>
        } else {
            Num::Const(Constant {value: ListOrNot::One(0 as u64) }) as Num<u64>
        }

    };
    println!("\na function before initialization: \n{:#?}", my_function);
    my_function.init(&umput);
    println!("\na function after initialization: \n{:#?}", my_function);
    // let tempo = Num::Const(Constant {value: ListOrNot::One(0) });
    println!("\na function applied to a const: {:#?}", (my_function.f)(vec![Num::Const(Constant {value: ListOrNot::One(0 as u64) }) as Num<u64>]));
    let mut my_temp_var = (my_function.f)(vec![Num::Const(Constant {value: ListOrNot::One(0 as u64) }) as Num<u64>]);
    println!("\na function applied to a const: {:#?}", my_temp_var);
    my_temp_var = (my_function.f)(vec![my_temp_var]);
    println!("\na function applied to a const: {:#?}", my_temp_var);
    my_temp_var = (my_function.f)(vec![my_temp_var]);
    println!("\na function applied to a const: {:#?}", my_temp_var);
    // println!("\na function applied to a const: {:#?}", (my_function.f)(vec![Num::Const(Constant {value: ListOrNot::One(0 as u64) }) as Num<u64>]));
    // println!("\na function applied to a const: {:#?}", (my_function.f)(vec![]));
}


// //

// enum num {
//     constant,
//     Variable,
// }

// enum constant {
//     usize,
//     isize,
//     fsize,
//     String
// }

// struct Interval {
//     min:num,
//     max:num,
//     parameter: Vec<String>,
// }

// struct List {
//     liste: Vec<num>,
//     parameters: Vec<String>,
// }

// enum ListInterval {
//     Interval,
//     List
// }

// struct Ensemble {
//     domaine: Vec<ListInterval>
// }

// struct Variable {
//     name: String,
//     description: String,
//     ensemble_def: Ensemble,
// }

// struct Equa {
//     main_equa: Vec<String>,
//     variables: Vec<Variable>,
//     translate_equa: Vec<num>,
// }

// impl  Equa {
//     fn new() -> Equa {}
//     fn simplify() -> Equa {}
//     fn develop() -> Equa {}
//     fn derive() -> Equa {}
//     fn integrate() -> Equa {}
//     fn solve() -> Equa {}
// }
 
// fn main() -> () {
//     let my_equa: Equa = "x+1 = 0";
// }

// //