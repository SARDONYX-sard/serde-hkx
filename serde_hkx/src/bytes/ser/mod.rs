//! Bytes Serialization
mod sub_ser;
mod trait_impls;

use crate::lib::*;

use self::sub_ser::structs::StructSerializer;
use self::trait_impls::{Align as _, ClassNamesWriter, ClassStartsMap};
use super::serde::{hkx_header::HkxHeader, section_header::SectionHeader};
use crate::errors::ser::{
    Error, InvalidEndianSnafu, MissingClassInClassnamesSectionSnafu, MissingGlobalFixupClassSnafu,
    OverflowSubtractAbsSnafu, Result, UnsupportedPtrSizeSnafu,
};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt as _};
use havok_serde::ser::{Serialize, Serializer};
use havok_types::{
    f16, CString, Matrix3, Matrix4, Pointer, QsTransform, Quaternion, Rotation, Signature,
    StringPtr, Transform, Variant, Vector4,
};
use indexmap::IndexMap;
use snafu::ensure;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{Cursor, Write as _};

/// To hkx binary file data.
pub fn to_bytes<V>(value: &V, header: &HkxHeader) -> Result<Vec<u8>>
where
    V: Serialize + ClassNamesWriter,
{
    to_bytes_with_opt(
        value,
        header,
        ByteSerializer {
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
        },
    )
}

/// Serialize to bytes with custom `BytesSerializer` settings.
///
/// # Note
/// This serializer assumes the following.
/// - `contents_class_name_section_index`: It is always assumed to be 0.
/// - `contents_section_index`: It is always assumed to be 2.
pub fn to_bytes_with_opt<V>(value: &V, header: &HkxHeader, ser: ByteSerializer) -> Result<Vec<u8>>
where
    V: Serialize + ClassNamesWriter,
{
    let mut serializer = ser;

    // 1/5: root header
    serializer.output.write_all(&header.to_bytes())?;

    // 2/5: Get Section headers positions.(because the values of the fixups are not yet known.)
    let classnames_header_start = 64 + header.padding_size() as u64; // 64: Root header size
    let types_header_start = classnames_header_start + 48; // next `SectionHeader`(size 48bytes) position.
    let data_header_start = types_header_start + 48;
    serializer.output.set_position(data_header_start + 48);

    // 3/5: section contents
    // - `__classnames__` section
    if serializer.is_little_endian {
        serializer.class_starts =
            value.write_classnames_section::<LittleEndian>(&mut serializer.output)?;
    } else {
        serializer.class_starts =
            value.write_classnames_section::<BigEndian>(&mut serializer.output)?;
    };
    #[cfg(feature = "tracing")]
    tracing::trace!("class_starts: {:#?}", serializer.class_starts);

    // - `__data__` section
    serializer.abs_data_offset = header.padding_size() + serializer.output.position() as u32;
    value.serialize(&mut serializer)?;

    // 4/5: Write fixups_offsets of `__data__` section header.
    serializer.output.zero_fill_align(16)?; // Always make the start of fixups a multiple of 16.
    let (local_offset, global_offset, virtual_offset) = serializer.write_data_fixups()?; // Write local, global and virtual fixups
    let exports_offset = serializer.relative_position()?; // This is where the exports_offset is finally obtained.

    // 5/5 Write remain offsets for `__types__` & `__data__` section header.
    let abs_data_start = serializer.abs_data_offset;
    let data_section_header = SectionHeader {
        section_tag: SectionHeader::DATA_SECTION_HEADER_TAG,
        section_tag_separator: 0xff,
        absolute_data_start: abs_data_start,
        local_fixups_offset: local_offset,
        global_fixups_offset: global_offset,
        virtual_fixups_offset: virtual_offset,
        exports_offset,
        imports_offset: exports_offset,
        end_offset: exports_offset,
    };
    #[cfg(feature = "tracing")]
    tracing::trace!("data_section_header = {data_section_header}");

    serializer.output.set_position(classnames_header_start);
    let section_offset = header.section_offset;
    if serializer.is_little_endian {
        // `__classnames__` header`
        SectionHeader::write_classnames::<LittleEndian>(
            &mut serializer.output,
            section_offset,
            abs_data_start,
        )?;
        // `__types__` header`
        serializer.output.set_position(types_header_start);
        SectionHeader::write_types::<LittleEndian>(&mut serializer.output, abs_data_start)?;
        // `__data__` header`
        serializer.output.set_position(data_header_start);
        data_section_header.write_bytes::<LittleEndian>(&mut serializer.output)?;
    } else {
        // `__classnames__` header`
        SectionHeader::write_classnames::<BigEndian>(
            &mut serializer.output,
            section_offset,
            abs_data_start,
        )?;
        // `__types__` header`
        serializer.output.set_position(types_header_start);
        SectionHeader::write_types::<BigEndian>(&mut serializer.output, abs_data_start)?;
        // `__data__` header`
        serializer.output.set_position(data_header_start);
        data_section_header.write_bytes::<BigEndian>(&mut serializer.output)?;
    };

    Ok(serializer.output.into_inner())
}

