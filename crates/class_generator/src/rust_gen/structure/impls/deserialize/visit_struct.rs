use proc_macro2::TokenStream;
use quote::quote;

fn visit_struct_matcher(index: usize, member_name: &str, member_ty: TokenStream) -> TokenStream {
    // It is a variant to determine which fields are to be deserialized by enum.
    let n_field_ident = quote::format_ident!("__field{index}");

    quote! {
        __Field::#n_field_ident => {
            if _serde::__private::Option::is_some(&#n_field_ident) {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::duplicate_field(
                        #member_name,
                    ),
                );
            }
            #n_field_ident = _serde::__private::Some(
                match _serde::de::MapAccess::next_value::<#member_ty>(&mut __map) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                },
            );
        }
    }
}

fn set_field_in_visit_struct(index: usize, member_name: &str) -> TokenStream {
    let n_field_ident = quote::format_ident!("__field{index}");

    quote! {
        let #n_field_ident = match #n_field_ident {
            _serde::__private::Some(#n_field_ident) => #n_field_ident,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(#member_name),
                )
            }
        };
    }
}

fn gen_visit_struct(
    class_name: &str,
    setter_declares: Vec<TokenStream>,
    set_fields: Vec<TokenStream>,
    visit_struct_matchers: Vec<TokenStream>,
) -> TokenStream {
    let class_name_ident = syn::Ident::new(class_name, proc_macro2::Span::call_site());
    let expecting_msg = format!("struct {class_name}");

    quote! {
        struct __Visitor<'de> {
            marker: core::marker::PhantomData<#class_name>,
            lifetime: core::marker::PhantomData<&'de ()>,
        }

        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = #class_name_ident;
            fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Formatter::write_str(__formatter, #expecting_msg)
            }

            #[inline]
            fn visit_struct<__A>(
                self,
                mut __map: __A,
            ) -> _serde::__private::Result<Self::Value, __A::Error>
            where
                __A: _serde::de::MapAccess<'de>,
            {
                let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                let mut __field1: _serde::__private::Option<i16> = _serde::__private::None;

                __A::pad(&mut __map, 4, 8); // before 1st field.
                while let _serde::__private::Some(__key) =
                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    }
                {
                    match __key {
                        #(#visit_struct_matchers)*
                        _ => {}
                    }
                }

                #(#set_fields)*

                _serde::__private::Ok(HkReferencedObject {
                        parent: HkBaseObject { _name: None },
                        __ptr_name_attr: __map.class_ptr(),
                        mem_size_and_flags: __field0,
                        reference_count: __field1,
                    },
                )
            }
        }

        const FIELDS: &[&str] = &["memSizeAndFlags", "referenceCount"];
        _serde::Deserializer::deserialize_struct(
            deserializer,
            #class_name,
            FIELDS,
            __Visitor {
                marker: _serde::__private::PhantomData::<#class_name_ident>,
                lifetime: _serde::__private::PhantomData,
            },
        )

    }
}
