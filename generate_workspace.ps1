# Arrêter le script en cas d'erreur
$ErrorActionPreference = "Stop"

# Créer les dossiers
New-Item -ItemType Directory -Force -Path ".\my_workspace\nested_impl_gen\src" | Out-Null
New-Item -ItemType Directory -Force -Path ".\my_workspace\tenseur_app\src" | Out-Null

# Créer le Cargo.toml du workspace
@"
[workspace]
members = [
    "nested_impl_gen",
    "tenseur_app",
]
"@ | Out-File -FilePath ".\my_workspace\Cargo.toml" -Encoding utf8

# Créer le Cargo.toml pour le crate proc-macro nested_impl_gen
@"
[package]
name = "nested_impl_gen"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
proc-macro2 = "1.0"
"@ | Out-File -FilePath ".\my_workspace\nested_impl_gen\Cargo.toml" -Encoding utf8

# Créer le fichier src\lib.rs pour nested_impl_gen
@"
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

/// Macro procédurale générant les implémentations du trait \`NestedVecMaker\` pour \`Nested<N>\`
/// pour N de 0 jusqu'à une profondeur maximale spécifiée.
#[proc_macro]
pub fn generate_nested_impl(input: TokenStream) -> TokenStream {
    let max_depth_lit = parse_macro_input!(input as LitInt);
    let max_depth: usize = max_depth_lit.base10_parse().unwrap();

    let mut impls = quote! {
        impl<T> NestedVecMaker for Nested<0> {
            type Output<U> = U;
        }
    };

    // Générer les implémentations pour N de 1 jusqu'à max_depth
    for n in 1..=max_depth {
        let current = n;
        let prev = n - 1;
        impls.extend(quote! {
            impl<T> NestedVecMaker for Nested<#current> {
                type Output<U> = Vec<<Nested<#prev> as NestedVecMaker>::Output<U>>;
            }
        });
    }

    impls.into()
}
"@ | Out-File -FilePath ".\my_workspace\nested_impl_gen\src\lib.rs" -Encoding utf8

# Créer le Cargo.toml pour le crate d'application tenseur_app
@"
[package]
name = "tenseur_app"
version = "0.1.0"
edition = "2021"

[dependencies]
nested_impl_gen = { path = "../nested_impl_gen" }
"@ | Out-File -FilePath ".\my_workspace\tenseur_app\Cargo.toml" -Encoding utf8

# Créer le fichier src\main.rs pour tenseur_app
@"
#![feature(adt_const_params, generic_const_exprs, generic_associated_types)]
#![recursion_limit = "512"]

use nested_impl_gen::generate_nested_impl;
use std::fmt::Debug;

/// Ce trait associe, pour une profondeur donnée, un type Output tel que :
/// - Pour N = 0, Output<U> = U
/// - Pour N > 0, Output<U> = Vec<<Nested<{N-1}> as NestedVecMaker>::Output<U>>
trait NestedVecMaker {
    type Output<U>: Debug;
}

/// Structure "porteuse" du paramètre de profondeur.
struct Nested<const N: usize>;

/// Génère automatiquement les implémentations pour N de 0 jusqu'à 72.
generate_nested_impl!(72);

/// La structure Tenseur encapsule une donnée dont le type est défini par la profondeur N et le type de base T.
/// Par exemple :
/// - Tenseur<3, f64> correspond à Vec<Vec<Vec<f64>>>
/// - Tenseur<6, u32> correspond à Vec<Vec<Vec<Vec<Vec<Vec<u32>>>>>>>
#[derive(Debug)]
struct Tenseur<const N: usize, T> {
    data: <Nested<N> as NestedVecMaker>::Output<T>,
}

impl<const N: usize, T> Tenseur<N, T> {
    pub fn new(data: <Nested<N> as NestedVecMaker>::Output<T>) -> Self {
        Tenseur { data }
    }
}

fn main() {
    // Exemple : Tenseur<3, f64> correspond à Vec<Vec<Vec<f64>>>
    let tens_f64: Tenseur<3, f64> = Tenseur::new(vec![
        vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ],
        vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ],
    ]);
    println!("Tenseur<3, f64> = {:#?}", tens_f64.data);

    // Exemple : Tenseur<6, u32> correspond à Vec<Vec<Vec<Vec<Vec<Vec<u32>>>>>>>
    // Pour alléger l'exemple, nous fournissons un seul élément par niveau.
    let tens_u32: Tenseur<6, u32> = Tenseur::new(vec![
        vec![
            vec![
                vec![
                    vec![
                        vec![1, 2],
                        vec![3, 4],
                    ],
                    vec![
                        vec![5, 6],
                        vec![7, 8],
                    ],
                ],
                vec![
                    vec![
                        vec![9, 10],
                        vec![11, 12],
                    ],
                    vec![
                        vec![13, 14],
                        vec![15, 16],
                    ],
                ],
            ],
        ],
    ]);
    println!("Tenseur<6, u32> = {:#?}", tens_u32.data);
}
"@ | Out-File -FilePath ".\my_workspace\tenseur_app\src\main.rs" -Encoding utf8

Write-Host "Workspace generated in .\my_workspace"
