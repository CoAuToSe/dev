use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;

fn main() {
    // Récupérer le dossier de sortie pour le build script.
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_nested_impl.rs");
    let mut f = BufWriter::new(File::create(&dest_path).unwrap());

    // Implémentation pour Nested<0>
    writeln!(f, "impl NestedVecMaker for Nested<0> {{").unwrap();
    writeln!(f, "    type Output<U: std::fmt::Debug> = U;").unwrap();
    writeln!(f, "}}").unwrap();

    writeln!(f, r#"
    impl<T: Debug> Tenseur<const N: usize, T> {{
        pub fn new(data: <Nested<{{N}}> as NestedVecMaker>::Output<T>) -> Self {{
            Tenseur {{ data }}
        }}
    }}
    "#).unwrap();

    // Génération des implémentations pour Nested<1> jusqu'à Nested<72>
    for n in 1..=72 {
        let prev = n - 1;
        writeln!(f, "impl NestedVecMaker for Nested<{n}> {{", n = n).unwrap();
        writeln!(f, "    type Output<U: std::fmt::Debug> = Vec<<Nested<{prev}> as NestedVecMaker>::Output<U>>;", prev = prev).unwrap();
        writeln!(f, "}}").unwrap();

        // /// La structure Tenseur encapsule une donnée dont le type est déterminé par la profondeur N
        // /// et le type de base T. Par exemple :
        // /// - Tenseur<3, f64> correspond à Vec<Vec<Vec<f64>>>
        // /// - Tenseur<6, u32> correspond à Vec<Vec<Vec<Vec<Vec<Vec<u32>>>>>>>
        // #[derive(Debug)]
        // struct Tenseur<const N: usize, T> {
        //     data: <Nested<N> as NestedVecMaker>::Output<T>,
        // }

        // impl<const N: usize, T> Tenseur<N, T> {
        //     pub fn new(data: <Nested<N> as NestedVecMaker>::Output<T>) -> Self {
        //         Tenseur { data }
        //     }
        // }
        
        writeln!(f, r#"
        #[derive(Debug)]
        struct Tenseur{n}<T: Debug> {{
            data: <Nested<{n}> as NestedVecMaker>::Output<T>,
        }}

        "#, n = n).unwrap();
    }
}
