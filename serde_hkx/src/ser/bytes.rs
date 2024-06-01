//! Bytes Serialization
use crate::common::bytes::hkx_header::HkxHeader;
use crate::common::bytes::section_header::SectionHeader;
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
use std::collections::HashMap;
use std::io::{Cursor, Write};
use zerocopy::{AsBytes, BigEndian, LittleEndian};

/// Serialize to bytes
pub fn to_bytes<T, O>(value: &T, header: HkxHeader<O>) -> Result<Vec<u8>>
where
    T: Serialize,
    O: zerocopy::ByteOrder,
{
    let mut serializer = ByteSerializer::default();

    // 1/6: root header
    serializer.output.write(header.as_bytes())?;

    // Section headers
    let section_offset = header.section_offset.get();
    // 2/6: classnames section header
    SectionHeader::<O>::write_classnames(&mut serializer.output, section_offset)?;
    // 3/6: types section header
    SectionHeader::<O>::write_types(&mut serializer.output, section_offset)?;
    // 4/6: data section header
    let fixups_offset_header =
        SectionHeader::<O>::write_data(&mut serializer.output, section_offset)?;

    // section contents
    // TODO: 5/6: `__classnames__` section
    // 6/6:  `__data__` section
    value.serialize(&mut serializer)?;

    // 7/7: Write fixups_offsets of `__data__` section header.
    let (local_offset, global_offset, virtual_offset) = serializer.write_fixups()?; // Write local, global and virtual fixups

    // Move back to fixup_offset of `__data__` section header.
    let exports_offset = serializer.output.position() as u32;
    serializer.output.set_position(fixups_offset_header);

    // Fixup offsets for `__data__` section header.
    serializer.output.write_u32::<O>(local_offset)?;
    serializer.output.write_u32::<O>(global_offset)?;
    serializer.output.write_u32::<O>(virtual_offset)?;
    serializer.output.write_u32::<O>(exports_offset)?; // exports offset
    serializer.output.write_u32::<O>(exports_offset)?; // imports offset
    serializer.output.write_u32::<O>(exports_offset)?; // end offset

    Ok(serializer.output.into_inner())
}

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

    /// Coordination information to associate a pointer of a pointer type of a field in a class with the data location to which it points.
    ///
    /// # Note
    /// All of these fixups are from the DATA SECTION.
    local_fixups: Vec<u8>,
    /// Coordination information to associate the starting point of the class field write with the class name that must be called the class constructor.
    /// # Note
    /// All of these fixups are from the DATA SECTION.
    virtual_fixups: Vec<u8>,

    /// A map that holds the src of global_fixups until the dst of virtual_fixups is known.
    /// - key: Unique class pointer.(e.g. XML: #0050 -> 50)
    /// - value: Starting point of the binary for which the pointer class write is requested.
    ///
    /// # Note
    /// These are fixups of the data section.
    global_fixups_src_ref: HashMap<Pointer, u32>,
    /// The `dst` of `global_fixup` is the write start position of `virtual_fixup`.
    ///
    /// Therefore, the write start position must be retained.
    /// The position of dst of global_fixup will be known after all the binary data of all classes are written.
    /// - key: Unique class pointer.(e.g. XML: #0050 -> 50)
    /// - value: Starting point where Havok Class binary data is written.
    ///
    /// # Note
    /// All of these fixups are from the DATA SECTION.
    virtual_fixups_dst_ref: HashMap<Pointer, u32>,

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

