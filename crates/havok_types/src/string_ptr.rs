use crate::lib::*;

/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
///
/// If it is null (substitute [`Option::None`] in Rust), it will not be written to the binary data.
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct StringPtr<'a> {
    inner: Option<Cow<'a, str>>,
}

impl<'a> StringPtr<'a> {
    /// Get inner value.
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.inner
    }

    /// Get inner ref.
    pub fn get_ref(&self) -> &Option<Cow<'a, str>> {
        &self.inner
    }

    /// Cast [`str`] with non copying.
    pub fn from_str(s: &'a str) -> StringPtr<'a> {
        Self {
            inner: Some(Cow::Borrowed(s)),
        }
    }
}

impl<'a> From<&'a str> for StringPtr<'a> {
    fn from(value: &'a str) -> Self {
        Self::from_str(value)
    }
}

impl<'a> From<Cow<'a, str>> for StringPtr<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self { inner: Some(value) }
    }
}

impl<'a> From<Option<Cow<'a, str>>> for StringPtr<'a> {
    fn from(value: Option<Cow<'a, str>>) -> Self {
        Self { inner: value }
    }
}
