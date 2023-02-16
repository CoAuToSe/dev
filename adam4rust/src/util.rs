use std::{any::Any, collections::HashMap, hash::Hash};

pub struct Group<T, U> {
    amsgrad: T,
    params: Paze<U>,
    weight_decay: T,
    eps: T,
    lr: T,
}

pub struct Paze<T> {
    grad: T,
}
