//! Bytes Serialization
mod trait_impls;

use crate::bytes::serde::section_header::SectionHeader;
use crate::common::trait_impls::Align as _;
use crate::error::{
    Error, InvalidEndianSnafu, MissingClassInClassnamesSectionSnafu, MissingGlobalFixupClassSnafu,
    MissingLocalFixupsSrcSnafu, Result, SubAbsOverflowSnafu, UnsupportedPtrSizeSnafu,
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
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Cursor, Write};
use trait_impls::{ClassNamesWriter as _, LocalFixupsWriter as _};
use zerocopy::{AsBytes, BigEndian, LittleEndian};

use super::serde::hkx_header::HkxHeader;

// TODO: It may be possible to cache classnames obtained at deserialization time.

/// Serialize to bytes
///
/// # Note
/// This serializer assumes the following.
/// - `contents_class_name_section_index`: It is always assumed to be 0.
/// - `contents_section_index`: It is always assumed to be 2.
pub fn to_bytes<O, K, V>(value: &IndexMap<K, V>, header: &HkxHeader<O>) -> Result<Vec<u8>>
where
    O: zerocopy::ByteOrder,
    K: core::fmt::Debug,
    V: Serialize + HavokClass,
{
    let mut serializer = ByteSerializer {
        is_little_endian: match header.endian {
            0 => false, // big endian
            1 => true,  // little endian
            invalid => InvalidEndianSnafu { invalid }.fail()?,
        },
        is_x86: match header.pointer_size {
            4 => true,
            8 => false,
            invalid => UnsupportedPtrSizeSnafu { invalid }.fail()?,
        },
        ..Default::default()
    };

    // 1/5: root header
    serializer.output.write(header.as_bytes())?;

    // 2/5: Section headers
    let section_offset = header.section_offset.get();
    SectionHeader::<O>::write_classnames(&mut serializer.output, section_offset)?;
    SectionHeader::<O>::write_types(&mut serializer.output, section_offset)?;
    let data_fixups_start = SectionHeader::<O>::write_data(&mut serializer.output)?;

    // 3/5: section contents
    // - `__classnames__` section
    serializer.class_starts = serializer
        .output
        .write_classnames_section::<O, K, V>(value)?;

    // Calculate absolute data offset
    // - The position after the `__classnames__` section write is the starting point of the data section, which is the abs_offset itself.
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
    let exports_offset = serializer.relative_position()?; // This is where the exports_offset is finally obtained.

    // Move back to fixup_offset of `__data__` section header.
    serializer.output.set_position(data_fixups_start);

    // 5/5 Write remain Fixup offsets for `__data__` section header.
    serializer
        .output
        .write_u32::<O>(serializer.abs_data_offset)?;

    #[cfg(feature = "tracing")]
    tracing::trace!(local_offset, global_offset, virtual_offset, exports_offset);

    serializer.output.write_u32::<O>(local_offset)?;
    serializer.output.write_u32::<O>(global_offset)?;
    serializer.output.write_u32::<O>(virtual_offset)?;
    serializer.output.write_u32::<O>(exports_offset)?;
    serializer.output.write_u32::<O>(exports_offset)?; // imports offset
    serializer.output.write_u32::<O>(exports_offset)?; // end offset

    Ok(serializer.output.into_inner())
}

/// Binary data serializer
#[derive(Debug, Default)]
pub struct ByteSerializer {
    /// Endianness of serialization target
    is_little_endian: bool,
    /// Ptr size of serialization target.
    /// 32bit: x86
    is_x86: bool,

    /// Bytes
    output: Cursor<Vec<u8>>,

    /// This is cached to find the relative position of the binary.
    abs_data_offset: u32,

    // ---- local fixup information
    /// - key: struct field name
    /// - value: local_fixup.src (i.e., the start position where the pointer size is written)
    local_fixups_name_src: HashMap<&'static str, u32>,
    /// Temporary standby location in local_fixup.src for iterators such as `Array<CString>`, `Array<StringPtr>`.
    ///
    /// A separate temporary save location is reserved in consideration of the possibility that it may be covered by the name of `field`.
    local_fixups_iter_src: Vec<u32>,
    /// Coordination information to associate a pointer of a pointer type of a field in a class with the data location to which it points.
    ///
    /// # Note
    /// All of these fixups are from the DATA SECTION.
    ///
    /// The following are not recorded in `local_fixup`.
    /// - If `Array<T>` is empty.
    /// - `CString`, `StringPtr` points to null ptr.
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
    /// C++ Class constructor positions map binary temporally buffer.
    ///
    /// Finally, write to the data for output.
    virtual_fixups: Vec<u8>,
    /// This information is needed in `virtual_fixup.name_offset`.
    ///
    /// This is created by writing the `__classnames__` section.
    /// - key: class name
    /// - value: class name start position
    class_starts: HashMap<&'static str, u32>,

