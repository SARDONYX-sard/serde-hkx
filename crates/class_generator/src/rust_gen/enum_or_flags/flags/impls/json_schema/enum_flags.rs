use crate::cpp_info::{Enum, EnumItem};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_schema_for_enum_flag(one_enum: &Enum) -> TokenStream {
    let Enum { name, .. } = one_enum;

    let flag_name = syn::Ident::new(name, proc_macro2::Span::call_site());
    // e.g. vec!["ROLE", "ROLE1"]
    let variants: Vec<TokenStream> = one_enum
        .enum_item
        .iter()
        .map(|enum_item| schema_flag_variant(enum_item))
        .collect();

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
                fn json_schema(generator: &mut SchemaGenerator) -> Schema {
                    use schemars::_private::serde_json::{self, Value};

                    // NOTE: More than 256 is an error due to macro's recursion limit,
                    // so do not rely on macros but implement manually.
                    // ref:https://docs.rs/schemars/1.0.0-alpha.15/schemars/macro.json_schema.html
                    let selection = &[#(#variants),*];
                    let selection = selection.iter().map(|s| Value::String(s.to_string())).collect();

                    let mut schema = Value::json_schema(generator);
                    let mut map= schema.ensure_object();
                    map.insert("type".to_string(), Value::String("string".to_string()));
                    map.insert("enum".to_string(), Value::Array(selection));
                    schema
                }
            }
        };
    }
}

/// Generate schema for each flag item.
fn schema_flag_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name: variant, .. } = one_enum;

    quote! { #variant }
}
