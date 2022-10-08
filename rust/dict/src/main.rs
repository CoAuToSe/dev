use dict::dict;

use std::{
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut},
    prelude::rust_2021::*,
};

// use num_traits::*;

const _DICT: &str = r#"dict!{ "left_eye": { "pos": 0, "final_pos": 0, "min_pos": 0, "max_pos": 0, "init": true, "current_zoom": -1, "compressed_img": None, "focus_flag": false, }, "right_eye": { "pos": 0, "final_pos": 0, "min_pos": 0, "max_pos": 0, "init": true, "current_zoom": -1, "compressed_img": None, "focus_flag": false, }, };"#;

fn main() {
    println!("Hello World!");
    // let temp = dict! { "left_eye": { "pos": 0, "final_pos": 0, "min_pos": 0, "max_pos": 0, "init": true, "current_zoom": -1, "compressed_img": None, "focus_flag": false, }, "right_eye": { "pos": 0, "final_pos": 0, "min_pos": 0, "max_pos": 0, "init": true, "current_zoom": -1, "compressed_img": None, "focus_flag": false, }, };
    // println!("{:?}", temp);
    // println!("{}", temp);
    // println!("{:?}", temp["left_eye"]);
    // println!("{}", temp["left_eye"]);
    let ea = tesy("ko".to_string(), "emos".to_string());
    println!("{:?}", ea);
    let zae = &ea["hi".to_string()];
    println!("{:?}", zae);
}

trait Indor: Debug + Index<String, Output = dyn Debug> {}
impl Indor for (dyn Debug + Index<String, Output = dyn Debug>) {}

fn tesy(key: String, some: String) -> impl Indor
// where
//     <impl Index<std::string::String> + Debug as Index<std::string::String>::Output: Debug,
{
    // Group(vec![key, some])
    Var(key)
}
#[derive(Debug)]
struct Group(Vec<String>);
impl Index<String> for Group {
    type Output = String;
    fn index(&self, index: String) -> &Self::Output {
        &self.0[0]
    }
}

#[derive(Debug)]
struct Var<T>(T);

impl Index<String> for Var<String> {
    type Output = dyn Debug;
    fn index(&self, index: String) -> &Self::Output {
        &self.0
    }
}
impl<T> Deref for Var<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Var<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// type Temp = dyn Deref<Target = Box<dyn Num>>;

// #[derive(Debug, Clone)]
// // enum VareType<T, N: Deref<Target = Var<T>>> {
// enum VareType<A, B, C, D> {
//     One(A),
//     Two(B),
//     Three(C),
//     Four(D),
//     Group(Vec<VareType<A, B, C, D>>),
//     None,
// }
// #[derive(Debug, Clone)]
// enum VareType<T, N: Deref<Target = Var<T>>, S: Into<String>> {
//     Num(N),
//     String(S),
//     Group(Vec<VareType<T, N, S>>),
//     None,
// }

// #[derive(Debug, Clone)]
// struct Same<'a, N: Num, S: Into<String>> {
//     vec: Vec<(Var<'a, N, S>, Var<'a, N, S>)>,
// }

// fn te() -> impl IndexMut<String> {
//     let vec = vec![
//         ("ok", 0),
//         ("lol", 1)
//     ];
//     vec
// }
// impl<T: Debug, N: Deref<Target = Var<T>> + Debug> Display for VareType<T, N> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl<T: Debug> Debug for Same<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.vec)
//     }
// }

// impl<'a, N: Num, S: Into<String>> From<Vec<T>> for Same<'a, N, S> {
//     fn from(vec: Vec<T>) -> Self {
//         Same { vec }
//     }
// }

// impl<T, N: Deref<Target = Var<T>>> Deref for VareType<T, N> {
//     type Target = Vec<(VareType<T, N>, VareType<T, N>)>;
//     fn deref(&self) -> &Self::Target {
//         &self
//     }
// }

// impl<T, N: Deref<Target = Var<T>>> DerefMut for VareType<T, N> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         // &mut self.0
//         self
//     }
// }
