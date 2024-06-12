// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// The following code was written by modifying serde ver. 1.0.202.
// See: https://github.com/serde-rs/serde/commit/58b3af4c2915c3ae789778a11f3b7a468c1cec17
//
// And serde holds the same license as Rust. https://github.com/rust-lang/rust/pull/43498
//
// The default implementation does not fully express Havok's special XML format.
//
// # Modification details
// - Rust std types -> Havok Types
// - Changed serde method to Havok XML& binary data signatures, which are easier to modify.
//! Deserialization

mod impls;

use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, StringPtr,
    Transform, Variant, Vector4,
};

use crate::lib::*;

#[cfg(feature = "std")]
#[doc(no_inline)]
pub use std::error::Error as StdError;

////////////////////////////////////////////////////////////////////////////////

macro_rules! declare_error_trait {
    (Error: Sized $(+ $($supertrait:ident)::+)*) => {
        /// The `Error` trait allows `Deserialize` implementations to create descriptive
        /// error messages belonging to the `Deserializer` against which they are
        /// currently running.
        ///
        /// Every `Deserializer` declares an `Error` type that encompasses both
        /// general-purpose deserialization errors as well as errors specific to the
        /// particular deserialization format. For example the `Error` type of
        /// `serde_json` can represent errors like an invalid JSON escape sequence or an
        /// unterminated string literal, in addition to the error cases that are part of
        /// this trait.
        ///
        /// Most deserializers should only need to provide the `Error::custom` method
        /// and inherit the default behavior for the other methods.
        ///
        /// # Example implementation
        ///
        /// The [example data format] presented on the website shows an error
        /// type appropriate for a basic JSON data format.
        ///
        /// [example data format]: https://serde.rs/data-format.html
        pub trait Error: Sized $(+ $($supertrait)::+)* {
            /// Raised when there is general error when deserializing a type.
            ///
            /// The message should not be capitalized and should not end with a period.
            ///
            /// ```edition2021
            /// # use std::str::FromStr;
            /// #
            /// # struct IpAddr;
            /// #
            /// # impl FromStr for IpAddr {
            /// #     type Err = String;
            /// #
            /// #     fn from_str(_: &str) -> Result<Self, String> {
            /// #         unimplemented!()
            /// #     }
            /// # }
            /// #
            /// use serde::de::{self, Deserialize, Deserializer};
            ///
            /// impl<'de> Deserialize<'de> for IpAddr {
            ///     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            ///     where
            ///         D: Deserializer<'de>,
            ///     {
            ///         let s = String::deserialize(deserializer)?;
            ///         s.parse().map_err(de::Error::custom)
            ///     }
            /// }
            /// ```
            fn custom<T>(msg: T) -> Self
            where
                T: Display;

            /// Raised when a `Deserialize` receives a type different from what it was
            /// expecting.
            ///
            /// The `unexp` argument provides information about what type was received.
            /// This is the type that was present in the input file or other source data
            /// of the Deserializer.
            ///
            /// The `exp` argument provides information about what type was being
            /// expected. This is the type that is written in the program.
            ///
            /// For example if we try to deserialize a String out of a JSON file
            /// containing an integer, the unexpected type is the integer and the
            /// expected type is the string.
            #[cold]
            fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
                Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
            }

            /// Raised when a `Deserialize` receives a value of the right type but that
            /// is wrong for some other reason.
            ///
            /// The `unexp` argument provides information about what value was received.
            /// This is the value that was present in the input file or other source
            /// data of the Deserializer.
            ///
            /// The `exp` argument provides information about what value was being
            /// expected. This is the type that is written in the program.
            ///
            /// For example if we try to deserialize a String out of some binary data
            /// that is not valid UTF-8, the unexpected value is the bytes and the
            /// expected value is a string.
            #[cold]
            fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
                Error::custom(format_args!("invalid value: {}, expected {}", unexp, exp))
            }

            /// Raised when deserializing a sequence or map and the input data contains
            /// too many or too few elements.
            ///
            /// The `len` argument is the number of elements encountered. The sequence
            /// or map may have expected more arguments or fewer arguments.
            ///
            /// The `exp` argument provides information about what data was being
            /// expected. For example `exp` might say that a tuple of size 6 was
            /// expected.
            #[cold]
            fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
                Error::custom(format_args!("invalid length {}, expected {}", len, exp))
            }

            /// Raised when a `Deserialize` enum type received a variant with an
            /// unrecognized name.
            #[cold]
            fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
                if expected.is_empty() {
                    Error::custom(format_args!(
                        "unknown variant `{}`, there are no variants",
                        variant
                    ))
                } else {
                    Error::custom(format_args!(
                        "unknown variant `{}`, expected {}",
                        variant,
                        OneOf { names: expected }
                    ))
                }
            }

            /// Raised when a `Deserialize` struct type received a field with an
            /// unrecognized name.
            #[cold]
            fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
                if expected.is_empty() {
                    Error::custom(format_args!(
                        "unknown field `{}`, there are no fields",
                        field
                    ))
                } else {
                    Error::custom(format_args!(
                        "unknown field `{}`, expected {}",
                        field,
                        OneOf { names: expected }
                    ))
                }
            }

            /// Raised when a `Deserialize` struct type expected to receive a required
            /// field with a particular name but that field was not present in the
            /// input.
            #[cold]
            fn missing_field(field: &'static str) -> Self {
                Error::custom(format_args!("missing field `{}`", field))
            }

            /// Raised when a `Deserialize` struct type received more than one of the
            /// same field.
            #[cold]
            fn duplicate_field(field: &'static str) -> Self {
                Error::custom(format_args!("duplicate field `{}`", field))
            }
        }
    }
}

