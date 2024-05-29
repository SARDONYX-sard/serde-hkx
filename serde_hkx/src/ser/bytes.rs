//! Bytes Serialization
use crate::cursor_ext::Align as _;
use crate::error::{Error, Result};
use byteorder::WriteBytesExt as _;
use havok_serde::ser::{
    Error as _, Serialize, SerializeFlags, SerializeSeq, SerializeStruct, Serializer,
};
use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Variant, Vector4,
};
use std::io::{Cursor, Write as _};
use zerocopy::{BigEndian, LittleEndian};

/// Bytes endianness
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ByteOrder {
    /// Little endian mode
    #[default]
    LittleEndian,
    /// Big endian mode
    BigEndian,
}

impl ByteOrder {
    /// Is little endian mode?
    fn is_little(&self) -> bool {
        *self == ByteOrder::LittleEndian
    }
}

/// Binary target platform
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    /// x86
    Win32,
    /// x86_64
    #[default]
    Amd64,
}

impl Platform {
    /// Serializer is win32(x86) mode?
    fn is_x86(&self) -> bool {
        *self == Platform::Win32
    }
}

/// Binary data serializer
#[derive(Debug, Default)]
pub struct ByteSerializer {
    endian: ByteOrder,
    target_platform: Platform,
    /// Bytes
    output: Cursor<Vec<u8>>,

    /// Temporary area to deal with pointer types within pointer types.
    ///
    /// # Details
    /// During serialization of [`SerializeSeq`], that is, during processing of an array, this will be [`Some`].
    /// Most write problems are eliminated thanks to the separation of ptr-type meta byte writes and pointer-pointed data writes.
    ///
    /// The exception is `Array<StringPtr>`.
    /// This is a pointer type with a pointer type inside.
    ///
    /// To write this data, follow these steps:
    /// 1. Do a 16-byte alignment before starting to write the data in the Array.
    /// 2. Write the meta information of StringPtr for the length of the Array.
    /// 3. Write the data pointed to by the pointer of StringPtr.
    /// 4. Do a 16-byte alignment for StringPtr.
    str_array_buf: Option<Vec<std::ffi::CString>>,
}

/// Endianness and a common write process that takes into account whether the array is being serialized or not.
macro_rules! impl_serialize_primitive {
    ($method:ident, $value_type:ty, $write:ident) => {
        fn $method(self, v: $value_type) -> Result<Self::Ok, Self::Error> {
            let little_endian = self.endian.is_little();

            match little_endian {
                true => self.output.$write::<LittleEndian>(v),
                false => self.output.$write::<BigEndian>(v),
            }?;
            Ok(())
        }
    };
}

/// Endianness and a common write process that takes into account whether the array is being serialized or not.
macro_rules! impl_serialize_math {
    ($method:ident, $value_type:ty) => {
        fn $method(self, v: &$value_type) -> Result<Self::Ok, Self::Error> {
            let little_endian = self.endian.is_little();

            match little_endian {
                true => self.output.write(v.to_le_bytes().as_slice()),
                false => self.output.write(v.to_be_bytes().as_slice()),
            }?;
            Ok(())
        }
    };
}

/// Serialize to bytes
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = ByteSerializer::default();
    value.serialize(&mut serializer)?;
    Ok(serializer.output.into_inner())
}

