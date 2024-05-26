//! Bytes Serialization
use std::collections::HashMap;

use crate::error::{Error, Result};
use bytes::{BufMut, BytesMut};
use havok_serde::ser::{
    Error as _, Serialize, SerializeFlags, SerializeSeq, SerializeStruct, Serializer,
};
use havok_types::{
    f16, variant::Variant, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation,
    Signature, StringPtr, Transform, Vector4,
};

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
    output: BytesMut,
    /// current reader position, actual data position
    local_fixups: HashMap<u32, u32>,
}

impl ByteSerializer {
    /// Serializer is little endian mode?
    pub fn is_little_endian(&self) -> bool {
        self.endian == ByteOrder::LittleEndian
    }

    /// Serializer is win32(x86) mode?
    pub fn is_x86(&self) -> bool {
        self.target_platform == Platform::Win32
    }

    pub fn current_position(&self) -> u32 {
        self.output.len() as u32
    }
}

/// Serialize to bytes
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = ByteSerializer::default();

    value.serialize(&mut serializer)?;

    Ok(serializer.output.into())
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
        self.output.put_u8(v as u8);
        Ok(())
    }

    /// Assume that the characters are ASCII characters. In that case, u8 is used to fit into 128 characters.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.output.put_u8(v as u8);
        Ok(())
    }

    fn serialize_int8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.put_i8(v);
        Ok(())
    }

    fn serialize_uint8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.output.put_u8(v);
        Ok(())
    }

    fn serialize_int16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_i16_le(v),
            false => self.output.put_i16(v),
        }
        Ok(())
    }

    fn serialize_uint16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_u16_le(v),
            false => self.output.put_u16(v),
        }
        Ok(())
    }

    fn serialize_int32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_i32_le(v),
            false => self.output.put_i32(v),
        }
        Ok(())
    }

    fn serialize_uint32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_u32_le(v),
            false => self.output.put_u32(v),
        }
        Ok(())
    }

    fn serialize_int64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_i64_le(v),
            false => self.output.put_i64(v),
        }
        Ok(())
    }

    fn serialize_uint64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_u64_le(v),
            false => self.output.put_u64(v),
        }
        Ok(())
    }

    fn serialize_real(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_f32_le(v),
            false => self.output.put_f32(v),
        }
        Ok(())
    }

    fn serialize_vector4(self, v: &Vector4) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(&v.to_le_bytes()[..]),
            false => self.output.put(&v.to_be_bytes()[..]),
        }
        Ok(())
    }

    fn serialize_quaternion(self, v: &Quaternion) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(&v.to_le_bytes()[..]),
            false => self.output.put(&v.to_be_bytes()[..]),
        }
        Ok(())
    }

    fn serialize_matrix3(self, v: &Matrix3) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(&v.to_le_bytes()[..]),
            false => self.output.put(&v.to_be_bytes()[..]),
        }
        Ok(())
    }

    fn serialize_rotation(self, v: &Rotation) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(v.to_le_bytes().as_slice()),
            false => self.output.put(v.to_le_bytes().as_slice()),
        }
        Ok(())
    }

    fn serialize_qstransform(self, v: &QsTransform) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(&v.to_le_bytes()[..]),
            false => self.output.put(&v.to_be_bytes()[..]),
        }
        Ok(())
    }

    fn serialize_matrix4(self, v: &Matrix4) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(&v.to_le_bytes()[..]),
            false => self.output.put(&v.to_be_bytes()[..]),
        }
        Ok(())
    }

    fn serialize_transform(self, v: &Transform) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put(v.to_le_bytes().as_slice()),
            false => self.output.put(v.to_le_bytes().as_slice()),
        }
        Ok(())
    }

    /// Pointer(Name attribute on XML) does not exist in bytes data(`.hkx`).
    fn serialize_pointer(self, _: Pointer) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_array(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.serialize_ulong(0)?;
        self.serialize_uint32(0 | 80 << 24)?;
        self.local_fixups.insert(self.current_position(), 0);
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
        if self.is_x86() {
            match self.is_little_endian() {
                true => self.output.put_u32_le(v as u32),
                false => self.output.put_u32(v as u32),
            }
        } else {
            match self.is_little_endian() {
                true => self.output.put_u64_le(v),
                false => self.output.put_u64(v),
            }
        }
        Ok(())
    }

    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags, Self::Error> {
        Ok(self)
    }

    fn serialize_half(self, v: f16) -> Result<Self::Ok, Self::Error> {
        match self.is_little_endian() {
            true => self.output.put_u16_le(v.to_bits()),
            false => self.output.put_u16(v.to_bits()),
        }
        Ok(())
    }

    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error> {
        let c_string = std::ffi::CString::new(v.as_bytes()).map_err(Error::custom)?;
        self.output.put(c_string.as_bytes());
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
        let _len = value.as_ref().len();
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn skip_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Even if it is skipped on XML, it is not skipped because it exists in binary data.
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

        println!("{:?}", to_bytes(&classes).unwrap());
    }
}
