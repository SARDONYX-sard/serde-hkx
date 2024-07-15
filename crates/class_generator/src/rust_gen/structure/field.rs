use super::to_rust_token::{member_to_rust_type, to_rust_field_ident};
use crate::cpp_info::{Member, TypeKind};
use proc_macro2::TokenStream;
use quote::quote;

/// C++ member info -> Rust field token
pub(super) fn gen_field(member: &Member, class_name: &str) -> TokenStream {
    let Member {
        name,
        vtype,
        arrsize,
        ..
    } = member;

    let field_type = member_to_rust_type(member, class_name);

    // `Default` implementations with huge sizes such as [0u8; 256] are not automatically supported, so use `educe` crate to define them.
    let default_attr = if *arrsize > 32 {
        let as_value = format!("[_; {arrsize}]"); // NOTE: need `serde_with`
        let serde_with_attr = quote! {
            #[cfg_attr(feature = "serde", serde_as(as = #as_value))]
        };

        let default_attr = match vtype {
            TypeKind::Int8
            | TypeKind::Uint8
            | TypeKind::Int32
            | TypeKind::Uint32
            | TypeKind::Int16
            | TypeKind::Uint16
            | TypeKind::Int64
            | TypeKind::Uint64 => {
                quote! {
                    #[educe(Default = [0; #arrsize])]
                }
            }
            _ => panic!(
                "Giant fixed-size arrays are supported only for Int or Uint 8~64. Got {vtype}"
            ),
        };
        quote! {
            #serde_with_attr
            #default_attr
        }
    } else {
        quote! {}
    };

    let doc = field_doc_tokens(member);
    let field_name = to_rust_field_ident(name);
    quote! {
        #doc
        #default_attr
        pub #field_name: #field_type
    }
}

fn field_doc_tokens(member: &Member) -> TokenStream {
    let Member {
        name,
        ctype,
        offset_x86,
        offset_x86_64,
        type_size_x86,
        type_size_x86_64,
        flags,
        ..
    } = member;

    let name = format!(" - name: `{name}`(ctype: `{ctype}`)");
    let offsets = format!(" - offset: `{offset_x86:3}`(x86)/`{offset_x86_64:3}`(x86_64)");
    let type_sizes =
        format!(" - type_size: `{type_size_x86:3}`(x86)/`{type_size_x86_64:3}`(x86_64)");
    let flags_doc = match flags.bits() {
        0 => quote! {},
        _ => {
            let doc = format!(" - flags: `{flags}`");
            quote! { #[doc = #doc]}
        }
    };

    quote! {
        /// # C++ Info
        #[doc = #name]
        #[doc = #offsets]
        #[doc = #type_sizes]
        #flags_doc
    }
}
