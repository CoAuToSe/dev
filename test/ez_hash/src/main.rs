extern crate dict;
extern crate dicto;
use dict::*;
use dicto::*;
use num::{self, Num};

fn main() {
    println!("Hello, world!");
    let mut daict: Dicto<&str> = Dicto::new();
    println!("{:?}", daict);
    daict["lol"] = 45.0.to_string().into();
    println!("{:?}", daict);
    println!("{:?}", daict["lol"].lol());

    // let mut daict: Dicto<String> = Dicto::new();
    // println!("{:?}", daict);
    // // *(daict["lol".to_string()].refe_mut()) = 45.0;
    // println!("{:?}", daict);
}

struct A;
struct B {
    f: usize,
}

struct Saze<T, Marker> {
    first: T,
    second: Marker,
}

trait MarkerTrait: Sized {
    fn mark<T>(&self, para: &Saze<T, Self>);
}

impl MarkerTrait for A {
    fn mark<T>(&self, para: &Saze<T, Self>) {
        todo!()
    }
}
impl MarkerTrait for B {
    fn mark<T>(&self, para: &Saze<T, Self>) {
        todo!()
    }
}
impl<T> Saze<T, A> {
    fn new() {}
}
impl<T> Saze<T, B> {
    fn new() {}
}

impl<T, Marker> Saze<T, Marker>
where
    T: Sized,
    Marker: MarkerTrait,
{
    fn working(&self) {
        self.second.mark(self)
    }
}

struct MyStruct<T> {
    f: T,
}
impl<T> MyStruct<T> {
    fn new(f: T) -> Self {
        MyStruct { f }
    }
}

trait MyStructExt: Sized {
    fn work(self) -> MyStruct<Self>;
}

impl<T> MyStructExt for T {
    fn work(self) -> MyStruct<Self> {
        MyStruct::new(self)
    }
}
