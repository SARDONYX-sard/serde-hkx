mod enums;
mod flags;
mod impls;

use self::enums::gen_enum;
use self::flags::gen_flag;
use crate::cpp_info::{Class, Enum, TypeKind};
use proc_macro2::TokenStream;
use quote::quote;

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
                let flag_struct = gen_flag(&one_enum);
                enums.push(quote! { #flag_struct });
            }
            TypeKind::Enum => {
                let item_enum = gen_enum(&one_enum);
                enums.push(quote! { #item_enum });
            }
            _ => panic!(
                "Expected TYPE_ENUM but another type is mixed in. Got enum {name}(vtype: {vtype})"
            ),
        };
    }

    enums
}
