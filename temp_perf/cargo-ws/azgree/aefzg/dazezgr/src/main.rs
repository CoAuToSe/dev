struct Container<T>(T, T);

// A trait which checks if 2 items are stored inside of container.
// Also retrieves first or last value.
trait Contains {
    // Define generic types here which methods will be able to utilize.
    type A: Eq;
    type B: Eq;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> Self::A;
    fn last(&self) -> Self::B;
}

impl<T: Eq + Copy> Contains for Container<T> {
    // Specify what types `A` and `B` are. If the `input` type
    // is `Container(i32, i32)`, the `output` types are determined
    // as `i32` and `i32`.
    type A = T;
    type B = T;

    // `&Self::A` and `&Self::B` are also valid here.
    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number.
    fn first(&self) -> Self::A {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> Self::B {
        self.1
    }
}

fn difference<D: std::ops::Sub<Output = D>, C: Contains<A = D, B = D>>(container: &C) -> D {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