/// Binary data serializer
///
/// # Note
/// - All of these fixups are from the `__data__` section.
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
    /// -   key: Starting point of the binary for which the pointer class write is requested.
    /// - value: Unique class pointer.(e.g. XML: #0050 -> 50)
    global_fixups_ptr_src: IndexMap<u32, Pointer>,
    /// The `global_fixup.dst` == `virtual_fixup.src`.
    ///
    /// Therefore, the write start position must be retained.
    /// The position of dst of global_fixup will be known after all the binary data of all classes are written.
    /// - key: Unique class pointer.(e.g. XML: #0050 -> 50)
    /// - value: Starting point where Havok Class binary data is written.
    ///
    /// # Note
    /// - This is used as a key since no duplicate ptr is required at the same relative position.
    ///   The ptr may be shared-referenced and cannot be keyed.
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
    class_starts: ClassStartsMap,

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
        let abs_data_offset = self.abs_data_offset;
        ensure!(
            position >= abs_data_offset,
            OverflowSubtractAbsSnafu {
                position,
                abs_data_offset
            }
        );

        Ok(position - self.abs_data_offset)
    }

    /// Write `global_fixups` of data section bytes to writer.
    ///
    /// # Info
    /// If all virtual_fixups are not obtained, references may not be available?
    ///
    /// # Note
    /// `global_fixup.dst` == `virtual_fixup.src`
    fn write_global_fixups(&mut self) -> Result<()> {
        for (&src, ptr) in &self.global_fixups_ptr_src {
            if let Some(&dst) = self.virtual_fixups_ptr_src.get(ptr) {
                #[cfg(feature = "tracing")]
                {
                    let src_abs = self.abs_data_offset + src;
                    let dst_abs = self.abs_data_offset + dst;
                    tracing::debug!(
                        "[global_fixups]({:#x}) src({src}/{src_abs:#x}) -> {ptr} dst({dst}/{dst_abs:#x})",
                        self.output.position()
                    );
                }

                if self.is_little_endian {
                    self.output.write_u32::<LittleEndian>(src)?; // src
                    self.output.write_u32::<LittleEndian>(2)?; // dst_section_index
                    self.output.write_u32::<LittleEndian>(dst)?; // dst(virtual_fixup.dst)
                } else {
                    self.output.write_u32::<BigEndian>(src)?; // src
                    self.output.write_u32::<BigEndian>(2)?; // dst_section_index
                    self.output.write_u32::<BigEndian>(dst)?; // dst(virtual_fixup.dst)
                }
            } else {
                return MissingGlobalFixupClassSnafu { ptr: *ptr }.fail();
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
            };
            Ok(())
        } else {
            MissingClassInClassnamesSectionSnafu { class_name }.fail()
        }
    }

    /// Write all(`local`, `global` and `virtual`) fixups of data section.
    ///
    /// # Returns
    /// (`local_offset`, `global_offset`, `virtual_offset`)
    fn write_data_fixups(&mut self) -> Result<(u32, u32, u32)> {
        let local_offset = self.relative_position()?;
        self.output.write_all(&self.local_fixups)?;
        self.output.align(16, 0xff)?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            "[global_fixups pointers]\nsrc: {:#?},\ndest(same as virtual.src): {:#?}",
            self.global_fixups_ptr_src,
            self.virtual_fixups_ptr_src
        );
        let global_offset = self.relative_position()?;
        self.write_global_fixups()?;
        self.output.align(16, 0xff)?;

        let virtual_offset = self.relative_position()?;
        self.output.write_all(&self.virtual_fixups)?;
        self.output.align(16, 0xff)?;
        Ok((local_offset, global_offset, virtual_offset))
    }

    /// Write the internal data pointed to by the pointer of `CString` or `StringPtr`.
    fn serialize_cow(&mut self, v: &Option<Cow<'_, str>>) -> Result<()> {
        // Skip if `Option::None`(null pointer).
        if let Some(v) = v {
            let c_string = std::ffi::CString::new(v.as_bytes())?;
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
        #[inline]
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
    type SerializeStruct = StructSerializer<'a>;
    type SerializeFlags = Self;

    #[inline]
    fn serialize_void(self, _: ()) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_uint8(v as u8)
    }

    #[inline]
    /// Assume that the characters are ASCII characters`c_char`. In that case, i8 is used to fit into 128 characters.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_int8(v as i8)
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
        #[allow(clippy::needless_else)]
        if !ptr.is_null() {
            // Write global_fixup src(write start) position.
            let start = self.relative_position()?;
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "Insert `global_fixup.src`({:#x}): {ptr}",
                self.output.position()
            );
            self.global_fixups_ptr_src.insert(start, ptr);
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("Skip global_fixup.src writing, because it's null ptr.");
        };
        self.serialize_ulong(0_u64)
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
        #[allow(clippy::needless_else)]
        if let Some((ptr, _sig)) = class_meta {
            self.output.zero_fill_align(16)?; // Make sure `virtual_fixup.src`(each Class) is `align16`.

            #[cfg(feature = "tracing")]
            tracing::debug!(
                "serialize struct {name}(index = {ptr}, signature = {_sig}, abs_position = {:#x})",
                self.output.position()
            );

            let virtual_src = self.relative_position()?;
            self.write_virtual_fixups_pair(name, virtual_src)?; // Ok, `virtual_fixup` is known.
            self.virtual_fixups_ptr_src.insert(ptr, virtual_src); // Backup to write `global_fixups`
        } else {
            #[cfg(feature = "tracing")]
            tracing::debug!("serialize struct {name}(A class within a field.)")
        }
        Ok(Self::SerializeStruct::new(self))
    }

    #[inline]
    fn serialize_variant(self, v: &Variant) -> Result<Self::Ok, Self::Error> {
        self.serialize_pointer(v.object)?;
        self.serialize_pointer(v.class)
    }

    #[inline]
    fn serialize_cstring(self, v: &CString) -> Result<Self::Ok, Self::Error> {
        self.serialize_cow(v.get_ref())
    }

    fn serialize_ulong(self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.is_x86 {
            true => self.serialize_uint32(v as u32),
            false => self.serialize_uint64(v),
        }
    }

    #[inline]
    fn serialize_enum_flags(self) -> Result<Self::SerializeFlags, Self::Error> {
        Ok(self)
    }

    #[inline]
    fn serialize_half(self, v: f16) -> Result<Self::Ok, Self::Error> {
        self.serialize_uint16(v.to_bits())
    }

    /// In the binary serialization of hkx, this is the actual data writing process beyond
    /// the pointer that is called only after all fields of the structure have been written.
    #[inline]
    fn serialize_stringptr(self, v: &StringPtr) -> Result<Self::Ok, Self::Error> {
        self.serialize_cow(v.get_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bytes::hexdump, mocks::new_defaultmale, HavokSort as _};

    #[test]
    #[cfg_attr(feature = "tracing", quick_tracing::try_init(test = "serialize_bytes"))]
    fn test_serialize() -> Result<()> {
        let mut classes = new_defaultmale();
        classes.sort_for_bytes();
        tracing::debug!("{classes:#?}");

        let bytes = to_bytes(&classes, &HkxHeader::new_skyrim_se())?;
        let actual = hexdump::to_string(&bytes);
        tracing::debug!("\n{actual}");

        let expected = hexdump::to_string(include_bytes!(
            "../../../../docs/handson_hex_dump/defaultmale/defaultmale.hkx"
        ));
        pretty_assertions::assert_eq!(actual, expected);
        Ok(())
    }
}
