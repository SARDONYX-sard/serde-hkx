use crate::cpp_info::Member;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_enum_fields(members: &[&Member]) -> TokenStream {
    let mut enum_variants = Vec::new();
    let mut visit_uint64_matchers = Vec::new();
    let mut visit_key_matchers = Vec::new();

    for (index, member) in members.iter().enumerate() {
        let field_ident = quote::format_ident!("__field{index}");

        enum_variants.push(quote! { #field_ident });

        // e.g. 0 => Ok(__Field::__field0),
        visit_uint64_matchers.push(quote! { index => Ok(__Field::#field_ident) });

        if !&member.flags.has_skip_serializing() {
            // e.g. "referenceCount" => Ok(__Field::__field1),
            let member_name = &member.name;
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

                /// Intended for use in binary.
                fn visit_uint64<E>(self, __value: u64) -> Result<Self::Value, E>
                where
                    E: havok_serde::de::Error,
                {
                    match __value {
                        #(#visit_uint64_matchers,)*
                        _ => Ok(__Field::__ignore),
                    }
                }

                /// Intended for use in XML.
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
