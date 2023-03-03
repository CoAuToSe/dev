use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
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
