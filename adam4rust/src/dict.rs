// //! WIP: not fully functionnal/practical
// use std::{any::Any, collections::HashMap, hash::Hash};

// pub struct Dict<T> {
//     pub(self) inner: HashMap<T, Box<dyn Any>>,
// }
// /// WIP: not fully functionnal/practical
// /// there is a way of getting a type name with `std::any::type_name::<T>()` which returns `"T" as &str`
// /// with a macro we can get the reversed processus by parsing the `"T"` inside `downcast_ref::<T>()`
// /// allowing a behavior such as `let value = dict.get!(my_key).unwrap();`, which could be simplified as
// /// `let value = dict[my_key]`, instead of `let value: &T = dict.get::<T>(my_key).unwrap();`
// /// `.unwrap()` is needed as it is the responsibility of the coder to handle missbehavior of the dict usage
// /// # Example of current usage
// /// ```rust
// /// # use adam::dict::Dict;
// /// let mut my_dict = Dict::new();
// /// my_dict.insert("key", "value");
// /// println!("{}", my_dict.get::<&str>("key").unwrap());
// /// ```
// impl<T> Dict<T>
// where
//     T: Eq + PartialEq + Hash,
// {
//     pub fn new() -> Self {
//         Dict {
//             inner: HashMap::new(),
//         }
//     }
//     pub fn insert<U: 'static>(&mut self, key: T, hole: U) {
//         self.inner.insert(key, Box::new(hole));
//     }

//     pub fn get<U: 'static>(&self, key: T) -> Result<&U, DictError> {
//         // match self.inner.get(&key).unwrap().as_ref().downcast_ref::<U>()
//         match self.inner.get(&key) {
//             Some(boxed_value) => match boxed_value.as_ref().downcast_ref::<U>() {
//                 Some(value) => Ok(value),
//                 None => Err(DictError::FailedCasting),
//             },
//             None => Err(DictError::KeyNotFound),
//         }
//     }
//     pub fn get_mut<U: 'static>(&mut self, key: T) -> Result<&mut U, DictError> {
//         // match self.inner.get(&key).unwrap().as_ref().downcast_ref::<U>()
//         match self.inner.get_mut(&key) {
//             Some(boxed_value) => match boxed_value.as_mut().downcast_mut::<U>() {
//                 Some(value) => Ok(value),
//                 None => Err(DictError::FailedCasting),
//             },
//             None => Err(DictError::KeyNotFound),
//         }
//     }
// }
// #[derive(Debug)]
// pub enum DictError {
//     KeyNotFound,
//     FailedCasting,
// }

use std::{
    collections::{
        hash_map::{Keys, Values},
        HashMap,
    },
    hash::Hash,
    iter::{zip, Zip},
    ops::{Index, IndexMut},
};

#[derive(Debug, Default)]
pub struct Dict<T, U>
where
    T: Eq + PartialEq + Hash,
{
    hash: HashMap<T, U>,
}
impl<T, U> Dict<T, U>
where
    T: Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Dict {
            hash: HashMap::new(),
        }
    }

    pub fn keys(&self) -> Keys<'_, T, U> {
        self.hash.keys()
    }

    pub fn values(&self) -> Values<'_, T, U> {
        self.hash.values()
    }

    pub fn items(&self) -> Zip<Keys<'_, T, U>, Values<'_, T, U>> {
        zip(self.hash.keys(), self.hash.values())
    }

    pub fn len(&self) -> usize {
        self.keys().len()
    }
}

impl<T, U> Index<T> for Dict<T, U>
where
    T: Eq + PartialEq + Hash,
{
    type Output = U;

    fn index(&self, index: T) -> &Self::Output {
        self.hash.get(&index).unwrap()
    }
}

impl<T, U> IndexMut<T> for Dict<T, U>
where
    T: Eq + PartialEq + Hash + Clone,
    U: Default,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        match self.hash.get(&index) {
            Some(_) => (),
            None => {
                self.hash.insert(index.clone(), U::default());
            }
        }
        self.hash.get_mut(&index).unwrap()
    }
}