#[cfg(feature = "std")]
declare_error_trait!(Error: Sized + StdError);

#[cfg(not(feature = "std"))]
declare_error_trait!(Error: Sized + Debug + Display);

/// `Unexpected` represents an unexpected invocation of any one of the `Visitor`
/// trait methods.
///
/// This is used as an argument to the `invalid_type`, `invalid_value`, and
/// `invalid_length` methods of the `Error` trait to build error messages.
///
/// ```edition2021
/// # use std::fmt;
/// #
/// # use serde::de::{self, Unexpected, Visitor};
/// #
/// # struct Example;
/// #
/// # impl<'de> Visitor<'de> for Example {
/// #     type Value = ();
/// #
/// #     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
/// #         write!(formatter, "definitely not a boolean")
/// #     }
/// #
/// fn visit_bool<E><V>(self, visitor: V) -> Result<Self::Value, E>
/// where
///     E: de::Error,
/// {
///     Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Debug)]
pub enum Unexpected<'a> {
    /// No type information.
    ///
    /// - C++ type: `void`
    ///
    /// # Examples
    ///
    /// This is often used to fill in generics elements with types for which generics are not used.
    /// - `hkArray<hkBool>` -> `vtype`: `TYPE_ARRAY`, `vsubtype`: `TYPE_BOOL`
    /// - `hkBool` -> `vtype`: `TYPE_BOOL`, `vsubtype`: `TYPE_VOID`
    /// - There is also a pattern `hkArray<void>`. The type information is unknown, but this member always contains the `SERIALIZE_IGNORED` flag and can be skipped.
    Void,

    /// - C++ type: `hkBool` (`bool`)
    Bool(bool),

    /// - C++ type: `hkChar` (`signed char`)
    Char(char),

    /// - C++ type: `hkInt8` (`signed char`)
    Int8(i8),

    /// - C++ type: `hkUint8` (`unsigned char`)
    Uint8(u8),

    /// - C++ type: `hkInt16` (`signed short`)
    Int16(i16),

    /// - C++ type: `hkUint16` (`unsigned short`)
    Uint16(u16),

    /// - C++ type: `hkInt32` (`signed int`)
    Int32(i32),

    /// - C++ type: `hkUint32` (`unsigned int`)
    Uint32(u32),

    /// - C++ type: `hkInt64` (`signed long long`)
    Int64(i64),

    /// - C++ type: `hkUint64` (`unsigned long long`)
    Uint64(u64),

    /// - C++ type: `hkReal` (`float`)
    Real(f32),

    /// - C++ type: `hkVector4`
    Vector4(Vector4),

    /// - C++ type: `hkQuaternion`
    Quaternion(Quaternion),

    /// - C++ type: `hkMatrix3`
    Matrix3(Matrix3),

    /// - C++ type: `hkRotation`
    Rotation(Rotation),

    /// - C++ type: `hkQsTransform`
    QsTransform(QsTransform),

    /// - C++ type: `hkMatrix4`
    Matrix4(Matrix4),

    /// - C++ type: `hkTransform`
    Transform(Transform),

    /// - C++ type: `T*`
    Pointer(Pointer),

    /// Array of items of type T.
    /// - C++ type: `hkArray<T>`
    Array,

    /// enum type that stores only the size of `SizeType` in memory.
    /// - C++ type: `hkEnum<Enum,SizeType>`
    Enum,

    /// - C++ type: `class` | `struct`
    Struct,

    /// - C++ type: `hkVariant` (void* and hkClass*) type
    Variant(Variant),

    /// Null terminated string.
    /// - C++ type: `char*`
    CString(CString<'a>),

    /// - C++ type: `hkUlong` (`unsigned long`), defined to always be the same size as a pointer
    Ulong(u64),

    /// - C++ type: `hkFlags<ENUM, SizeType>` - 8,16,32 bits of named values.
    Flags,

    /// - C++ type: `hkHalf` (`hkInt16`), 16-bit float value
    Half(f16),

    /// Null-terminated string type.
    ///
    /// There is a flag `StringFlags::OWNED_FLAG = 0x1` defined in the class, so `Owned` is also possible.
    ///
    /// It is unclear which segment (stack, heap, or other) is being pointed to because of the raw pointer.
    /// - C++ type: `hkStringPtr`
    StringPtr(StringPtr<'a>),

    /// Other types
    Other(&'a str),
}

impl<'a> fmt::Display for Unexpected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use self::Unexpected::*;
        match *self {
            Void => write!(formatter, "void"),
            Bool(b) => write!(formatter, "boolean `{}`", b),
            Char(c) => write!(formatter, "character `{}`", c),
            Int8(i) => write!(formatter, "8-bit integer `{}`", i),
            Uint8(i) => write!(formatter, "8-bit unsigned integer `{}`", i),
            Int16(i) => write!(formatter, "16-bit integer `{}`", i),
            Uint16(i) => write!(formatter, "16-bit unsigned integer `{}`", i),
            Int32(i) => write!(formatter, "32-bit integer `{}`", i),
            Uint32(i) => write!(formatter, "32-bit unsigned integer `{}`", i),
            Int64(i) => write!(formatter, "64-bit integer `{}`", i),
            Uint64(i) => write!(formatter, "64-bit unsigned integer `{}`", i),
            Real(f) => write!(formatter, "floating point `{}`", f),
            Vector4(ref v) => write!(formatter, "vector4 `{:?}`", v),
            Quaternion(ref q) => write!(formatter, "quaternion `{:?}`", q),
            Matrix3(ref m) => write!(formatter, "matrix3 `{:?}`", m),
            Rotation(ref r) => write!(formatter, "rotation `{:?}`", r),
            QsTransform(ref t) => write!(formatter, "qs transform `{:?}`", t),
            Matrix4(ref m) => write!(formatter, "matrix4 `{:?}`", m),
            Transform(ref t) => write!(formatter, "transform `{:?}`", t),
            Pointer(ref p) => write!(formatter, "pointer `{:?}`", p),
            Array => formatter.write_str("array"),
            Enum => formatter.write_str("enum"),
            Struct => formatter.write_str("struct"),
            Variant(ref v) => write!(formatter, "variant `{:?}`", v),
            CString(ref s) => write!(formatter, "C string `{:?}`", s),
            Ulong(u) => write!(formatter, "ulong `{}`", u),
            Flags => formatter.write_str("flags"),
            Half(h) => write!(formatter, "half `{}`", h),
            StringPtr(ref s) => write!(formatter, "string pointer `{:?}`", s),
            Other(ref other) => formatter.write_str(other),
        }
    }
}

