use ndarray::{array, Array2};

fn main() {
    // Create two 2D matrices
    let a: Array2<f32> = array![[1., 2.], [3., 4.]];
    let b: Array2<f32> = array![[5., 6.], [7., 8.]];

    // Calculate the Hadamard product and then sum the elements
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(a, b)| a * b).sum();

    println!("The dot product is {}", dot_product);
    // Create two 2D matrices
    let a: Array2<f32> = array![[1., 2.], [3., 4.]];
    let b: Array2<f32> = array![[5., 6.], [7., 8.]];

    // Calculate the matrix multiplication
    let result: Array2<f32> = a.dot(&b);

    println!("The result of matrix multiplication is {:?}", result);
}
