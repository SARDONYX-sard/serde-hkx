// SPDX-License-Identifier: MIT
//! HexDump Display(For binary)/XML human-readable error message
//! This code is a fork of winnow's docs.
//!
//! # Ref
//! - [MIT License](https://github.com/winnow-rs/winnow/blob/v0.7.10/LICENSE-MIT)
//! - [Code](https://github.com/winnow-rs/winnow/blob/v0.7.10/src/error.rs#L1316)
mod combinator;
mod readable_error;
mod space;

pub use self::combinator::take_until_ext;
pub use self::readable_error::ReadableError;
pub use self::space::delimited_multispace0;
