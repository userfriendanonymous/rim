use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, LitStr};

// #[proc_macro]
// pub fn ident(input: TokenStream) -> TokenStream {
//     let lit: LitStr = parse(input).unwrap();
//     lit.value().parse::<Ident>().unwrap();
//     quote!()
// }