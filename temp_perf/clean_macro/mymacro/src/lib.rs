// External dependencies
extern crate proc_macro;
use crate::proc_macro::TokenStream;
use proc_macro2;
use quote::{format_ident, quote};
use std::fmt;
use syn::{self, *};

// impl fmt::Debug for DeriveInput {
//     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         let mut formatter = formatter.debug_struct("DeriveInput");
//         formatter.field("attrs", &self.attrs);
//         formatter.field("vis", &self.vis);
//         formatter.field("ident", &self.ident);
//         formatter.field("generics", &self.generics);
//         formatter.field("data", &self.data);
//         formatter.finish()
//     }
// }
// Define a derive macro
#[proc_macro_derive(Duplicate, attributes(dupe_me))]
pub fn duplicate_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    println!("{:#?}\n\n{:?}\n\n{}", input, input, input);
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    // println!("\n{:?}", ast);

    // Grab the name of the struct
    let name = &ast.ident;

    // Find the name of members we need to duplicate
    let mut duped: Vec<(proc_macro2::Ident, syn::Type)> = vec![];
    match ast.data {
        // Only process structs
        syn::Data::Struct(ref data_struct) => {
            // println!("{:?}\n\n{}", data_struct, data_struct);
            // Check the kind of fields the struct contains
            match data_struct.fields {
                // Structs with named fields
                syn::Fields::Named(ref fields_named) => {
                    // println!("{:?}\n\n{}", fields_named, fields_named);
                    // Iterate over the fields
                    for field in fields_named.named.iter() {
                        // println!("{:?}\n\n{}", field, field);
                        // Get attributes #[..] on each field
                        for attr in field.attrs.iter() {
                            // println!("{:?}\n\n{}", attr, attr);
                            // Parse the attribute
                            match attr.parse_meta().unwrap() {
                                // Find the duplicated idents
                                syn::Meta::Path(ref path)
                                    if path.get_ident().unwrap().to_string() == "dupe_me" =>
                                {
                                    // println!("{:?}\n\n{}", path, path);
                                    // Save the duped elements
                                    let item = field.clone();
                                    duped.push((item.ident.unwrap(), item.ty))
                                }
                                _ => (),
                            }
                        }
                    }
                }

                // Struct with unnamed fields
                _ => (),
            }
        }

        // Panic when we don't have a struct
        _ => panic!("Must be a struct"),
    }

    // Transform the marked elements into new struct fields
    let duped = duped
        .iter()
        .fold(quote!(), |es, (name, ty)| quote!(#es #name : #ty,));

    // Create the new structure
    let myname = format_ident!("{}Duplicated", name);
    let gen = quote! {
        #[derive(Debug)]
        struct #myname {
            #duped
        }
    };
    gen.into()
    //panic!(gen.to_string());
}
