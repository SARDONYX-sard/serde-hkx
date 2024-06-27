use crate::cpp_info::{Class, Enum, EnumItem};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_serialize_for_enum(class: &Class) -> TokenStream {
    class
        .enums
        .iter()
        .map(|one_enum| impl_one_enum(one_enum))
        .collect()
}

fn impl_one_enum(one_enum: &Enum) -> TokenStream {
    let enum_name = syn::Ident::new(&one_enum.name, proc_macro2::Span::call_site());
    let variants: Vec<TokenStream> = one_enum
        .enum_item
        .iter()
        .map(|enum_item| serialize_enum_variant(enum_item))
        .collect();

    let cast_method = "to_i8";
    let method_ident = syn::Ident::new(cast_method, proc_macro2::Span::call_site());
    let err_msg = format!("Failed enum {enum_name} {cast_method}");

    quote! {
        const _: () = {
            use havok_serde as __serde;

            impl __serde::Serialize for #enum_name {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: __serde::ser::Serializer
                {
                    let mut serializer = __serializer.serialize_enum_flags()?;
                    match self {
                        #(#variants,)*
                    }?;

                    let for_bin = self.#method_ident().ok_or(S::Error::custom(#err_msg))?;
                    __serializer.serialize_bits(&for_bin)?;

                    serializer.end()
                }
        }
        };
    }
}

fn serialize_enum_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name, value } = one_enum;
    let variant = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let value = (*value) as usize;
    quote! {
        Self::#variant => serializer.serialize_field(#name, &#value)
    }
}
