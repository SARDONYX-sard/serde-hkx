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
//! Serialization

use crate::lib::*;

mod impls;

use havok_types::{
    f16, variant::Variant, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation,
    Signature, StringPtr, Transform, Vector4,
};

#[cfg(feature = "std")]
#[doc(no_inline)]
pub use std::error::Error as StdError;

////////////////////////////////////////////////////////////////////////////////

macro_rules! declare_error_trait {
    (Error: Sized $(+ $($super_trait:ident)::+)*) => {
        /// Trait used by `Serialize` implementations to generically construct
        /// errors belonging to the `Serializer` against which they are
        /// currently running.
        ///
        /// # Example implementation
        ///
        /// The [example data format] presented on the website shows an error
        /// type appropriate for a basic JSON data format.
        ///
        /// [example data format]: https://serde.rs/data-format.html
        pub trait Error: Sized $(+ $($super_trait)::+)* {
            /// Used when a [`Serialize`] implementation encounters any error
            /// while serializing a type.
            ///
            /// The message should not be capitalized and should not end with a
            /// period.
            ///
            /// For example, a filesystem [`Path`] may refuse to serialize
            /// itself if it contains invalid UTF-8 data.
            ///
            /// ```edition2021
            /// # struct Path;
            /// #
            /// # impl Path {
            /// #     fn to_str(&self) -> Option<&str> {
            /// #         unimplemented!()
            /// #     }
            /// # }
            /// #
            /// use serde::ser::{self, Serialize, Serializer};
            ///
            /// impl Serialize for Path {
            ///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            ///     where
            ///         S: Serializer,
            ///     {
            ///         match self.to_str() {
            ///             Some(s) => serializer.serialize_str(s),
            ///             None => Err(ser::Error::custom("path contains invalid UTF-8 characters")),
            ///         }
            ///     }
            /// }
            /// ```
            ///
            /// [`Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
            /// [`Serialize`]: ../trait.Serialize.html
            fn custom<T>(msg: T) -> Self
            where
                T: Display;
        }
    }
}

#[cfg(feature = "std")]
declare_error_trait!(Error: Sized + StdError);

#[cfg(not(feature = "std"))]
declare_error_trait!(Error: Sized + Debug + Display);

////////////////////////////////////////////////////////////////////////////////

/// A **data structure** that can be serialized into any data format supported
/// by Serde.
///
/// Serde provides `Serialize` implementations for all C++ Havok types.
///
/// Additionally, Serde provides a procedural macro called [`havok_serde_derive`] to
/// automatically generate `Serialize` implementations for structs and enums in
/// your program. See the [derive section of the manual] for how to use this.
///
/// In rare cases it may be necessary to implement `Serialize` manually for some
/// type in your program. See the [Implementing `Serialize`] section of the
/// manual for more about this.
///
/// # Support types
///
/// # Unsupported types
/// There're never used in the Havok classes.(ver. hk2010)
/// - `Zero`, `FunctionPointer`, `InplaceArray`, `HomogeneousArray`, `RelArray`, `Max`
pub trait Serialize {
    /// Serialize this value into the given Serde serializer.
    ///
    /// ```edition2021
    /// use havok_serde::ser::{Serialize, SerializeStruct, Serializer};
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    ///     phones: Vec<String>,
    /// }
    ///
    /// // This is what #[derive(Serialize)] would generate.
    /// impl Serialize for Person {
    ///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    ///     where
    ///         S: Serializer,
    ///     {
    ///         let mut s = serializer.serialize_struct("hkDummyClass", Some(50.into(), 0x12345678.into()))?;
    ///         s.serialize_field("name", &self.name)?;
    ///         s.serialize_field("age", &self.age)?;
    ///         s.serialize_field("phones", &self.phones)?;
    ///         s.end()
    ///     }
    /// }
    /// ```
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

////////////////////////////////////////////////////////////////////////////////

/// A **data format** that can serialize any data structure supported by Serde.
///
/// The role of this trait is to define the serialization half of the [Serde
/// data model], which is a way to categorize every Rust data structure into one
/// of 29 possible types. Each method of the `Serializer` trait corresponds to
/// one of the types of the data model.
///
/// Implementations of `Serialize` map themselves into this data model by
/// invoking exactly one of the `Serializer` methods.
pub trait Serializer {
    /// The output type produced by this `Serializer` during successful
    /// serialization. Most serializers that produce text or binary output
    /// should set `Ok = ()` and serialize into an [`io::Write`] or buffer
    /// contained within the `Serializer` instance. Serializers that build
    /// in-memory data structures may be simplified by using `Ok` to propagate
    /// the data structure around.
    ///
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    type Ok;

