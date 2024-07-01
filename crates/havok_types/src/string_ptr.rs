use crate::lib::*;

/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
///
/// If it is null (substitute [`Option::None`] in Rust), it will not be written to the binary data.
///
/// # Deserialization alloc patterns
/// - hkx(`Vec<u8>` -> [`CStr`]) -> Struct(alloc [`String`]) => Need copy
/// - xml([`String`]) -> Struct([`str`])                     => non copy
/// - json: [`String`] -> Struct([`str`])                    => non copy
///
/// [`CStr`]: https://doc.rust-lang.org/stable/core/ffi/c_str/struct.CStr.html
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct StringPtr<'a> {
    inner: Option<Cow<'a, str>>,
}

impl<'a> StringPtr<'a> {
    /// Get inner value.
    #[inline]
    pub fn into_inner(self) -> Option<Cow<'a, str>> {
        self.inner
    }

    /// Get inner ref.
    #[inline]
    pub fn get_ref(&self) -> &Option<Cow<'a, str>> {
        &self.inner
    }

    /// Cast [`str`] with non copying.
    #[inline]
    pub fn from_str(s: &'a str) -> Self {
        Self {
            inner: Some(Cow::Borrowed(s)),
        }
    }

    /// Inner to [`Self`]
    #[inline]
    pub fn from_option(s: Option<Cow<'a, str>>) -> Self {
        Self { inner: s }
    }

    /// Null pointer or not?
    ///
    /// This indicates that no binary data was present.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.get_ref().is_none()
    }

    /// Should the data pointed to by the pointer be written to the binary data or not?
    ///
    /// This is an invalid value or not.
    #[inline]
    pub fn should_write_binary(&self) -> bool {
        match self.get_ref() {
            Some(_) => return true,
            _ => false,
        }
    }
}

impl fmt::Display for StringPtr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.inner.as_ref().map(|s| s.as_ref()).unwrap_or("");
        write!(f, "{s}")
    }
}

impl<'a> From<&'a str> for StringPtr<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::from_str(s)
    }
}

impl<'a> From<Cow<'a, str>> for StringPtr<'a> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        Self { inner: Some(value) }
    }
}

impl<'a> From<Option<Cow<'a, str>>> for StringPtr<'a> {
    #[inline]
    fn from(inner: Option<Cow<'a, str>>) -> Self {
        Self { inner }
    }
}
