pub mod impls;

use crate::cpp_info::Enum;
use crate::cpp_info::EnumItem;
use crate::rust_gen::enum_or_flags::to_rust_storage_size_ident;
use proc_macro2::TokenStream;
use quote::quote;

/// # Note
/// This function is a code generator for `TYPE_ENUM` but does not check for `TypeKind` since that
/// is left to upstream functions.
pub fn gen_enum(one_enum: &Enum) -> syn::ItemEnum {
    let Enum {
        name,
        vsubtype,
        enum_item,
        ..
    } = one_enum;

    let enum_name = syn::Ident::new(name, proc_macro2::Span::call_site());
    let variants: Vec<TokenStream> = enum_item
        .iter()
        .enumerate()
        .map(|(index, enum_item)| {
            let variant = gen_enum_variant(enum_item);
            match index {
                0 => quote! {
                    #[default]
                    #variant
                },
                _ => variant,
            }
        })
        .collect();

    // e.g. `Uint64`
    let storage_size_ident = to_rust_storage_size_ident(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));
    // e.g. `hkUint64`
    let c_type_storage_size = format!("hk{storage_size_ident}");
    let name = format!(" - name: `{name}`(ctype: `hkEnum<{name}, {c_type_storage_size}>`)");

    syn::parse_quote! {
        /// # C++ Info
        #[doc = #name]
        #[allow(non_upper_case_globals, non_snake_case)]
        #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(
            Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord,
            num_derive::ToPrimitive, num_derive::FromPrimitive,
        )]
        pub enum #enum_name {
            #(#variants,)*
        }
    }
}

/// e.g. `ROLE = 0`
fn gen_enum_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name, value } = one_enum;
    let name = syn::Ident::new(name, proc_macro2::Span::call_site());
    let value = (*value) as isize;
    quote! { #name = #value }
}
