use tch::{Tensor, Device};

fn main() {
    println!("Hello, LibTorch avec Rust!");

    // Création d'un tenseur 2x2 rempli de 1
    let tensor = Tensor::ones(&[2, 2], (tch::Kind::Float, Device::Cpu));
    println!("Tenseur initial:\n{}", tensor);

    // Addition simple : on ajoute 1 à chaque élément du tenseur
    let resultat = &tensor + 1;
    println!("Résultat après addition:\n{}", resultat);
}
