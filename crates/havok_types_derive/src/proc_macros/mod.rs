extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, punctuated::Punctuated, token::PathSep, Ident, Token};

#[proc_macro_attribute]
pub fn impl_flags_methods(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let bit_flags = syn::parse_macro_input!(input as BitFlagsMacro);
    let struct_ = &bit_flags.struct_;
    let struct_name = struct_.ident.clone();

    let fields_to_str = struct_.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_name_str = &field.name.to_string();
        quote! {
            #struct_name::#field_name => flags.push(#field_name_str.into()),
        }
    });
    let str_to_fields = struct_.fields.iter().map(|field| {
        let field_name = &field.name;
        let field_name_str = &field.name.to_string();
        quote! {
            #field_name_str => flags |= #struct_name::#field_name,
        }
    });

    let code = quote! {
        #bit_flags

        impl Default for #struct_name {
            fn default() -> Self {
                Self::empty()
            }
        }

        impl core::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                if self.is_empty() {
                    return write!(f, "0");
                }

                let mut flags: Vec<std::borrow::Cow<'_, str>> = Vec::new();
                for flag in self.iter() {
                    match flag {
                        #(#fields_to_str)*
                        remain => flags.push(remain.bits().to_string().into()),
                    };
                }

                write!(f, "{}", flags.join("|"))
            }
        }

        impl core::str::FromStr for #struct_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s == "0" {
                    return Ok(#struct_name::empty());
                }

                let mut flags = #struct_name::empty();
                for token in s.split('|') {
                    match token.trim() {
                        #(#str_to_fields)*
                        unknown => {
                            let bits = ::havok_types_derive::parse_int::parse(unknown).map_err(|_| {
                                format!("Expected #name or bits, but it is not a number: '{unknown}'")
                            })?;
                            flags |= #struct_name::from_bits_retain(bits);
                        }
                    }
                }

                Ok(flags)
            }
        }
    };

    code.into()
}

/// bitflags::bitflags! {
#[derive(Debug, Clone, PartialEq, Eq)]
struct BitFlagsMacro {
    crate_name: Ident,
    path_seq_token: PathSep,
    macro_name: Ident,
    exclamation_token: Token![!],
    brace_token: syn::token::Brace,
    struct_: Struct,
}

impl Parse for BitFlagsMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // bitflags::bitflags! {
        let crate_name = input.parse::<syn::Ident>()?;
        let path_seq_token = input.parse::<syn::Token![::]>()?;
        let macro_name = input.parse::<syn::Ident>()?;
        let exclamation_token = input.parse::<syn::Token![!]>()?;
        let content;
        let brace_token: syn::token::Brace = braced!(content in input);

        Ok(Self {
            crate_name,
            path_seq_token,
            macro_name,
            exclamation_token,
            brace_token,
            struct_: Struct::parse(&content)?,
        })
    }
}

/// We have to implement this to use `quote!` to generate Rust code.
impl quote::ToTokens for BitFlagsMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.crate_name.to_tokens(tokens);
        self.path_seq_token.to_tokens(tokens);
        self.macro_name.to_tokens(tokens);
        self.exclamation_token.to_tokens(tokens);
        self.brace_token
            .surround(tokens, |tokens| self.struct_.to_tokens(tokens));
    }
}

/// ///
/// pub struct <name>: <type> {
#[derive(Debug, Clone, PartialEq, Eq)]
struct Struct {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    struct_token: syn::Token![struct],
    ident: syn::Ident,
    colon_token: syn::Token![:],
    ty: syn::Type,
    brace_token: syn::token::Brace,
    fields: Punctuated<Field, Token![;]>,
}

impl Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // ///
        // pub struct <name>: <type> {
        let content;
        let attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let vis: syn::Visibility = input.parse()?;
        let struct_token = input.parse::<syn::Token![struct]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let colon_token = input.parse::<syn::Token![:]>()?;
        let ty = input.parse::<syn::Type>()?;
        let brace_token: syn::token::Brace = braced!(content in input);

        Ok(Self {
            attrs,
            vis,
            struct_token,
            ident,
            colon_token,
            ty,
            brace_token,
            fields: content.parse_terminated(Field::parse, Token![;])?,
        })
    }
}

/// We have to implement this to use `quote!` to generate Rust code.
impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        self.struct_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.brace_token
            .surround(tokens, |tokens| self.fields.to_tokens(tokens))
    }
}

/// #[doc] or `///`
/// const <NAME> = <value>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    attrs: Vec<syn::Attribute>,
    const_token: Token![const],
    eq_token: Token![=],
    name: Ident,
    value: syn::Expr,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let const_token = input.parse::<syn::Token![const]>()?;
        let name = input.parse::<syn::Ident>()?;
        let eq_token = input.parse::<syn::Token![=]>()?;
        let value: syn::Expr = input.parse()?;

        Ok(Field {
            attrs,
            const_token,
            eq_token,
            name,
            value,
        })
    }
}

/// We have to implement this to use `quote!` to generate Rust code.
impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.const_token.to_tokens(tokens);
        self.name.to_tokens(tokens);
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}