/// `Expected` represents an explanation of what data a `Visitor` was expecting
/// to receive.
///
/// This is used as an argument to the `invalid_type`, `invalid_value`, and
/// `invalid_length` methods of the `Error` trait to build error messages. The
/// message should be a noun or noun phrase that completes the sentence "This
/// Visitor expects to receive ...", for example the message could be "an
/// integer between 0 and 64". The message should not be capitalized and should
/// not end with a period.
///
/// Within the context of a `Visitor` implementation, the `Visitor` itself
/// (`&self`) is an implementation of this trait.
///
/// ```edition2021
/// # use serde::de::{self, Unexpected, Visitor};
/// # use std::fmt;
/// #
/// # struct Example;
/// #
/// # impl<'de> Visitor<'de> for Example {
/// #     type Value = ();
/// #
/// #     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
/// #         write!(formatter, "definitely not a boolean")
/// #     }
/// #
/// fn visit_bool<E><V>(self, visitor: V) -> Result<Self::Value, E>
/// where
///     E: de::Error,
/// {
///     Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
/// }
/// # }
/// ```
///
/// Outside of a `Visitor`, `&"..."` can be used.
///
/// ```edition2021
/// # use serde::de::{self, Unexpected};
/// #
/// # fn example<E>() -> Result<(), E>
/// # where
/// #     E: de::Error,
/// # {
/// #     let v = true;
/// return Err(de::Error::invalid_type(
///     Unexpected::Bool(v),
///     &"a negative integer",
/// ));
/// # }
/// ```
pub trait Expected {
    /// Format an explanation of what data was being expected. Same signature as
    /// the `Display` and `Debug` traits.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result;
}

