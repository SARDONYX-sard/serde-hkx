mod field;
mod impl_serialize;
mod to_rust_token;

use self::field::gen_field;
use crate::cpp_info::Class;
use proc_macro2::TokenStream;
use quote::quote;

pub use self::impl_serialize::impl_serialize;
pub fn generate(class: &Class) -> syn::ItemStruct {
    let class_name = &class.name;
    let fields: Vec<TokenStream> = class
        .members
        .iter()
        .map(|member| gen_field(member, class_name))
        .collect();

    let doc_attrs = struct_doc_attrs(class);
    let struct_name = syn::Ident::new(class_name, proc_macro2::Span::call_site());
    let lifetime = match class.has_string {
        true => quote! { <'a> },
        false => quote! {},
    };

    let parent = match &class.parent {
        Some(parent) => {
            let parent_struct_name = syn::Ident::new(parent, proc_macro2::Span::call_site());
            let lifetime = match class.parent_has_string {
                true => quote! { <'a> },
                false => quote! {},
            };
            quote! {
                /// Alternative to C++ class inheritance.
                pub parent: #parent_struct_name #lifetime,
            }
        }
        None => quote! {},
    };

    syn::parse_quote! {
        #doc_attrs
        #[allow(non_upper_case_globals, non_snake_case)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct #struct_name #lifetime {
            /// - Represents a pointer on XML (`<hkobject name="#0001">`)
            /// - [`Option::None`] => This class is `class in field`.(`<hkobject>`)
            ///
            /// # Note
            /// Not present in the binary or actual C++ field
            pub __ptr_name_attr: Option<Pointer>,
            #parent
            #(#fields,)*
        }
    }
}

#[rustfmt::skip]
fn struct_doc_attrs(class: &Class) -> TokenStream {
    let Class {
        name,
        version,
        signature,
        size_x86,
        size_x86_64,
        parent,
        vtable,
        ..

    } = class;

    let name =             format!(" -            name: `{name}`");
    let version =          format!(" -         version: `{version}`");
    let signature =        format!(" -       signature: `{signature}`");
    let class_size =       format!(" -          size: {size_x86:3}(x86)/{size_x86_64:3}(x86_64)");
    let parent= {
        match parent{
          Some(parent) => format!(" -          parent: `{parent}`"),
                              _ => format!(" -          parent: `None`"),
        }
    };
    let vtable =           format!(" -          vtable: {vtable}");

    quote! {
        /// # C++ Info
        #[doc = #name]
        #[doc = #version]
        #[doc = #signature]
        #[doc = #class_size]
        #[doc = #parent]
        #[doc = #vtable]
        #[doc = ""]
    }
}
