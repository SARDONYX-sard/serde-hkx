pub mod impls;

use crate::cpp_info::{Enum, EnumItem, TypeKind};
use proc_macro2::TokenStream;
use quote::quote;

/// # Note
/// This function is a code generator for `TYPE_FLAGS` or `TYPE_ENUM` (`enum` with duplicate blood),
/// but does not check for `TypeKind` since that is left to upstream functions.
pub fn gen_flag(one_enum: &Enum) -> syn::Macro {
    let Enum {
        vtype,
        vsubtype,
        enum_item,
        ..
    } = one_enum;

    let enum_name = syn::Ident::new(&one_enum.name, proc_macro2::Span::call_site());
    let size_ty = to_rust_storage_type(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));

    let variants: Vec<TokenStream> = enum_item
        .iter()
        .map(|enum_item| gen_variant_token(enum_item, vsubtype))
        .collect();

    let doc = format!("- size(C++): `{vsubtype}`");
    let doc = if *vtype == TypeKind::Enum {
        quote! {
           /// Bit flags that represented `enum hkEnum<Enum, SizeType>`(C++).
           #[doc = #doc]
           ///
           /// # Why this `enum` defined as `bitflags`?
           /// These are not really `TYPE_FLAGS` but `TYPE_ENUM`, but since Rust does not allow the definition of
           /// `enum` with duplicate discriminant values, they are defined as `bitflags`.
        }
    } else {
        quote! {
           /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
           #[doc = #doc]
        }
    };

    // Check
    syn::parse_quote! {
        bitflags::bitflags! {
            #doc
            #[allow(non_upper_case_globals, non_snake_case)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
            pub struct #enum_name: #size_ty {
                #(#variants)*
            }
        }
    }
}

/// Returns the name of the rust type
/// Return [`None`] except the following types.
/// - `TYPE_INT8`, `TYPE_UINT8`,
/// - `TYPE_INT16`, `TYPE_UINT16`,
/// - `TYPE_INT32`, `TYPE_UINT32`,
/// - `TYPE_INT64`, `TYPE_UINT64`
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

/// Returns `variant = value` expr.
///
/// # Panics
/// The following types.
/// - `TYPE_INT8`, `TYPE_UINT8`,
/// - `TYPE_INT16`, `TYPE_UINT16`,
/// - `TYPE_INT32`, `TYPE_UINT32`,
/// - `TYPE_INT64`, `TYPE_UINT64`
fn gen_variant_token(one_enum: &EnumItem, size_type: &TypeKind) -> TokenStream {
    let EnumItem { name, value } = one_enum;
    let name = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let variant_stmt = match size_type {
        TypeKind::Int8 => {
            let value = (*value) as i8;
            quote! { #name = #value }
        }
        TypeKind::Uint8 => {
            let value = (*value) as u8;
            quote! { #name = #value }
        }
        TypeKind::Int16 => {
            let value = (*value) as i16;
            quote! { #name = #value }
        }
        TypeKind::Uint16 => {
            let value = (*value) as u16;
            quote! { #name = #value }
        }
        TypeKind::Int32 => {
            let value = (*value) as i32;
            quote! { #name = #value }
        }
        TypeKind::Uint32 => {
            let value = (*value) as u32;
            quote! { #name = #value }
        }
        TypeKind::Int64 => {
            let value = (*value) as i64;
            quote! { #name = #value }
        }
        TypeKind::Uint64 => {
            let value = (*value) as u64;
            quote! { #name = #value }
        }
        _ => panic!("Unknown size type"),
    };

    // NOTE:
    // flag is displayed as <layout-error> even if it is hovered and the value is unknown,
    // so write the value in doc for ease of API use
    let doc = format!("{value}");
    quote! {
        #[doc = #doc]
        const #variant_stmt;
    }
}
