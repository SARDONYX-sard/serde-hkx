use crate::{NULL_STR, lib::*};

/// # StringPtr
///
/// # C++ Info
/// - name: `hkStringPtr`
/// - type_size: ` 4`(x86)/` 8`(x86_64)
/// - align: ` 4`(x86)/` 8`(x86_64)
///
/// # Null representation
/// - hkx: It will not be written to the binary data.
/// - XML: `\u{2400}`.
/// - Rust: If it is null then [`Option::None`].(To eliminate the risk of always being null ptr by type)
///
/// # Deserialization patterns
/// - hkx(`Vec<u8>`)   -> Struct([`str`] in `Cow<'_, str>`) => non copy
/// - xml([`String`])  -> Struct([`str`] in `Cow<'_, str>`) => non copy
/// - json: [`String`] -> Struct([`str`] in `Cow<'_, str>`) => non copy
///
/// # Serialization is alloc
/// Struct([`str`]) -> (alloc [`String`])
///
/// [`str`]: https://doc.rust-lang.org/std/primitive.str.html
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "json_schema", schemars(transparent))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringPtr<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
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

    /// Gets a reference as [`&str`].
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_ref().map_or(NULL_STR, |s| s.as_ref())
    }
}

impl fmt::Display for StringPtr<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PartialEq<str> for StringPtr<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for StringPtr<'_> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<String> for StringPtr<'_> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
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
        assert_eq!(string_ptr, TEST_STR);

        let none_string_ptr = StringPtr::new(None);
        assert_eq!(none_string_ptr, NULL_STR);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn stringptr_serialize() {
        let string_ptr = StringPtr::from_str(TEST_STR);
        let json = serde_json::to_string(&string_ptr).unwrap();
        assert_eq!(json, format!("\"{TEST_STR}\""));

        let none_string_ptr = StringPtr::new(None);
        let json = serde_json::to_string(&none_string_ptr).unwrap();
        assert_eq!(json, "null");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn stringptr_deserialize() {
        let json = format!("\"{TEST_STR}\"");
        let string_ptr: StringPtr = serde_json::from_str(&json).unwrap();
        assert_eq!(string_ptr.get_ref().as_deref(), Some(TEST_STR));

        let none_json = "null";
        let none_string_ptr: StringPtr = serde_json::from_str(none_json).unwrap();
        assert!(none_string_ptr.is_null());
    }
}
