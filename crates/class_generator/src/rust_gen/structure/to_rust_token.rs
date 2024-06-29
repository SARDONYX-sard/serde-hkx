use crate::cpp_info::TypeKind;
use proc_macro2::TokenStream;
use quote::quote;

/// `field` -> `m_field`
pub fn to_rust_field_ident(name: &str) -> syn::Ident {
    quote::format_ident!("m_{name}")
}

pub fn to_rust_type(ty: &TypeKind) -> Option<TokenStream> {
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
