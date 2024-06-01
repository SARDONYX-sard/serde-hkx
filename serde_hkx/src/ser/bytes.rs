//! Bytes Serialization
use crate::common::bytes::hkx_header::HkxHeader;
use crate::common::bytes::section_header::SectionHeader;
use crate::cursor_ext::{Align as _, HavokWrite};
use crate::error::{
    Error, MissingClassInClassnamesSectionSnafu, MissingGlobalFixupClassSnafu, Result,
    SubAbsOverflowSnafu,
};
use byteorder::WriteBytesExt as _;
use havok_serde::ser::{
    Error as _, Serialize, SerializeFlags, SerializeSeq, SerializeStruct, Serializer,
};
use havok_serde::HavokClass;
use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Variant, Vector4,
};
use indexmap::IndexMap;
use snafu::ensure;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use zerocopy::{AsBytes, BigEndian, LittleEndian};

// TODO: If we have the classnames information at deserialization time, we can reuse it and may not need this loop. If that happens, you should create another method such as `to_bytes_with_cache`.
/// Serialize to bytes
pub fn to_bytes<T, O>(value: &[T], header: HkxHeader<O>) -> Result<Vec<u8>>
where
    T: Serialize + HavokClass,
    O: zerocopy::ByteOrder,
{
    let mut serializer = ByteSerializer::default();

    // 1/5: root header
    serializer.output.write(header.as_bytes())?;

    // 2/5: Section headers
    let section_offset = header.section_offset.get();
    SectionHeader::<O>::write_classnames(&mut serializer.output, section_offset)?;
    SectionHeader::<O>::write_types(&mut serializer.output, section_offset)?;
    let data_fixups_start = SectionHeader::<O>::write_data(&mut serializer.output)?;

    // 3/5: section contents
    // Need `class_starts` to write `virtual_fixups`
    serializer.class_starts = serializer.output.write_classnames_section::<T, O>(value)?;

    // Calculate absolute data offset
    // - The position after the `classnames__` section write is the starting point of the data section, which is the abs_offset itself.
    //   Therefore, abs_offset must be calculated at this point.
    let section_offset = match header.section_offset.get() {
        ..=0 => 0,
        others => others as u32,
    };
    serializer.abs_data_offset = section_offset + serializer.output.position() as u32;

    // - `__data__` section
    value.serialize(&mut serializer)?;

    // 4/5: Write fixups_offsets of `__data__` section header.
    let (local_offset, global_offset, virtual_offset) = serializer.write_fixups()?; // Write local, global and virtual fixups
    let exports_offset = serializer.relative_position()? as u32; // This is where the exports_offset is finally obtained.

    // Move back to fixup_offset of `__data__` section header.
    serializer.output.set_position(data_fixups_start);

    // 5/5 Write remain Fixup offsets for `__data__` section header.
    serializer
        .output
        .write_u32::<O>(serializer.abs_data_offset)?;
    serializer.output.write_u32::<O>(local_offset)?;
    serializer.output.write_u32::<O>(global_offset)?;
    serializer.output.write_u32::<O>(virtual_offset)?;
    serializer.output.write_u32::<O>(exports_offset)?;
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

    abs_data_offset: u32,

    /// Coordination information to associate a pointer of a pointer type of a field in a class with the data location to which it points.
    ///
    /// # Note
    /// All of these fixups are from the DATA SECTION.
    local_fixups: Vec<u8>,

    // ---- Global fixup information
    /// A map that holds the src of global_fixups until the dst of virtual_fixups is known.
    /// - key: Unique class pointer.(e.g. XML: #0050 -> 50)
    /// - value: Starting point of the binary for which the pointer class write is requested.
    ///
    /// # Note
    /// These are fixups of the data section.
    global_fixups_ptr_src: IndexMap<Pointer, u32>,
    /// The `global_fixup.dst` == `virtual_fixup.src`.
    ///
    /// Therefore, the write start position must be retained.
    /// The position of dst of global_fixup will be known after all the binary data of all classes are written.
    /// - key: Unique class pointer.(e.g. XML: #0050 -> 50)
    /// - value: Starting point where Havok Class binary data is written.
    ///
    /// # Note
    /// All of these fixups are from the DATA SECTION.
    virtual_fixups_ptr_src: HashMap<Pointer, u32>,

    // ---- Virtual fixup information
    /// - key: class name
    /// - value: virtual_fixup.src(start position)
    virtual_fixups_name_src: IndexMap<&'static str, u32>,
    /// This information is needed in `virtual_fixup.name_offset`.
    ///
    /// This is found when writing the `__classnames__` section.
    /// - key: class name
    /// - value: class name start position
    class_starts: HashMap<&'static str, u32>,

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
    /// Get the position relative to the start of the `__data__` section.
    #[inline]
    fn relative_position(&self) -> Result<u32> {
        let position = self.output.position() as u32;
        ensure!(
            position >= self.abs_data_offset,
            SubAbsOverflowSnafu {
                position,
                abs_data_offset: self.abs_data_offset
            }
        );

        Ok(position - self.abs_data_offset)
    }

    /// Write `global_fixups` of data section bytes to writer.
    fn write_global_fixups(&mut self) -> Result<()> {
        #[cfg(feature = "tracing")]
        {
            tracing::debug!(global_fixups_ptr_src = ?&self.global_fixups_ptr_src);
            tracing::debug!(virtual_fixups_ptr_src = ?&self.virtual_fixups_ptr_src);
        }

        self.global_fixups_ptr_src.sort_keys();
        for (ptr, g_src) in &self.global_fixups_ptr_src {
            // NOTE: `global_fixup.dst` == `virtual_fixup.src`
            if let Some(g_dst) = self.virtual_fixups_ptr_src.get(ptr) {
                match self.endian.is_little() {
                    true => {
                        self.output.write_u32::<LittleEndian>(*g_src)?; // src
                        self.output.write_u32::<LittleEndian>(2)?; // dst_section_index
                        self.output.write_u32::<LittleEndian>(*g_dst)?; // dst(virtual_fixup.dst)
                    }
                    false => {
                        self.output.write_u32::<BigEndian>(*g_src)?; // src
                        self.output.write_u32::<BigEndian>(2)?; // dst_section_index
                        self.output.write_u32::<BigEndian>(*g_dst)?; // dst(virtual_fixup.dst)
                    }
                }
            } else {
                return MissingGlobalFixupClassSnafu {
                    ptr: ptr.to_string(),
                }
                .fail();
            }
        }
        Ok(())
    }

    /// Write `virtual_fixups` of data section bytes to writer.
    fn write_virtual_fixups(&mut self) -> Result<()> {
        for (class_name, v_src) in &self.virtual_fixups_name_src {
            if let Some(class_name_offset) = self.class_starts.get(class_name) {
                match self.endian.is_little() {
                    true => {
                        self.output.write_u32::<LittleEndian>(*v_src)?; // src
                        self.output.write_u32::<LittleEndian>(0)?; // dst_section_index, `__classnames__` section is 0
                        self.output.write_u32::<LittleEndian>(*class_name_offset)?;
                        // dst(virtual_fixup.dst)
                    }
                    false => {
                        self.output.write_u32::<BigEndian>(*v_src)?; // src
                        self.output.write_u32::<BigEndian>(0)?; // dst_section_index, `__classnames__` section is 0
                        self.output.write_u32::<BigEndian>(*class_name_offset)?;
                        // dst(virtual_fixup.dst)
                    }
                }
            } else {
                return MissingClassInClassnamesSectionSnafu { class: *class_name }.fail();
            }
        }
        Ok(())
    }

    /// Write all(`local`, `global` and `virtual`) fixups of data section.
    fn write_fixups(&mut self) -> Result<(u32, u32, u32)> {
        let local_offset = self.relative_position()?;
        self.output.write(&self.local_fixups)?;
        self.output.align(16, 0xff)?;

        let global_offset = self.relative_position()?;
        self.write_global_fixups()?;
        self.output.align(16, 0xff)?;

        let virtual_offset = self.relative_position()?;
        self.write_virtual_fixups()?;
        self.output.align(16, 0xff)?;
        Ok((local_offset, global_offset, virtual_offset))
    }
}

