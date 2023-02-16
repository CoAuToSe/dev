use std::{
    collections::HashMap,
    fmt::{Debug, DebugMap},
    hash::Hash,
    marker::PhantomData,
    ops::{AddAssign, Deref, Index, IndexMut},
};

// use serde::{Deserialize, Serialize};
// // use serde_json::*;
// // extern crate futures; // 0.1.24
use serde::{Deserialize, Serialize};
use serde_json::Value;

// #[derive(Serialize, Deserialize, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let point = Point { x: 1, y: 2 };

//     let serialized = serde_json::to_string(&point).unwrap();
//     println!("serialized = {}", serialized);

//     let deserialized: Point = serde_json::from_str(&serialized).unwrap();
//     println!("deserialized = {:?}", deserialized);
// }
#[derive(Serialize, Deserialize, Debug)]
pub struct MyString {
    s: String,
}

struct MyStruct<'a, T: Serialize + Deserialize<'a>> {
    f: T,
    phantom_data: PhantomData<&'a T>,
}
impl<'a, T: Serialize + Deserialize<'a>> MyStruct<'a, T> {
    fn new(f: T) -> Self {
        MyStruct {
            f,
            phantom_data: PhantomData,
        }
    }
}

trait MyStructExt: Sized {
    fn new<'a>(self) -> MyStruct<'a, Self>
    where
        Self: Serialize + Deserialize<'a>;
}

impl<T> MyStructExt for T {
    fn new<'a>(self) -> MyStruct<'a, Self>
    where
        Self: Serialize + Deserialize<'a>,
    {
        MyStruct::new(self)
    }
}

// use futures::Future;
// use std::io;
// pub struct Context;
// pub trait DontCareType {
//     fn val(self: Self) -> Self;
//     fn refe<'a>(self: &'a Self) -> &'a Self;
//     fn refe_mut<'a>(self: &'a mut Self, some: Self) -> &'a mut Self;
// }
// impl<T> DontCareType for T {
//     fn val(self, some: T) -> Self {
//         some
//     }

//     fn refe<'a>(&'a self, some: T) -> &'a Self {
//         &some
//     }
//     fn refe_mut<'a>(&'a mut self, some: T) -> &'a mut Self {
//         &mut some
//     }
// }
// trait NewTrait<'a>: Serialize + Deserialize<'a> {
//     fn encode(&self) -> String
//     where
//         Self: Sized;
// }
// impl<'a, T: Serialize + Deserialize<'a>> NewTrait<'a> for T {
//     fn encode(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
// }
// #[derive(Debug)]
pub struct Dicto<'a, T>
where
    T: Eq + PartialEq + Hash,
{
    hash: HashMap<T, MyStruct<'a, MyString>>,
}

impl<'a, T> Debug for Dicto<'a, T>
where
    T: Eq + PartialEq + Hash + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dicto")
            .field("hash_keys", &self.hash.keys())
            .finish()
    }
}

impl<'a, T> Dicto<'a, T>
where
    T: Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Dicto {
            hash: HashMap::new(),
        }
    }
}

impl<T> Index<T> for Dicto<'static, T>
where
    T: Eq + PartialEq + Hash,
{
    type Output = MyString;

    fn index(&self, index: T) -> &Self::Output {
        &self.hash.get(&index).unwrap().f
    }
}

impl<T> IndexMut<T> for Dicto<'static, T>
where
    T: Eq + PartialEq + Hash + Clone,
    // U: Default,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        match self.hash.get(&index) {
            Some(_) => (),
            None => {
                self.hash.insert(
                    index.clone(),
                    MyStruct {
                        f: "".to_string().into(),
                        phantom_data: PhantomData,
                    },
                );
            }
        }
        &mut self.hash.get_mut(&index).unwrap().f
    }
}
impl<'a, T> AddAssign<T> for MyString
where
    T: Eq + PartialEq + Hash + Clone + Serialize + Deserialize<'a>,
{
    fn add_assign(&mut self, rhs: T) {
        self.s = serde_json::to_string(&rhs).unwrap();
    }
}

// impl<'a, T> Into<MyString> for MyStruct<'a, T>
// where
//     T: Into<String> + Serialize + Deserialize<'a>,
// {
//     fn into(self) -> MyString {
//         MyString { s: T::into(self.f) }
//     }
// }

// impl From<String> for MyString {
//     fn from(value: String) -> Self {
//         MyString { s: value }
//     }
// }
impl<T> From<T> for MyString
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        MyString { s: value.into() }
    }
}

impl Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl MyString {
    pub fn lol(&self) -> Value {
        serde_json::to_value(self.s.as_str()).unwrap()
    }
}
