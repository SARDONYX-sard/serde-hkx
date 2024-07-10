use crate::{
    cpp_info::{Class, Member},
    rust_gen::structure::{
        impls::deserialize::{member_to_de_rust_type, to_visitor_ident},
        to_rust_token::to_rust_field_ident,
    },
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate `visit_struct` (for XML)
///
/// # Note
/// - `members`: current class members.
pub fn gen(class: &Class) -> TokenStream {
    let mut first_recv_fields = Vec::new(); // after call `next_value`
    let mut visit_fields_matcher = Vec::new(); // 　The process of removing the Option and inserting the value into the field at the end.
    let mut last_recv_fields = Vec::new();
    let mut field_idents = Vec::new();

    let mut has_skip_once = false; // All fields whose serialize is skipped are made to use `Default::default`.

    for member in &class.members {
        let Member { name, flags, .. } = member;

        if flags.has_skip_serializing() {
            has_skip_once = true;
            continue;
        }

        let field_ident = to_rust_field_ident(name); // e.g. `m_fieldName`
        let rust_type = member_to_de_rust_type(member, &class.name); // e.g. `u64`

        field_idents.push(field_ident.clone());

        first_recv_fields.push(quote! {
            let mut #field_ident: _serde::__private::Option<#rust_type> = _serde::__private::None;
        });

        visit_fields_matcher.push(quote! {
            __Field::#field_ident => {
                if _serde::__private::Option::is_some(&#field_ident) {
                    return _serde::__private::Err(
                        <__A::Error as _serde::de::Error>::duplicate_field(#name),
                    );
                }
                #field_ident = _serde::__private::Some(
                    match __A::next_value::<#rust_type>(&mut __map) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    },
                );
            }
        });

        last_recv_fields.push(quote! {
            let #field_ident = match #field_ident {
                _serde::__private::Some(__field) => __field,
                _serde::__private::None => {
                    return _serde::__private::Err(
                        <__A::Error as _serde::de::Error>::missing_field(#name),
                    )
                }
            };
        });
    }

    let default = if has_skip_once {
        quote! { ..Default::default() }
    } else {
        quote! {}
    };

    let (deserialize_parent, parent_field) = if let Some(parent_name) = &class.parent {
        let parent_visitor_name = to_visitor_ident(parent_name);
        (
            quote! { let parent = #parent_visitor_name::visit_as_parent(&mut __map)?; },
            quote! { parent, },
        )
    } else {
        (quote! {}, quote! {})
    };

    let class_name = format_ident!("{}", class.name);
    quote! {
            fn visit_struct<__A>(
                self,
                mut __map: __A,
            ) -> _serde::__private::Result<Self::Value, __A::Error>
            where
                __A: _serde::de::MapAccess<'de>,
            {
                #deserialize_parent
                #(#first_recv_fields)*
                while let _serde::__private::Some(__key) =
                    match __A::next_key::<__Field>(&mut __map) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    }
                {
                    match __key {
                        #(#visit_fields_matcher)*
                        _ => {}
                    }
                }
                #(#last_recv_fields)*

                _serde::__private::Ok(#class_name {
                    __ptr: __A::class_ptr(&mut __map),
                    #parent_field
                    #(#field_idents,)*
                    #default
                })
            }
    }
}
