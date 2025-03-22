#![allow(missing_docs, non_camel_case_types, non_snake_case, unused)]
// https://doc.rust-lang.org/stable/rustc/lints/listing/warn-by-default.html?highlight=abi#ambiguous-glob-reexports
#![allow(ambiguous_glob_reexports)]
#![allow(
    clippy::collapsible_match,
    clippy::enum_variant_names,
    clippy::match_overlapping_arm,
    clippy::redundant_static_lifetimes,
    clippy::needless_lifetimes
)]
//! If the file name is the same as the struct name representing each generated class,
//! a troublesome error will occur in namespace invocation, so a trailing `_` is added to avoid this.
//!
//! e.g. `hkClass` -> `struct hkClass` in `hkClass_.rs` file.
//!
//! # Note
//!  Note that the following enum and bitFlags have duplicate names.
//!
//! To use it correctly, you must import it from a module as follows.
//! - e.g. `havok_classes::hkbEvaluateHandleModifier_::HandleChangeMode`
//!
//! `enum HandleChangeMode`(`TYPE_ENUM`)
//! - `hkbEvaluateHandleModifier`
//! - `hkbHandIkControlData`
//!
//! `enum Flags`(`TYPE_FLAGS`)
//! - `hkbEventInfo`
//! - `hkMeshVertexBuffer`
//!
//! `enum FlagValues`(`TYPE_FLAGS`)
//! - `hkClass`
//! - `hkClassMember`
//!
//! `enum Type`(`TYPE_ENUM`)
//! - `hkClassMember`
//! - `hkResourceBase`
#[rustfmt::skip]
mod generated;
pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let class = hkAabbUint32::default();
        dbg!(hkColor_::ExtendedColors::SLATEGRAY);
        dbg!(class);
    }
}
