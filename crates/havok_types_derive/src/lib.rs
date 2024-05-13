extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FromStr)]
pub fn from_str(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Enum(_) => panic!("Unsupported enum. Only struct"),
        Data::Struct(ref data) => &data.fields,
        Data::Union(_) => panic!("Unsupported union. Only struct"),
    };
    let fields = fields.iter().map(|field| {
        quote! {
            stringify!(#name::#field) => flags |= #name::#field,
        }
    });

    let code = quote! {
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s == "0" {
                    return Ok(#name::empty());
                }

                let mut flags = #name::empty();
                for token in s.split('|') {
                    match token.trim() {
                        #(#fields)*
                        unknown => {
                            let bits = parse_int::parse(unknown).map_err(|_| {
                                format!("Expected #name or bits, but it is not a number: '{}'", unknown)
                            })?;
                            flags |= #name::from_bits_retain(bits);
                        }
                    }
                }

                Ok(flags)
            }
        }
    };
    code.into()
}

#[proc_macro_derive(Display)]
pub fn display(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match ast.data {
        Data::Enum(_) => panic!("Unsupported enum. Only struct"),
        Data::Struct(ref data) => &data.fields,
        Data::Union(_) => panic!("Unsupported union. Only struct"),
    };
    let fields = fields.iter().map(|field| {
        quote! {
            stringify!(#name::#field) => flags |= #name::#field,
        }
    });

    let code = quote! {
        impl core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.is_empty() {
                    return write!(f, "0");
                }

                let mut flags: Vec<std::borrow::Cow<'_, str>> = Vec::new();
                for flag in self.iter() {
                    match flag {
                        #(#fields)*
                        remain => flags.push(remain.bits().to_string().into()),
                    };
                }

                write!(f, "{}", flags.join("|"))
            }
        }
    };
    code.into()
}
