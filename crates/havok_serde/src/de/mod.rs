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
mod lazy_init_array;
mod seed;
mod size_hint;

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
            Other(other) => formatter.write_str(other),
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

/// A data structure that can be deserialized without borrowing any data from
/// the deserializer.
///
/// This is primarily useful for trait bounds on functions. For example a
/// `from_str` function may be able to deserialize a data structure that borrows
/// from the input string, but a `from_reader` function may only deserialize
/// owned data.
///
/// ```edition2021
/// # use serde::de::{Deserialize, DeserializeOwned};
/// # use std::io::{Read, Result};
/// #
/// # trait Ignore {
/// fn from_str<'a, T>(s: &'a str) -> Result<T>
/// where
///     T: Deserialize<'a>;
///
/// fn from_reader<R, T>(rdr: R) -> Result<T>
/// where
///     R: Read,
///     T: DeserializeOwned;
/// # }
/// ```
///
/// # Lifetime
///
/// The relationship between `Deserialize` and `DeserializeOwned` in trait
/// bounds is explained in more detail on the page [Understanding deserializer
/// lifetimes].
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}

/// `DeserializeSeed` is the stateful form of the `Deserialize` trait. If you
/// ever find yourself looking for a way to pass data into a `Deserialize` impl,
/// this trait is the way to do it.
///
/// As one example of stateful deserialization consider deserializing a JSON
/// array into an existing buffer. Using the `Deserialize` trait we could
/// deserialize a JSON array into a `Vec<T>` but it would be a freshly allocated
/// `Vec<T>`; there is no way for `Deserialize` to reuse a previously allocated
/// buffer. Using `DeserializeSeed` instead makes this possible as in the
/// example code below.
///
/// The canonical API for stateless deserialization looks like this:
///
/// ```edition2021
/// # use serde::Deserialize;
/// #
/// # enum Error {}
/// #
/// fn func<'de, T: Deserialize<'de>>() -> Result<T, Error>
/// # {
/// #     unimplemented!()
/// # }
/// ```
///
/// Adjusting an API like this to support stateful deserialization is a matter
/// of accepting a seed as input:
///
/// ```edition2021
/// # use serde::de::DeserializeSeed;
/// #
/// # enum Error {}
/// #
/// fn func_seed<'de, T: DeserializeSeed<'de>>(seed: T) -> Result<T::Value, Error>
/// # {
/// #     let _ = seed;
/// #     unimplemented!()
/// # }
/// ```
///
/// In practice the majority of deserialization is stateless. An API expecting a
/// seed can be appeased by passing `std::marker::PhantomData` as a seed in the
/// case of stateless deserialization.
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the lifetime of data that may be
/// borrowed by `Self::Value` when deserialized. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
///
/// # Example
///
/// Suppose we have JSON that looks like `[[1, 2], [3, 4, 5], [6]]` and we need
/// to deserialize it into a flat representation like `vec![1, 2, 3, 4, 5, 6]`.
/// Allocating a brand new `Vec<T>` for each subarray would be slow. Instead we
/// would like to allocate a single `Vec<T>` and then deserialize each subarray
/// into it. This requires stateful deserialization using the `DeserializeSeed`
/// trait.
///
/// ```edition2021
/// use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
/// use std::fmt;
/// use std::marker::PhantomData;
///
/// // A DeserializeSeed implementation that uses stateful deserialization to
/// // append array elements onto the end of an existing vector. The preexisting
/// // state ("seed") in this case is the Vec<T>. The `deserialize` method of
/// // `ExtendVec` will be traversing the inner arrays of the JSON input and
/// // appending each integer into the existing Vec.
/// struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);
///
/// impl<'de, 'a, T> DeserializeSeed<'de> for ExtendVec<'a, T>
/// where
///     T: Deserialize<'de>,
/// {
///     // The return type of the `deserialize` method. This implementation
///     // appends onto an existing vector but does not create any new data
///     // structure, so the return type is ().
///     type Value = ();
///
///     fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
///     where
///         D: Deserializer<'de>,
///     {
///         // Visitor implementation that will walk an inner array of the JSON
///         // input.
///         struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);
///
///         impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
///         where
///             T: Deserialize<'de>,
///         {
///             type Value = ();
///
///             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
///                 write!(formatter, "an array of integers")
///             }
///
///             fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
///             where
///                 A: SeqAccess<'de>,
///             {
///                 // Decrease the number of reallocations if there are many elements
///                 if let Some(size_hint) = seq.size_hint() {
///                     self.0.reserve(size_hint);
///                 }
///
///                 // Visit each element in the inner array and push it onto
///                 // the existing vector.
///                 while let Some(elem) = seq.next_element()? {
///                     self.0.push(elem);
///                 }
///                 Ok(())
///             }
///         }
///
///         deserializer.deserialize_seq(ExtendVecVisitor(self.0))
///     }
/// }
///
/// // Visitor implementation that will walk the outer array of the JSON input.
/// struct FlattenedVecVisitor<T>(PhantomData<T>);
///
/// impl<'de, T> Visitor<'de> for FlattenedVecVisitor<T>
/// where
///     T: Deserialize<'de>,
/// {
///     // This Visitor constructs a single Vec<T> to hold the flattened
///     // contents of the inner arrays.
///     type Value = Vec<T>;
///
///     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
///         write!(formatter, "an array of arrays")
///     }
///
///     fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
///     where
///         A: SeqAccess<'de>,
///     {
///         // Create a single Vec to hold the flattened contents.
///         let mut vec = Vec::new();
///
///         // Each iteration through this loop is one inner array.
///         while let Some(()) = seq.next_element_seed(ExtendVec(&mut vec))? {
///             // Nothing to do; inner array has been appended into `vec`.
///         }
///
///         // Return the finished vec.
///         Ok(vec)
///     }
/// }
///
/// # fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
/// # where
/// #     D: Deserializer<'de>,
/// # {
/// let visitor = FlattenedVecVisitor(PhantomData);
/// let flattened: Vec<u64> = deserializer.deserialize_seq(visitor)?;
/// #     Ok(())
/// # }
/// ```

