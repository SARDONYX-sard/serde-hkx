#![allow(non_camel_case_types, non_snake_case, unused)]
// https://doc.rust-lang.org/stable/rustc/lints/listing/warn-by-default.html?highlight=abi#ambiguous-glob-reexports
#![allow(ambiguous_glob_reexports)]
//! # Note
//!  Note that the following enum and bitFlags have duplicate names.
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

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let class = crate::BGSGamebryoSequenceGenerator::default();
        dbg!(class);
    }
}
