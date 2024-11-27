mod enum_or_flags;
mod structure;

use crate::{cpp_info::Class, ClassMap};
use syn::Result;

pub fn from_cpp_class(class: &Class, class_map: &ClassMap) -> Result<syn::File> {
    let class_code = {
        let struct_define = structure::generate(class)?;
        let impl_ser_for_struct = structure::impl_serialize(class, class_map);
        let impl_de_for_struct = structure::impl_deserialize(class, class_map)?;

        let enum_defines = enum_or_flags::generate(class);
        let impl_ser_for_enum_or_flags = enum_or_flags::impl_serialize(class);
        let impl_de_for_enum_or_flags = enum_or_flags::impl_deserialize(class);

        quote::quote! {
            use super::class_requires::*;
            use super::*;

            #struct_define
            #impl_ser_for_struct
            #impl_de_for_struct

            #(#enum_defines)*
            #(#impl_ser_for_enum_or_flags)*
            #(#impl_de_for_enum_or_flags)*
        }
    };

    syn::parse2(class_code)
}
