mod enum_fields;
mod visit_struct;
mod visit_struct_for_bytes;

use crate::get_class_map::get_inherited_members;
use crate::{
    ClassMap, bail_syn_err,
    cpp_info::{Class, Member, TypeKind},
    syn_error,
};
use enum_fields::gen_enum_visitor;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Result;

pub fn impl_deserialize(class: &Class, class_map: &ClassMap) -> Result<TokenStream> {
    let members = &class.members;
    let enum_visitor = gen_enum_visitor(get_inherited_members(&class.name, class_map).as_slice());

    let class_name_str = &class.name;

    let visitor_ident = to_visitor_ident(class_name_str);
    let visitor_for_bytes = visit_struct_for_bytes::generate(class, class_map)?;
    let visitor_for_xml = visit_struct::generate(class, class_map)?;

    let expected_msg = format!("struct {class_name_str}");

    let class_name = format_ident!("{class_name_str}");
    let lifetime = if class.has_ref {
        quote! { <'de> }
    } else {
        quote! {}
    };

    let mut member_names = Vec::new();
    for member in members {
        member_names.push(&member.name);
    }

    Ok(quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            use havok_serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #class_name #lifetime {
                fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #enum_visitor

                    struct #visitor_ident<'de> {
                        marker: _serde::__private::PhantomData<#class_name #lifetime>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }

                    #[allow(clippy::match_single_binding)]
                    #[allow(clippy::reversed_empty_ranges)]
                    #[allow(clippy::single_match)]
                    impl<'de> _serde::de::Visitor<'de> for #visitor_ident<'de> {
                        type Value = #class_name #lifetime;
                        fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                            core::fmt::Formatter::write_str(__formatter, #expected_msg)
                        }

                        #visitor_for_bytes
                        #visitor_for_xml
                    }

                    const FIELDS: &[&str] = &[#(#member_names,)*];
                    _serde::Deserializer::deserialize_struct(
                        deserializer,
                        #class_name_str,
                        FIELDS,
                        #visitor_ident {
                            marker: _serde::__private::PhantomData::<#class_name>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    })
}

/// `ClassName` -> `__ClassNameVisitor`
pub(super) fn to_visitor_ident(class_name_str: &str) -> syn::Ident {
    format_ident!("__{class_name_str}Visitor")
}

/// C++ member info -> Rust type token
///
/// # Note
/// lifetime annotation is `'de` (not `'a`)
pub(super) fn member_to_de_rust_type(member: &Member, class_name: &str) -> Result<TokenStream> {
    let crate::cpp_info::Member {
        name,
        class_ref,
        enum_ref,
        has_ref: has_string,
        vtype,
        vsubtype,
        arrsize,
        ..
    } = member;

    let lifetime = match has_string {
        true => quote! { <'de> },
        false => quote! {},
    };

    let field_type = match member.vtype {
        TypeKind::Struct => match class_ref {
            Some(class_ref) => {
                let struct_name = format_ident!("{class_ref}");
                quote! { #struct_name #lifetime }
            }
            None => {
                bail_syn_err!("{class_name}({name}): Struct subtype must have a class_ref")
            }
        },
        TypeKind::Enum | TypeKind::Flags => {
            if let Some(enum_ref) = enum_ref {
                quote::ToTokens::to_token_stream(&format_ident!("{enum_ref}"))
            } else {
                // NOTE: Fall back `enum Unknown` to enum storage size type(`vsubtype`).
                to_rust_type(vsubtype).ok_or_else(|| {
                    syn_error!("{class_name}({name}): Couldn't cast {vtype} to Rust type")
                })?
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

    Ok(if *arrsize > 0 {
        quote! { [#field_type; #arrsize] }
    } else {
        quote! { #field_type }
    })
}

/// with `'de` annotation
fn to_rust_type(ty: &TypeKind) -> Option<TokenStream> {
    Some(match ty {
        TypeKind::Void => quote!(()),
        TypeKind::Bool => quote!(bool),
        TypeKind::Char => quote!(char),
        TypeKind::Int8 => quote!(I8<'de>),
        TypeKind::Uint8 => quote!(U8<'de>),
        TypeKind::Int16 => quote!(I16<'de>),
        TypeKind::Uint16 => quote!(U16<'de>),
        TypeKind::Int32 => quote!(I32<'de>),
        TypeKind::Uint32 => quote!(U32<'de>),
        TypeKind::Int64 => quote!(I64<'de>),
        TypeKind::Uint64 => quote!(U64<'de>),
        TypeKind::Real => quote!(f32),
        TypeKind::Vector4 => quote!(Vector4),
        TypeKind::Quaternion => quote!(Quaternion),
        TypeKind::Matrix3 => quote!(Matrix3),
        TypeKind::Rotation => quote!(Rotation),
        TypeKind::QsTransform => quote!(QsTransform),
        TypeKind::Matrix4 => quote!(Matrix4),
        TypeKind::Transform => quote!(Transform),
        // TypeKind::Zero => todo!(),
        TypeKind::Pointer => quote!(Pointer<'de>),
        // TypeKind::FnPtr => todo!(),
        TypeKind::Array | TypeKind::SimpleArray => quote!(Vec),
        // TypeKind::InplaceArray => todo!(),
        // TypeKind::Enum => todo!(),
        // TypeKind::Struct => todo!(),
        // TypeKind::SimpleArray => quote!(Vec),
        // TypeKind::HomogeneousArray => todo!(),
        TypeKind::Variant => quote!(Variant<'de>),
        TypeKind::CString => quote!(CString<'de>),
        TypeKind::Ulong => quote!(Ulong),
        // TypeKind::Flags => todo!(),
        TypeKind::Half => quote!(f16),
        TypeKind::StringPtr => quote!(StringPtr<'de>),
        // TypeKind::RelArray => todo!(),
        // TypeKind::Max => todo!(),
        _ => return None,
    })
}
