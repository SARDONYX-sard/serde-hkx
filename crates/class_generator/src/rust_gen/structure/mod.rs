mod field;
mod impls;
mod to_rust_token;

use self::field::gen_field;
use crate::cpp_info::Class;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, ItemStruct};

pub use self::impls::deserialize::impl_deserialize;
pub use self::impls::serialize::impl_serialize;
pub fn generate(class: &Class) -> Result<ItemStruct, Error> {
    let class_name = &class.name;

    let fields = {
        let mut fields = Vec::new();
        for member in &class.members {
            fields.push(gen_field(member, class_name)?);
        }
        fields
    };

    let doc_attrs = struct_doc_attrs(class);
    let struct_name = format_ident!("{class_name}");
    let lifetime = match class.has_string {
        true => quote! { <'a> },
        false => quote! {},
    };

    let parent = class.parent.as_ref().map_or_else(
        || quote! {},
        |parent| {
            let parent_struct_name = format_ident!("{parent}");
            let lifetime = match class.parent_has_string {
                true => quote! { <'a> },
                false => quote! {},
            };
            quote! {
                /// Alternative to C++ class inheritance.
                #[cfg_attr(feature = "json_schema", schemars(flatten))]
                #[cfg_attr(feature = "serde", serde(flatten))]
                pub parent: #parent_struct_name #lifetime,
            }
        },
    );

    // `Default` implementations with huge sizes such as [0u8; 256] are not automatically supported, so use `educe` crate to define them.
    Ok(syn::parse_quote! {
        #doc_attrs
        #[allow(non_upper_case_globals, non_snake_case)]
        #[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(educe::Educe)]
        #[educe(Debug, Clone, Default, PartialEq)]
        pub struct #struct_name #lifetime {
            /// # Unique index for this class
            /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
            /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
            ///
            /// # Note
            /// Not present in the binary & Not exist actual C++ field.
            #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
            pub __ptr: Option<Pointer>,
            #parent
            #(#fields,)*
        }
    })
}

fn struct_doc_attrs(class: &Class) -> TokenStream {
    let Class {
        name,
        version,
        signature,
        size_x86,
        size_x86_64,
        vtable,
        ..
    } = class;

    let name = format!(" - name: `{name}`");
    let version = format!(" - version: `{version}`");
    let signature = format!(" - signature: `{signature}`");
    let class_sizes = format!(" - size: `{size_x86:3}`(x86)/`{size_x86_64:3}`(x86_64)");
    let vtable = format!(" -  vtable: `{vtable}`");

    quote! {
        /// # C++ Info
        #[doc = #name]
        #[doc = #version]
        #[doc = #signature]
        #[doc = #class_sizes]
        #[doc = #vtable]
    }
}
