pub mod math;
pub mod pointer;
pub mod signature;
pub mod variant;

#[doc(hidden)]
pub use half;

pub use half::f16;
pub use math::*;
pub use pointer::*;
pub use signature::*;
pub use variant::*;

use std::borrow::Cow;
/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
/// - If frontend: `&str`
///
/// # Alloc patterns
/// If `StringPtr` is `&str`
/// - hkx: [`CStr`] -> xml:  [`str`] => Need alloc: [`String`]
/// - xml:  [`str`] -> hkx: [`CStr`] => Need alloc: [`CString`]
/// ---
/// -  xml: [`str`] -> json: [`str`] => [`str`]
/// - json: [`str`] ->  xml: [`str`] => [`str`]
///
/// [`CStr`]: https://doc.rust-lang.org/stable/core/ffi/c_str/struct.CStr.html
/// [`CString`]: https://doc.rust-lang.org/stable/alloc/ffi/c_str/struct.CString.html
pub type StringPtr<'a> = Cow<'a, str>;

/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
/// - If frontend: `&str`
///
/// # Alloc patterns
/// - hkx: [`CStr`] -> xml:  [`str`] => Need alloc: [`String`]
/// - xml:  [`str`] -> hkx: [`CStr`] => Need alloc: [`CString`]
/// ---
/// -  xml: [`str`] -> json: [`str`] => [`str`]
/// - json: [`str`] ->  xml: [`str`] => [`str`]
///
/// [`CStr`]: https://doc.rust-lang.org/stable/core/ffi/c_str/struct.CStr.html
/// [`CString`]: https://doc.rust-lang.org/stable/alloc/ffi/c_str/struct.CString.html
pub type CString<'a> = Cow<'a, str>;
