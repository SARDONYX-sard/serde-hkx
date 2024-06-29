mod enums;
mod flags;
mod impls;

use self::enums::gen_enum;
use self::flags::gen_flag;
use crate::cpp_info::{Class, Enum, TypeKind};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;

pub use impls::serialize::impl_serialize;
pub fn generate(class: &Class) -> Vec<TokenStream> {
    let mut enums = Vec::new();
    for one_enum in &class.enums {
        let Enum {
            name,
            vtype,
            vsubtype,
            ..
        } = one_enum;

        if *vsubtype == TypeKind::Void {
            tracing::info!("Skip automatic enum generation because this enum {name} is a void storage type, indicating that it is not used.");
            continue;
        };

        match vtype {
            TypeKind::Flags => {
                let flag_struct = gen_flag(one_enum);
                enums.push(quote! { #flag_struct });
            }
            TypeKind::Enum => {
                // An enum with the same value is not valid as an enum in Rust. Therefore, express them as BitFlag
                let item_enum = if has_duplicate_value(one_enum) {
                    let flag_struct = gen_flag(one_enum);
                    quote! { #flag_struct }
                } else {
                    let item_enum = gen_enum(one_enum);
                    quote! { #item_enum }
                };
                enums.push(quote! { #item_enum });
            }
            _ => panic!(
                "Expected TYPE_ENUM|TYPE_FLAGS, but another type is mixed in. Got enum {name}(vtype: {vtype})"
            ),
        };
    }

    enums
}

/// An enum with the same value is not valid as an enum in Rust.
///
/// Therefore, express them as BitFlag, but note that implementations such as `Serialize` must be handled as an enum.
pub(super) fn has_duplicate_value(one_enum: &Enum) -> bool {
    let mut values = HashSet::new();
    for enum_item in &one_enum.enum_item {
        let is_contain = values.insert(enum_item.value);
        if !is_contain {
            return true; // Duplicate found
        }
    }
    false // No duplicates
}
