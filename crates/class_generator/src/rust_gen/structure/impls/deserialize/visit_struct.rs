use crate::{
    cpp_info::{Class, Member},
    get_inherited_class, get_inherited_members,
    rust_gen::structure::{
        impls::deserialize::member_to_de_rust_type, to_rust_token::to_rust_field_ident,
    },
    ClassMap,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate `visit_struct` (for XML)
pub fn gen(class: &Class, classes_map: &ClassMap) -> TokenStream {
    let mut first_recv_fields = Vec::new(); // after call `next_value`
    let mut visit_fields_matcher = Vec::new(); // 　The process of removing the Option and inserting the value into the field at the end.
    let mut last_recv_fields = Vec::new();

    let mut last_recv_parents = Vec::new();
    if let Some(parent) = &class.parent {
        last_recv_parents.push(quote! { let __ptr = None; });
        last_recv_parents.extend(
            get_inherited_class(parent, classes_map)
                .iter()
                .map(|class| {
                    let new_struct = create_struct(class);
                    quote! {
                        let parent = #new_struct;
                    }
                }),
        );
    }

    for member in get_inherited_members(&class.name, classes_map) {
        let Member {
            name,
            flags,
            arrsize,
            ..
        } = member;

        if flags.has_skip_serializing() {
            continue;
        }

        let field_ident = to_rust_field_ident(name); // e.g. `m_fieldName`
        let field_type = member_to_de_rust_type(member, &class.name); // e.g. `u64`

        first_recv_fields.push(quote! {
            let mut #field_ident: _serde::__private::Option<#field_type> = _serde::__private::None;
        });

        let default_value = match arrsize {
            33.. => quote! { [Default::default(); #arrsize] },
            _ => quote! { Default::default() },
        };
        visit_fields_matcher.push(quote! {
            __Field::#field_ident => {
                #[cfg(any(feature = "strict", feature = "ignore_duplicates"))]
                if _serde::__private::Option::is_some(&#field_ident) {
                    #[cfg(feature = "ignore_duplicates")]
                    continue;
                    #[cfg(feature = "strict")]
                    return _serde::__private::Err(
                        <__A::Error as _serde::de::Error>::duplicate_field(#name),
                    );
                }
                #field_ident = _serde::__private::Some(
                    match __A::next_value::<#field_type>(&mut __map) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(__err);
                            #[cfg(not(feature = "strict"))]
                            #default_value // If not, fall back to the default value.
                        }
                    },
                );
            }
        });

        last_recv_fields.push(quote! {
            let #field_ident = match #field_ident {
                _serde::__private::Some(__field) => __field,
                _serde::__private::None => {
                    #[cfg(feature = "strict")]
                    return _serde::__private::Err(
                        <__A::Error as _serde::de::Error>::missing_field(#name),
                    );
                    #[cfg(not(feature = "strict"))]
                    #default_value // If not, fall back to the default value.
                }
            };
        });
    }

    let new_struct = create_struct(class);
    quote! {
            #[allow(clippy::manual_unwrap_or_default)]
            fn visit_struct<__A>(
                self,
                mut __map: __A,
            ) -> _serde::__private::Result<Self::Value, __A::Error>
            where
                __A: _serde::de::MapAccess<'de>,
            {
                #(#first_recv_fields)*

                while let _serde::__private::Some(__key) = {
                    #[cfg(not(feature = "strict"))]
                    let __key = __A::next_key::<__Field>(&mut __map).unwrap_or(Some(__Field::__ignore));
                    #[cfg(feature = "strict")]
                    let __key = __A::next_key::<__Field>(&mut __map)?;
                    __key
                } {
                    match __key {
                        #(#visit_fields_matcher)*
                        _ => {}
                    }
                }
                #(#last_recv_fields)*
                #(#last_recv_parents)*

                let __ptr = __A::class_ptr(&mut __map); // First make the `__ptr` of the inheritance source `Option::None` by taking `__ptr` here
                _serde::__private::Ok(#new_struct)
            }
    }
}

fn create_struct<'a>(class: &'a Class<'a>) -> TokenStream {
    let mut fields = Vec::new();
    let mut has_skip_once = false; // All fields whose serialize is skipped are made to use `Default::default`.

    fields.push(quote! { __ptr });
    if class.parent.is_some() {
        fields.push(quote! { parent });
    };

    for member in &class.members {
        let Member { name, flags, .. } = member;
        if flags.has_skip_serializing() {
            has_skip_once = true;
            continue;
        }

        let field_ident = to_rust_field_ident(name); // e.g. `m_fieldName`
        fields.push(quote!(#field_ident));
    }

    let default = if has_skip_once {
        quote! { ..Default::default() }
    } else {
        quote! {}
    };

    let class_name = format_ident!("{}", class.name);
    quote! {
        #class_name {
            #(#fields,)*
            #default
        }
    }
}
