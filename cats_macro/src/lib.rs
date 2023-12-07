extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, visit_mut::VisitMut, BinOp, Expr, ExprBinary, ItemFn};

#[proc_macro_attribute]
pub fn cats(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let mut visitor = CatsVisitor;
    visitor.visit_block_mut(&mut input_fn.block);

    TokenStream::from(quote! { #input_fn })
}

struct CatsVisitor;

impl VisitMut for CatsVisitor {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        if let Expr::Binary(ExprBinary {
            op: BinOp::Add(a), ..
        }) = i
        {
            println!("{:?}", (i));
            *i = syn::parse_quote! {
                {
                    println!("hello world");
                    #i.left.lit.token
                }
            };
        }

        // Continue walking through the tree
        syn::visit_mut::visit_expr_mut(self, i);
    }
}
