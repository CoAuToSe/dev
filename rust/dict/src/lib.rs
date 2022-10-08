use proc_macro::TokenStream;

#[proc_macro]
pub fn dict(item: TokenStream) -> TokenStream {
    println!("{:#?}", item);
    // item
    TokenStream::new()
}

// Punct {
//     ch: '=',
//     spacing: Joint,
//     span: #0 bytes(555..557),
// },
// Punct {
//     ch: '>',
//     spacing: Alone,
//     span: #0 bytes(555..557),
// },
