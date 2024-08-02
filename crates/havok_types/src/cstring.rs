use crate::{lib::*, StringPtr, NULL_STR};

/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
///
/// If it is null (substitute [`Option::None`] in Rust), an empty string or `\u{2400}`,
/// it will not be written to the binary data.
///
/// # Deserialization patterns
/// - hkx(`Vec<u8>`)   -> Struct([`str`] in `Cow<'_, str>`) => non copy
/// - xml([`String`])  -> Struct([`str`] in `Cow<'_, str>`) => non copy
/// - json: [`String`] -> Struct([`str`] in `Cow<'_, str>`) => non copy
///
/// # Serialization is alloc
/// Struct([`str`]) -> (alloc [`String`])
///
/// [`str`]: https://doc.rust-lang.org/stable/core/ffi/c_str/struct.CStr.html
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CString<'a> {
    inner: Option<Cow<'a, str>>,
}

impl<'a> CString<'a> {
    /// Create a new `CString`
    pub const fn new(inner: Option<Cow<'a, str>>) -> Self {
        Self { inner }
    }

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
    #[allow(clippy::should_implement_trait)]
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
            Some(s) => {
                if s.is_empty() || s == NULL_STR {
                    return false;
                };
                true
            }
            _ => false,
        }
    }
}

impl fmt::Display for CString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.inner.as_ref().map(|s| s.as_ref()).unwrap_or(NULL_STR);
        write!(f, "{s}")
    }
}

impl<'a> From<&'a str> for CString<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::from_str(value)
    }
}

impl<'a> From<CString<'a>> for StringPtr<'a> {
    #[inline]
    fn from(value: CString<'a>) -> Self {
        Self::from_option(value.into_inner())
    }
}