pub trait DeserializeSeed<'de>: Sized {
    /// The type produced by using this seed.
    type Value;

    /// Equivalent to the more common `Deserialize::deserialize` method, except
    /// with some initial piece of data (the seed) passed in.
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de, T> DeserializeSeed<'de> for PhantomData<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    #[inline]
    fn deserialize<D>(self, deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

/////////////////////////////////////////////////////////////////////

/// Deserializer
pub trait Deserializer<'de>: Sized {
    /// The error type that can be returned if some error occurs during
    /// deserialization.
    type Error: Error;

    /// Hint that the `Deserialize` type is expecting the discriminant of an enum variant.
    fn deserialize_identifier<V>(
        self,
        size: ReadEnumSize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize map key.
    ///
    /// - It is mainly called to deserialize the key of a field when implementing [`Deserialize`] of struct.
    /// - The most intended use is to parse XML name strings. The `key` part of `<hkparam name="key">`.
    ///
    /// The default implementation does nothing.
    fn deserialize_key<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_void(())
    }

    /// Deserialize class index.
    fn deserialize_class_index<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

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

    /// Deserialize an `Pointer` value.
    fn deserialize_pointer<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Array` value.
    fn deserialize_array<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `enum` value.
    ///
    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;

    /// Deserialize an `Struct` value.
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
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

    /// Deserialize an `Flags` value.
    ///
    /// Hint that the `Deserialize` type is expecting flags value with a
    /// particular name and possible variants.
    fn deserialize_flags<V>(self, size: ReadEnumSize, visitor: V) -> Result<V::Value, Self::Error>
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

    /// The input contains a [`MapAccess`] key.
    ///
    /// The default implementation fails with a type error.
    fn visit_key<E>(self, key: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = key;
        Err(Error::invalid_type(Unexpected::Other(key), &self))
    }

    /// The input contains a havok class.
    ///
    /// The default implementation fails with a type error.
    fn visit_class_index<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: ClassIndexAccess<'de>,
    {
        let _ = map;
        Err(Error::invalid_type(Unexpected::Struct, &self))
    }

    /// The input contains a void.
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

    /// The input contains an Array.
    ///
    /// The default implementation fails with a type error.
    fn visit_array<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let _ = seq;
        Err(Error::invalid_type(Unexpected::Array, &self))
    }

    /// The input contains an enum.
    ///
    /// The default implementation fails with a type error.
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let _ = data;
        Err(Error::invalid_type(Unexpected::Enum, &self))
    }

    /// The input contains a key-value map.(serde: `visit_map`)
    ///
    /// The default implementation fails with a type error.
    fn visit_struct<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let _ = map;
        Err(Error::invalid_type(Unexpected::Struct, &self))
    }

    /// The input contains a CString.
    ///
    /// The default implementation fails with a type error.
    fn visit_cstring<E>(self, v: CString<'de>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::CString(v), &self))
    }

    /// The input contains a u64.
    ///
    /// The default implementation fails with a type error.
    fn visit_ulong<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Ulong(v), &self))
    }

    /// The input contains flags.
    ///
    /// The default implementation fails with a type error.
    fn visit_flags<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let _ = data;
        Err(Error::invalid_type(Unexpected::Flags, &self))
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

/// Provides a `Visitor` access to each element of a sequence in the input.
///
/// This is a trait that a `Deserializer` passes to a `Visitor` implementation,
/// which deserializes each item in a sequence.
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the lifetime of data that may be
/// borrowed by deserialized sequence elements. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
///
/// # Example implementation
///
/// The [example data format] presented on the website demonstrates an
/// implementation of `SeqAccess` for a basic JSON data format.
///
/// [example data format]: https://serde.rs/data-format.html
pub trait SeqAccess<'de> {
    /// The error type that can be returned if some error occurs during
    /// deserialization.
    type Error: Error;

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// `Deserialize` implementations should typically use
    /// `SeqAccess::next_primitive_element` instead.
    fn next_primitive_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>;

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// `Deserialize` implementations should typically use
    /// `SeqAccess::next_class_element` instead.
    fn next_class_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>; // + crate::HavokClass

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// `Deserialize` implementations should typically use
    /// `SeqAccess::next_math_element` instead.
    fn next_math_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>;

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// `Deserialize` implementations should typically use
    /// `SeqAccess::next_cstring_element` instead.
    fn next_cstring_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>;

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// `Deserialize` implementations should typically use
    /// `SeqAccess::next_stringptr_element` instead.
    fn next_stringptr_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>;

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `SeqAccess` implementations should not override the default behavior.
    #[inline]
    fn next_primitive_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.next_primitive_element_seed(PhantomData)
    }

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `SeqAccess` implementations should not override the default behavior.
    #[inline]
    fn next_class_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de> + crate::HavokClass,
    {
        self.next_class_element_seed(PhantomData)
    }

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `SeqAccess` implementations should not override the default behavior.
    #[inline]
    fn next_math_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        self.next_math_element_seed(PhantomData)
    }

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `SeqAccess` implementations should not override the default behavior.
    #[inline]
    fn next_cstring_element(&mut self) -> Result<Option<CString<'de>>, Self::Error> {
        self.next_cstring_element_seed(PhantomData)
    }

    /// This returns `Ok(Some(value))` for the next value in the sequence, or
    /// `Ok(None)` if there are no more remaining items.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `SeqAccess` implementations should not override the default behavior.
    #[inline]
    fn next_stringptr_element(&mut self) -> Result<Option<StringPtr<'de>>, Self::Error> {
        self.next_stringptr_element_seed(PhantomData)
    }

    /// Returns the number of elements remaining in the sequence, if known.
    #[inline]
    fn size_hint(&self) -> Option<usize> {
        None
    }
}

// NOTE: Deref does not result in a circular call error for `&mut A` because it calls the method of A.
impl<'de, 'a, A> SeqAccess<'de> for &'a mut A
where
    A: ?Sized + SeqAccess<'de>,
{
    type Error = A::Error;

    #[inline]
    fn next_primitive_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        (**self).next_primitive_element_seed(seed)
    }

    fn next_class_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        (**self).next_class_element_seed(seed)
    }

    fn next_math_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        (**self).next_math_element_seed(seed)
    }

    fn next_cstring_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        (**self).next_cstring_element_seed(seed)
    }

    fn next_stringptr_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        (**self).next_stringptr_element_seed(seed)
    }

    #[inline]
    fn next_primitive_element<T>(&mut self) -> Result<Option<T>, Self::Error>
    where
        T: Deserialize<'de>,
    {
        (**self).next_primitive_element()
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        (**self).size_hint()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Provides a `Visitor` access to each entry of a map in the input.
///
/// This is a trait that a `Deserializer` passes to a `Visitor` implementation.
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the lifetime of data that may be
/// borrowed by deserialized map entries. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
///
/// # Example implementation
///
/// The [example data format] presented on the website demonstrates an
/// implementation of `MapAccess` for a basic JSON data format.
///
/// [example data format]: https://serde.rs/data-format.html
pub trait MapAccess<'de> {
    /// The error type that can be returned if some error occurs during
    /// deserialization.
    type Error: Error;

    /// Get this class pointer name (e.g. `Pointer::new(1`)
    fn class_ptr(&mut self) -> Option<Pointer>;

    /// - Skip reading the current position of binary data by pad minutes.
    /// - The XML deserializer does nothing.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn pad(&mut self, x86_pad: usize, x64_pad: usize) -> Result<(), Self::Error> {
        let _ = x86_pad;
        let _ = x64_pad;
        Ok(())
    }

    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    ///
    /// `Deserialize` implementations should typically use
    /// `MapAccess::next_key` or `MapAccess::next_entry` instead.
    fn skip_next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>;

    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    ///
    /// `Deserialize` implementations should typically use
    /// `MapAccess::next_key` or `MapAccess::next_entry` instead.
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>;

    /// # For Array
    /// In XML, key parsing is different as in `<hkparam name="" numelements="3">`,
    /// so the API that exists to support this.
    ///
    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    ///
    /// `Deserialize` implementations should typically use
    /// `MapAccess::next_key` or `MapAccess::next_entry` instead.
    fn next_array_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>;

    /// Processing of `SERIALIZE_IGNORED` fields.
    /// - XML: need to do nothing and get the default value.
    /// - bytes: most likely contains 0, but you need to get it because the data exists on the binary data.
    ///
    /// # Panics
    ///
    /// Calling `next_value` before `next_key` is incorrect and is allowed to
    /// panic or return bogus results.
    fn skip_next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>;

    /// This returns a `Ok(value)` for the next value in the map.
    ///
    /// `Deserialize` implementations should typically use
    /// `MapAccess::next_value` instead.
    ///
    /// # Panics
    ///
    /// Calling `next_value_seed` before `next_key_seed` is incorrect and is
    /// allowed to panic or return bogus results.
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>;

    /// This returns `Ok(Some((key, value)))` for the next (key-value) pair in
    /// the map, or `Ok(None)` if there are no more remaining items.
    ///
    /// `MapAccess` implementations should override the default behavior if a
    /// more efficient implementation is possible.
    ///
    /// `Deserialize` implementations should typically use
    /// `MapAccess::next_entry` instead.
    #[allow(clippy::type_complexity)]
    #[inline]
    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        match tri!(self.next_key_seed(kseed)) {
            Some(key) => {
                let value = tri!(self.next_value_seed(vseed));
                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }

    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `MapAccess` implementations should not override the default behavior.
    #[inline]
    fn skip_next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.next_key_seed(PhantomData)
    }

    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `MapAccess` implementations should not override the default behavior.
    #[inline]
    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.next_key_seed(PhantomData)
    }

    /// # For Array
    /// This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`
    /// if there are no more remaining entries.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `MapAccess` implementations should not override the default behavior.
    #[inline]
    fn next_array_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.next_array_key_seed(PhantomData)
    }

    /// Processing of `SERIALIZE_IGNORED` fields.
    /// - XML: need to do nothing and get the default value.
    /// - bytes: most likely contains 0, but you need to get it because the data exists on the binary data.
    ///
    /// # Panics
    ///
    /// Calling `next_value` before `next_key` is incorrect and is allowed to
    /// panic or return bogus results.
    #[inline]
    fn skip_next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(PhantomData)
    }

    /// This returns a `Ok(value)` for the next value in the map.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `MapAccess` implementations should not override the default behavior.
    ///
    /// # Panics
    ///
    /// Calling `next_value` before `next_key` is incorrect and is allowed to
    /// panic or return bogus results.
    #[inline]
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(PhantomData)
    }

    /// This returns `Ok(Some((key, value)))` for the next (key-value) pair in
    /// the map, or `Ok(None)` if there are no more remaining items.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `MapAccess` implementations should not override the default behavior.
    #[inline]
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        self.next_entry_seed(PhantomData, PhantomData)
    }

    /// Returns the number of entries remaining in the map, if known.
    #[inline]
    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<'de, 'a, A> MapAccess<'de> for &'a mut A
