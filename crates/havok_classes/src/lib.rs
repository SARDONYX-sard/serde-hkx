#![allow(non_camel_case_types, non_snake_case, unused)]
// https://doc.rust-lang.org/stable/rustc/lints/listing/warn-by-default.html?highlight=abi#ambiguous-glob-reexports
#![allow(ambiguous_glob_reexports)]
//! If the file name is the same as the struct name representing each generated class,
//! a troublesome error will occur in namespace invocation, so a trailing `_` is added to avoid this.
//!
//! e.g. `hkClass` -> `struct hkClass` in `hkClass_.rs` file.
//!
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

pub mod generated;
use generated::*;

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let class = crate::BGSGamebryoSequenceGenerator::default();
        dbg!(class);
    }
}