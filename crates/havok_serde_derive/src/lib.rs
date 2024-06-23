mod ser;

extern crate proc_macro2;
extern crate quote;
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize, attributes(havok_serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    ser::expand_derive_serialize(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// #[proc_macro_derive(Deserialize, attributes(serde))]
// pub fn derive_deserialize(input: TokenStream) -> TokenStream {
//     let mut input = parse_macro_input!(input as DeriveInput);
//     de::expand_derive_deserialize(&mut input)
//         .unwrap_or_else(syn::Error::into_compile_error)
//     .into()
// }
