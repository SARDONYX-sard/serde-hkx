pub mod impl_serialize;

use crate::cpp_info::Class;
use crate::cpp_info::Enum;
use crate::cpp_info::EnumItem;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_flags(class: &Class) -> Vec<syn::ItemEnum> {
    class
        .enums
        .iter()
        .map(|one_enum| gen_flag(one_enum))
        .collect()
}

fn gen_flag(one_enum: &Enum) -> syn::ItemEnum {
    let enum_name = syn::Ident::new(&one_enum.name, proc_macro2::Span::call_site());
    let size_ty = syn::Ident::new(&one_enum.name, proc_macro2::Span::call_site());

    let variants: Vec<TokenStream> = one_enum
        .enum_item
        .iter()
        .enumerate()
        .map(|(index, enum_item)| {
            let variant = gen_flag_variant(enum_item);
            match index {
                0 => quote! {
                    #[default]
                    #variant
                },
                _ => variant,
            }
        })
        .collect();

    syn::parse_quote! {
        #[allow(non_upper_case_globals, non_snake_case)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct #enum_name: #size_ty {
            #(#variants;)*
        }
    }
}

fn gen_flag_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name, value } = one_enum;
    let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let value = (*value) as usize;
    quote! { const #name = #value }
}
