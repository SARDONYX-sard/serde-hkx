pub use havok_types_derive_proc_macros::impl_flags_methods;

#[doc(hidden)]
pub use ::parse_int; // Hack to make proc_macro depend on 3rd_party libraries so that users do not have to install those dependent libraries.