impl<'a> Serializer for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeFlags = Self;

    fn serialize_void(self, _: ()) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_uint8(v as u8)?;
        Ok(())
    }

    /// Assume that the characters are ASCII characters. In that case, u8 is used to fit into 128 characters.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_int8(v as i8)?;
        Ok(())
    }

    fn serialize_int8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.write_i8(v)?;
        Ok(())
    }

    fn serialize_uint8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.write_u8(v)?;
        Ok(())
    }

    impl_serialize_primitive!(serialize_int16, i16, write_i16);
    impl_serialize_primitive!(serialize_uint16, u16, write_u16);

    impl_serialize_primitive!(serialize_int32, i32, write_i32);
    impl_serialize_primitive!(serialize_uint32, u32, write_u32);

    impl_serialize_primitive!(serialize_int64, i64, write_i64);
    impl_serialize_primitive!(serialize_uint64, u64, write_u64);

    impl_serialize_primitive!(serialize_real, f32, write_f32);

    impl_serialize_math!(serialize_vector4, Vector4);

    impl_serialize_math!(serialize_quaternion, Quaternion);
    impl_serialize_math!(serialize_matrix3, Matrix3);

    impl_serialize_math!(serialize_rotation, Rotation);
    impl_serialize_math!(serialize_matrix4, Matrix4);

    impl_serialize_math!(serialize_qstransform, QsTransform);
    impl_serialize_math!(serialize_transform, Transform);

    /// Pointer(Name attribute on XML) does not exist in bytes data(`.hkx`).
    fn serialize_pointer(self, v: Pointer) -> Result<Self::Ok, Self::Error> {
        self.serialize_ulong(v.get() as u64)?;
        Ok(())
    }

    fn serialize_array(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _class_meta: Option<(Pointer, Signature)>,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_variant(self, v: &Variant) -> Result<Self::Ok, Self::Error> {
        self.serialize_ulong(v.object as u64)?;
        self.serialize_ulong(v.class as u64)?;
        Ok(())
    }

    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok, Self::Error> {
        if let Some(s) = v {
            if s.is_empty() || s == "\u{2400}" {
                return Ok(());
            };
            self.serialize_stringptr(v)?;
        };
        Ok(())
    }

    fn serialize_ulong(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.target_platform.is_x86() {
            true => self.serialize_uint32(v as u32),
            false => self.serialize_uint64(v),
        }?;
        Ok(())
    }

    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags, Self::Error> {
        Ok(self)
    }

    fn serialize_half(self, v: f16) -> Result<Self::Ok, Self::Error> {
        self.serialize_uint16(v.to_bits())?;
        Ok(())
    }

    /// In the binary serialization of hkx, this is the actual data writing process beyond
    /// the pointer that is called only after all fields of the structure have been written.
    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error> {
        // Skip if `Option::None`(null pointer).
        if let Some(v) = v {
            let _local_dst = self.output.position() as usize;

            let c_string = std::ffi::CString::new(v.as_bytes()).map_err(Error::custom)?;
            match self.str_array_buf {
                Some(ref mut array_buf) => array_buf.push(c_string),
                None => {
                    // If it is not a StringPtr inside an Array, it must be written here because the pointers are
                    // not nested and there is no additional overhead.
                    let _ = self.output.write(c_string.as_bytes_with_nul())?;
                    self.output.align(16)?;
                }
            };
        };
        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_primitive_element<T>(
        &mut self,
        value: &T,
        _index: usize,
        _len: usize,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_class_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_string_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_ulong(0)?; // ptr size
        value.serialize(&mut **self)
    }

    /// In Byte Serializer, [`SerializeSeq`] is called only when writing the data pointed to by the pointer.
    /// When the data has been written, if it is a StringPtr, the actual state of the data must be written here.
    fn end(self) -> Result<()> {
        if let Some(c_strings) = self.str_array_buf.take() {
            for c_string in c_strings {
                self.output.write(c_string.as_bytes_with_nul())?;
                self.output.align(16)?;
            }
        };
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `StringPtr` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 4bytes, x64: 8bytes).
    fn serialize_string_meta_field<T>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_ulong(0) // ptr size
    }

    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `Array` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 12bytes, x64: 16bytes).
    fn serialize_array_meta_field<V, T>(&mut self, _key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let _array_start_pos = self.output.position() as usize; // TODO: local_fixup

        let size = value.as_ref().len() as u32;
        self.serialize_ulong(0)?; // ptr size
        self.serialize_uint32(size)?; // array size
        self.serialize_uint32(size | 0x80 << 24)?; // capacity | flags
        Ok(())
    }

    fn serialize_string_field<T>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn serialize_array_field<V, T>(&mut self, _key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        // The data pointed to by the Array pointer (`T* m_data`) must first be aligned 16 bytes before it is written.
        self.output.align(16)?;
        value.serialize(&mut **self)
    }

    /// Even if it is skipped on XML, it is not skipped because it exists in binary data.
    fn skip_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn skip_string_meta_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_string_meta_field(key, value)
    }

    #[inline]
    fn skip_array_meta_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        self.serialize_array_meta_field(key, value)
    }

    fn pad_field<T>(&mut self, x86_pads: &T, x64_pads: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        match self.target_platform.is_x86() {
            true => x86_pads.serialize(&mut **self),
            false => x64_pads.serialize(&mut **self),
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeFlags for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_empty_bit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn serialize_field<T>(&mut self, _key: &str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_bits<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::classes::*;

    #[test]
    fn test_serialize() {
        let hkb_project_string_data = HkbProjectStringData {
            _name: Some(54.into()),
            animation_filenames: vec![Some("Characters\\DefaultMale.hkx".into())],
            behavior_filenames: vec![],
            character_filenames: vec![],
            event_names: vec![],
            animation_path: Some("".into()),
            behavior_path: Some("".into()),
            character_path: Some("".into()),
            full_path_to_source: Some("".into()),
            root_path: Some("".into()),
            ..Default::default()
        };

        let classes = vec![
            Classes::HkbProjectStringData(hkb_project_string_data),
            // Classes::AllTypesTestClass(AllTypesTestClass {
            //     _name: Some(53.into()),
            //     ..Default::default()
            // }),
        ];

        rhexdump::rhexdump!(to_bytes(&classes).unwrap());
    }
}
