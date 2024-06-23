use proc_macro2::TokenStream;
use quote::quote;

#[derive(Default, deluxe::ParseMetaItem, deluxe::ExtractAttributes)]
pub struct SerdeClass {
    pub name: String,
    pub signature: u32,
    x86_size: usize,
    x64_size: usize,
}

#[derive(Default, deluxe::ParseMetaItem, deluxe::ExtractAttributes)]
struct SerdeAttr {
    #[deluxe(default)]
    rename: String,
    #[deluxe(default)]
    x86_offset: usize,
    #[deluxe(default)]
    x64_offset: usize,
    #[deluxe(default)]
    skip_serializing: bool,
}

pub fn expand_derive_serialize(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let errors = deluxe::Errors::new();
    let SerdeClass {
        name,
        signature,
        x86_size,
        x64_size,
    } = deluxe::extract_attributes_optional(input, &errors);

    let mut field_tokens = Vec::new();
    if let syn::Data::Struct(s) = &mut input.data {
        field_tokens = serialize_field_tokens(s, x86_size, x64_size);
    }

    let c_name = proc_macro2::Literal::c_string(&std::ffi::CString::new(name.clone()).unwrap());
    let ident = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    // Make sure to include the errors in the output
    let tokens = quote::quote! {
        #errors
        const _: () = {
            use havok_serde as __serde;
            use __serde::HavokClass;

                impl #impl_generics __serde::HavokClass for #ident #type_generics #where_clause {
                    fn name(&self) -> &'static core::ffi::CStr {
                        #c_name
                    }

                    fn signature(&self) -> __serde::__private::Signature {
                        __serde::__private::Signature::new(#signature)
                    }
                }

            impl #impl_generics __serde::Serialize for #ident #type_generics #where_clause {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: __serde::ser::Serializer
                {
                    use __serde::ser::SerializeStruct;
                    let class_meta = self.__ptr_name_attr.map(|name| (name, self.signature()));
                    let mut serializer = __serializer.serialize_struct(#name, class_meta)?;

                    #(#field_tokens)*
                    serializer.end()
                }
        }
        };
    };
    Ok(tokens)
}

/// Returns a token array as follows
/// ```no_run
/// serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
/// serializer.skip_field("memSizeAndFlags", &self.mem_size_and_flags)?; // offset: 4(+2), 8(+2)
/// serializer.skip_field("referenceCount", &self.reference_count)?; // offset: 6(+2), 10(+2)
/// serializer.pad_field(&[0u8; 0].as_slice(), &[0u8; 4].as_slice())?; // offset: 8(+0), 12(+4) Tailing align by ptr size bytes.
/// ```
fn serialize_field_tokens(
    s: &mut syn::DataStruct,
    x86_size: usize,
    x64_size: usize,
) -> Vec<TokenStream> {
    let mut x86_current_offset: usize = 0;
    let mut x64_current_offset: usize = 0;
    let mut field_tokens = Vec::new();
    let mut ptr_after_write_fields = Vec::new();

    for field in s.fields.iter_mut() {
        if field.ident.as_ref().map(|ident| ident.to_string())
            == Some("__ptr_name_attr".to_string())
        {
            continue;
        }

        // Aggregate any errors to avoid exiting the loop early
        match deluxe::extract_attributes(field) {
            Ok(SerdeAttr {
                rename,
                x86_offset,
                x64_offset,
                skip_serializing,
            }) => {
                if x86_current_offset > 0 {
                    let x86_pad = x86_current_offset.abs_diff(x86_offset);
                    let x64_pad = x64_current_offset.abs_diff(x64_offset);
                    field_tokens.push(quote! {
                            serializer.pad_field([0u8; #x86_pad].as_slice(), [0u8; #x64_pad].as_slice())?;
                        });

                    x86_current_offset += x86_offset;
                    x64_current_offset += x64_offset;
                }

                let field_name = &field.ident;
                if skip_serializing {
                    use syn::Type::*;
                    match field.ty {
                        Array(_) | Slice(_) => {
                            ptr_after_write_fields.push(quote! {
                                serializer.skip_array_field(#rename, &self.#field_name)?;
                            });
                        }
                        _ => {
                            field_tokens.push(quote! {
                                serializer.skip_field(#rename, &self.#field_name)?;
                            });
                        }
                    };
                } else {
                    use syn::Type::*;
                    match field.ty {
                        Array(_) | Slice(_) => {
                            field_tokens.push(quote! {
                                serializer.serialize_array_meta_field(#rename, &self.#field_name)?;
                            });
                            ptr_after_write_fields.push(quote! {
                                serializer.serialize_array_field(#rename, &self.#field_name)?;
                            });
                        }
                        _ => {
                            field_tokens.push(quote! {
                                serializer.serialize_field(#rename, &self.#field_name)?;
                            });
                        }
                    };
                }
            }
            Err(e) => panic!("{e}"),
        }
    }

    // Struct tailing alignment.
    let x86_pad = if x86_size > x86_current_offset {
        x86_current_offset.abs_diff(x86_size)
    } else {
        0
    };
    let x64_pad = if x86_size > x64_current_offset {
        x64_current_offset.abs_diff(x64_size)
    } else {
        0
    };
    if !(x86_pad == 0 && x64_pad == 0) {
        field_tokens.push(quote! {
            serializer.pad_field([0u8; #x86_pad].as_slice(), [0u8; #x64_pad].as_slice())?;
        });
    };

    field_tokens.extend(ptr_after_write_fields);
    field_tokens
}
