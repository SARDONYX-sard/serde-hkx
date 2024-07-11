mod enum_fields;
mod visit_struct;
mod visit_struct_for_bytes;

use crate::cpp_info::{Class, TypeKind};
use enum_fields::gen_enum_fields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn impl_deserialize(class: &Class) -> TokenStream {
    let members = &class.members;
    let enum_fields = gen_enum_fields(members);

    let class_name_str = &class.name;

    let visitor_ident = to_visitor_ident(class_name_str);
    let visitor_for_bytes = visit_struct_for_bytes::gen(class);
    let visitor_for_xml = visit_struct::gen(class);

    let expected_msg = format!("struct {class_name_str}");

    let class_name = format_ident!("{class_name_str}");
    let lifetime = if class.has_string {
        quote! { <'de> }
    } else {
        quote! {}
    };

    let mut member_names = Vec::new();
    for member in members {
        member_names.push(&member.name);
    }

    quote! {
        use havok_serde as _serde;

        #enum_fields

        pub(super) struct #visitor_ident<'de> {
                marker: core::marker::PhantomData<#class_name #lifetime>,
                lifetime: core::marker::PhantomData<&'de ()>,
        }

        impl<'de> #visitor_ident<'de> {
            /// # Purpose of this method
            /// To reproduce C++ field inheritance, we will have the field internal implementation
            /// of deserialization partially exposed and reused.
            #[inline]
            pub(super) fn visit_as_parent<__A>(
                __map: &mut __A,
            ) -> _serde::__private::Result<#class_name #lifetime, __A::Error>
            where
                __A: _serde::de::MapAccess<'de>,
            {
                _serde::de::Visitor::visit_struct(
                    Self {
                        marker: _serde::__private::PhantomData::<#class_name #lifetime>,
                        lifetime: _serde::__private::PhantomData,
                    },
                    __map,
                )
            }
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

        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #class_name #lifetime {
                fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
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
    }
}

/// `ClassName` -> `__ClassNameVisitor`
pub(super) fn to_visitor_ident(class_name_str: &str) -> syn::Ident {
    format_ident!("__{class_name_str}Visitor")
}

/// C++ member info -> Rust type token
///
/// # Note
/// lifetime annotation is `'de` (not `'a`)
pub(super) fn member_to_de_rust_type(
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
        true => quote! { <'de> },
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

/// with `'de` annotation
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
        TypeKind::CString => quote!(CString<'de>),
        TypeKind::Ulong => quote!(u64),
        // TypeKind::Flags => todo!(),
        TypeKind::Half => quote!(f16),
        TypeKind::StringPtr => quote!(StringPtr<'de>),
        // TypeKind::RelArray => todo!(),
        // TypeKind::Max => todo!(),
        _ => return None,
    })
}
