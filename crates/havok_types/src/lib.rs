//! All havok types definitions.
pub mod error;
pub mod str_parser;

pub mod cstring;
pub mod math;
pub mod pointer;
pub mod signature;
pub mod string_ptr;
pub mod variant;
#[doc(hidden)]
pub use half;

pub use cstring::*;
pub use half::f16;
pub use math::*;
pub use pointer::*;
pub use signature::*;
pub use string_ptr::*;
pub use variant::*;

mod lib {
    pub use std::borrow::Cow;

    pub use core::fmt;
    pub use core::str::from_utf8_unchecked;
    pub use core::str::FromStr;
}

/// Try parse to error
macro_rules! tri {
    ($parser:expr) => {
        match $parser {
            Ok(res) => res,
            Err(err) => {
                return Err(crate::error::ParseSnafu {
                    reason: err.to_string(),
                }
                .build());
            }
        }
    };
}

pub(crate) use tri;
