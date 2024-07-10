mod enum_fields;
mod visit_struct;
mod visit_struct_for_bytes;

use crate::cpp_info::Class;
use enum_fields::gen_enum_fields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// TODO: Implement binary deserializer
///
/// Note: that it is only temporarily placed until the specifications are finalized.
pub fn impl_deserialize(class: &Class) -> TokenStream {
    let class_name = format_ident!("{}", class.name);
    let lifetime = if class.has_string {
        quote! { <'de> }
    } else {
        quote! {}
    };

    let block = impl_visitors(class);

    quote::quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate havok_serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #class_name #lifetime {
                fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #block
                }
            };
        }
    }
}

fn impl_visitors(class: &Class) -> TokenStream {
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

    let member_names = members
        .iter()
        .map(|member| &member.name)
        .collect::<Vec<_>>();

    quote! {
        #enum_fields

        pub(super) struct #visitor_ident<'de> {
                marker: core::marker::PhantomData<#class_name>,
                lifetime: core::marker::PhantomData<&'de ()>,
        }

        impl<'de> #visitor_ident<'de> {
            /// # Purpose of this method
            /// To reproduce C++ field inheritance, we will have the field internal implementation
            /// of deserialization partially exposed and reused.
            #[inline]
            pub fn visit_as_parent<__A>(
                mut __map: __A,
            ) -> _serde::__private::Result<#class_name, __A::Error>
            where
                __A: _serde::de::MapAccess<'de>,
            {
                Visitor::visit_struct(
                    Self {
                        marker: _serde::__private::PhantomData::<#class_name>,
                        lifetime: _serde::__private::PhantomData,
                    },
                    __map,
                )
            }
        }

        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
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
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate havok_serde as _serde;
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
                        __Visitor {
                            marker: _serde::__private::PhantomData::<#class_name>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            };
        }
    }
}

pub(super) fn to_visitor_ident(class_name_str: &str) -> syn::Ident {
    format_ident!("__{class_name_str}Visitor")
}
