use std::{
    fmt::{self, *},
    ops::{self, *},
};

trait Ops
where
    Self: Sized + Add + Mul + Sub + Div + Neg,
{
}

struct Function<'a, TypeOn: Ops> {
    raw: &'a mut String,
    funct: (dyn std::ops::Fn(TypeOn) -> TypeOn + 'static),
}

impl<'a, TypeOn: Ops> Debug for Function<'a, TypeOn> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\na function", self.raw)
    }
}

impl<'a, TypeOn: Ops> Function<'a, TypeOn> {
    fn new() -> Self {
        Self {
            raw: &mut String::new(),
            funct: |lol| lol,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let function = Function::new();
}