impl<'de, T> Expected for T
where
    T: Visitor<'de>,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.expecting(formatter)
    }
}

impl<'a> Expected for &'a str {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self)
    }
}

impl<'a> Display for dyn Expected + 'a {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Expected::fmt(self, formatter)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// A **data structure** that can be deserialized from any data format supported
/// by Serde.
///
/// Serde provides `Deserialize` implementations for many Rust primitive and
/// standard library types. The complete list is [here][crate::de]. All of these
/// can be deserialized using Serde out of the box.
///
/// Additionally, Serde provides a procedural macro called `serde_derive` to
/// automatically generate `Deserialize` implementations for structs and enums
/// in your program. See the [derive section of the manual][derive] for how to
/// use this.
///
/// In rare cases it may be necessary to implement `Deserialize` manually for
/// some type in your program. See the [Implementing
/// `Deserialize`][impl-deserialize] section of the manual for more about this.
///
/// Third-party crates may provide `Deserialize` implementations for types that
/// they expose. For example the `linked-hash-map` crate provides a
/// `LinkedHashMap<K, V>` type that is deserializable by Serde because the crate
/// provides an implementation of `Deserialize` for it.
///
/// [derive]: https://serde.rs/derive.html
/// [impl-deserialize]: https://serde.rs/impl-deserialize.html
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the lifetime of data that may be
/// borrowed by `Self` when deserialized. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
pub trait Deserialize<'de>: Sized {
    /// Deserialize this value from the given Serde deserializer.
    ///
    /// See the [Implementing `Deserialize`][impl-deserialize] section of the
    /// manual for more information about how to implement this method.
    ///
    /// [impl-deserialize]: https://serde.rs/impl-deserialize.html
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;

    /// Deserializes a value into `self` from the given Deserializer.
    ///
    /// The purpose of this method is to allow the deserializer to reuse
    /// resources and avoid copies. As such, if this method returns an error,
    /// `self` will be in an indeterminate state where some parts of the struct
    /// have been overwritten. Although whatever state that is will be
    /// memory-safe.
    ///
    /// This is generally useful when repeatedly deserializing values that
    /// are processed one at a time, where the value of `self` doesn't matter
    /// when the next deserialization occurs.
    ///
    /// If you manually implement this, your recursive deserializations should
    /// use `deserialize_in_place`.
    ///
    /// This method is stable and an official public API, but hidden from the
    /// documentation because it is almost never what newbies are looking for.
    /// Showing it in rustdoc would cause it to be featured more prominently
    /// than it deserves.
    #[doc(hidden)]
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        // Default implementation just delegates to `deserialize` impl.
        *place = tri!(Deserialize::deserialize(deserializer));
        Ok(())
    }
}