/// Endianness and a common write process that takes into account whether the array is being serialized or not.
macro_rules! impl_serialize_primitive {
    ($method:ident, $value_type:ty, $write:ident) => {
        fn $method(self, v: $value_type) -> Result<Self::Ok, Self::Error> {
            match self.endian.is_little() {
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
            match self.endian.is_little() {
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
    /// Assume that the characters are ASCII characters`c_char`. In that case, i8 is used to fit into 128 characters.
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
        let start = self.relative_position()?;
        self.global_fixups_ptr_src.insert(ptr, start);

        self.serialize_ulong(0 as u64)?;
        Ok(())
    }

    fn serialize_array(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    /// This is called in the Havok Class array or HashMap, or in a serializer in a field.
    /// Classes in the field may be inlined, in which case `class_meta` will be [`None`].
    ///
    /// # what's `class_meta`?
    /// If `class_meta` exists, it is when writing an `hkobject` with the `name` attribute in XML.
    /// This must be written to virtual_fixup so that the constructor can be called.
    fn serialize_struct(
        self,
        name: &'static str,
        class_meta: Option<(Pointer, Signature)>,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if let Some((ptr, _)) = class_meta {
            let start_pos = self.relative_position()?; // Write virtual_fixup source position.
            self.virtual_fixups_ptr_src.insert(ptr, start_pos);
            self.virtual_fixups_name_src.insert(name, start_pos);
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
            let local_dst = self.relative_position()?;

            #[cfg(feature = "tracing")]
            tracing::trace!("local_dst = {local_dst:#0x})");

            match self.endian.is_little() {
                true => self.local_fixups.write_u32::<LittleEndian>(local_dst)?,
                false => self.local_fixups.write_u32::<BigEndian>(local_dst)?,
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

    #[inline]
    /// This method is called on HavokClasses array.(Write start)
    ///
    /// Therefore, it is necessary to record the write position of this in virtual_fixup.
    fn serialize_class_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_math_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    #[inline]
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
        let local_src = self.relative_position()?;
        #[cfg(feature = "tracing")]
        tracing::trace!("local_src = {local_src:#0x})");

        match self.endian.is_little() {
            true => self.local_fixups.write_u32::<LittleEndian>(local_src)?,
            false => self.local_fixups.write_u32::<BigEndian>(local_src)?,
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
        let local_src = self.relative_position()?;
        #[cfg(feature = "tracing")]
        tracing::trace!("local_src = {local_src:#0x})");

        match self.endian.is_little() {
            true => self.local_fixups.write_u32::<LittleEndian>(local_src)?,
            false => self.local_fixups.write_u32::<BigEndian>(local_src)?,
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
        T: ?Sized + AsRef<[u8]>,
    {
        match self.target_platform.is_x86() {
            true => self.output.write(x86_pads.as_ref()),
            false => self.output.write(x64_pads.as_ref()),
        }?;
        Ok(())
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
    use crate::common::mocks::{classes::*, enums::EventMode};

    #[test]
    fn test_serialize() {
        let _guard = tracing_easy::init();

        let hk_root_level_container = HkRootLevelContainer {
            _name: Some(50.into()),
            named_variants: vec![HkRootLevelContainerNamedVariant {
                _name: None,
                name: Some("hkbProjectData".into()),
                class_name: Some("hkbProjectData".into()),
                variant: Pointer::new(51),
            }],
        };

        let hkb_project_data = HkbProjectData {
            _name: Some(51.into()),
            world_up_ws: Vector4::new(0.0, 0.0, 1.0, 0.0),
            string_data: Pointer::new(52),
            default_event_mode: EventMode::EventModeIgnoreFromGenerator,
            ..Default::default()
        };

        let hkb_project_string_data = HkbProjectStringData {
            _name: Some(52.into()),
            animation_filenames: vec![Some("Characters\\DefaultMale.hkx".into())],
            behavior_filenames: vec![],
            character_filenames: vec![],
            event_names: vec![],
            animation_path: Some("".into()),
            behavior_path: Some("".into()),
            character_path: Some("".into()),
            full_path_to_source: Some("".into()),
            root_path: None,
            ..Default::default()
        };

        let classes = vec![
            Classes::HkRootLevelContainer(hk_root_level_container),
            Classes::HkbProjectData(hkb_project_data),
            Classes::HkbProjectStringData(hkb_project_string_data),
        ];

        rhexdump::rhexdump!(to_bytes(&classes, HkxHeader::new_skyrim_se()).unwrap());
    }
}
