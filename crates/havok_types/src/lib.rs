pub mod math;
pub mod pointer;
pub mod signature;

pub use half;
pub use half::f16;
pub use math::*;
pub use pointer::*;
pub use signature::*;

use std::borrow::Cow;
// use std::ffi::CStr;

/// - binary data(.hkx): null-terminated string
/// - XML: &str
/// - If frontend: &str
///
/// # Alloc patterns
/// If StringPtr is &str
/// - hkx(&'de CStr) ->  xml(&'de str) => String(need alloc)
/// -  xml(&'de str) -> hkx(&'de CStr) => CString(need alloc)
/// -  xml(&'de str) -> json(&'de str) => &'de str
/// - json(&'de str) ->  xml(&'de str) => &'de str
pub type StringPtr<'a> = Cow<'a, str>;
pub type CString<'a> = Cow<'a, str>;
