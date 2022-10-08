// #![allow(unused)]
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
    process::Output,
    rc::Rc,
};

// trait Null<T> {
//     fn null() -> T;
// }

// impl<T: Default> Null<T> for T {
//     fn null() -> T {
//         Default::default()
//     }
// }

// #[derive(Clone)]
// struct Var<T, B>(Option<Box<dyn Fn(T) -> B>>, PhantomData<T>, PhantomData<B>);
// type Var<'a, A, B> = &'a mut dyn Fn(A) -> B;
// trait Var<T, B>: Fn(T) -> B {}

// #[derive(Default)]
// struct Function<T> {
//     some: T,
// }

// type Ome<T, U> = dyn FnOnce(T) -> U;

// impl<T, U, V> Add<V> for Ome<T, U>
// where
//     U: Add<V>,
// {
//     type Output = Ome<T, <U as Add<V>>::Output>;
//     fn add(self, rhs: V) -> Self::Output {
//         Default::default()
//     }
// }

// trait Bitch<'a, T: 'a, A: 'a>: Add<T, Output = A> + Sized {
//     fn add(self, _: T) -> &'a mut dyn Fn(T) -> A;
// }
// trait Bitch<'a, T: 'a, A: 'a>: Add<T, Output = A> + Sized {
//     fn add(self, _: T) -> Var<T, A>;
// }

// // impl<'a, B, U> Bitch<'a, U, B> for U
// impl<'a, U, B> Add<U> for Var<U, B>
// where
//     U: 'a + Into<B> + Clone, // TODO: replace Add<U,B> by Into<B>
//     B: 'a + Clone + Sized,
// {
//     type Output = B;
//     fn add(self, rhs: U) -> Self::Output {
//         let temp = Box::new(move || -> B { rhs.clone().into() });
//         temp()
//     }
// }
// impl<'a, B, U> Bitch<'a, U, B> for Var<U, B>
// where
//     U: 'a + std::ops::Add<U, Output = B> + Clone + 'static,
//     B: 'a + Clone + std::convert::From<U> + 'static + std::ops::Add<U, Output = B>,
// {
//     fn add(self, rhs: U) -> Var<U, B> {
//         // let temp = Box::new(move |var: U| -> B { var.clone() + rhs.clone() });
//         let temp = Box::new(move |var: U| -> B {$ops
//             match &self.0 {
//                 Some(ref old_fn) => old_fn(var.clone()) + rhs.clone(),
//                 None => var.clone() + rhs.clone(),
//             }
//         });
//         // let azdaz = Box::leak(temp);
//         Var(Some(temp), PhantomData, PhantomData)
//     }
// }

#[derive(Default)]
struct Var<T, B> {
    nf: Option<Box<dyn Fn(T) -> B>>,
    // ops: Vec<Rc<Box<dyn Fn(T) -> B>>>,
    // rnf: Option<Box<dyn Fn(B) -> T>>,
}

impl<'a, B, U> std::ops::Add<U> for Var<U, B>
where
    U: 'a + std::ops::Add<U, Output = B> + Clone + 'static,
    B: 'a + std::ops::Add<U, Output = B> + Clone + 'static,
{
    fn add(self, rhs: U) -> Var<U, B> {
        // let rhs1 = Rc::new(rhs);
        // let rhs2 = Rc::clone(&rhs1);
        let temp = Box::new(move |var: U| -> B {
            match &self.nf {
                Some(ref old_fn) => old_fn(var.clone()) + rhs.clone(),
                None => var.clone() + rhs.clone(),
            }
        });
        // let rhs2 = Rc::clone(&rhs1);
        // let rev_temp = Box::new(move |var: B| -> U {
        //     match &self.rnf {
        //         Some(ref old_fn) => old_fn(var.clone()) - *rhs2.clone(),
        //         None => var.clone() - rhs.clone(),
        //     }
        // });
        // let mut tza = self.ops;
        // tza.push(Rc::new(temp));
        Var {
            nf: Some(temp),
            // ops: tza,
            // rnf: Some(rev_temp),
        }
    }

    type Output = Var<U, B>;
}
// impl<'a, B, U> std::ops::AddAssign<U> for Var<U, B>
// where
//     U: 'a + std::ops::Add<U, Output = B> + std::ops::Sub<U, Output = B> + Clone + 'static,
//     B: 'a + std::ops::Add<U, Output = B> + std::ops::Sub<U, Output = B> + Clone + 'static,
// {
//     fn add_assign(&mut self, rhs: U) {
//         *self = *self + rhs;
//     }
// }

