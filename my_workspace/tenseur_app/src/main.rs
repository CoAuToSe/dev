#![feature(adt_const_params, generic_const_exprs, generic_associated_types)]
#![recursion_limit = "512"]

use nested_impl_gen::{Nested, NestedVecMaker};
use std::fmt::Debug;



fn main() {
    // Exemple : Tenseur<3, f64> ~ Vec<Vec<Vec<f64>>>
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

    // Exemple : Tenseur<6, u32> ~ Vec<Vec<Vec<Vec<Vec<Vec<u32>>>>>>>
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



// #![feature(adt_const_params, generic_const_exprs, generic_associated_types)]
// #![recursion_limit = "1024"]

// use nested_impl_gen::generate_nested_impl;
// use std::fmt::Debug;

// /// Ce trait associe, pour une profondeur donnÃ©e, un type Output tel que :
// /// - Pour N = 0, Output<U> = U
// /// - Pour N > 0, Output<U> = Vec<<Nested<{N-1}> as NestedVecMaker>::Output<U>>
// trait NestedVecMaker {
//     type Output<U>: Debug;
// }

// /// Structure "porteuse" du paramÃ¨tre de profondeur.
// struct Nested<const N: usize>;

// /// GÃ©nÃ¨re automatiquement les implÃ©mentations pour N de 0 jusqu'Ã  72.
// generate_nested_impl!(7);

// /// La structure Tenseur encapsule une donnÃ©e dont le type est dÃ©fini par la profondeur N et le type de base T.
// /// Par exemple :
// /// - Tenseur<3, f64> correspond Ã  Vec<Vec<Vec<f64>>>
// /// - Tenseur<6, u32> correspond Ã  Vec<Vec<Vec<Vec<Vec<Vec<u32>>>>>>>
// #[derive(Debug)]
// struct Tenseur<const N: usize, T> {
//     data: <Nested<{N}> as NestedVecMaker>::Output<T>,
// }

// impl<const N: usize, T> Tenseur<N, T> {
//     pub fn new(data: <Nested<N> as NestedVecMaker>::Output<T>) -> Self {
//         Tenseur { data }
//     }
// }

// fn main() {
//     // Exemple : Tenseur<3, f64> correspond Ã  Vec<Vec<Vec<f64>>>
//     let tens_f64: Tenseur<3, f64> = Tenseur::new(vec![
//         vec![
//             vec![1.0, 2.0],
//             vec![3.0, 4.0],
//         ],
//         vec![
//             vec![5.0, 6.0],
//             vec![7.0, 8.0],
//         ],
//     ]);
//     println!("Tenseur<3, f64> = {:#?}", tens_f64.data);

//     // Exemple : Tenseur<6, u32> correspond Ã  Vec<Vec<Vec<Vec<Vec<Vec<u32>>>>>>>
//     // Pour allÃ©ger l'exemple, nous fournissons un seul Ã©lÃ©ment par niveau.
//     let tens_u32: Tenseur<6, u32> = Tenseur::new(vec![
//         vec![
//             vec![
//                 vec![
//                     vec![
//                         vec![1, 2],
//                         vec![3, 4],
//                     ],
//                     vec![
//                         vec![5, 6],
//                         vec![7, 8],
//                     ],
//                 ],
//                 vec![
//                     vec![
//                         vec![9, 10],
//                         vec![11, 12],
//                     ],
//                     vec![
//                         vec![13, 14],
//                         vec![15, 16],
//                     ],
//                 ],
//             ],
//         ],
//     ]);
//     println!("Tenseur<6, u32> = {:#?}", tens_u32.data);
// }
