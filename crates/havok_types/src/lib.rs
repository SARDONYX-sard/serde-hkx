//! All havok types definitions.
pub mod cstring;
pub mod half;
pub mod math;
pub mod pointer;
pub mod primitive;
pub mod signature;
pub mod string_ptr;
pub mod ulong;
pub mod variant;

pub use cstring::*;
pub use half::f16;
pub use math::*;
pub use pointer::*;
pub use primitive::*;
pub use signature::*;
pub use string_ptr::*;
pub use ulong::*;
pub use variant::*;

mod lib {
    pub use std::borrow::Cow;

    pub use core::fmt;
    pub use core::fmt::Debug;
    pub use core::str::FromStr;
}

/// Unicode null
pub const NULL_STR: &str = "\u{2400}";
