
use std::thread;
use std::time::Duration;
use std::rc::Rc;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// #[derive(Debug, Clone)] 
// struct Cachet<T, U>
// where
//     T: Fn(u32) -> u32,
//     U: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<Rc<U>>,
// }


impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// impl<T, U> Cachet<T,U>
// where
//     T: Fn(u32) -> u32,
//     U: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cachet<T,U> {
//         Cachet {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: Rc<U> ) -> U {
//         match &self.value {
//             Some(v) => *((*v).clone()),
//             None => {
//                 let v = arg;
//                 self.value = Some(v);
//                 *v
//             }
//         }
//     }
// }

// #[derive(Clone, Copy)] 
// union Lol<'a> {
//     f1: Option<&'a fn(u32)->u32>,
//     f2: fn(u32)->u32
// }
// use std::fmt;
// impl<'a> fmt::Debug for Lol<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             f1 => f.debug_struct("Option<&'a fn(u32)->u32>").finish(),
//             f2 => f.debug_struct("fn(u32)->u32").finish(),
//         }
//         // f.debug_struct("Point")
//         //  .field("x", &self.x)
//         //  .field("y", &self.y)
//         //  .finish()
//     }
// }
// #[derive(Debug, Clone, Copy)] 
// enum Lol<'a> {
//     f1(Option<&'a fn(u32)->u32>),
//     f2(fn(u32)->u32),
// }
// use std::fmt;
// impl<'a> fmt::Debug for Lol<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             f1 => f.debug_struct("Option<&'a fn(u32)->u32>").finish(),
//             f2 => f.debug_struct("fn(u32)->u32").finish(),
//         }
//         // f.debug_struct("Point")
//         //  .field("x", &self.x)
//         //  .field("y", &self.y)
//         //  .finish()
//     }
// }
// #[derive(Debug, Clone, Copy)] 
// struct Cacheton<'a> {
//     calculation: fn(u32)->u32,
//     value: Lol<'a>,
// }

#[derive(Debug, Clone, Copy)] 
struct Cachetone<'a> {
    calculation: fn(u32)->u32,
    value: Option<&'a fn(u32)->u32>,
}

// impl<'a> Cacheton<'a> {
//     fn new(calculation: fn(u32)->u32) -> Cacheton<'a> {
//         Cacheton {
//             calculation,
//             value: Lol::f1(None),
//         }
//     }

//     fn value(&mut self, arg: &'a fn(u32)->u32 ) -> fn(u32)->u32 {
//         match &self.value {
//             Lol::f1(v) => { 
//                 match v {
//                     Some(v)=> **v,
//                     None => {
//                         let v = arg;
//                         self.value = Lol::f2(*v);
//                         *v
//                     }
//                 }
//             },
//             Lol::f2(v) => *v,
//         }
//     }
// }
impl<'a> Cachetone<'a> {
    fn new(calculation: fn(u32)->u32) -> Cachetone<'a> {
        Cachetone {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: &'a fn(u32)->u32 ) -> fn(u32)->u32 {
        match &self.value {
            Some(v)=> **v,
            None => {
                let v = arg;
                self.value = Some(v);
                self.calculation = *v;
                *v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// fn generate_workout_cacheton(intensity: fn(u32)->u32, random_number: u32) {
//     let mut expensive_result = Cacheton::new(|num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     });

//     if random_number < 25 {
//         println!("{:?}", (expensive_result.value));
//         println!("Today, do {:?} pushups!", expensive_result.value(&intensity)(random_number));
//         println!("{:?}", (expensive_result.value));
//         println!("Today, do {:?} pushups!", expensive_result.value(&intensity)(random_number));
//         println!("{:?}", (expensive_result.value));
//         println!("{:?}", expensive_result.value);
//         if let Lol::f2(var) = expensive_result.value {
//             println!("Next, do {:?} situps!", var(random_number));
//         }
//         // println!("Next, do {:?} situps!", (Lol::f2(expensive_result.value)));
//         // println!("Next, do {:?} situps!", (Lol::f2(expensive_result.value))(random_number));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {:?} minutes!",
//                 expensive_result.value(&intensity)(random_number)
//             );
//         }
//     }
// }

fn generate_workout_cachetone(intensity: fn(u32)->u32, random_number: u32) {
    let mut expensive_result = Cachetone::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if random_number < 25 {
        println!("{:?}", (expensive_result.value));
        println!("Today, do {:?} pushups!", expensive_result.value(&intensity)(random_number));
        println!("{:?}", (expensive_result.value));
        println!("Today, do {:?} pushups!", expensive_result.value(&intensity)(random_number));
        println!("{:?}", (expensive_result.value));
        println!("{:?}", expensive_result.value);
        println!("Today, do {:?} pushups!", (expensive_result.value.unwrap())(random_number));
        println!("Today, do {:?} pushups!", (expensive_result.calculation)(random_number));
        // if let Lol::f2(var) = expensive_result.value {
        //     println!("Next, do {:?} situps!", var(random_number));
        // }
        // println!("Next, do {:?} situps!", (Lol::f2(expensive_result.value)));
        // println!("Next, do {:?} situps!", (Lol::f2(expensive_result.value))(random_number));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {:?} minutes!",
                expensive_result.value(&intensity)(random_number)
            );
        }
    }
}

fn main() {
    println!("Hello, world!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let simulated_user_specified_value2 = |e:u32|->u32 { 10*e };
    let simulated_random_number2 = 7;

    generate_workout_cachetone(simulated_user_specified_value2, simulated_random_number2);
}
