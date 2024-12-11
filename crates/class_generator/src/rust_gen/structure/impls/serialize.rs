use crate::get_class_map::get_inherited_class;
use crate::rust_gen::structure::{
    impls::{n_time_parent_ident, str2lit},
    to_rust_token::to_rust_field_ident,
};
use crate::{
    cpp_info::{Class, Member, TypeKind},
    ClassMap,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_serialize(class: &Class, class_map: &ClassMap) -> TokenStream {
    let name = class.name.as_ref();
    let hex_signature = str2lit(&class.signature.to_string());
    let (size_x86, size_x64) = (class.size_x86 as u64, class.size_x86_64 as u64);
    let class_name = syn::Ident::new(name, proc_macro2::Span::call_site());
    let fields = impl_serialize_fields(class, class_map);
    let deps_class_indexes = deps_class_indexes_token(&class.name, class_map);
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
                    _serde::__private::Signature::new(#hex_signature)
                }

                #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
                fn deps_indexes(&self) -> Vec<usize> {
                    let mut v = Vec::new();
                    #(#deps_class_indexes;)*
                    v
                }
            }

            impl #lifetime _serde::Serialize for #class_name #lifetime {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: _serde::ser::Serializer
                {
                    let class_meta = self.__ptr.map(|name| (name, _serde::__private::Signature::new(#hex_signature)));
                    let mut serializer = __serializer.serialize_struct(#name, class_meta, (#size_x86, #size_x64))?;
                    #(#fields)*
                    serializer.end()
                }
        }
        };
    }
}

fn impl_serialize_fields(class: &Class, class_map: &ClassMap) -> Vec<TokenStream> {
    let mut serialize_calls = Vec::new();
    let mut x86_current_offset = 0;
    let mut x64_current_offset = 0;

    let all_class = get_inherited_class(&class.name, class_map);
    let mut parent_depth = all_class.len();

    for class in all_class {
        parent_depth -= 1;
        let meta_fields = impl_serialize_self_fields(
            &class.members,
            class.size_x86,
            class.size_x86_64,
            &mut x86_current_offset,
            &mut x64_current_offset,
            parent_depth,
            class_map,
        );
        serialize_calls.extend(meta_fields);
    }

    serialize_calls
}

/// # Returns
/// serialize_method_calls
fn impl_serialize_self_fields(
    members: &[Member],
    x86_size: u32,
    x64_size: u32,
    x86_current_offset: &mut u32,
    x64_current_offset: &mut u32,
    parent_depth: usize,
    class_map: &ClassMap,
) -> Vec<TokenStream> {
    let mut serialize_calls = Vec::new();

    for member in members {
        let Member {
            name,
            vtype,
            vsubtype,
            class_ref,
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
        let cpp_field_key = &name;
        let rust_field_name = to_rust_field_ident(name);
        let parent_ident = n_time_parent_ident(parent_depth);

        if arrsize == &0 {
            if vtype == &Array {
                let ser_method = if flags.has_skip_serializing() {
                    quote! { skip_array_field }
                } else {
                    quote! { serialize_array_field }
                };

                match vsubtype {
                    Struct => {
                        let (x86_size, x64_size) = {
                            let class_name = class_ref.as_ref().unwrap().as_ref();
                            let class = class_map.get(class_name).unwrap();
                            (class.size_x86 as u64, class.size_x86_64 as u64)
                        };
                        serialize_calls.push(quote! {
                            serializer.#ser_method(
                                #cpp_field_key,
                                &self #parent_ident.#rust_field_name,
                                TypeSize::Struct {
                                    size_x86: #x86_size,
                                    size_x86_64: #x64_size,
                            })?;
                        });
                    }
                    CString | StringPtr => {
                        serialize_calls.push(quote! {
                            serializer.#ser_method(#cpp_field_key, &self #parent_ident.#rust_field_name, TypeSize::String)?;
                        });
                    }
                    _ => {
                        serialize_calls.push(quote! {
                            serializer.#ser_method(#cpp_field_key, &self #parent_ident.#rust_field_name, TypeSize::NonPtr)?;
                        });
                    }
                };
            } else {
                let ser_method = if flags.has_skip_serializing() {
                    quote! { skip_field }
                } else {
                    quote! { serialize_field }
                };
                serialize_calls.push(quote! { serializer.#ser_method(#cpp_field_key, &self #parent_ident.#rust_field_name)?; });
            };
        } else {
            let ser_method = if flags.has_skip_serializing() {
                quote! { skip_fixed_array_field }
            } else {
                quote! { serialize_fixed_array_field }
            };

            match vtype {
                Struct => {
                    let (x86_size, x64_size) = {
                        let class_name = class_ref.as_ref().unwrap().as_ref();
                        let class = class_map.get(class_name).unwrap();
                        (class.size_x86 as u64, class.size_x86_64 as u64)
                    };
                    serialize_calls.push(quote! {
                        serializer.#ser_method(
                            #cpp_field_key,
                            self #parent_ident.#rust_field_name.as_slice(),
                            TypeSize::Struct {
                                size_x86: #x86_size,
                                size_x86_64: #x64_size,
                            }
                        )?;
                    });
                }
                CString | StringPtr => {
                    serialize_calls.push(quote! {
                        serializer.#ser_method(
                            #cpp_field_key,
                            self #parent_ident.#rust_field_name.as_slice(),
                            TypeSize::String,
                        )?;
                    });
                }
                _ => {
                    serialize_calls.push(quote! {
                        serializer.#ser_method(
                            #cpp_field_key,
                            self #parent_ident.#rust_field_name.as_slice(),
                            TypeSize::NonPtr,
                        )?;
                    });
                }
            };
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

    serialize_calls
}

fn deps_class_indexes_token(class_name: &str, classes_map: &ClassMap) -> Vec<TokenStream> {
    let mut deps_class_indexes = Vec::new();
    let inherited_class = get_inherited_class(class_name, classes_map);
    let mut parent_depth = inherited_class.len();
    for class in &inherited_class {
        parent_depth -= 1;

        for member in &class.members {
            let push_code = match member.vtype {
                // pointer array
                _ if (member.vtype == TypeKind::Pointer && member.arrsize > 0)
                    || member.vsubtype == TypeKind::Pointer =>
                {
                    let parent_ident = n_time_parent_ident(parent_depth);
                    let member_name = to_rust_field_ident(&member.name);
                    quote! {  v.extend(self #parent_ident.#member_name.iter().map(|ptr| ptr.get())) }
                }

                // struct array(get ptr of fields)
                _ if (member.vtype == TypeKind::Struct && member.arrsize > 0)
                    || member.vtype == TypeKind::Array && member.vsubtype == TypeKind::Struct =>
                {
                    let parent_ident = n_time_parent_ident(parent_depth);
                    let member_name = to_rust_field_ident(&member.name);
                    quote! {
                        v.extend(self #parent_ident.#member_name.iter().flat_map(|class| class.deps_indexes()).collect::<Vec<usize>>())
                    }
                }

                // struct(get ptr of fields)
                _ if member.vtype == TypeKind::Struct => {
                    let parent_ident = n_time_parent_ident(parent_depth);
                    let member_name = to_rust_field_ident(&member.name);
                    quote! {  v.extend(self #parent_ident.#member_name.deps_indexes()) }
                }

                TypeKind::Pointer => {
                    let parent_ident = n_time_parent_ident(parent_depth);
                    let member_name = to_rust_field_ident(&member.name);
                    quote! { v.push(self #parent_ident.#member_name.get()) }
                }

                _ => continue,
            };
            deps_class_indexes.push(push_code);
        }
    }

    deps_class_indexes
}
