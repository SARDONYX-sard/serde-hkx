//! Automatic generation of a method to retrieve the location of an enum Error in `snafu` crate.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Automatic generation of a method to retrieve the location of an enum Error in `snafu` crate.
#[proc_macro_derive(Location)]
pub fn location(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the enum
    let name = &input.ident;

    // Extract the variants of the enum
    let variants = match input.data {
        Data::Enum(ref data) => &data.variants,
        _ => panic!("Only enums are supported for #[derive(location)]"),
    };

    // Generate match arms for each variant
    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Named(fields) => {
                let location_field = fields
                    .named
                    .iter()
                    .find(|f| f.ident.as_ref().map(|i| i == "location").unwrap_or(false));
                if let Some(location_field) = location_field {
                    let location_ident = &location_field.ident;
                    quote! {
                        #name::#variant_name { #location_ident, .. } => Some(#location_ident)
                    }
                } else {
                    quote! {
                        #name::#variant_name { .. } => None
                    }
                }
            }
            _ => quote! {
                #name::#variant_name { .. } => None
            },
        }
    });

    // Generate the implementation of the location method
    let expanded = quote! {
        impl #name {
            /// Get location from Error.
            pub fn location(&self) -> Option<&snafu::Location> {
                match self {
                    #(#match_arms),*
                }
            }

            #[inline]
            /// Get location string.
            ///
            /// # Note
            /// Return [`String::new`] if `self.location` == [`Option::None`].
            ///
            /// [`String::new`]: https://doc.rust-lang.org/stable/alloc/string/struct.String.html#method.new
            pub fn location_string(&self) -> String {
                match self.location() {
                    Some(location) => location.to_string(),
                    None => String::new()
                }
            }
        }
    };

    // Convert the generated code into a token stream and return it
    TokenStream::from(expanded)
}
