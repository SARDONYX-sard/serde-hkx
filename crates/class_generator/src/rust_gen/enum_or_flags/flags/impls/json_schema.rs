use crate::cpp_info::Enum;
use proc_macro2::TokenStream;
use quote::quote;

/// `bitflags` cannot automatically implement the `JsonSchema` trait, so you have to impl it manually here.
///
/// Expected json e.g.: `BIT_FLAG|BIT_FLAG2|4`(This is `SerdeWithDisplay` of bitflags as same as XML display.)
///
/// # Why?
/// Default would be `{ bits: 2 }` and so on, but since it is difficult to understand, I would like to make it the same as XML.
pub fn impl_schema_for_flag(one_enum: &Enum) -> TokenStream {
    let Enum { name, .. } = one_enum;

    let flag_name = syn::Ident::new(name, proc_macro2::Span::call_site());

    // Generate descriptions for each specific flag item
    let descriptions: Vec<String> = one_enum
        .enum_item
        .iter()
        .map(|enum_item| format!("{}: {}", enum_item.name, enum_item.value))
        .collect();
    let full_description = format!(
        "Bitflags field. Specific flags: {}. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
        descriptions.join(", ")
    );

    quote! {
        #[cfg(feature = "json_schema")]
        const _: () = {
            use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
            use std::borrow::Cow;

            impl JsonSchema for #flag_name {
                fn schema_name() -> Cow<'static, str>{
                    #name.into()
                }

                // Include the module, in case a type with the same name is in another module/crate
                // `module_path` macro is std lib.
                fn schema_id() -> Cow<'static, str> {
                    concat!(module_path!(), "::", #name).into()
                }

                // Define the schema for the `bits` field as an integer type
                fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
                    json_schema!({
                        "description": #full_description,
                        "type": "string",
                    })
                }
            }
        };
    }
}