    /// The error type when some error occurs during serialization.
    type Error: Error;

    /// Type returned from [`serialize_seq`] for serializing the content of the
    /// sequence.
    ///
    /// [`serialize_seq`]: #tymethod.serialize_seq
    type SerializeSeq: SerializeSeq<Ok = Self::Ok, Error = Self::Error>;

    /// [`serialize_struct`]: #tymethod.serialize_struct
    type SerializeStruct: SerializeStruct<Ok = Self::Ok, Error = Self::Error>;

    /// Type returned from [`serialize_enum_flags`] for serializing the
    /// content of the struct variant.
    ///
    /// [`serialize_enum_flags`]: #tymethod.serialize_flags
    type SerializeFlags: SerializeFlags<Ok = Self::Ok, Error = Self::Error>;

    /// Serialize a `Void` value.
    ///
    /// No type information.
    ///
    /// This is often used to fill in generics elements with types for which generics are not used.
    ///
    /// - `hkArray<hkBool>` -> `vtype`: `TYPE_ARRAY`, `vsubtype`: `TYPE_BOOL`
    /// - `hkBool -> `vtype`: `TYPE_BOOL`, `vsubtype`: `TYPE_VOID`
    /// - There is also a pattern `hkArray<void>`. The type information is unknown, but this member always contains the `SERIALIZE_IGNORED` flag and can be skipped.
    fn serialize_void(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `bool` value.
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `char` value.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i8` value.
    ///
    /// If the format does not differentiate between `i8` and `i64`, a
    /// reasonable implementation would be to cast the value to `i64` and
    /// forward to `serialize_i64`.
    fn serialize_int8(self, v: i8) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `u8` value.
    fn serialize_uint8(self, v: u8) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i16` value.
    fn serialize_int16(self, v: i16) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `u16` value.
    fn serialize_uint16(self, v: u16) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i32` value.
    fn serialize_int32(self, v: i32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `u32` value.
    fn serialize_uint32(self, v: u32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `u64` value.
    fn serialize_int64(self, v: i64) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i64` value.
    fn serialize_uint64(self, v: u64) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `f32` value.
    fn serialize_real(self, v: f32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_vector4(self, v: &Vector4) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Quaternion` value.
    fn serialize_quaternion(self, v: &Quaternion) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Matrix3` value.
    fn serialize_matrix3(self, v: &Matrix3) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Rotation` value.
    fn serialize_rotation(self, v: &Rotation) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `QsTransform` value.
    fn serialize_qstransform(self, v: &QsTransform) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Matrix4` value.
    fn serialize_matrix4(self, v: &Matrix4) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Transform` value.
    fn serialize_transform(self, v: &Transform) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_pointer(self, v: Pointer) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Array` value.
    fn serialize_array(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;

    /// Serialize an `Struct` value.
    fn serialize_struct(
        self,
        name: &'static str,
        class_meta: Option<(Pointer, Signature)>,
    ) -> Result<Self::SerializeStruct, Self::Error>;

    /// Serialize an `Variant` value.
    ///
    /// # Note
    /// Never used(In the 2010 Havok classes)
    ///
    /// `hkVariant` is a structure with two pointers, but its identity is unknown,
    /// so u128(`[u64; 2]`) of binary data is used as an argument instead. (If it is 32bit, you would need to cast it to u64(`[u32;2]`).)
    fn serialize_variant(self, v: &Variant) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `CString` value.
    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `ULong`(pointer size(u32 or u64)) value.
    fn serialize_ulong(self, v: u64) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `enum` or `Flags` value.
    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags, Self::Error>;

    /// Serialize an `Half`(`f16`) value.
    fn serialize_half(self, v: f16) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `StringPtr` value.
    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error>;
}

/// Returned from `Serializer::serialize_array`.
pub trait SerializeSeq {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok;

    /// Must match the `Error` type of our `Serializer`.
    type Error: Error;

    /// Serialize a primitive sequence element.
    /// (e.g. `char`, `bool` `u8`..`u64`, `i8`..`i64`, `f32`, `Pointer`)
    ///
    /// # Note
    /// index must be 0 based.
    fn serialize_primitive_element<T>(
        &mut self,
        value: &T,
        index: usize,
        len: usize,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Serialize a Havok Class sequence element.(e.g. `T: impl HavokClass`)
    fn serialize_class_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize + crate::HavokClass;

    /// Serialize a math sequence element.(e.g. `Matrix3`)
    fn serialize_math_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Serialize a string sequence element.(e.g. `CString`, `StringPtr`)
    fn serialize_string_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Finish serializing a sequence.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Returned from `Serializer::serialize_struct`.
///
/// # Example use
///
/// ```edition2021
/// use havok_serde::ser::{Serialize, SerializeStruct, Serializer};
///
/// struct Rgb {
///     r: u8,
///     g: u8,
///     b: u8,
/// }
///
/// impl Serialize for Rgb {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: Serializer,
///     {
///         let mut rgb = serializer.serialize_struct("Rgb", Some((50.into(), 0x12345678.into())))?;
///         rgb.serialize_field("r", &self.r)?;
///         rgb.serialize_field("g", &self.g)?;
///         rgb.serialize_field("b", &self.b)?;
///         rgb.end()
///     }
/// }
/// ```
///
/// # Example implementation
///
/// The [example data format] presented on the website demonstrates an
/// implementation of `SerializeStruct` for a basic JSON data format.
///
/// [example data format]: https://serde.rs/data-format.html
pub trait SerializeStruct {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok;

    /// Must match the `Error` type of our `Serializer`.
    type Error: Error;

    /// Serialize a struct field.
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Serialize a struct field for `CString` and `StringPtr`.
    /// -   XML: Same as `serialize_field`.
    /// - Bytes: Alloc ptr size -> Write String after write all struct fields.
    ///
    /// # Reason for method separation
    /// In Bytes(hkx), this method is separated because the write position of an `Array` and a single `StringPtr` are different.
    fn serialize_string_meta_field<T>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Serialize a struct field for array.
    /// -   XML: add `numelements` attribute to `hkparam` and write vec contents.
    /// - Bytes: Alloc meta(ptr size + size + capAndFlags) bytes. (x86: 12, x64: 16)
    fn serialize_array_meta_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize;

    /// - XML: Do nothing (because writing has already finished at the meta stage)
    /// - Bytes: Writes the data to which the pointer points
    ///
    /// The default implementation does nothing.
    #[inline]
    fn serialize_string_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = (key, value);
        Ok(())
    }

    /// - XML: Do nothing (because writing has already finished at the meta stage)
    /// - Bytes: Writes the data to which the pointer points.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn serialize_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let _ = (key, value);
        Ok(())
    }

    /// Process for fields with `SERIALIZE_IGNORED` flag.
    /// -   XML: Replaced by a comment and the actual data is not displayed.
    /// - Bytes: the data itself is read/written
    ///
    /// The default implementation does nothing.
    #[inline]
    fn skip_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = key;
        let _ = value;
        Ok(())
    }

    /// Process for fields with `SERIALIZE_IGNORED` flag.
    /// -   XML: Replaced by a comment and the actual data is not displayed.
    /// - Bytes: Serialize ptr size of string ptr.
    ///
    /// The default implementation same as `skip_field`
    #[inline]
    fn skip_string_meta_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.skip_field(key, value)
    }

    /// Process for fields with `SERIALIZE_IGNORED` flag.
    /// -   XML: Replaced by a comment and the actual data is not displayed.
    /// - Bytes: Serialize Array meta(ptr size, size and capAndFlags).
    ///
    /// The default implementation same as `skip_field`
    #[inline]
    fn skip_array_meta_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.skip_field(key, &value)
    }

    /// Processing for padding to serialize binary data
    ///
    /// The default implementation does nothing.
    #[inline]
    fn pad_field<T>(&mut self, x86_pads: &T, x64_pads: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + AsRef<[u8]>,
    {
        let _ = x86_pads;
        let _ = x64_pads;
        Ok(())
    }

    /// Finish serializing a struct.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Returned from `Serializer::serialize_enum_flags`.
pub trait SerializeFlags {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok;

    /// Must match the `Error` type of our `Serializer`.
    type Error: Error;

    /// Serialization process when the flag is 0bits.(Only used by XML serializer.)
    ///
    /// The default implementation does nothing.
    #[inline]
    fn serialize_empty_bit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Serialize a enum or bit field.(Only used by XML serializer.)
    fn serialize_field<T>(&mut self, key: &str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

    /// Serialize all bits of a flag.(Only used by binary serializer.)
    ///
    /// The default implementation does nothing.
    #[inline]
    fn serialize_bits<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let _ = value;
        Ok(())
    }

    /// Finish serializing flags.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