/// Deserializer
pub trait Deserializer<'de>: Sized {
    /// The error type that can be returned if some error occurs during
    /// deserialization.
    type Error: Error;

    /// Deserialize a `Void` value.
    ///
    /// No type information.
    ///
    /// This is often used to fill in generics elements with types for which generics are not used.
    ///
    /// - `hkArray<hkBool>` -> `vtype`: `TYPE_ARRAY`, `vsubtype`: `TYPE_BOOL`
    /// - `hkBool -> `vtype`: `TYPE_BOOL`, `vsubtype`: `TYPE_VOID`
    /// - There is also a pattern `hkArray<void>`. The type information is unknown, but this member always contains the `SERIALIZE_IGNORED` flag and can be skipped.
    fn deserialize_void<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize a `bool` value.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `char` value.
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `i8` value.
    ///
    /// If the format does not differentiate between `i8` and `i64`, a
    /// reasonable implementation would be to cast the value to `i64` and
    /// forward to `deserialize_i64`.
    fn deserialize_int8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `u8` value.
    fn deserialize_uint8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `i16` value.
    fn deserialize_int16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `u16` value.
    fn deserialize_uint16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `i32` value.
    fn deserialize_int32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `u32` value.
    fn deserialize_uint32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `u64` value.
    fn deserialize_int64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `i64` value.
    fn deserialize_uint64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `f32` value.
    fn deserialize_real<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Vector4` value.
    fn deserialize_vector4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Quaternion` value.
    fn deserialize_quaternion<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Matrix3` value.
    fn deserialize_matrix3<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Rotation` value.
    fn deserialize_rotation<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `QsTransform` value.
    fn deserialize_qstransform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Matrix4` value.
    fn deserialize_matrix4<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Transform` value.
    fn deserialize_transform<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Vector4` value.
    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Array` value.
    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Struct` value.
    fn deserialize_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Variant` value.
    ///
    /// # Note
    /// Never used(In the 2010 Havok classes)
    ///
    /// `hkVariant` is a structure with two pointers, but its identity is unknown,
    /// so u128(`[u64; 2]`) of binary data is used as an argument instead. (If it is 32bit, you would need to cast it to u64(`[u32;2]`).)
    fn deserialize_variant<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `CString` value.
    fn deserialize_cstring<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `ULong`(pointer size(u32 or u64)) value.
    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `enum` or `Flags` value.
    fn deserialize_enum_flags<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Half`(`f16`) value.
    fn deserialize_half<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `StringPtr` value.
    fn deserialize_stringptr<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}

////////////////////////////////////////////////////////////////////////////////

/// This trait represents a visitor that walks through a deserializer.
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the requirement for lifetime of data
/// that may be borrowed by `Self::Value`. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
///
/// # Example
///
/// ```edition2021
/// # use serde::de::{self, Unexpected, Visitor};
/// # use std::fmt;
/// #
/// /// A visitor that deserializes a long string - a string containing at least
/// /// some minimum number of bytes.
/// struct LongString {
///     min: usize,
/// }
///
/// impl<'de> Visitor<'de> for LongString {
///     type Value = String;
///
///     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
///         write!(formatter, "a string containing at least {} bytes", self.min)
///     }
///
///     fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
///     where
///         E: de::Error,
///     {
///         if s.len() >= self.min {
///             Ok(s.to_owned())
///         } else {
///             Err(de::Error::invalid_value(Unexpected::Str(s), &self))
///         }
///     }
/// }
/// ```
pub trait Visitor<'de>: Sized {
    /// The value produced by this visitor.
    type Value;

    /// Format a message stating what data this Visitor expects to receive.
    ///
    /// This is used in error messages. The message should complete the sentence
    /// "This Visitor expects to receive ...", for example the message could be
    /// "an integer between 0 and 64". The message should not be capitalized and
    /// should not end with a period.
    ///
    /// ```edition2021
    /// # use std::fmt;
    /// #
    /// # struct S {
    /// #     max: usize,
    /// # }
    /// #
    /// # impl<'de> serde::de::Visitor<'de> for S {
    /// #     type Value = ();
    /// #
    /// fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    ///     write!(formatter, "an integer between 0 and {}", self.max)
    /// }
    /// # }
    /// ```
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result;

    /// The input contains a boolean.
    ///
    /// The default implementation fails with a type error.
    fn visit_void<E>(self, _: ()) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Void, &self))
    }

    /// The input contains a boolean.
    ///
    /// The default implementation fails with a type error.
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Bool(v), &self))
    }

    /// The input contains char.
    ///
    /// The default implementation fails with a type error.
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Char(v), &self))
    }

    /// The input contains a i8.
    ///
    /// The default implementation fails with a type error.
    fn visit_int8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Int8(v), &self))
    }

    /// The input contains a u8.
    ///
    /// The default implementation fails with a type error.
    fn visit_uint8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Uint8(v), &self))
    }

    /// The input contains a i16.
    ///
    /// The default implementation fails with a type error.
    fn visit_int16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Int16(v), &self))
    }

    /// The input contains a u16.
    ///
    /// The default implementation fails with a type error.
    fn visit_uint16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Uint16(v), &self))
    }

    /// The input contains a i32.
    ///
    /// The default implementation fails with a type error.
    fn visit_int32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Int32(v), &self))
    }

    /// The input contains a u32.
    ///
    /// The default implementation fails with a type error.
    fn visit_uint32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Uint32(v), &self))
    }

    /// The input contains a i64.
    ///
    /// The default implementation fails with a type error.
    fn visit_int64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Int64(v), &self))
    }

    /// The input contains a u64.
    ///
    /// The default implementation fails with a type error.
    fn visit_uint64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Uint64(v), &self))
    }

    /// The input contains a f32.
    ///
    /// The default implementation fails with a type error.
    fn visit_real<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Real(v), &self))
    }

    /// The input contains a Vector4.
    ///
    /// The default implementation fails with a type error.
    fn visit_vector4<E>(self, v: Vector4) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Vector4(v), &self))
    }

    /// The input contains a Quaternion.
    ///
    /// The default implementation fails with a type error.
    fn visit_quaternion<E>(self, v: Quaternion) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Quaternion(v), &self))
    }

    /// The input contains a Matrix3.
    ///
    /// The default implementation fails with a type error.
    fn visit_matrix3<E>(self, v: Matrix3) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Matrix3(v), &self))
    }

    /// The input contains a Rotation.
    ///
    /// The default implementation fails with a type error.
    fn visit_rotation<E>(self, v: Rotation) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Rotation(v), &self))
    }

    /// The input contains a QsTransform.
    ///
    /// The default implementation fails with a type error.
    fn visit_qstransform<E>(self, v: QsTransform) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::QsTransform(v), &self))
    }

    /// The input contains a Matrix4.
    ///
    /// The default implementation fails with a type error.
    fn visit_matrix4<E>(self, v: Matrix4) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Matrix4(v), &self))
    }

    /// The input contains a Transform.
    ///
    /// The default implementation fails with a type error.
    fn visit_transform<E>(self, v: Transform) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Transform(v), &self))
    }

    /// The input contains a Pointer.
    ///
    /// The default implementation fails with a type error.
    fn visit_pointer<E>(self, v: Pointer) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Pointer(v), &self))
    }

    /// The input contains a Variant.
    ///
    /// The default implementation fails with a type error.
    fn visit_variant<E>(self, v: Variant) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Variant(v), &self))
    }

    // TODO: Array
    //
    // TODO: Struct

    /// The input contains a CString.
    ///
    /// The default implementation fails with a type error.
    fn visit_cstring<E>(self, v: CString<'de>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::CString(v), &self))
    }

    // TODO: Enum flags

    /// The input contains a u64.
    ///
    /// The default implementation fails with a type error.
    fn visit_ulong<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Ulong(v), &self))
    }

    /// The input contains a f16.
    ///
    /// The default implementation fails with a type error.
    fn visit_half<E>(self, v: f16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Half(v), &self))
    }

    /// The input contains a StringPtr.
    ///
    /// The default implementation fails with a type error.
    fn visit_stringptr<E>(self, v: StringPtr<'de>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::StringPtr(v), &self))
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Used in error messages.
///
/// - expected `a`
/// - expected `a` or `b`
/// - expected one of `a`, `b`, `c`
///
/// The slice of names must not be empty.
struct OneOf {
    names: &'static [&'static str],
}

impl Display for OneOf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.names.len() {
            0 => panic!(), // special case elsewhere
            1 => write!(formatter, "`{}`", self.names[0]),
            2 => write!(formatter, "`{}` or `{}`", self.names[0], self.names[1]),
            _ => {
                tri!(formatter.write_str("one of "));
                for (i, alt) in self.names.iter().enumerate() {
                    if i > 0 {
                        tri!(formatter.write_str(", "));
                    }
                    tri!(write!(formatter, "`{}`", alt));
                }
                Ok(())
            }
        }
    }
}
