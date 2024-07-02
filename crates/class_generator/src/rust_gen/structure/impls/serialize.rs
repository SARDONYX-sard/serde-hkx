use crate::{
    cpp_info::{Class, Member, TypeKind},
    get_inherited_class,
    rust_gen::structure::{impls::n_time_parent_ident, to_rust_token::to_rust_field_ident},
    ClassMap,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_serialize(class: &Class, class_map: &ClassMap) -> TokenStream {
    let name = class.name.as_ref();
    let signature = class.signature.get();
    let class_name = syn::Ident::new(name, proc_macro2::Span::call_site());
    let fields = impl_serialize_fields(class, class_map);
    let lifetime = match class.has_string {
        true => quote! { <'a> },
        false => quote! {},
    };

    quote::quote! {
        const _: () = {
            use havok_serde as _serde;

            impl #lifetime _serde::HavokClass for #class_name #lifetime {
                #[inline]
                fn name(&self) -> &'static str {
                    #name
                }

                #[inline]
                fn signature(&self) -> _serde::__private::Signature {
                    _serde::__private::Signature::new(#signature)
                }
            }

            impl #lifetime _serde::Serialize for #class_name #lifetime {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: _serde::ser::Serializer
                {
                    let class_meta = self.__ptr.map(|name| (name, _serde::__private::Signature::new(#signature)));
                    let mut serializer = __serializer.serialize_struct(#name, class_meta)?;
                    #(#fields)*
                    serializer.end()
                }
        }
        };
    }
}

fn impl_serialize_fields(class: &Class, class_map: &ClassMap) -> Vec<TokenStream> {
    let mut serialize_calls = Vec::new();
    // The ptr type must serialize the data pointed to by ptr after serializing all fields. This is an array for that purpose.
    let mut ptr_after_write_fields = Vec::new();
    let mut x86_current_offset = 0;
    let mut x64_current_offset = 0;

    let all_class = get_inherited_class(&class.name, class_map);
    let mut parent_depth = all_class.len();

    for class in all_class {
        parent_depth -= 1;
        let (meta_fields, ptr_fields) = impl_serialize_self_fields(
            &class.members,
            class.size_x86,
            class.size_x86_64,
            &mut x86_current_offset,
            &mut x64_current_offset,
            parent_depth,
        );
        serialize_calls.extend(meta_fields);
        ptr_after_write_fields.extend(ptr_fields);
    }

    serialize_calls.extend(ptr_after_write_fields);
    serialize_calls
}

/// # Returns
/// (serialize_method_calls, serialize_ptr_pointed_data_method_calls)
fn impl_serialize_self_fields(
    members: &[Member],
    x86_size: u32,
    x64_size: u32,
    x86_current_offset: &mut u32,
    x64_current_offset: &mut u32,
    parent_depth: usize,
) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut serialize_calls = Vec::new();
    // The ptr type must serialize the data pointed to by ptr after serializing all fields. This is an array for that purpose.
    let mut ptr_after_write_fields = Vec::new();

    for member in members {
        let Member {
            name,
            vtype,
            offset_x86,
            offset_x86_64,
            type_size_x86,
            type_size_x86_64,
            arrsize,
            flags,
            ..
        } = member;

        // Align
        let pad_x86 = (*offset_x86 - *x86_current_offset) as usize;
        let pad_x64 = (*offset_x86_64 - *x64_current_offset) as usize;

        if pad_x86 != 0 || pad_x64 != 0 {
            tracing::debug!(?name, pad_x86, x86_current_offset);
            tracing::debug!(?name, pad_x64, x64_current_offset);
            serialize_calls.push(quote! {
                serializer.pad_field([0u8; #pad_x86].as_slice(), [0u8; #pad_x64].as_slice())?;
            });

            *x86_current_offset = *offset_x86;
            *x64_current_offset = *offset_x86_64;
        }
        *x86_current_offset += type_size_x86;
        *x64_current_offset += type_size_x86_64;

        use TypeKind::*;
        let (meta_method, pointed_method) = match vtype {
            Array => {
                let meta_method = match flags.has_skip_serializing() {
                    true => quote! { skip_array_meta_field },
                    false => quote! { serialize_array_meta_field },
                };
                (meta_method, Some(quote! { serialize_array_field }))
            }
            CString => {
                let meta_method = match flags.has_skip_serializing() {
                    true => quote! { skip_cstring_meta_field },
                    false => quote! { serialize_cstring_meta_field },
                };
                (meta_method, Some(quote! { serialize_cstring_field }))
            }
            StringPtr => {
                let meta_method = match flags.has_skip_serializing() {
                    true => quote! { skip_stringptr_meta_field },
                    false => quote! { serialize_stringptr_meta_field },
                };
                (meta_method, Some(quote! { serialize_stringptr_field }))
            }
            _ => match flags.has_skip_serializing() {
                true => (quote! { skip_field }, None),
                false => (quote! { serialize_field }, None),
            },
        };

        let cpp_field_key = &name;
        let rust_field_name = to_rust_field_ident(name);
        let parent_ident = n_time_parent_ident(parent_depth);

        let maybe_as_slice = match arrsize {
            0 => quote! {},
            1.. => quote! { .as_slice() },
        };
        serialize_calls
            .push(quote! { serializer.#meta_method(#cpp_field_key, &self #parent_ident.#rust_field_name #maybe_as_slice)?; });

        // For `Array`, `CString` or `StringPtr`.(Not use `[StringPtr; 4]`)
        if let Some(pointed_method) = pointed_method {
            ptr_after_write_fields.push(
                quote! { serializer.#pointed_method(#cpp_field_key, &self #parent_ident.#rust_field_name)?; },
            );
        };
    }

    // Struct tailing alignment.
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
        serialize_calls.push(quote! {
            serializer.pad_field([0u8; #x86_pad].as_slice(), [0u8; #x64_pad].as_slice())?;
        });

        *x86_current_offset = x86_size;
        *x64_current_offset = x64_size;
    };

    (serialize_calls, ptr_after_write_fields)
}
