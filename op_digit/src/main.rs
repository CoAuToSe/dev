#![no_std]
extern crate core;
use core::{
    // any:&:&Any,
    convert::{Into, TryFrom},
    fmt::Debug,
    ops::Add,
};
extern crate alloc;
// use alloc::vec::Vec;

// #[derive(Debug)]
struct Numbar<'a, A, B, C> {
    pl: &'a [A],
    pn: &'a [B],
    r: C,
}
macro_rules! typ {
    ($type:ty) => {
        core::any::type_name::<$type>()
    };
}

// pub(crate) mod macros {
//     /// Derive macro generating an impl of the trait `Debug`.
//     #[rustc_builtin_macro]
//     #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
//     #[allow_internal_unstable(core_intrinsics)]
//     pub macro Debug($item:item) {
//         /* compiler built-in */
//     }
// }

impl<'a, A: fmt::Debug, B: fmt::Debug, C: fmt::Display> fmt::Debug for Numbar<'a, A, B, C> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct(&format!("Numbar<{},{},{}>", typ!(A), typ!(B), typ!(B)))
            .field("pl", &self.pl)
            .field("pn", &self.pn)
            .field("r", &format_args!("{}", self.r))
            .finish()
    }
}

// impl Into<i32> for Numbar<'_, i32> {
//     fn into(self) -> i32 {
//         self.r
//     }
// }
// struct Wrapper<T>(Vec<T>);
// impl<T> From<Wrapper<T>> for Vec<T> {
//     fn from(w: Wrapper<T>) -> Vec<T> {
//         w.0
//     }
// }
// struct Wrapper<T>(Vec<T>);
// impl<T> Into<Vec<T>> for Wrapper<T> {
//     fn into(self) -> Vec<T> {
//         self.0
//     }
// }

impl<C> From<C> for Numbar<'_, C, C, C> {
    fn from(num: C) -> Self {
        Numbar {
            pl: &[],
            pn: &[],
            r: num,
        }
    }
}
macro_rules! impl_from_for {
    (impl From<Numbar> for $($typa:ty),*  ) => {
        $(
        impl<A, B> From<Numbar<'_,A,B, $typa>> for $typa {
            fn from(num : Numbar<'_, A, B, $typa>) -> $typa {
                num.r
            }
        }
        )*
    };
}
impl_from_for!(impl From<Numbar> for usize,i32, isize );
// macro_rules! impl_from_for {
//     (impl From<Numbar> for $($typa:ty),*  ) => {
//         $(
//         impl<A, B, C> From<Numbar<'_,A,B, C>> for D
//             where C == $typa,
//         {
//             fn from(num : Numbar<'_, A, B, $typa>) -> $typa {
//                 num.r
//             }
//         }
//         )*
//     };
// }
// impl_from_for!(impl From<Numbar> for i32 );

// trait NewTrait: Any + Sized {}
// impl<A, Y, U> From<Numbar<'_, Y, U, A>> for dyn NewTrait {
//     fn from<T, V>(num: Numbar<'_, T, V, A>) -> A {
//         num.r
//     }
// }
// pub type Infallible = dyn std::any::Any;
// impl<T, U> TryFrom<U> for T
// where
//     U: Into<T>,
// {
//     type Error = Infallible;

//     fn try_from(value: U) -> Result<Self, Infallible> {
//         Ok(U::into(value)) // Never returns `Err`
//     }
// }

// impl<T> From<Numbar<'_, T>> for T {
//     fn from(num: Numbar<'_, T>) -> Self {
//         num.r
//     }
// }

use core::fmt;

use alloc::format;

// impl<'a, T: Sized> From<Numbar<'a, T>> for T {
//     fn from(num: Numbar<'a, T>) -> Self {
//         num.r
//     }
// }
fn main() {
    extern crate std;
    use std::println;
    println!("Hello, world!");
    let numbers = Numbar {
        pl: &[2, 3, 5],
        pn: &[1, 0, 0],
        r: 1,
    };
    println!("{:?}", Numbar::from(10));
    let aze: i32 = numbers.into();
    println!("{:?}", aze);
}
