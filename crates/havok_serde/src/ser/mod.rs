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
    Signature, StringPtr, Transform, Ulong, Vector4,
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

    /// Serialize an `Pointer` value.
    fn serialize_pointer(self, v: Pointer) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `Array` value.
    fn serialize_array(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;

    /// Serialize an `Struct` value.
    ///
    /// sizes: C++ class size (x86, x86_64)
    fn serialize_struct(
        self,
        name: &'static str,
        class_meta: Option<(Pointer, Signature)>,
        sizes: (u64, u64),
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
    fn serialize_ulong(self, v: Ulong) -> Result<Self::Ok, Self::Error>;

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

    // # Reason for existence of `cstring` & `stringptr` methods.
    // - The work on XML is different from bytes and could not be abstracted.
    // - Creating a combined type of `CString` and `StringPtr` would require cloning in array processing.

    /// Serialize a cstring sequence element.
    fn serialize_cstring_element(&mut self, value: &CString) -> Result<(), Self::Error>;

    /// Serialize a stringptr sequence element.
    fn serialize_stringptr_element(&mut self, value: &StringPtr) -> Result<(), Self::Error>;

    /// Finish serializing a sequence.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Used for writing binary data in Array. (To write after the pointer type data)
/// - `array_size` = `size of the pointer type` * `array len`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeSize {
    /// C++ Class size
    Struct {
        /// x86 class size
        size_x86: u64,
        /// x86_64 class size
        size_x86_64: u64,
    },
    /// ptr size
    String,
    /// No size calculation is required for NonPtr.
    /// This is an option for types other than `StringPtr`, `CString`, and `Struct`.
    NonPtr,
}

/// Returned from `Serializer::serialize_struct`.
///
/// The [example data format] presented on the website demonstrates an
/// implementation of `SerializeStruct` for a basic JSON data format.
///
/// [example data format]: https://serde.rs/data-format.html
///
/// # Flatten is unsupportable
/// If the parent is a ptr type, the process of simply writing the parent cannot be used as it is because the data writing of the ptr destination is waiting after the writing of all fields.
/// Therefore, `serialize` of the parent struct cannot be reused.
pub trait SerializeStruct {
    /// Must match the `Ok` type of our `Serializer`.
    type Ok;

    /// Must match the `Error` type of our `Serializer`.
    type Error: Error;

    /// Serialize a struct field.
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize;

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

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Fixed Array

    /// Serialize a struct field for fixed array.
    /// -   XML: add `numelements` attribute to `hkparam` and write array contents.(same as `hkArray`)
    /// - Bytes: write each bytes.
    fn serialize_fixed_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize;

    /// Process for fields with `SERIALIZE_IGNORED` flag.
    ///
    /// The default implementation same as `skip_field`
    #[inline]
    fn skip_fixed_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let _ = size;
        self.skip_field(key, &value)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Array

    /// Serialize a struct field for array.
    /// -   XML: add `numelements` attribute to `hkparam` and write vec contents.
    /// - Bytes: Alloc meta(ptr size + size + capAndFlags) bytes. (x86: 12, x64: 16) & Serialize pointed data.
    #[inline]
    fn serialize_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let _ = (key, value, size);
        Ok(())
    }

    /// Process for fields with `SERIALIZE_IGNORED` flag.
    ///
    /// The default implementation same as `skip_field`
    #[inline]
    fn skip_array_field<V, T>(
        &mut self,
        key: &'static str,
        value: V,
        size: TypeSize,
    ) -> Result<(), Self::Error>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let _ = size;
        self.skip_field(key, &value)
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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
