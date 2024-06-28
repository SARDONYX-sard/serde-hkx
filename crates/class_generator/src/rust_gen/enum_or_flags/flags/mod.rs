pub mod impls;

use crate::cpp_info::{Enum, EnumItem, TypeKind};
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_flag(one_enum: &Enum) -> syn::Macro {
    let Enum {
        vsubtype,
        enum_item,
        ..
    } = one_enum;

    let enum_name = syn::Ident::new(&one_enum.name, proc_macro2::Span::call_site());
    let size_ty = to_rust_storage_type(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));

    let variants: Vec<TokenStream> = enum_item
        .iter()
        .map(|enum_item| gen_flag_variant(enum_item))
        .collect();

    // Check
    syn::parse_quote! {
        bitflags::bitflags! {
            /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
            #[allow(non_upper_case_globals, non_snake_case)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
            pub struct #enum_name: #size_ty {
                #(const #variants;)*
            }
        }
    }
}

fn gen_flag_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name, value } = one_enum;
    let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let value = (*value) as usize;
    quote! { #name = #value }
}

/// Returns the name of the rust type
/// Return [`None`] except the following types.
/// - "TYPE_INT8", "TYPE_UINT8",
/// - "TYPE_INT16", "TYPE_UINT16",
/// - "TYPE_INT32", "TYPE_UINT32",
/// - "TYPE_INT64", "TYPE_UINT64"
fn to_rust_storage_type(ty: &TypeKind) -> Option<TokenStream> {
    Some(match ty {
        TypeKind::Int8 => quote!(i8),
        TypeKind::Uint8 => quote!(u8),
        TypeKind::Int16 => quote!(i16),
        TypeKind::Uint16 => quote!(u16),
        TypeKind::Int32 => quote!(i32),
        TypeKind::Uint32 => quote!(u32),
        TypeKind::Int64 => quote!(i64),
        TypeKind::Uint64 => quote!(u64),
        _ => return None,
    })
}
