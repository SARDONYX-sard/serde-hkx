use crate::{cpp_info::Member, rust_gen::structure::to_rust_token::to_rust_type};
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_visit(
    members: &[Member],
    x86_size: u32,
    x64_size: u32,
    x86_current_offset: &mut u32,
    x64_current_offset: &mut u32,
) -> Vec<TokenStream> {
    let mut visit_struct_for_bytes_matcher = Vec::new();

    // after call `next_value`
    // e.g. `let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;`
    let mut value_res_fields = Vec::new();

    for (index, member) in members.iter().enumerate() {
        let Member {
            name,
            vtype,
            offset_x86,
            offset_x86_64,
            type_size_x86,
            type_size_x86_64,
            arrsize,
            ..
        } = member;

        // Align
        let pad_x86 = (*offset_x86 - *x86_current_offset) as usize;
        let pad_x64 = (*offset_x86_64 - *x64_current_offset) as usize;

        let pad_code = if pad_x86 != 0 || pad_x64 != 0 {
            tracing::debug!(?name, pad_x86, x86_current_offset);
            tracing::debug!(?name, pad_x64, x64_current_offset);
            *x86_current_offset = *offset_x86;
            *x64_current_offset = *offset_x86_64;

            quote! {
                __A.pad(#pad_x86, #pad_x64)?;
            }
        } else {
            quote! {}
        };
        *x86_current_offset += type_size_x86;
        *x64_current_offset += type_size_x86_64;

        let field_ident = quote::format_ident!("__field{index}");
        value_res_fields.push(quote! {
            let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
        });

        let rust_ty = to_rust_type(vtype).unwrap();
        let rust_ty = match *arrsize {
            0 => rust_ty,
            _ => quote! { [#rust_ty; #arrsize]},
        };

        visit_struct_for_bytes_matcher.push(quote! {
            __Field::#field_ident => {
                if _serde::__private::Option::is_some(&#field_ident) {
                    return _serde::__private::Err(
                        <__A::Error as _serde::de::Error>::duplicate_field(#name),
                    );
                }
                // As we need pad this line.
                #pad_code
                //
                #field_ident = _serde::__private::Some(
                    match __A::next_value::<#rust_ty>(&mut __map) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    },
                );
            }
        });
    }

    // Struct tailing alignment.
    let tailing_pad_code = {
        let x86_pad = match x86_size {
            x86_size if x86_size > *x86_current_offset => {
                x86_current_offset.abs_diff(x86_size) as usize
            }
            x86_size if x86_size == *x86_current_offset => 0,
            _ => panic!("x86_size({x86_size}) < x86_current_offset({x86_current_offset})"),
        };
        let x64_pad = match x64_size {
            x64_size if x64_size > *x64_current_offset => {
                x64_current_offset.abs_diff(x64_size) as usize
            }
            x64_size if x64_size == *x64_current_offset => 0,
            _ => panic!("x64_size({x64_size}) < x64_current_offset({x64_current_offset})"),
        };

        if x86_pad != 0 || x64_pad != 0 {
            tracing::debug!(x86_pad, x86_current_offset);
            tracing::debug!(x64_pad, x64_current_offset);
            *x86_current_offset = x86_size;
            *x64_current_offset = x64_size;

            quote! {
                __A.pad(#x86_pad, #x64_pad)?;
            }
        } else {
            quote! {}
        }
    };

    value_res_fields
}
