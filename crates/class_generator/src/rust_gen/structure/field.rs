use super::to_rust_token::{to_rust_field_ident, to_rust_type};
use crate::cpp_info::{Member, TypeKind};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// C++ member info -> Rust field token
pub(super) fn gen_field(member: &Member, class_name: &str) -> TokenStream {
    let Member {
        name,
        class_ref,
        enum_ref,
        has_string,
        vtype,
        vsubtype,
        arrsize,
        ..
    } = member;

    let lifetime = match has_string {
        true => quote! { <'a> },
        false => quote! {},
    };

    let field_type = match member.vtype {
        TypeKind::Struct => {
            let struct_name = syn::Ident::new(
                class_ref.as_ref().expect("Struct must have class_ref"),
                proc_macro2::Span::call_site(),
            );
            quote! { #struct_name #lifetime }
        }
        TypeKind::Enum | TypeKind::Flags => {
            if let Some(enum_ref) = enum_ref {
                syn::Ident::new(enum_ref, proc_macro2::Span::call_site()).to_token_stream()
            } else {
                // NOTE: Fall back `enum Unknown` to enum storage size type(`vsubtype`).
                to_rust_type(&vsubtype).expect(&format!(
                    "{class_name}({name}) couldn't cast {vsubtype} to Rust type"
                ))
            }
        }
        TypeKind::SimpleArray | TypeKind::Array => {
            let field_subtype = match vsubtype {
                TypeKind::Struct => {
                    let struct_name = syn::Ident::new(
                        class_ref.as_ref().expect("Struct must have class_ref"),
                        proc_macro2::Span::call_site(),
                    );
                    quote! { #struct_name #lifetime }
                }
                TypeKind::Enum | TypeKind::Flags => {
                    syn::Ident::new(enum_ref.as_ref().unwrap(), proc_macro2::Span::call_site())
                        .to_token_stream()
                }
                _ => to_rust_type(&vsubtype).expect(&format!(
                    "{class_name}({name}) couldn't cast {vsubtype} to Rust type"
                )),
            };
            quote! { Vec<#field_subtype> }
        }
        _ => to_rust_type(&vtype).expect(&format!(
            "{class_name}({name}) couldn't cast {vtype} to Rust type"
        )),
    };

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
    if *arrsize > 0 {
        quote! {
            #doc
            #default_attr
            pub #field_name: [#field_type; #arrsize]
        }
    } else {
        quote! {
            #doc
            pub #field_name: #field_type
        }
    }
}

#[allow(unused)]
#[rustfmt::skip]
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

    let name =             format!(" -          name: `{name}`(ctype: `{ctype}`)");
    let offsets =          format!(" -        offset: {offset_x86:3}(x86)/{offset_x86_64:3}(x86_64)");
    let type_sizes =       format!(" -     type_size: {type_size_x86:3}(x86)/{type_size_x86_64:3}(x86_64)");
    let flags_doc = match flags.bits() {
        0 => quote! {},
        _ => {
            let doc =      format!(" -         flags: `{flags}`");
            quote! { #[doc = #doc]}
        }
    };

    quote! {
        /// # C++ Info
        #[doc = #name]
        #[doc = #offsets]
        #[doc = #type_sizes]
        #flags_doc
        #[doc = ""]
    }
}