where
    A: ?Sized + MapAccess<'de>,
{
    type Error = A::Error;

    #[inline]
    fn class_ptr(&mut self) -> Option<Pointer> {
        (**self).class_ptr()
    }

    #[inline]
    fn skip_next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        (**self).skip_next_key_seed(seed)
    }

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        (**self).next_key_seed(seed)
    }

    #[inline]
    fn next_array_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        (**self).next_key_seed(seed)
    }

    #[inline]
    fn skip_next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        (*self).skip_next_value_seed(seed)
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        (**self).next_value_seed(seed)
    }

    #[inline]
    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        (**self).next_entry_seed(kseed, vseed)
    }

    #[inline]
    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        (**self).next_entry()
    }

    #[inline]
    fn skip_next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        (**self).skip_next_key()
    }

    #[inline]
    fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        (**self).next_key()
    }

    #[inline]
    fn next_array_key<K>(&mut self) -> Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        (**self).next_array_key()
    }

    #[inline]
    fn skip_next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        (**self).skip_next_value()
    }

    #[inline]
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        (**self).next_value()
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        (**self).size_hint()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Class Indexed Deserializer trait
pub trait ClassIndexAccess<'de> {
    /// Class name Error
    type Error: Error;

    /// next class name.
    fn next_key(&self) -> Result<&'de str, Self::Error>;

    /// deserialize class method.
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>;

    /// Deserialize class method
    #[inline]
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(PhantomData)
    }
}

