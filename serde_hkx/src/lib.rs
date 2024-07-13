pub mod bytes;
pub mod errors;
pub mod xml;

#[cfg(test)]
pub mod mocks;

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
    pub use self::core::ops::AddAssign;
    pub use self::core::ops::MulAssign;
    pub use self::core::ops::Range;
    pub use self::core::str;

    pub use self::core::fmt;
    pub use self::core::fmt::Display;
    pub use self::core::str::FromStr;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::{String, ToString};
}

/// Avoiding type inference by the compiler, such as `?` and `into`, can speed up compile
/// time by a small amount, so use macros to avoid type inference.
///
/// # Info
/// This is a hack I learned at serde.
macro_rules! tri {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err),
        }
    };
}
pub(crate) use tri;

pub use bytes::de::{from_bytes, from_bytes_file};
pub use bytes::ser::to_bytes;
pub use xml::de::{from_str, from_str_file};
pub use xml::ser::to_string;
