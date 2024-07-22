use crate::{cpp_info::Member, rust_gen::structure::to_rust_token::to_rust_field_ident};
use proc_macro2::TokenStream;
use quote::quote;

/// Implement deserialization to parse key(`<hkparam name="key">`) for XML and convert it to each variant of enum.
pub fn gen_enum_visitor(members: &[&Member]) -> TokenStream {
    let mut enum_variants = Vec::new();
    let mut visit_key_matchers = Vec::new();

    for member in members {
        let Member { name, flags, .. } = member;

        if flags.has_skip_serializing() {
            continue;
        }
        let field_ident = to_rust_field_ident(name);

        enum_variants.push(quote! { #field_ident });
        if !&member.flags.has_skip_serializing() {
            // e.g. "referenceCount" => Ok(__Field::__field1),
            let member_name = name;
            visit_key_matchers.push(quote! { #member_name => Ok(__Field::#field_ident) });
        };
    }

    quote! {
            #[allow(non_camel_case_types)]
            enum __Field {
                #(#enum_variants,)*
                __ignore,
            }

            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;

                fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }

                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        #(#visit_key_matchers,)*
                        _ => Ok(__Field::__ignore),
                    }
                }
            }

            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
    }
}
