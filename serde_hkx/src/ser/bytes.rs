//! Bytes Serialization
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

/// Binary target platform
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    /// x86
    Win32,
    /// x86_64
    #[default]
    Amd64,
}

/// Binary data serializer
#[derive(Debug, Default)]
pub struct ByteSerializer {
    endian: ByteOrder,
    target_platform: Platform,
    /// Bytes
    output: Cursor<Vec<u8>>,
}

impl ByteSerializer {
    /// Serializer is little endian mode?
    fn is_little_endian(&self) -> bool {
        self.endian == ByteOrder::LittleEndian
    }

    /// Serializer is win32(x86) mode?
    fn is_x86(&self) -> bool {
        self.target_platform == Platform::Win32
    }

    /// Align bytes.
    fn align(&mut self, align: usize) -> Result<()> {
        let position = self.output.position() as usize;
        let offset = position % align;

        if offset != 0 {
            debug_assert!(align >= offset);
            let padding = align - offset;

            // Length cannot be determined at compile time, so vec must be used.
            let padding_bytes = vec![0u8; padding];
            self.output.write_all(&padding_bytes)?;
        }
        Ok(())
    }
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

    fn serialize_int16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_i16::<LittleEndian>(v),
            false => self.output.write_i16::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_uint16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_u16::<LittleEndian>(v),
            false => self.output.write_u16::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_int32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_i32::<LittleEndian>(v),
            false => self.output.write_i32::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_uint32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_u32::<LittleEndian>(v),
            false => self.output.write_u32::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_int64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_i64::<LittleEndian>(v),
            false => self.output.write_i64::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_uint64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_u64::<LittleEndian>(v),
            false => self.output.write_u64::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_real(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write_f32::<LittleEndian>(v),
            false => self.output.write_f32::<BigEndian>(v),
        }?;
        Ok(())
    }

    fn serialize_vector4(self, v: &Vector4) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(v.to_le_bytes().as_slice()),
            false => self.output.write(v.to_be_bytes().as_slice()),
        }?;
        Ok(())
    }

    fn serialize_quaternion(self, v: &Quaternion) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(v.to_le_bytes().as_slice()),
            false => self.output.write(v.to_be_bytes().as_slice()),
        }?;
        Ok(())
    }

    fn serialize_matrix3(self, v: &Matrix3) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(v.to_le_bytes().as_slice()),
            false => self.output.write(v.to_be_bytes().as_slice()),
        }?;
        Ok(())
    }

    fn serialize_rotation(self, v: &Rotation) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(v.to_le_bytes().as_slice()),
            false => self.output.write(v.to_le_bytes().as_slice()),
        }?;
        Ok(())
    }

    fn serialize_qstransform(self, v: &QsTransform) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(&v.to_le_bytes()[..]),
            false => self.output.write(&v.to_be_bytes()[..]),
        }?;
        Ok(())
    }

    fn serialize_matrix4(self, v: &Matrix4) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(v.to_le_bytes().as_slice()),
            false => self.output.write(v.to_be_bytes().as_slice()),
        }?;
        Ok(())
    }

    fn serialize_transform(self, v: &Transform) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.write(v.to_le_bytes().as_slice()),
            false => self.output.write(v.to_le_bytes().as_slice()),
        }?;
        Ok(())
    }

    /// Pointer(Name attribute on XML) does not exist in bytes data(`.hkx`).
    fn serialize_pointer(self, _: Pointer) -> Result<Self::Ok, Self::Error> {
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
        self.serialize_stringptr(v)
    }

    fn serialize_ulong(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.is_x86() {
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

    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error> {
        self.serialize_ulong(0)?; // Must alloc ptr size.
        let c_string = std::ffi::CString::new(v.as_bytes()).map_err(Error::custom)?;
        self.output.write(&c_string.into_bytes())?;
        self.align(16)
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
        value.serialize(&mut **self)?;
        Ok(())
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
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
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
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_array_field<V, T>(&mut self, _key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        let size = value.as_ref().len() as u32;
        self.serialize_ulong(0)?; // ptr size
        self.serialize_uint32(size)?; // array size
        self.serialize_uint32(size | 0x80 << 24)?; // capacity | flags
        value.serialize(&mut **self)?;
        Ok(())
    }

    /// Even if it is skipped on XML, it is not skipped because it exists in binary data.
    fn skip_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn pad_field<T>(&mut self, pads: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        pads.serialize(&mut **self)
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

    fn serialize_field<T>(&mut self, _key: &str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
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
        let classes = vec![
            Classes::HkbProjectStringData(HkbProjectStringData {
                _name: Some(54.into()),
                animation_filenames: vec!["Hi".into(), "Hello".into(), "World".into()],
                ..Default::default()
            }),
            //
            Classes::AllTypesTestClass(AllTypesTestClass {
                _name: Some(53.into()),
                ..Default::default()
            }),
        ];

        rhexdump::rhexdump!(to_bytes(&classes).unwrap());
    }
}
