#![feature(adt_const_params, generic_const_exprs, generic_associated_types)]
#![recursion_limit = "512"]

use std::fmt::Debug;

/// Trait qui, pour une profondeur donnée, associe un type imbriqué.
/// - Pour Nested<0>, Output<U> = U.
/// - Pour Nested<N> (N > 0), Output<U> = Vec<<Nested<{N-1}> as NestedVecMaker>::Output<U>>.
pub trait NestedVecMaker {
    type Output<U: std::fmt::Debug> : Debug;
}

/// Structure "porteuse" du paramètre de profondeur.
pub struct Nested<const N: usize>;

//
// Inclusion du fichier généré par build.rs.
// Ce fichier contient 73 implémentations pour Nested<0> jusqu'à Nested<72>.
//
include!(concat!(env!("OUT_DIR"), "/generated_nested_impl.rs"));


// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, LitInt};

// /// Macro procÃ©durale gÃ©nÃ©rant les implÃ©mentations du trait \NestedVecMaker\ pour \Nested<N>\
// /// pour N de 0 jusqu'Ã  une profondeur maximale spÃ©cifiÃ©e.
// #[proc_macro]
// pub fn generate_nested_impl(input: TokenStream) -> TokenStream {
//     let max_depth_lit = parse_macro_input!(input as LitInt);
//     let max_depth: usize = max_depth_lit.base10_parse().unwrap();

//     let mut impls = quote! {
//         impl<U> NestedVecMaker for Nested<0> {
//             type Output<U> = U;
//         }
//     };

//     // GÃ©nÃ©rer les implÃ©mentations pour N de 1 jusqu'Ã  max_depth
//     for n in 1..=max_depth {
//         let current = n;
//         let prev = n - 1;
//         let ok = format!("U{current}");
//         impls.extend(quote! {
//             impl<#ok> NestedVecMaker for Nested<#current> {
//                 type Output<#ok> = Vec<<Nested<#prev> as NestedVecMaker>::Output<#ok>>;
//             }
//         });
//     }

//     impls.into()
// }

