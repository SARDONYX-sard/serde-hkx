use crate::{
    cpp_info::{Class, Member},
    rust_gen::structure::to_rust_token::{member_to_rust_type, to_rust_field_ident},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate `visit_struct_for_bytes` (For binary)
pub fn gen(class: &Class) -> TokenStream {
    let mut first_recv_fields = Vec::new(); // after call `next_value`
    let mut visit_fields_matcher = Vec::new(); // 　The process of removing the Option and inserting the value into the field at the end.
    let mut last_recv_fields = Vec::new();
    let mut field_idents = Vec::new();

    let mut x86_current_offset = match class.members.first() {
        Some(member) => member.offset_x86,
        None => 0,
    };
    let mut x64_current_offset = match class.members.first() {
        Some(member) => member.offset_x86_64,
        None => 0,
    };

    for (index, member) in class.members.iter().enumerate() {
        let Member {
            name,
            offset_x86,
            offset_x86_64,
            type_size_x86,
            type_size_x86_64,
            ..
        } = member;

        // Align
        let pad_x86 = (*offset_x86 - x86_current_offset) as usize;
        let pad_x64 = (*offset_x86_64 - x64_current_offset) as usize;

        let pad_code = if pad_x86 != 0 || pad_x64 != 0 {
            tracing::debug!(?name, pad_x86, x86_current_offset);
            tracing::debug!(?name, pad_x64, x64_current_offset);
            x86_current_offset = *offset_x86;
            x64_current_offset = *offset_x86_64;

            quote! {
                __A.pad(#pad_x86, #pad_x64)?;
            }
        } else {
            quote! {}
        };
        x86_current_offset += type_size_x86;
        x64_current_offset += type_size_x86_64;

        let field_ident = to_rust_field_ident(&member.name);
        let rust_type = member_to_rust_type(member, &class.name);

        first_recv_fields.push(quote! {
            let mut #field_ident: _serde::__private::Option<#rust_type> = _serde::__private::None;
        });

        field_idents.push(field_ident.clone());

        visit_fields_matcher.push(quote! {
            #index => {
                if _serde::__private::Option::is_some(&#field_ident) {
                    return _serde::__private::Err(
                        <__A::Error as _serde::de::Error>::duplicate_field(#name),
                    );
                }
                #pad_code
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
                        <__A::Error as _serde::de::Error>::missing_field(
                            #name,
                        ),
                    )
                }
            };
        });
    }

    // Struct tailing alignment.
    let tailing_align_method = gen_tailing_align_method(
        class.size_x86,
        class.size_x86_64,
        x86_current_offset,
        x64_current_offset,
    );

    let parent_variable = if class.parent.is_some() {
        quote! { let parent = __A::next_value(&mut __map)?; }
    } else {
        quote! {}
    };
    if class.parent.is_some() {
        field_idents.insert(0, format_ident!("parent"));
    };

    let class_name = format_ident!("{}", class.name);
    let member_len = class.members.len();
    quote! {
            fn visit_struct_for_bytes<__A>(
                self,
                mut __map: __A,
            ) -> _serde::__private::Result<Self::Value, __A::Error>
            where
                __A: _serde::de::MapAccess<'de>,
            {
                #parent_variable
                #(#first_recv_fields)*

                for i in 0..#member_len {
                    match i {
                        #(#visit_fields_matcher)*
                        _ => {}
                    }
                }
                #tailing_align_method

                #(#last_recv_fields)*

                _serde::__private::Ok(#class_name {
                    __ptr: __A::class_ptr(__map),
                    #(#field_idents,)*
                })
            }
    }
}

/// If struct need, then generate.
fn gen_tailing_align_method(
    x86_size: u32,
    x64_size: u32,
    x86_current_offset: u32,
    x64_current_offset: u32,
) -> TokenStream {
    let x86_pad = match x86_size {
        x86_size if x86_size > x86_current_offset => x86_current_offset.abs_diff(x86_size) as usize,
        x86_size if x86_size == x86_current_offset => 0,
        _ => panic!("x86_size({x86_size}) < x86_current_offset({x86_current_offset})"),
    };
    let x64_pad = match x64_size {
        x64_size if x64_size > x64_current_offset => x64_current_offset.abs_diff(x64_size) as usize,
        x64_size if x64_size == x64_current_offset => 0,
        _ => panic!("x64_size({x64_size}) < x64_current_offset({x64_current_offset})"),
    };

    if x86_pad != 0 || x64_pad != 0 {
        tracing::debug!(x86_pad, x86_current_offset);
        tracing::debug!(x64_pad, x64_current_offset);
        // x86_current_offset = x86_size;
        // x64_current_offset = x64_size;

        quote! {
            __A.pad(#x86_pad, #x64_pad)?;
        }
    } else {
        quote! {}
    }
}