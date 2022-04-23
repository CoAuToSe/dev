/**/
pub(self) struct NeuralNetwork<T> {
    inputs: Vec<Input<T>>,
    outputs: Vec<Output<T>>,
    network: NNStructure<T>,
}

pub(self) struct Neurone<T> {
    internal_value: Option<T>,
    internal_func: Option<(*mut Fn(T) -> T, *mut Fn(T) -> T)>,
    from: Vec<(*mut Neurone<T>, T)>,
    to: Vec<(*mut Neurone<T>, T)>,
}

impl<T> Neurone<T> {
    pub(self) fn new() -> Neurone<T> {
        Neurone {
            internal_value: None,
            internal_func: None,
            from: vec![],
            to: vec![],
        }
    }
}

pub(self) enum Input<T> {
    LOL(T),
}
pub(self) enum Output<T> {
    LOL(T),
}
pub(self) enum VecOrOne<T> {
    Vec(Vec<T>),
    One(T),
}
pub(self) enum NNStructure<T> {
    Layers {
        length: usize,
        width: VecOrOne<usize>,
        all_neurones: Vec<Vec<Neurone<T>>>,
        function: Option<()>,
        inverse_function: Option<()>,
    },
}
fn main() {
    println!("Hello, world!");
}