    /// Used only when writing `Array<StringPtr>` or `Array<CString>`.
    ///
    /// # Details
    /// During serialization of [`SerializeSeq`], that is, during processing of an array, this will be [`Some`].
    /// Most write problems are eliminated thanks to the separation of ptr-type meta byte writes and pointer-pointed data writes.
    ///
    /// The exception is `Array<StringPtr>`.
    /// This is a pointer type with a pointer type inside.
    ///
    /// each `StringPtr` of `Array<StringPtr>` writing steps:
    ///
    /// First, Align(16-byte) once.
    ///
    /// Next Foreach `StringPtr`:
    /// 1. Write the meta(0 of Ptr size) of `StringPtr`.
    /// 2. Write the string of StringPtr.
    /// 3. Align(16-byte)
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

    /// Write a pair of local_fixups to the temporary local_fixups buffer
    ///
    /// # Info
    /// When dst is known, src is already known and can be written.
    fn write_local_fixup_pair(&mut self, key: &str, local_dst: u32) -> Result<()> {
        match self.local_fixups_name_src.get(key) {
            Some(local_src) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(key, local_src, local_dst);

                self.local_fixups.write_local_fixups(
                    *local_src,
                    local_dst,
                    self.is_little_endian,
                )?;
                Ok(())
            }
            None => MissingLocalFixupsSrcSnafu { dst: local_dst }.fail()?,
        }
    }

    /// Write a pair of local_fixups to the temporary local_fixups buffer
    ///
    /// # Info
    /// When dst is known, src is already known and can be written.
    fn write_iter_local_fixup_pair(&mut self, index: usize, local_dst: u32) -> Result<()> {
        match self.local_fixups_iter_src.get(index) {
            Some(local_src) => {
                #[cfg(feature = "tracing")]
                tracing::trace!(index, local_src, local_dst);

                self.local_fixups.write_local_fixups(
                    *local_src,
                    local_dst,
                    self.is_little_endian,
                )?;
                Ok(())
            }
            None => MissingLocalFixupsSrcSnafu { dst: local_dst }.fail()?,
        }
    }

    /// Write `global_fixups` of data section bytes to writer.
    ///
    /// # Info
    /// If all virtual_fixups are not obtained, references may not be available?
    ///
    /// # Note
    /// `global_fixup.dst` == `virtual_fixup.src`
    fn write_global_fixups(&mut self) -> Result<()> {
        for (ptr, g_src) in &self.global_fixups_ptr_src {
            if let Some(g_dst) = self.virtual_fixups_ptr_src.get(ptr) {
                #[cfg(feature = "tracing")]
                tracing::debug!("[global_fixups] src: {g_src}, dst: {g_dst}");

                match self.is_little_endian {
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

    /// Write to temporary virtual_fixup data.
    ///
    /// # Info
    /// Since the `class_name` and its location are known when the `__classnames__` section is written, the pair can be
    /// written the moment virtual_fixup.src is available.
    fn write_virtual_fixups_pair(
        &mut self,
        class_name: &'static str,
        virtual_src: u32,
    ) -> Result<()> {
        if let Some(class_name_offset) = self.class_starts.get(class_name) {
            match self.is_little_endian {
                true => {
                    self.virtual_fixups.write_u32::<LittleEndian>(virtual_src)?; // src
                    self.virtual_fixups.write_u32::<LittleEndian>(0)?; // dst_section_index, `__classnames__` section is 0
                    self.virtual_fixups
                        .write_u32::<LittleEndian>(*class_name_offset)?;
                    // dst(virtual_fixup.dst)
                }
                false => {
                    self.virtual_fixups.write_u32::<BigEndian>(virtual_src)?; // src
                    self.virtual_fixups.write_u32::<BigEndian>(0)?; // dst_section_index, `__classnames__` section is 0
                    self.virtual_fixups
                        .write_u32::<BigEndian>(*class_name_offset)?;
                    // dst(virtual_fixup.dst)
                }
            }
        } else {
            return MissingClassInClassnamesSectionSnafu { class_name }.fail();
        }
        Ok(())
    }

    /// Write all(`local`, `global` and `virtual`) fixups of data section.
    ///
    /// # Returns
    /// (`local_offset`, `global_offset`, `virtual_offset`)
    fn write_fixups(&mut self) -> Result<(u32, u32, u32)> {
        let local_offset = self.relative_position()?;
        self.output.write(&self.local_fixups)?;
        self.output.align(16, 0xff)?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            "[global_fixups]\nsources: {:#?},\ndestinations: {:#?}",
            self.global_fixups_ptr_src,
            self.virtual_fixups_ptr_src
        );
        let global_offset = self.relative_position()?;
        self.write_global_fixups()?;
        self.output.align(16, 0xff)?;

        let virtual_offset = self.relative_position()?;
        self.output.write(&self.virtual_fixups)?;
        self.output.align(16, 0xff)?;
        Ok((local_offset, global_offset, virtual_offset))
    }

    /// Write the internal data pointed to by the pointer of `CString` or `StringPtr`.
    pub fn serialize_cow(&mut self, v: &Option<Cow<'_, str>>) -> Result<()> {
        // Skip if `Option::None`(null pointer).
        if let Some(v) = v {
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

/// Endianness and a common write process that takes into account whether the array is being serialized or not.
macro_rules! impl_serialize_primitive {
    ($method:ident, $value_type:ty, $write:ident) => {
        fn $method(self, v: $value_type) -> Result<Self::Ok, Self::Error> {
            match self.is_little_endian {
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
            match self.is_little_endian {
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

    #[inline]
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
            let virtual_src = self.relative_position()?;

            self.virtual_fixups_ptr_src.insert(ptr, virtual_src); // For global_fixups
                                                                  // At this point, one set of values for virtual_fixup is known.
            self.write_virtual_fixups_pair(name, virtual_src)?;
        }
        Ok(self)
    }

    #[inline]
    fn serialize_variant(self, v: &Variant) -> Result<Self::Ok, Self::Error> {
        self.serialize_pointer(v.object)?;
        self.serialize_pointer(v.class)?;
        Ok(())
    }

    #[inline]
    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok, Self::Error> {
        self.serialize_cow(v.get_ref())
    }

    fn serialize_ulong(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.is_x86 {
            true => self.serialize_uint32(v as u32),
            false => self.serialize_uint64(v),
        }?;
        Ok(())
    }

    #[inline]
    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags, Self::Error> {
        Ok(self)
    }

    #[inline]
    fn serialize_half(self, v: f16) -> Result<Self::Ok, Self::Error> {
        self.serialize_uint16(v.to_bits())?;
        Ok(())
    }

    /// In the binary serialization of hkx, this is the actual data writing process beyond
    /// the pointer that is called only after all fields of the structure have been written.
    #[inline]
    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error> {
        self.serialize_cow(v.get_ref())
    }
}

impl<'a> SerializeSeq for &'a mut ByteSerializer {
    type Ok = ();
    type Error = Error;

    #[inline]
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
    #[inline]
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
    fn serialize_cstring_element(&mut self, value: &CString) -> Result<()> {
        let iter_src = self.relative_position()?;
        self.local_fixups_iter_src.push(iter_src);
        self.serialize_ulong(0)?; // ptr size

        // At this point, the data pointed to by the pointer is written to the temporary save
        // area. (Merged into output at the end of the array.
        if self.str_array_buf.is_none() {
            self.str_array_buf = Some(Vec::new());
        }
        value.serialize(&mut **self)
    }

    #[inline]
    fn serialize_stringptr_element(&mut self, value: &StringPtr) -> Result<()> {
        let iter_src = self.relative_position()?;
        self.local_fixups_iter_src.push(iter_src);
        self.serialize_ulong(0)?; // ptr size

        // At this point, the data pointed to by the pointer is written to the temporary save
        // area. (Merged into output at the end of the array.
        if self.str_array_buf.is_none() {
            self.str_array_buf = Some(Vec::new());
        }
        value.serialize(&mut **self)
    }

    /// In Byte Serializer, [`SerializeSeq`] is called only when writing the data pointed to by the pointer.
    /// When the data has been written, if it is a StringPtr, the actual state of the data must be written here.
    fn end(self) -> Result<()> {
        if let Some(strings) = self.str_array_buf.take() {
            for (index, string) in strings.iter().enumerate() {
                self.write_iter_local_fixup_pair(index, self.relative_position()?)?;

                self.output.write(string.as_bytes_with_nul())?;
                self.output.zero_fill_align(16)?;
            }
        };
        self.local_fixups_iter_src.clear();
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

    fn serialize_cstring_meta_field(
        &mut self,
        key: &'static str,
        value: &CString,
    ) -> Result<(), Self::Error> {
        if value.should_write_binary() {
            let str_start = self.relative_position()?;
            self.local_fixups_name_src.insert(key, str_start);
        };

        // Write meta fields
        self.serialize_ulong(0) // ptr size
    }

    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `StringPtr` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 4bytes, x64: 8bytes).
    fn serialize_stringptr_meta_field(
        &mut self,
        key: &'static str,
        value: &StringPtr,
    ) -> Result<(), Self::Error> {
        if value.should_write_binary() {
            let str_start = self.relative_position()?;
            self.local_fixups_name_src.insert(key, str_start);
        };

        // Write meta fields
        self.serialize_ulong(0) // ptr size
    }

    // 0x2c0
    /// In the binary serialization of hkx, we are at this stage writing each field of the structure.
    /// ptr type writes only the size of C++ `Array` here, since the data pointed to by the pointer
    /// will be written later.
    ///
    /// That is, ptr(x86: 12bytes, x64: 16bytes).
    fn serialize_array_meta_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        if !value.as_ref().is_empty() {
            // Ptr type need to pointing data position(local.dst).
            let array_start = self.relative_position()?;
            self.local_fixups_name_src.insert(key, array_start);
        };

        // Write Array meta field
        let size = value.as_ref().len() as u32;
        self.serialize_ulong(0)?; // ptr size
        self.serialize_uint32(size)?; // array size
        self.serialize_uint32(size | 0x80 << 24)?; // capacity | Owned flag
        Ok(())
    }

    /// Write `T` of `T* m_data`.
    #[inline]
    fn serialize_cstring_field(
        &mut self,
        key: &'static str,
        value: &CString,
    ) -> Result<(), Self::Error> {
        let str_dst = self.relative_position()?;
        self.write_local_fixup_pair(key, str_dst)?;

        value.serialize(&mut **self)
    }

    /// Write `T` of `T* m_data`.
    #[inline]
    fn serialize_stringptr_field(
        &mut self,
        key: &'static str,
        value: &StringPtr,
    ) -> Result<(), Self::Error> {
        let str_dst = self.relative_position()?;
        self.write_local_fixup_pair(key, str_dst)?;

        value.serialize(&mut **self)
    }

    fn serialize_array_field<V, T>(&mut self, key: &'static str, value: V) -> Result<()>
    where
        V: AsRef<[T]> + Serialize,
        T: Serialize,
    {
        // The data pointed to by the Array pointer (`T* m_data`) must first be aligned 16 bytes before it is written.
        self.output.zero_fill_align(16)?;

        if !value.as_ref().is_empty() {
            // The actual data location, i.e., the data position pointed to by ptr. It is local_fixup.dst.
            let array_dst = self.relative_position()?;
            self.write_local_fixup_pair(key, array_dst)?;
        }

        value.serialize(&mut **self)
    }

    /// Even if it is skipped on XML, it is not skipped because it exists in binary data.
    #[inline]
    fn skip_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    /// This skip is for XML. Binary data must be written as usual.
    #[inline]
    fn skip_cstring_meta_field(&mut self, key: &'static str, value: &CString) -> Result<()> {
        self.serialize_cstring_meta_field(key, value)
    }

    /// This skip is for XML. Binary data must be written as usual.
    #[inline]
    fn skip_stringptr_meta_field(&mut self, key: &'static str, value: &StringPtr) -> Result<()> {
        self.serialize_stringptr_meta_field(key, value)
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
        match self.is_x86 {
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

    /// Ignore this method because it is an XML method.
    #[inline]
    fn serialize_field<T>(&mut self, _key: &str, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    #[inline]
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
    use crate::{
        bytes::serde::hkx_header::HkxHeader,
        common::mocks::{classes::*, constructors::new_defaultmale},
    };

    #[test]
    #[quick_tracing::try_init(test = "serialize_bytes")]
    fn test_serialize() -> Result<()> {
        let mut classes = new_defaultmale();

        // For binary writing, the youngest pointer index must be first after sorting in reverse order.
        // Usually a shift operation is required, but a dummy and a swap can speed up the process.
        classes.insert(usize::MAX, Classes::PhantomData);
        classes.sort_by(|k_1, _v_1, k_2, _v_2| k_2.cmp(&k_1)); // Reverse order
        classes.swap_indices(0, classes.len() - 1);
        let _ = classes.pop();
        tracing::debug!("{classes:#?}");

        let bytes = to_bytes(&classes, &HkxHeader::new_skyrim_se())?;
        let actual = rhexdump::rhexdumps!(bytes);
        tracing::debug!("\n{actual}");

        let expected = rhexdump::rhexdumps!(include_bytes!(
            "../../../../docs/handson_hex_dump/defaultmale/defaultmale.hkx"
        ));
        pretty_assertions::assert_eq!(actual, expected);
        Ok(())
    }
}
