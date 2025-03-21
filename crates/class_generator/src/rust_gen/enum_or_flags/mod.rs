mod enums;
mod flags;
mod impls;

use self::enums::gen_enum;
use self::flags::gen_flag;
use crate::cpp_info::{Class, Enum, TypeKind};
use enums::impls::json_schema::impl_schema_for_enum_flag;
use flags::impls::json_schema::impl_schema_for_flag;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub use impls::deserialize::impl_deserialize;
pub use impls::serialize::impl_serialize;

pub fn generate(class: &Class) -> Vec<TokenStream> {
    let mut enums = Vec::new();
    for one_enum in &class.enums {
        let Enum {
            name,
            vtype,
            vsubtype,
            ..
        } = one_enum;

        if *vsubtype == TypeKind::Void {
            tracing::info!("Skip automatic enum generation because this enum {name} is a void storage type, indicating that it is not used.");
            continue;
        };

        match vtype {
            TypeKind::Flags => {
                let flag_struct = gen_flag(one_enum);
                let impl_json_schema =  impl_schema_for_flag(one_enum);

                enums.push(quote! {
                    #[havok_types_derive::impl_flags_methods]
                    #flag_struct

                    #impl_json_schema
                });
            }
            TypeKind::Enum => {
                // An enum with the same value is not valid as an enum in Rust. Therefore, express them as BitFlag
                let item_enum = if has_duplicate_value(one_enum) {
                    let flag_struct = gen_flag(one_enum);
                    let impl_json_schema =  impl_schema_for_enum_flag(one_enum);
                    // NOTE: enum can make schema with derive, so it is not impl here.

                    quote! {
                        #[havok_types_derive::impl_flags_methods]
                        #flag_struct

                        #impl_json_schema
                    }
                } else {
                    let item_enum = gen_enum(one_enum);
                    quote! { #item_enum }
                };
                enums.push(item_enum);
            }
            _ => panic!(
                "Expected TYPE_ENUM|TYPE_FLAGS, but another type is mixed in. Got enum {name}(vtype: {vtype})"
            ),
        };
    }

    enums
}

/// An enum with the same value is not valid as an enum in Rust.
///
/// Therefore, express them as BitFlag, but note that implementations such as `Serialize` must be handled as an enum.
pub(super) fn has_duplicate_value(one_enum: &Enum) -> bool {
    let mut values = HashSet::new();
    for enum_item in &one_enum.enum_item {
        let is_contain = values.insert(enum_item.value);
        if !is_contain {
            return true; // Duplicate found
        }
    }
    false // No duplicates
}

/// Returns the name of the rust type
/// Return [`None`] except the following types.
/// - `TYPE_INT8`, `TYPE_UINT8`,
/// - `TYPE_INT16`, `TYPE_UINT16`,
/// - `TYPE_INT32`, `TYPE_UINT32`,
/// - `TYPE_INT64`, `TYPE_UINT64`
///
/// e.g. `TypeKind::Int8` => `i8`
pub(super) fn to_rust_storage_type(ty: &TypeKind) -> Option<TokenStream> {
    Some(match ty {
        TypeKind::Int8 => quote! {    i8 },
        TypeKind::Uint8 => quote! {   u8 },
        TypeKind::Int16 => quote! {  i16 },
        TypeKind::Uint16 => quote! { u16 },
        TypeKind::Int32 => quote! {  i32 },
        TypeKind::Uint32 => quote! { u32 },
        TypeKind::Int64 => quote! {  i64 },
        TypeKind::Uint64 => quote! { u64 },
        _ => return None,
    })
}

/// Returns the name of the rust type
/// Return [`None`] except the following types.
/// - `TYPE_INT8`, `TYPE_UINT8`,
/// - `TYPE_INT16`, `TYPE_UINT16`,
/// - `TYPE_INT32`, `TYPE_UINT32`,
/// - `TYPE_INT64`, `TYPE_UINT64`
///
/// e.g. `TypeKind::Int8` => `I8`
pub(super) fn to_rust_wrapper_type(ty: &TypeKind) -> Option<TokenStream> {
    Some(match ty {
        TypeKind::Int8 => quote! {   I8},
        TypeKind::Uint8 => quote! {  U8},
        TypeKind::Int16 => quote! { I16},
        TypeKind::Uint16 => quote! {U16},
        TypeKind::Int32 => quote! { I32},
        TypeKind::Uint32 => quote! {U32},
        TypeKind::Int64 => quote! { I64},
        TypeKind::Uint64 => quote! {U64},
        _ => return None,
    })
}

/// enum value to storage type token.
///
/// e.g. 0 => `0u8`
pub(super) fn cast_number_to_token(
    enum_item: &crate::cpp_info::EnumItem,
    size_type: &TypeKind,
) -> TokenStream {
    let value = &enum_item.value;

    match size_type {
        TypeKind::Int8 => {
            let value = (*value) as i8;
            quote! { #value }
        }
        TypeKind::Uint8 => {
            let value = (*value) as u8;
            quote! { #value }
        }
        TypeKind::Int16 => {
            let value = (*value) as i16;
            quote! { #value }
        }
        TypeKind::Uint16 => {
            let value = (*value) as u16;
            quote! { #value }
        }
        TypeKind::Int32 => {
            let value = (*value) as i32;
            quote! { #value }
        }
        TypeKind::Uint32 => {
            let value = (*value) as u32;
            quote! { #value }
        }
        TypeKind::Int64 => {
            let value = (*value) as i64;
            quote! { #value }
        }
        TypeKind::Uint64 => {
            let value = (*value) as u64;
            quote! { #value }
        }
        _ => panic!("Unknown size type: {size_type}"),
    }
}

/// Returns the name of the method you get when you derive ToPrimitive.
/// Return [`None`] except the following types.
/// - "TYPE_INT8", "TYPE_UINT8",
/// - "TYPE_INT16", "TYPE_UINT16",
/// - "TYPE_INT32", "TYPE_UINT32",
/// - "TYPE_INT64", "TYPE_UINT64"
///
/// `TypeKind::Uint64` => `Uint64`
pub(super) fn to_rust_storage_size_ident(ty: &TypeKind) -> Option<syn::Ident> {
    Some(match ty {
        TypeKind::Int8 => syn::parse_quote!(Int8),
        TypeKind::Uint8 => syn::parse_quote!(Uint8),
        TypeKind::Int16 => syn::parse_quote!(Int16),
        TypeKind::Uint16 => syn::parse_quote!(Uint16),
        TypeKind::Int32 => syn::parse_quote!(Int32),
        TypeKind::Uint32 => syn::parse_quote!(Uint32),
        TypeKind::Int64 => syn::parse_quote!(Inti64),
        TypeKind::Uint64 => syn::parse_quote!(Uint64),
        _ => return None,
    })
}