// #[derive(Default)]
// struct Var<T, B> {
//     nf: Option<Box<dyn Fn(T) -> B>>,
//     ops: Vec<Rc<Box<dyn Fn(T) -> B>>>,
// }
// macro_rules! impl_att {
//     ($t1:ident ; $t2:ident ;$ops:tt) => {
//         impl<'a, B, U> std::ops::$t1<U> for Var<U, B>
//         where
//             U: 'a + std::ops::$t1<U, Output = B> + Clone + 'static,
//             B: 'a + std::ops::$t1<U, Output = B> + Clone + 'static,
//         {
//             fn $t2(self, rhs: U) -> Var<U, B> {
//                 let temp = Box::new(move |var: U| -> B {
//                     match &self.nf {
//                         Some(ref old_fn) => old_fn(var.clone()) $ops rhs.clone(),
//                         None => var.clone() $ops rhs.clone(),
//                     }
//                 });
//                 let mut tza = self.ops;
//                 tza.push(Rc::new(temp));
//                 Var {
//                     nf: Some(temp),
//                     ops: tza,
//                 }
//             }

//             type Output = Var<U, B>;
//         }
//     };
// }

// impl_att!(Add;add;+);
// impl_att!(Sub;sub;-);
// impl_att!(Mul;mul;*);
// impl_att!(Div;div;/);
// -------------------------------------------------------
// =======================================================
// struct Var<T, B>(Option<Box<dyn Fn(T) -> B>>);
// macro_rules! impl_att{
//     ($t1:ident ; $t2:ident ;$ops:tt) => {
//         impl<'a, B, U> std::ops::$t1<U> for Var<U, B>
//         where
//             U: 'a + std::ops::$t1<U, Output = B> + Clone + 'static,
//             B: 'a + std::ops::$t1<U, Output = B> + Clone + 'static,
//         {
//             fn $t2(self, rhs: U) -> Var<U, B> {
//                 let temp = Box::new(move |var: U| -> B {
//                     match &self.0 {
//                         Some(ref old_fn) => old_fn(var.clone()) $ops rhs.clone(),
//                         None => var.clone() $ops rhs.clone(),
//                     }
//                 });
//                 Var(Some(temp))
//             }

//             type Output = Var<U, B>;
//         }
//     };
// }
// impl_att!(Add;add;+);
// impl_att!(Sub;sub;-);
// impl_att!(Mul;mul;*);
// impl_att!(Div;div;/);
// =======================================================
// -------------------------------------------------------
// impl<'a, B, U> Add<U> for Var<U, B>
// where
//     U: 'a + std::ops::Add<U, Output = B> + Clone + 'static,
//     B: 'a + Clone + std::convert::From<U> + 'static + std::ops::Add<U, Output = B>,
// {
//     fn add(self, rhs: U) -> Var<U, B> {
//         // let temp = Box::new(move |var: U| -> B { var.clone() + rhs.clone() });
//         let temp = Box::new(move |var: U| -> B {
//             match &self.0 {
//                 Some(ref old_fn) => old_fn(var.clone()) + rhs.clone(),
//                 None => var.clone() + rhs.clone(),
//             }
//         });
//         // let azdaz = Box::leak(temp);
//         Var(Some(temp), PhantomData, PhantomData)
//     }

//     type Output = Var<U, B>;
// }
// impl<'a, B, U> Mul<U> for Var<U, B>
// where
//     U: 'a + std::ops::Mul<U, Output = B> + Clone + 'static,
//     B: 'a + Clone + std::convert::From<U> + 'static + std::ops::Mul<U, Output = B>,
// {
//     fn mul(self, rhs: U) -> Var<U, B> {
//         // let temp = Box::new(move |var: U| -> B { var.clone() + rhs.clone() });
//         let temp = Box::new(move |var: U| -> B {
//             match &self.0 {
//                 Some(ref old_fn) => old_fn(var.clone()) * rhs.clone(),
//                 None => var.clone() * rhs.clone(),
//             }
//         });
//         // let azdaz = Box::leak(temp);
//         Var(Some(temp), PhantomData, PhantomData)
//     }

//     type Output = Var<U, B>;
// }
// impl<'a, U, B> Add<U> for &'a mut dyn Fn(U) -> B
// where
//     U: 'a + std::ops::Add<U, Output = B> + Clone, // TODO: replace Add<U,B> by Into<B>
//     B: 'a + Clone,
// {
//     type Output = B;
//     fn add(self, rhs: U) -> Self::Output {
//         let temp = Box::new(move || -> B { rhs.clone() + rhs.clone() });
//         temp()
//     }
// }
// impl<'a, B, U> Bitch<'a, U, B> for &'a mut dyn Fn(U) -> B
// where
//     U: 'a + std::ops::Add<U, Output = B> + Clone,
//     B: 'a + Clone,
// {
//     fn add(self, rhs: U) -> &'a mut dyn Fn(U) -> B {
//         let temp = Box::new(move |var: U| -> B { var.clone() + rhs.clone() });
//         Box::leak(temp)
//     }
// }

