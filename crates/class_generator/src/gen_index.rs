use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_index(mods: &[TokenStream]) -> String {
    let tokens = quote! {
        /// Reduce the burden of individual imports by importing a set of types needed to create a havok class structure here.
        mod class_requires {
            pub use havok_serde::{
                ser::{Error as _, SerializeFlags, SerializeStruct, Serializer},
                HavokClass,
            };
            pub use havok_types::*;
        }
        #(#mods)*
    };

    let ast = match syn::parse2(tokens) {
        Ok(ast) => ast,
        Err(err) => panic!("span = {:?}, {err}", err.span()),
    };
    prettyplease::unparse(&ast)
}