impl ByteSerializer {
    /// Write `global_fixups` of data section bytes to writer.
    fn write_global_fixups(&mut self) -> Result<()> {
        for (ptr, g_src) in &self.global_fixups_src_ref {
            // NOTE: `global_fixup.dst` == `virtual_fixup.src`
            let g_dst = self.virtual_fixups_dst_ref.get(ptr);

            if let Some(g_dst) = g_dst {
                match self.endian.is_little() {
                    true => {
                        self.output.write_u32::<LittleEndian>(*g_src)?;
                        self.output.write_u32::<LittleEndian>(*g_dst)?;
                    }
                    false => {
                        self.output.write_u32::<LittleEndian>(*g_src)?;
                        self.output.write_u32::<LittleEndian>(*g_dst)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Write all(`local`, `global` and `virtual`) fixups of data section.
    fn write_fixups(&mut self) -> Result<(u32, u32, u32)> {
        let local_offset = self.output.position() as u32;
        self.output.write(&self.local_fixups)?;
        self.output.align(16, 0xff)?;

        let global_offset = self.output.position() as u32;
        self.write_global_fixups()?;
        self.output.align(16, 0xff)?;

        let virtual_offset = self.output.position() as u32;
        self.output.write(&self.virtual_fixups)?;
        self.output.align(16, 0xff)?;
        Ok((local_offset, global_offset, virtual_offset))
    }
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

impl<'a> Serializer for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeFlags = Self;

    #[inline]
    fn serialize_void(self, _: ()) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_uint8(v as u8)?;
        Ok(())
    }

    #[inline]
    /// Assume that the characters are ASCII characters. In that case, u8 is used to fit into 128 characters.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_int8(v as i8)?;
        Ok(())
    }

    #[inline]
    fn serialize_int8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.output.write_i8(v)?;
        Ok(())
    }

    #[inline]
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
    fn serialize_pointer(self, ptr: Pointer) -> Result<Self::Ok, Self::Error> {
        // Write global_fixup src(write start) position.
        let start = self.output.position() as u32;
        self.global_fixups_src_ref.insert(ptr, start);

        self.serialize_ulong(ptr.get() as u64)?;
        Ok(())
    }

    fn serialize_array(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    /// This is called in the Havok Class array or HashMap, or in a serializer in a field.
    /// Classes in the field may be inlined, in which case `class_meta` will be [`None`].
    fn serialize_struct(
        self,
        _name: &'static str,
        class_meta: Option<(Pointer, Signature)>,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        // If `class_meta` exists, it is when writing an `hkobject` with the `name` attribute in XML.
        // This must be written to virtual_fixup so that the constructor can be called.
        if let Some((ptr, _)) = class_meta {
            // Write virtual_fixup source position.
            let start_pos = self.output.position() as u32;
            match self.endian.is_little() {
                true => self.virtual_fixups.write_u32::<LittleEndian>(start_pos)?,
                false => self.virtual_fixups.write_u32::<BigEndian>(start_pos)?,
            };

            self.virtual_fixups_dst_ref.insert(ptr, start_pos);
        }

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
            // Write local destination position.
            let start = self.output.position() as u32;
            match self.endian.is_little() {
                true => self.local_fixups.write_u32::<LittleEndian>(start)?,
                false => self.local_fixups.write_u32::<BigEndian>(start)?,
            };

            let c_string = std::ffi::CString::new(v.as_bytes()).map_err(Error::custom)?;
            match self.str_array_buf {
                Some(ref mut array_buf) => array_buf.push(c_string),
                None => {
                    // If it is not a StringPtr inside an Array, it must be written here because the pointers are
                    // not nested and there is no additional overhead.
                    let _ = self.output.write(c_string.as_bytes_with_nul())?;
                    self.output.zero_fill_align(16)?;
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

    /// This method is called on HavokClasses array.(Write start)
    ///
    /// Therefore, it is necessary to record the write position of this in virtual_fixup.
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
                self.output.zero_fill_align(16)?;
            }
        };
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    #[inline]
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
        // Write local_fixup source position,
        // because it is a pointer type, the position of the pointer and the write position of the data it points
        // to must be noted in local_fixup.
        let start_pos = self.output.position() as u32;
        match self.endian.is_little() {
            true => self.local_fixups.write_u32::<LittleEndian>(start_pos)?,
            false => self.local_fixups.write_u32::<BigEndian>(start_pos)?,
        };

        // Write meta fields
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
        // Write local_fixup source position,
        // because it is a pointer type, the position of the pointer and the write position of the data it points
        // to must be noted in local_fixup.
        let start_pos = self.output.position() as u32;
        match self.endian.is_little() {
            true => self.local_fixups.write_u32::<LittleEndian>(start_pos)?,
            false => self.local_fixups.write_u32::<BigEndian>(start_pos)?,
        };

        // Write Array meta field
        let size = value.as_ref().len() as u32;
        self.serialize_ulong(0)?; // ptr size
        self.serialize_uint32(size)?; // array size
        self.serialize_uint32(size | 0x80 << 24)?; // capacity | flags
        Ok(())
    }

    /// Write `T` of `T* m_data`.
    #[inline]
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
        self.output.zero_fill_align(16)?;
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

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> SerializeFlags for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

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

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::mocks::classes::*;

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
            Classes::AllTypesTestClass(AllTypesTestClass {
                _name: Some(53.into()),
                ..Default::default()
            }),
        ];

        rhexdump::rhexdump!(to_bytes(&classes, HkxHeader::new_skyrim_se()).unwrap());
    }
}