impl<'de, 'a, A> ClassIndexAccess<'de> for &'a mut A
where
    A: ?Sized + ClassIndexAccess<'de>,
{
    type Error = A::Error;

    #[inline]
    fn next_key(&self) -> Result<&'de str, Self::Error> {
        (**self).next_key()
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        (**self).next_value_seed(seed)
    }

    #[inline]
    fn next_value<V>(&mut self) -> Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(PhantomData)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Size to read to deserialize `enum` and `flags`.
/// This is used to deserialize binary data
pub enum ReadEnumSize {
    /// Deserialize binary data from i8
    Int8,
    /// Deserialize binary data from i16
    Int16,
    /// Deserialize binary data from i32
    Int32,
    /// Deserialize binary data from i64
    Int64,
    /// Deserialize binary data from u8
    Uint8,
    /// Deserialize binary data from u16
    Uint16,
    /// Deserialize binary data from u32
    Uint32,
    /// Deserialize binary data from u64
    Uint64,
}

/// Provides a `Visitor` access to the data of an enum in the input.
///
/// `EnumAccess` is created by the `Deserializer` and passed to the
/// `Visitor` in order to identify which variant of an enum to deserialize.
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the lifetime of data that may be
/// borrowed by the deserialized enum variant. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
///
/// # Example implementation
///
/// The [example data format] presented on the website demonstrates an
/// implementation of `EnumAccess` for a basic JSON data format.
///
/// [example data format]: https://serde.rs/data-format.html
pub trait EnumAccess<'de>: Sized {
    /// The error type that can be returned if some error occurs during
    /// deserialization.
    type Error: Error;
    /// The `Visitor` that will be used to deserialize the content of the enum
    /// variant.
    type Variant: VariantAccess<'de, Error = Self::Error>;

    /// `variant` is called to identify which variant to deserialize.
    ///
    /// `Deserialize` implementations should typically use `EnumAccess::variant`
    /// instead.
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>;

    /// `variant` is called to identify which variant to deserialize.
    ///
    /// This method exists as a convenience for `Deserialize` implementations.
    /// `EnumAccess` implementations should not override the default behavior.
    #[inline]
    fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.variant_seed(PhantomData)
    }
}

/// `VariantAccess` is a visitor that is created by the `Deserializer` and
/// passed to the `Deserialize` to deserialize the content of a particular enum
/// variant.
///
/// # Lifetime
///
/// The `'de` lifetime of this trait is the lifetime of data that may be
/// borrowed by the deserialized enum variant. See the page [Understanding
/// deserializer lifetimes] for a more detailed explanation of these lifetimes.
///
/// [Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html
///
/// # Example implementation
///
/// The [example data format] presented on the website demonstrates an
/// implementation of `VariantAccess` for a basic JSON data format.
///
/// [example data format]: https://serde.rs/data-format.html
pub trait VariantAccess<'de>: Sized {
    /// The error type that can be returned if some error occurs during
    /// deserialization. Must match the error type of our `EnumAccess`.
    type Error: Error;

    /// Called when deserializing a variant with no values.
    ///
    /// Check that the value of the enum is of a valid type immediately after deserialization with `EnumAccess::variant`.
    fn unit_variant(self) -> Result<(), Self::Error>;
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
