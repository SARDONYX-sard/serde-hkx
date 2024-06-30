// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// The following code was written by modifying serde ver. 1.0.202.
// See: https://github.com/serde-rs/serde/commit/58b3af4c2915c3ae789778a11f3b7a468c1cec17
//
// And serde holds the same license as Rust. https://github.com/rust-lang/rust/pull/43498
//
// The default implementation does not fully express Havok's special XML format.
//
// # Modification details
// - Rust std types -> Havok Types
// - Changed serde method to Havok XML& binary data signatures, which are easier to modify.
//! Forked serde to serialize and deserialize Havok Class.

// Restrictions
#![deny(clippy::question_mark_used)] // Speed up compilation by eliminating inference with `?`. Use instead of `tri!`
// Rustc lints.
#![deny(missing_docs, unused_imports)]

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
extern crate alloc;

/// A facade around all the types we need from the `std`, `core`, and `alloc`
/// crates. This avoids elaborate import wrangling having to happen in every
/// module.
mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::f32;
    pub use self::core::str;
    pub use self::core::{i16, i32, i64, i8};
    pub use self::core::{u16, u32, u64, u8, usize};

    pub use self::core::clone;
    pub use self::core::convert;
    pub use self::core::default;
    pub use self::core::marker;
    pub use self::core::option;
    pub use self::core::ptr;
    pub use self::core::result;
    pub use self::core::slice;

    pub use self::core::cmp;
    pub use self::core::fmt;
    pub use self::core::fmt::Display;
    pub use self::core::marker::PhantomData;
    pub use self::core::mem;
    pub use self::core::mem::MaybeUninit;

    #[cfg(not(feature = "std"))]
    pub use core::ffi::CStr;
    #[cfg(feature = "std")]
    pub use std::ffi::CStr;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;
}

// None of this crate's error handling needs the `From::from` error conversion
// performed implicitly by the `?` operator or the standard library's `try!`
// macro. This simplified macro gives a 5.5% improvement in compile time
// compared to standard `try!`, and 9% improvement compared to `?`.
macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err),
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

pub mod de;
pub mod ser;

// Used by generated code and doc tests. Not public API.(For derive)
#[doc(hidden)]
#[path = "private/mod.rs"]
pub mod __private;

pub use de::{Deserialize, Deserializer};
pub use ser::Serialize;

use havok_types::Signature;
use lib::*;

/// Trait whether it is Havok Class or not.
///
/// # Purpose
/// This tray exists for the following purposes.
/// - Writing `__classnames__` sections when creating binary data.
/// - (De)Serialization process for array classes.
pub trait HavokClass {
    /// Get Class name.
    fn name(&self) -> &'static CStr;
    /// Get signature.
    fn signature(&self) -> Signature;
}
