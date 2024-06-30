use crate::{
    cpp_info::{Enum, EnumItem, TypeKind},
    rust_gen::enum_or_flags::{
        cast_number_to_token, to_rust_storage_size_ident, to_rust_storage_type,
    },
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_de_for_enum(one_enum: &Enum) -> TokenStream {
    let Enum {
        name: enum_name,
        vtype,
        vsubtype,
        enum_item,
        ..
    } = one_enum;

    if *vtype != TypeKind::Enum {
        panic!(
            "Expected TYPE_ENUM but another type is mixed in. Got enum {enum_name}(vtype: {vtype})"
        )
    };
    if *vsubtype == TypeKind::Void {
        tracing::info!("Skip automatic enum generation because this enum {enum_name} is a void storage type, indicating that it is not used.");
        return quote! {};
    };
    let enum_ident = syn::Ident::new(&enum_name, proc_macro2::Span::call_site());

    let mut values = Vec::new();
    let mut fields_str = Vec::new();
    let mut visit_number_matcher = Vec::new();
    let mut visit_stringptr_matcher = Vec::new();
    let mut visit_enum_matcher = Vec::new();
    let mut field_variants = Vec::new();
    for (index, enum_item) in enum_item.iter().enumerate() {
        let EnumItem { name, value, .. } = enum_item;
        fields_str.push(name); // For expected value info in `visit_stringptr`
        values.push(value.to_string()); // For expected value info in `visit_uint64`

        // NOTE: Avoid using value as ident because of errors if the enum has a minus value.
        let field_variant = quote::format_ident!("__field{index}");

        // e.g. `0u64`
        let num_with_suffix = cast_number_to_token(enum_item, vsubtype);
        // `3u64 => _serde::__private::Ok(__Field::__field3)`
        visit_number_matcher.push(quote! {
            #num_with_suffix => _serde::__private::Ok(__Field::#field_variant)
        });

        // # Example
        // ```
        // (__Field::__field0, __variant) => {
        //     _serde::de::VariantAccess::unit_variant(__variant)?;
        //     _serde::__private::Ok(EventMode::EventModeDefault)
        // }
        // ```
        let name_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        visit_enum_matcher.push(quote! {
            (__Field::#field_variant, __variant) => {
                _serde::de::VariantAccess::unit_variant(__variant)?;
                _serde::__private::Ok(#enum_ident::#name_ident)
            }
        });

        // # Example
        // ```
        // v if v == "0" || v.eq_ignore_ascii_case("EVENT_MODE_DEFAULT") => {
        //     _serde::__private::Ok(__Field::__field0)
        // }
        // ```
        let value_str = value.to_string();
        visit_stringptr_matcher.push(quote! {
            v if v == #value_str || v.eq_ignore_ascii_case(#name) => {
                _serde::__private::Ok(__Field::#field_variant)
            }
        });

        // To define enum. e.g.  `__field1`, `__field2`, ...
        field_variants.push(field_variant);
    }
    let rust_storage_type = to_rust_storage_type(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));
    let expected_msg_in_visit_number = format!(
        "value({rust_storage_type}) of variant is one of {}",
        values.join(", ")
    );
    let expecting = format!("enum {enum_name}"); // For `__Visitor`

    let storage_size_ident = to_rust_storage_size_ident(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));
    let visitor_method_for_binary =
        quote::format_ident!("visit_{}", storage_size_ident.to_string().to_lowercase());
    let rust_storage_type = to_rust_storage_type(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));

    quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate havok_serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #enum_ident {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        #(#field_variants,)*
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn #visitor_method_for_binary<__E>(
                            self,
                            __value: #rust_storage_type,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                #(#visit_number_matcher,)*
                                _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::#storage_size_ident(__value),
                                    &#expected_msg_in_visit_number,
                                )),
                            }
                        }
                        fn visit_stringptr<__E>(
                            self,
                            __value: StringPtr<'de>,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            if let Some(__value) = __value.into_inner() {
                                match __value.as_ref() {
                                    #(#visit_stringptr_matcher)*
                                    _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                        &__value, VARIANTS,
                                    )),
                                }
                            } else {
                                _serde::__private::Err(_serde::de::Error::unknown_variant("None", VARIANTS))
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<#enum_ident>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #enum_ident;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, #expecting)
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                #(#visit_enum_matcher)*
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &[#(#fields_str,)*];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        #enum_name,
                        VARIANTS,
                        _serde::de::ReadEnumSize::#storage_size_ident,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<#enum_ident>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
