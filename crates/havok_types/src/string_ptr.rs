use crate::{lib::*, NULL_STR};

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringPtr<'a> {
    inner: Option<Cow<'a, str>>,
}

impl<'a> StringPtr<'a> {
    /// Create a new `StringPtr`
    #[inline]
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
    pub const fn get_ref(&self) -> &Option<Cow<'a, str>> {
        &self.inner
    }

    /// Cast [`str`] with non copying.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub const fn from_str(s: &'a str) -> Self {
        Self {
            inner: Some(Cow::Borrowed(s)),
        }
    }

    /// Inner to [`Self`]
    #[inline]
    pub const fn from_option(s: Option<Cow<'a, str>>) -> Self {
        Self { inner: s }
    }

    /// Null pointer or not?
    ///
    /// This indicates that no binary data was present.
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.get_ref().is_none()
    }

    /// Should the data pointed to by the pointer be written to the binary data or not?
    ///
    /// This is an invalid value or not.
    #[inline]
    pub const fn should_write_binary(&self) -> bool {
        self.get_ref().is_some()
    }
}

impl fmt::Display for StringPtr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.inner.as_ref().map_or(NULL_STR, |s| s.as_ref());
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "test";

    #[test]
    fn stringptr_is_null() {
        let string_ptr = StringPtr::new(None);
        assert!(string_ptr.is_null());
        let string_ptr_with_value = StringPtr::from_str(TEST_STR);
        assert!(!string_ptr_with_value.is_null());
    }

    #[test]
    fn stringptr_should_write_binary() {
        let string_ptr = StringPtr::from_str(TEST_STR);
        assert!(string_ptr.should_write_binary());

        let none_string_ptr = StringPtr::new(None);
        assert!(!none_string_ptr.should_write_binary());
    }

    #[test]
    fn stringptr_display() {
        let string_ptr = StringPtr::from_str(TEST_STR);
        assert_eq!(format!("{}", string_ptr), TEST_STR);

        let none_string_ptr = StringPtr::new(None);
        assert_eq!(format!("{}", none_string_ptr), NULL_STR);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn stringptr_serialize() {
        let string_ptr = StringPtr::from_str(TEST_STR);
        let json = serde_json::to_string(&string_ptr).unwrap();
        assert_eq!(json, format!("\"{}\"", TEST_STR));

        let none_string_ptr = StringPtr::new(None);
        let json = serde_json::to_string(&none_string_ptr).unwrap();
        assert_eq!(json, "null");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn stringptr_deserialize() {
        let json = format!("\"{}\"", TEST_STR);
        let string_ptr: StringPtr = serde_json::from_str(&json).unwrap();
        assert_eq!(string_ptr.get_ref().as_deref(), Some(TEST_STR));

        let none_json = "null";
        let none_string_ptr: StringPtr = serde_json::from_str(none_json).unwrap();
        assert!(none_string_ptr.is_null());
    }
}
