use crate::{lib::*, StringPtr};

/// - binary data(.hkx): null-terminated string
/// - XML: `&str`
///
/// If it is null (substitute [`Option::None`] in Rust), an empty string or `\u{2400}`,
/// it will not be written to the binary data.
///
/// # Deserialization alloc patterns
/// - hkx(`Vec<u8>` -> [`CStr`]) -> Struct(alloc [`String`]) => Need copy
/// - xml([`String`]) -> Struct([`str`])                     => non copy
/// - json: [`String`] -> Struct([`str`])                    => non copy
///
/// [`CStr`]: https://doc.rust-lang.org/stable/core/ffi/c_str/struct.CStr.html
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_new::new)]
pub struct CString<'a> {
    inner: Option<Cow<'a, str>>,
}

impl<'a> CString<'a> {
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
    pub fn from_str(s: &'a str) -> CString<'a> {
        Self {
            inner: Some(Cow::Borrowed(s)),
        }
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
                if s.is_empty() || s == "\u{2400}" {
                    return false;
                };
                return true;
            }
            _ => false,
        }
    }
}

impl fmt::Display for CString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.inner.as_ref() {
            Some(s) => unsafe { from_utf8_unchecked(s.as_bytes()) }, // Safety: str is always utf8
            None => "",
        };
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
