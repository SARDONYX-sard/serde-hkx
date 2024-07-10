use crate::cpp_info::TypeKind;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// `field` -> `m_field`
pub fn to_rust_field_ident(name: &str) -> syn::Ident {
    quote::format_ident!("m_{name}")
}

/// C++ member info -> Rust type token
pub(super) fn member_to_rust_type(
    member: &crate::cpp_info::Member,
    class_name: &str,
) -> TokenStream {
    let crate::cpp_info::Member {
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
                quote::ToTokens::to_token_stream(&format_ident!("{enum_ref}"))
            } else {
                // NOTE: Fall back `enum Unknown` to enum storage size type(`vsubtype`).
                to_rust_type(vsubtype).unwrap_or_else(|| {
                    panic!("{class_name}({name}) couldn't cast {vsubtype} to Rust type")
                })
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
                TypeKind::Enum | TypeKind::Flags => quote::ToTokens::to_token_stream(
                    &format_ident!("{}", enum_ref.as_ref().unwrap()),
                ),
                _ => to_rust_type(vsubtype).unwrap_or_else(|| {
                    panic!("{class_name}({name}) couldn't cast {vsubtype} to Rust type")
                }),
            };
            quote! { Vec<#field_subtype> }
        }
        _ => to_rust_type(vtype)
            .unwrap_or_else(|| panic!("{class_name}({name}) couldn't cast {vtype} to Rust type")),
    };

    if *arrsize > 0 {
        quote! { [#field_type; #arrsize] }
    } else {
        quote! { #field_type }
    }
}

fn to_rust_type(ty: &TypeKind) -> Option<TokenStream> {
    Some(match ty {
        TypeKind::Void => quote!(()),
        TypeKind::Bool => quote!(bool),
        TypeKind::Char => quote!(char),
        TypeKind::Int8 => quote!(i8),
        TypeKind::Uint8 => quote!(u8),
        TypeKind::Int16 => quote!(i16),
        TypeKind::Uint16 => quote!(u16),
        TypeKind::Int32 => quote!(i32),
        TypeKind::Uint32 => quote!(u32),
        TypeKind::Int64 => quote!(i64),
        TypeKind::Uint64 => quote!(u64),
        TypeKind::Real => quote!(f32),
        TypeKind::Vector4 => quote!(Vector4),
        TypeKind::Quaternion => quote!(Quaternion),
        TypeKind::Matrix3 => quote!(Matrix3),
        TypeKind::Rotation => quote!(Rotation),
        TypeKind::QsTransform => quote!(QsTransform),
        TypeKind::Matrix4 => quote!(Matrix4),
        TypeKind::Transform => quote!(Transform),
        // TypeKind::Zero => todo!(),
        TypeKind::Pointer => quote!(Pointer),
        // TypeKind::FnPtr => todo!(),
        TypeKind::Array => quote!(Vec),
        // TypeKind::InplaceArray => todo!(),
        // TypeKind::Enum => todo!(),
        // TypeKind::Struct => todo!(),
        TypeKind::SimpleArray => quote!(Vec),
        // TypeKind::HomogeneousArray => todo!(),
        TypeKind::Variant => quote!(Variant),
        TypeKind::CString => quote!(CString<'a>),
        TypeKind::Ulong => quote!(u64),
        // TypeKind::Flags => todo!(),
        TypeKind::Half => quote!(f16),
        TypeKind::StringPtr => quote!(StringPtr<'a>),
        // TypeKind::RelArray => todo!(),
        // TypeKind::Max => todo!(),
        _ => return None,
    })
}
