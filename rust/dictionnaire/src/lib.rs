// #![recursion_limit = "4"]
// #![type_length_limit = "4"]

// use std::{collections::HashMap, fmt::Display, hash::Hash};

// // #[derive(PartialEq, Eq, Hash)]
// pub enum HorV<Indexer, Values>
// where
//     Indexer: Hash + PartialEq + Eq,
//     Values: Hash + Display + PartialEq + Eq,
//     // HashMap<Indexer, HorV<Indexer, Values>>: Hash + PartialEq + Eq,
// {
//     Value(Values),
//     Index(Indexer),
//     HashMap(HashMap<Indexer, HorV<Indexer, Values>>),
// }
// // let mut set = HorV::HashMap(HashMap::new());
// // if let HorV::HashMap(_set) = set {
// //     $(
// //         _set.insert(HorV::Index($key), HorV::Value($value));
// //     )+
// // }
// // _set
// #[macro_export]
// macro_rules! HorVa {
//     { {$( $key:expr => $value:literal),+ $(,)*} } => {
//         {
//             let mut _set = ::std::collections::HashMap::new();
//             $(
//                 _set.insert($key,$value);
//             )+
//             _set
//         }
//     };
//     { $( $key:expr => $value:literal),+ $(,)*} => {
//         {
//             let mut _set = ::std::collections::HashMap::new();
//             $(
//                 _set.insert($key,$value);
//             )+
//             _set
//         }
//     };
//     { $( $key:expr => $value:tt),+ $(,)*} => {
//         {
//             let mut _set = ::std::collections::HashMap::new();
//             $(
//                 // println!("{} => {:?}", stringify!($key), HorVa!($value));
//                 _set.insert($key,HorVa!($value));
//             )+
//             _set
//         }
//     };
// }
extern crate proc_macro;
use proc_macro::*;

#[proc_macro]
pub fn _make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(AnswerFn)]
pub fn _derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn _show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro]
pub fn scanner(item: TokenStream) -> TokenStream {
    println!("{:?}", item);
    item
}