// impl<'a, B, U> Add<U> for &'a mut dyn Fn(U) -> B
// where
//     B: 'a + Clone,
//     U: 'a + std::ops::Add<U, Output = B> + Clone,
// {
//     type Output = B;
//     fn add(self, rhs: U) -> Self::Output {
//         let temp = Box::new(move || -> B { rhs.clone() + rhs.clone() });
//         temp()
//     }
// }
// impl<'a, B, U> Bitch<'a, U, B> for &'a mut dyn Fn(U) -> B
// where
//     B: 'a + Clone,
//     U: 'a + std::ops::Add<U, Output = B> + Clone,
// {
//     fn add(self, rhs: U) -> &'a mut dyn Fn(U) -> B {
//         let temp = Box::new(move |var: U| -> B { var.clone() + rhs.clone() });
//         Box::leak(temp)
//     }
// }
// ===============================================
// type Ome<T, U> = fn(T) -> U;
// trait Bitch<'a, T: 'a, A: 'a>: Add<T, Output = A> + Sized {
//     fn add(self, _: T) -> &'a mut dyn Fn(T) -> A;
// }

// impl<'a, B, U> Bitch<'a, U, B> for U
// where
//     B: 'a + Clone + Copy,
//     U: 'a + std::ops::Add<U, Output = B> + Clone + Copy,
// {
//     fn add(self, rhs: U) -> &'a mut dyn Fn(U) -> B {
//         // let fucku = move |var: U| -> U /*where U: std::ops::Add<Output = U> + Clone*/{
//         //     var.clone() + rhs.clone()
//         // };
//         let temp = Box::new(
//             move |var: U| -> B /*where U: std::ops::Add<Output = U> + Clone*/{
//             var.clone() + rhs.clone()
//         },
//         );
//         Box::leak(temp)
//     }
// }
// ================================================
// ---------------------------------------------
// type Ome<T, U> = fn(T) -> U;
// trait Bitch<'a, T: 'a>: Add<T, Output = T> + Sized {
//     fn add(self, _: T) -> &'a mut dyn Fn(T) -> T;
// }

// impl<'a, U: 'a + std::ops::Add<Output = U> + Clone + Copy> Bitch<'a, U> for U
// // where
// // <T as Add<U>>::Output = V,
// {
//     fn add(self, rhs: U) -> &'a mut dyn Fn(U) -> U {
//         let fucku = move |var: U| -> U /*where U: std::ops::Add<Output = U> + Clone*/{
//             var.clone() + rhs.clone()
//         };
//         let temp = Box::new(
//             move |var: U| -> U /*where U: std::ops::Add<Output = U> + Clone*/{
//             var.clone() + rhs.clone()
//         },
//         );
//         Box::leak(temp)
//     }
// }
// ---------------------------------------------
// type Ome<T, U> = fn(T) -> U;
// trait Bitch<T>: Add<T, Output = T> + Sized {
//     fn add(self, _: T) -> fn(T) -> T;
// }

// impl<U: std::ops::Add<Output = U> + Clone> Bitch<U> for U
// // where
// // <T as Add<U>>::Output = V,
// {
//     fn add(self, rhs: U) -> fn(U) -> U {
//         fn lol<U: std::ops::Add<Output = U> + Clone>(var: U) -> U {
//             var.clone() + var
//         }
//         lol
//     }
// }

// type Ome<T, U> = fn(T) -> U;

// impl<T, U> Add<U> for Ome<T, T>
// // where
// // <T as Add<U>>::Output = V,
// {
//     type Output = Ome<T, U>;
//     fn add(self, rhs: U) -> Self::Output {
//         |var: T| -> U { var + rhs } as fn(T) -> U
//     }
// }

// impl<T, U> Add<U> for Ome<T, U>
// // where
// // <T as Add<U>>::Output = V,
// {
//     type Output = Ome<T, <T as Add<U>>::Output>;
//     fn add(self, rhs: U) -> Self::Output {
//         |var: T| -> <T as Add<U>>::Output { var + rhs } as fn(T) -> <T as Add<U>>::Output
//     }
// }

// #[derive(Default)]
// enum Interaction<T> {
//     Add(T),
//     // Association(Box<Operation<T>>, Box<Operation<U>>),
//     #[default]
//     None,
// }

// enum Association<T, U> {
//     Interactions(Interaction<T>, Interaction<U>),
// }

// struct AssOp<T, U>(Operation<T>, Operation<U>);

// impl<T> Add<T> for Var {
//     type Output = Operation<T>;
//     fn add(self, rhs: T) -> Self::Output {
//         Operation::Add(rhs)
//     }
// }
// impl<'a, T, U> Add<U> for Operation<T> {
//     type Output = AssOp<T, U>;
//     fn add(self, rhs: U) -> Self::Output {
//         AssOp(self, Operation::Add(rhs))
//     }
// }

fn main() {
    println!("Hello, world!");
    let mut x = Var::default();
    // let temps = Bitch::add(x, 1);
    // // println!("{}", (temps.0.unwrap())(2));
    // let temps2 = Bitch::add(temps, 1);
    // x += 1;
    // x += 1;
    // x += x;
    // println!("{}", (x.nf.unwrap())(2));
    x = x + 1;
    x = x + 1;
    // x = x * 2;
    println!("{}", (x.nf.unwrap())(2));
    // let temps = Bitch::add(x, 7.1);
    // println!("{}", temps(48.5));
    // println!("{:?}", x);
}
