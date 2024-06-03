use crate::{lib::*, StringPtr};

/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
///
/// If it is null (substitute [`Option::None`] in Rust), an empty string or `\u{2400}`,
/// it will not be written to the binary data.
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct CString<'a> {
    inner: Option<Cow<'a, str>>,
}

impl<'a> CString<'a> {
    /// Get inner value.
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.inner
    }

    /// Get inner ref.
    pub fn get_ref(&self) -> &Option<Cow<'a, str>> {
        &self.inner
    }

    /// Cast [`str`] with non copying.
    pub fn from_str(s: &'a str) -> CString<'a> {
        Self {
            inner: Some(Cow::Borrowed(s)),
        }
    }
}

impl<'a> From<&'a str> for CString<'a> {
    fn from(value: &'a str) -> Self {
        Self::from_str(value)
    }
}

impl<'a> From<CString<'a>> for StringPtr<'a> {
    fn from(value: CString<'a>) -> Self {
        value.into_inner().into()
    }
}
