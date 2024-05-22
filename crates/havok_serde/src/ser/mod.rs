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

pub mod bytes;
pub mod xml;

use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Vector4,
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
/// The complete list is [here][crate::ser]. All of these can be serialized using
/// Serde out of the box.
///
/// Additionally, Serde provides a procedural macro called [`havok_serde_derive`] to
/// automatically generate `Serialize` implementations for structs and enums in
/// your program. See the [derive section of the manual] for how to use this.
///
/// In rare cases it may be necessary to implement `Serialize` manually for some
/// type in your program. See the [Implementing `Serialize`] section of the
/// manual for more about this.
///
/// [Implementing `Serialize`]: https://serde.rs/impl-serialize.html
/// [`havok_serde_derive`]: https://crates.io/crates/havok_serde_derive
/// [derive section of the manual]: https://serde.rs/derive.html
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
    ///         let mut s = serializer.serialize_struct("hkDummyClass", Some(50, 0x12345678))?;
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

    /// Serialize a `()` value. (Unused ver.hk2010)
    ///
    /// ```edition2021
    /// # use serde::Serializer;
    /// #
    /// # serde::__private_serialize!();
    /// #
    /// impl Serialize for () {
    ///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    ///     where
    ///         S: Serializer,
    ///     {
    ///         serializer.serialize_void(*self)
    ///     }
    /// }
    /// ```
    fn serialize_void(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `bool` value.
    ///
    /// ```edition2021
    /// # use serde::Serializer;
    /// #
    /// # serde::__private_serialize!();
    /// #
    /// impl Serialize for bool {
    ///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    ///     where
    ///         S: Serializer,
    ///     {
    ///         serializer.serialize_bool(*self)
    ///     }
    /// }
    /// ```
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `char` value.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i8` value.
    ///
    /// If the format does not differentiate between `i8` and `i64`, a
    /// reasonable implementation would be to cast the value to `i64` and
    /// forward to `serialize_i64`.
    ///
    /// ```edition2021
    /// # use serde::Serializer;
    /// #
    /// # serde::__private_serialize!();
    /// #
    /// impl Serialize for i8 {
    ///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    ///     where
    ///         S: Serializer,
    ///     {
    ///         serializer.serialize_int8(*self)
    ///     }
    /// }
    /// ```
    fn serialize_int8(self, v: i8) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `u8` value.
    fn serialize_uint8(self, v: u8) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i16` value.
    ///
    /// If the format does not differentiate between `i16` and `i64`, a
    /// reasonable implementation would be to cast the value to `i64` and
    /// forward to `serialize_i64`.
    ///
    /// ```edition2021
    /// # use serde::Serializer;
    /// #
    /// # serde::__private_serialize!();
    /// #
    /// impl Serialize for i16 {
    ///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    ///     where
    ///         S: Serializer,
    ///     {
    ///         serializer.serialize_int16(*self)
    ///     }
    /// }
    /// ```
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

    /// Serialize an `Zero` value. (Never used)
    fn serialize_zero(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_pointer(self, v: Pointer) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value. (Never used)
    fn serialize_functionpointer(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_array(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;

    /// Serialize an `Vector4` value. (Never used)
    fn serialize_inplacearray(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_enum(self, v: u32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_struct(
        self,
        name: &'static str,
        class_meta: Option<(Pointer, Signature)>,
    ) -> Result<Self::SerializeStruct, Self::Error>;

    /// Serialize an `Vector4` value.
    fn serialize_simplearray(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `HomogeneousArray`(``) value. (Never used)
    fn serialize_homogeneousarray(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Variant` value.
    fn serialize_variant(self, v: u32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `CString` value.
    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `ULong`(`u64`) value.
    fn serialize_ulong(self, v: u64) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Flag` value.
    fn serialize_flags(self, v: u32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Half`(`f16`) value.
    fn serialize_half(self, v: f16) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `StringPtr` value.
    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `RelArray`(``) value.
    fn serialize_relarray(self, v: ()) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Max`(``) value.
    fn serialize_max(self, x: ()) -> Result<Self::Ok, Self::Error>;
}

/// Array
pub trait SerializeSeq {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok;

    /// Must match the `Error` type of our `Serializer`.
    type Error: Error;

    /// Serialize a Havok Class sequence element.(e.g. `T: impl HavokClass`)
    fn serialize_class_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

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

    /// Serialize a struct field for array.
    fn serialize_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize;

    /// Indicate that a struct field has been skipped.
    ///
    /// The default implementation does nothing.
    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }

    /// Finish serializing a struct.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
