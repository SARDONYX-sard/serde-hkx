use crate::cpp_info::{Enum, EnumItem};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_ser_for_flag(one_enum: &Enum) -> TokenStream {
    let flag_name = syn::Ident::new(&one_enum.name, proc_macro2::Span::call_site());
    let variants: Vec<TokenStream> = one_enum
        .enum_item
        .iter()
        .map(|enum_item| serialize_flag_variant(enum_item))
        .collect();

    quote! {
        const _: () = {
            use havok_serde as __serde;

            impl __serde::Serialize for #flag_name {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: __serde::ser::Serializer
                {
                    let mut serializer = __serializer.serialize_enum_flags()?;
                    if self.is_empty() {
                        __serializer.serialize_empty_bit()?;
                        return __serializer.end();
                    };

                    match self {
                        #(#variants,)*
                        remain => __serializer.serialize_field(&remain.bits().to_string(), &remain.bits()),
                    }?;

                    __serializer.serialize_bits(&self.bits())?;
                    serializer.end()
                }
            }
        };
    }
}

fn serialize_flag_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name, .. } = one_enum;
    let variant = syn::Ident::new(&name, proc_macro2::Span::call_site());
    quote! {
        Self::#variant => serializer.serialize_field(#name, &Self::#variant)
    }
}
