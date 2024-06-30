use crate::{
    cpp_info::Enum,
    rust_gen::enum_or_flags::{to_rust_storage_size_ident, to_rust_storage_type},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_de_for_flag(one_enum: &Enum) -> TokenStream {
    let Enum { name, vsubtype, .. } = one_enum;

    let flag_ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let expected = format!("struct {}(flags)", name);

    // e.g. `u64`
    let storage_type = to_rust_storage_type(vsubtype);
    // e.g. `Uint64`
    let storage_size_ident = to_rust_storage_size_ident(vsubtype)
        .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));
    // e.g. `visit_uint64`
    let visitor_method_for_binary =
        quote::format_ident!("visit_{}", storage_size_ident.to_string().to_lowercase());

    quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate havok_serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for #flag_ident {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<#flag_ident>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = #flag_ident;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, #expected)
                        }

                        #[inline]
                        fn #visitor_method_for_binary<__E>(
                            self,
                            __value: #storage_type,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            Ok(#flag_ident::from_bits_retain(__value as _)) // Contain unknown bits.
                        }

                        fn visit_stringptr<__E>(
                            self,
                            __value: StringPtr<'de>,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match <#flag_ident as core::str::FromStr>::from_str(
                                __value.into_inner().unwrap().as_ref(),
                            ) {
                                Ok(flags) => Ok(flags),
                                Err(err) => Err(_serde::de::Error::custom(err)),
                            }
                        }
                    }

                    _serde::Deserializer::deserialize_flags(
                        __deserializer,
                        _serde::de::ReadEnumSize::#storage_size_ident,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<#flag_ident>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
}
