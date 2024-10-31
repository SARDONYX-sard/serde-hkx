pub mod impls;

use crate::{
    cpp_info::{Enum, EnumItem, TypeKind},
    rust_gen::enum_or_flags::{
        cast_number_to_token, to_rust_storage_size_ident, to_rust_storage_type,
    },
};
use proc_macro2::TokenStream;
use quote::quote;

/// # Note
/// This function is a code generator for `TYPE_FLAGS` or `TYPE_ENUM` (`enum` with duplicate blood),
/// but does not check for `TypeKind` since that is left to upstream functions.
pub fn gen_flag(one_enum: &Enum) -> syn::Macro {
    let Enum {
        name,
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

    // e.g. `Uint64`
    let storage_size_ident = to_rust_storage_size_ident(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));
    // e.g. `hkUint64`
    let c_type_storage_size = format!("hk{storage_size_ident}");

    let name = if *vtype == TypeKind::Enum {
        format!(" - name: `{name}`(ctype: `hkEnum<{name}, {c_type_storage_size}>`)")
    } else {
        format!(" - name: `{name}`(ctype: `hkFlags<{name}, {c_type_storage_size}>`)")
    };

    let doc = if *vtype == TypeKind::Enum {
        quote! {
            /// Enum as bit flags
            ///
            /// # C++ Info
            #[doc = #name]
            ///
            /// # Why this `enum` defined as `bitflags`?
            /// Since Rust does not allow the definition of `enum` with duplicate discriminant values, they are defined as `bitflags`.
        }
    } else {
        quote! {
            /// Bit flags
            ///
            /// # C++ Info
            #[doc = #name]
        }
    };

    // Check
    syn::parse_quote! {
        bitflags::bitflags! {
            #doc
            #[allow(non_upper_case_globals, non_snake_case)]
            #[cfg_attr(
                feature = "serde",
                derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
            )]
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #enum_name: #size_ty {
                #(#variants)*
            }
        }

    }
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
    let EnumItem { name, value, .. } = one_enum;
    let name = syn::Ident::new(name, proc_macro2::Span::call_site());
    let num_with_suffix = cast_number_to_token(one_enum, size_type);

    // NOTE:
    // flag is displayed as <layout-error> even if it is hovered and the value is unknown,
    // so write the value in doc for ease of API use
    let doc = value.to_string();
    quote! {
        #[doc = #doc]
        const #name = #num_with_suffix;
    }
}
