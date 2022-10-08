use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Index, IndexMut},
};
// trait NewTrait: Display {}
trait NewTrait: std::fmt::Display {}
struct Dict<'a> {
    keys: Vec<&'a (dyn Display)>,
}

impl<'a> Index<&'a dyn Display> for Dict<'a> {
    type Output = &'a (dyn Display);

    fn index(&self, index: &'a dyn Display) -> &&'a (dyn Display) {
        &self.keys[index].into()
    }
}

impl<'a> IndexMut<&'a dyn Display> for Dict<'a> {
    fn index_mut(&mut self, index: &'a dyn Display) -> &mut Self::Output {
        todo!()
    }
}

// impl Index<String> for JsonValue {
//     type Output = JsonValue;

//     fn index(&self, index: String) -> &JsonValue {
//         self.index(index.deref())
//     }
// }

// impl<'a> Index<&'a String> for JsonValue {
//     type Output = JsonValue;

//     fn index(&self, index: &String) -> &JsonValue {
//         self.index(index.deref())
//     }
// }

fn main() {
    println!("Hello, world!");
}

// impl Index<usize> for JsonValue {
//     type Output = JsonValue;

//     fn index(&self, index: usize) -> &JsonValue {
//         match *self {
//             JsonValue::Array(ref vec) => vec.get(index).unwrap_or(&NULL),
//             _ => &NULL
//         }
//     }
// }
// impl IndexMut<usize> for JsonValue {
//     fn index_mut(&mut self, index: usize) -> &mut JsonValue {
//         match *self {
//             JsonValue::Array(ref mut vec) => {
//                 let in_bounds = index < vec.len();

//                 if in_bounds {
//                     &mut vec[index]
//                 } else {
//                     vec.push(JsonValue::Null);
//                     vec.last_mut().unwrap()
//                 }
//             }
//             _ => {
//                 *self = JsonValue::new_array();
//                 self.push(JsonValue::Null).unwrap();
//                 self.index_mut(index)
//             }
//         }
//     }
// }
// impl<'a> Index<&'a str> for JsonValue {
//     type Output = JsonValue;

//     fn index(&self, index: &str) -> &JsonValue {
//         match *self {
//             JsonValue::Object(ref object) => &object[index],
//             _ => &NULL
//         }
//     }
// }

// impl Index<String> for JsonValue {
//     type Output = JsonValue;

//     fn index(&self, index: String) -> &JsonValue {
//         self.index(index.deref())
//     }
// }

// impl<'a> Index<&'a String> for JsonValue {
//     type Output = JsonValue;

//     fn index(&self, index: &String) -> &JsonValue {
//         self.index(index.deref())
//     }
// }
