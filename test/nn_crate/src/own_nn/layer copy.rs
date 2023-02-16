use crate::neural_ops::NeuralOps;

// pub struct UnSizedLayer;
// pub struct SizedLayer(usize, usize);
// pub enum TOL<T> {
//     In(Box<[T]>),
//     Out(Box<[T]>),
//     Hidden(Box<[Box<[T]>]>),
//     Uninitialized,
// }

pub struct In<'a, T>(Box<&'a [T]>, Option<NeuralOps<T>>);
pub struct Out<'a, T>(Box<&'a [T]>, Option<NeuralOps<T>>);
pub struct Hidden<'a, T>(Box<&'a [Box<[T]>]>, Option<NeuralOps<T>>);
pub struct Uninitialized;

pub struct Layer<TypeOfLayer> {
    layer: TypeOfLayer,
}

impl Layer<Uninitialized> {
    fn new() -> Layer<Uninitialized> {
        Layer {
            layer: Uninitialized,
        }
    }

    fn input_unknown<T>(inputs: &[T]) -> Layer<In<T>> {
        Layer {
            layer: In(Box::new(inputs), None),
        }
    }

    fn out_unknown<T>(ouputs: &[T]) -> Layer<Out<T>> {
        Layer {
            layer: Out(Box::new(ouputs), None),
        }
    }

    fn hidden_unknown<T>(weights: &[Box<[T]>]) -> Layer<Hidden<T>> {
        Layer {
            layer: Hidden(Box::new(weights), None),
        }
    }
}

impl<'a, T> Layer<In<'a, T>> {
    fn set_activation_function(&mut self, active_fn: fn(T) -> T, derive_fn: fn(T) -> T) {
        self.layer.1 = Some(NeuralOps {
            activation: active_fn,
            derivative: derive_fn,
        });
    }
}
impl<'a, T> Layer<Out<'a, T>> {
    fn set_activation_function(&mut self, active_fn: fn(T) -> T, derive_fn: fn(T) -> T) {
        self.layer.1 = Some(NeuralOps {
            activation: active_fn,
            derivative: derive_fn,
        });
    }
}
impl<'a, T> Layer<Hidden<'a, T>> {
    fn set_activation_function(&mut self, active_fn: fn(T) -> T, derive_fn: fn(T) -> T) {
        self.layer.1 = Some(NeuralOps {
            activation: active_fn,
            derivative: derive_fn,
        });
    }
}
// impl<T> Layer<T, TOL<T>::In> {
//     fn new(weights: TOL<T>, operator: NeuralOps<T>) -> Self {
//         Self { weights, operator }
//     }
// }

// struct WeightHolder<TypeOfLayer> {}

// impl<T> Layer<T> {}

// pub trait LayerTrait<T>: Sized {
//     const IN: usize;
//     const OUT: usize;
//     fn weights(self: &mut Self) -> &mut [[T; Self::IN]; Self::OUT];
// }
