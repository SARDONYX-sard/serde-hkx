use crate::{
    bail_syn_err,
    cpp_info::{Member, TypeKind},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Error;

/// `field` -> `m_field`
pub fn to_rust_field_ident(name: &str) -> syn::Ident {
    quote::format_ident!("m_{name}")
}

/// C++ member info -> Rust type token
pub(super) fn member_to_rust_type(member: &Member, class_name: &str) -> Result<TokenStream, Error> {
    let Member {
        name,
        class_ref,
        enum_ref,
        has_ref,
        vtype,
        vsubtype,
        arrsize,
        ..
    } = member;

    let lifetime = match has_ref {
        true => quote! { <'a> },
        false => quote! {},
    };

    let field_type = match member.vtype {
        TypeKind::Struct => {
            if let Some(class_ref) = class_ref {
                let struct_name = format_ident!("{class_ref}");
                quote! { #struct_name #lifetime }
            } else {
                bail_syn_err!("{class_name}({name}): Struct must have a class_ref")
            }
        }
        TypeKind::Enum | TypeKind::Flags => {
            if let Some(enum_ref) = enum_ref {
                quote::ToTokens::to_token_stream(&format_ident!("{enum_ref}"))
            } else {
                // NOTE: Fall back `enum Unknown` to enum storage size type(`vsubtype`).
                to_rust_type(vsubtype)
                    .ok_or_else(|| create_cast_error(class_name, name, vsubtype))?
            }
        }
        TypeKind::SimpleArray | TypeKind::Array => {
            let field_subtype = match vsubtype {
                TypeKind::Struct => match class_ref {
                    Some(class_ref) => {
                        let struct_name = format_ident!("{class_ref}");
                        quote! { #struct_name #lifetime }
                    }
                    None => {
                        bail_syn_err!("{class_name}({name}): Struct subtype must have a class_ref")
                    }
                },
                TypeKind::Enum | TypeKind::Flags => match enum_ref {
                    Some(enum_ref) => {
                        quote::ToTokens::to_token_stream(&format_ident!("{enum_ref}"))
                    }
                    None => {
                        bail_syn_err!("{class_name}({name}): Enum subtype must have an enum_ref")
                    }
                },
                _ => to_rust_type(vsubtype)
                    .ok_or_else(|| create_cast_error(class_name, name, vsubtype))?,
            };
            quote! { Vec<#field_subtype> }
        }
        _ => to_rust_type(vtype).ok_or_else(|| create_cast_error(class_name, name, vtype))?,
    };

    Ok(if *arrsize == 0 {
        quote! { #field_type }
    } else {
        quote! { [#field_type; #arrsize] }
    })
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
        TypeKind::Array | TypeKind::SimpleArray => quote!(Vec),
        // TypeKind::InplaceArray => todo!(),
        // TypeKind::Enum => todo!(),
        // TypeKind::Struct => todo!(),
        // TypeKind::SimpleArray => quote!(Vec),
        // TypeKind::HomogeneousArray => todo!(),
        TypeKind::Variant => quote!(Variant),
        TypeKind::CString => quote!(CString<'a>),
        TypeKind::Ulong => quote!(Ulong),
        // TypeKind::Flags => todo!(),
        TypeKind::Half => quote!(f16),
        TypeKind::StringPtr => quote!(StringPtr<'a>),
        // TypeKind::RelArray => todo!(),
        // TypeKind::Max => todo!(),
        _ => return None,
    })
}

fn create_cast_error(class_name: &str, name: &str, vtype: &TypeKind) -> Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!("{class_name}({name}): Couldn't cast {vtype} to Rust type"),
    )
}
