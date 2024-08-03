use crate::{
    cpp_info::{Enum, EnumItem, TypeKind},
    rust_gen::enum_or_flags::has_duplicate_value,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_ser_for_enum(one_enum: &Enum) -> TokenStream {
    let Enum {
        name,
        vtype,
        vsubtype,
        enum_item,
        ..
    } = one_enum;

    let enum_name = syn::Ident::new(name, proc_macro2::Span::call_site());
    let variants: Vec<TokenStream> = enum_item
        .iter()
        .map(|enum_item| serialize_enum_variant(enum_item))
        .collect();

    if *vtype != TypeKind::Enum {
        panic!("Expected TYPE_ENUM but another type is mixed in. Got enum {name}(vtype: {vtype})")
    };

    if *vsubtype == TypeKind::Void {
        tracing::info!("Skip automatic enum generation because this enum {name} is a void storage type, indicating that it is not used.");
        return quote! {};
    };

    // An enum with the same value is not valid as an enum in Rust.
    // Therefore, express them as BitFlag.
    let serialize_inner = if has_duplicate_value(one_enum) {
        // NOTE: In the case of bit flags, since self is a reference, it is necessary to add deref.
        let err_msg = format!("The enum {name} contains an unknown value ({{}}).");
        quote! {
            match *self {
                #(#variants,)*
                unknown => return Err(S::Error::custom(format!(#err_msg, unknown.bits())))
            }?;
            __serializer.serialize_bits(&self.bits())?;
        }
    } else {
        let cast_method = to_rust_cast_method(vsubtype)
            .unwrap_or_else(|| panic!("Unsupported enum storage type: {vsubtype}"));
        let err_msg = format!("Failed enum {enum_name} {cast_method}");
        quote! {
            match self {
                #(#variants,)*
            }?;

            use num_traits::ToPrimitive as _;
            let num = self.#cast_method().ok_or(S::Error::custom(#err_msg))?;
            __serializer.serialize_bits(&num)?;
        }
    };

    quote! {
        const _: () = {
            use havok_serde as __serde;

            impl __serde::Serialize for #enum_name {
                fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
                    where S: __serde::ser::Serializer
                {
                    let mut __serializer = __serializer.serialize_enum_flags()?;
                    #serialize_inner
                    __serializer.end()
                }
        }
        };
    }
}

fn serialize_enum_variant(one_enum: &EnumItem) -> TokenStream {
    let EnumItem { name, value } = one_enum;
    let variant = syn::Ident::new(name, proc_macro2::Span::call_site());
    let value = (*value) as u64; // NOTE: This method is for XML and does not care about size.
    quote! {
        Self::#variant => __serializer.serialize_field(#name, &#value)
    }
}

/// Returns the name of the method you get when you derive ToPrimitive.
/// Return [`None`] except the following types.
/// - "TYPE_INT8", "TYPE_UINT8",
/// - "TYPE_INT16", "TYPE_UINT16",
/// - "TYPE_INT32", "TYPE_UINT32",
/// - "TYPE_INT64", "TYPE_UINT64"
fn to_rust_cast_method(ty: &TypeKind) -> Option<TokenStream> {
    Some(match ty {
        TypeKind::Int8 => quote!(to_i8),
        TypeKind::Uint8 => quote!(to_u8),
        TypeKind::Int16 => quote!(to_i16),
        TypeKind::Uint16 => quote!(to_u16),
        TypeKind::Int32 => quote!(to_i32),
        TypeKind::Uint32 => quote!(to_u32),
        TypeKind::Int64 => quote!(to_i64),
        TypeKind::Uint64 => quote!(to_u64),
        _ => return None,
    })
}
