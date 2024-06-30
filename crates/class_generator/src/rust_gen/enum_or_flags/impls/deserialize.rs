use crate::cpp_info::{Class, TypeKind};
use crate::rust_gen::enum_or_flags::{
    enums::impls::deserialize::impl_de_for_enum, flags::impls::deserialize::impl_de_for_flag,
};
use proc_macro2::TokenStream;

pub fn impl_deserialize(class: &Class) -> Vec<TokenStream> {
    let mut enums = Vec::new();
    for one_enum in &class.enums {
        if one_enum.vsubtype == TypeKind::Void {
            tracing::info!("Skip automatic enum generation because this enum {} is a void storage type, indicating that it is not used.",
                one_enum.name
        );
            continue;
        };

        if one_enum.vtype == TypeKind::Flags {
            enums.push(impl_de_for_flag(&one_enum));
        } else if one_enum.vtype == TypeKind::Enum {
            enums.push(impl_de_for_enum(&one_enum));
        } else {
            panic!(
                "Expected TYPE_ENUM but another type is mixed in. Got enum {}(vtype: {})",
                one_enum.name, one_enum.vtype,
            );
        };
    }

    enums
}
