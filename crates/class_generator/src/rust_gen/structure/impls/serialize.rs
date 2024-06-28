use crate::{
    cpp_info::{Class, Member, TypeKind},
    rust_gen::structure::to_rust_token::to_rust_field_ident,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_serialize(class: &Class) -> TokenStream {
    let name = class.name.as_ref();
    let signature = class.signature.get();
    let class_name = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let class_name_c_str = proc_macro2::Literal::c_string(&std::ffi::CString::new(name).unwrap());
    let fields = impl_serialize_fields(class);
    let lifetime = match class.has_string {
        true => quote! { <'a> },
        false => quote! {},
    };

    quote::quote! {
        const _: () = {
            use havok_serde as __serde;
            use __serde::HavokClass;

            impl #lifetime __serde::HavokClass for #class_name #lifetime {
                fn name(&self) -> &'static core::ffi::CStr {
                    #class_name_c_str
                }

                fn signature(&self) -> __serde::__private::Signature {
                    __serde::__private::Signature::new(#signature)
                }
            }

            impl #lifetime __serde::Serialize for #class_name #lifetime {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: __serde::ser::Serializer
                {
                    let class_meta = self.__ptr_name_attr.map(|name| (name, self.signature()));
                    let mut serializer = __serializer.serialize_struct(#name, class_meta)?;
                    #(#fields)*
                    serializer.end()
                }
        }
        };
    }
}

fn impl_serialize_fields(class: &Class) -> Vec<TokenStream> {
    let mut x86_current_offset = 0;
    let mut x64_current_offset = 0;

    let mut serialize_calls = Vec::new();
    // The ptr type must serialize the data pointed to by ptr after serializing all fields. This is an array for that purpose.
    let mut ptr_after_write_fields = Vec::new();

    for member in &class.members {
        let Member {
            name,
            vtype,
            offset_x86,
            offset_x86_64,
            type_size_x86,
            type_size_x86_64,
            flags,
            ..
        } = member;

        // Align
        let pad_x86 = (offset_x86 - x86_current_offset) as usize;
        let pad_x64 = (offset_x86_64 - x64_current_offset) as usize;
        if pad_x86 != 0 || pad_x64 != 0 {
            serialize_calls.push(quote! {
                serializer.pad_field([0u8; #pad_x86].as_slice(), [0u8; #pad_x64].as_slice())?;
            });

            x86_current_offset = *offset_x86;
            x64_current_offset = *offset_x86_64;
        }
        x86_current_offset += type_size_x86;
        x64_current_offset += type_size_x86_64;

        let cpp_field_key = &name;
        let rust_field_name = to_rust_field_ident(name);
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

        serialize_calls
            .push(quote! { serializer.#meta_method(#cpp_field_key, &self.#rust_field_name)?; });

        if let Some(pointed_method) = pointed_method {
            ptr_after_write_fields.push(
                quote! { serializer.#pointed_method(#cpp_field_key, &self.#rust_field_name)?; },
            );
        };
    }

    let x86_size = class.size_x86;
    let x64_size = class.size_x86_64;

    // Struct tailing alignment.
    let x86_pad = if x86_size > x86_current_offset {
        x86_current_offset.abs_diff(x86_size) as usize
    } else if x86_size == x86_current_offset {
        0
    } else {
        panic!("x86_size({x86_size}) < x86_current_offset({x86_current_offset})");
    };
    let x64_pad = if x64_size > x64_current_offset {
        x64_current_offset.abs_diff(x64_size) as usize
    } else if x64_size == x64_current_offset {
        0
    } else {
        panic!("x64_size({x64_size}) < x64_current_offset({x64_current_offset})");
    };
    if x86_pad != 0 || x64_pad != 0 {
        serialize_calls.push(quote! {
            serializer.pad_field([0u8; #x86_pad].as_slice(), [0u8; #x64_pad].as_slice())?;
        });
    };

    serialize_calls.extend(ptr_after_write_fields);
    serialize_calls
}
