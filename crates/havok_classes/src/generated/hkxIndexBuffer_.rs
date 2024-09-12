use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkxIndexBuffer`
/// - version: `1`
/// - signature: `0xc12c8197`
/// - size: ` 44`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxIndexBuffer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `indexType`(ctype: `enum IndexType`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_indexType: IndexType,
    /// # C++ Info
    /// - name: `indices16`(ctype: `hkArray<hkUint16>`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_indices16: Vec<u16>,
    /// # C++ Info
    /// - name: `indices32`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_indices32: Vec<u32>,
    /// # C++ Info
    /// - name: `vertexBaseOffset`(ctype: `hkUint32`)
    /// - offset: ` 36`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_vertexBaseOffset: u32,
    /// # C++ Info
    /// - name: `length`(ctype: `hkUint32`)
    /// - offset: ` 40`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_length: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxIndexBuffer {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxIndexBuffer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc12c8197)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkxIndexBuffer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc12c8197)));
            let mut serializer = __serializer
                .serialize_struct("hkxIndexBuffer", class_meta, (44u64, 64u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("indexType", &self.m_indexType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "indices16",
                    &self.m_indices16,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "indices32",
                    &self.m_indices32,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("vertexBaseOffset", &self.m_vertexBaseOffset)?;
            serializer.serialize_field("length", &self.m_length)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxIndexBuffer {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_indexType,
                m_indices16,
                m_indices32,
                m_vertexBaseOffset,
                m_length,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "indexType" => Ok(__Field::m_indexType),
                        "indices16" => Ok(__Field::m_indices16),
                        "indices32" => Ok(__Field::m_indices32),
                        "vertexBaseOffset" => Ok(__Field::m_vertexBaseOffset),
                        "length" => Ok(__Field::m_length),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkxIndexBufferVisitor<'de> {
                marker: _serde::__private::PhantomData<hkxIndexBuffer>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkxIndexBufferVisitor<'de> {
                type Value = hkxIndexBuffer;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkxIndexBuffer")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_indexType: _serde::__private::Option<IndexType> = _serde::__private::None;
                    let mut m_indices16: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
                    let mut m_indices32: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_vertexBaseOffset: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_length: _serde::__private::Option<u32> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_indexType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexType",
                                        ),
                                    );
                                }
                                m_indexType = _serde::__private::Some(
                                    match __A::next_value::<IndexType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_indices16) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indices16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 7usize)?;
                                m_indices16 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_indices32) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indices32",
                                        ),
                                    );
                                }
                                m_indices32 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_vertexBaseOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBaseOffset",
                                        ),
                                    );
                                }
                                m_vertexBaseOffset = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_length) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("length"),
                                    );
                                }
                                m_length = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_indexType = match m_indexType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexType",
                                ),
                            );
                        }
                    };
                    let m_indices16 = match m_indices16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indices16",
                                ),
                            );
                        }
                    };
                    let m_indices32 = match m_indices32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indices32",
                                ),
                            );
                        }
                    };
                    let m_vertexBaseOffset = match m_vertexBaseOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBaseOffset",
                                ),
                            );
                        }
                    };
                    let m_length = match m_length {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("length"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkxIndexBuffer {
                        __ptr,
                        parent,
                        m_indexType,
                        m_indices16,
                        m_indices32,
                        m_vertexBaseOffset,
                        m_length,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_indexType: _serde::__private::Option<IndexType> = _serde::__private::None;
                    let mut m_indices16: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
                    let mut m_indices32: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_vertexBaseOffset: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_length: _serde::__private::Option<u32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_indexType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indexType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indexType",
                                        ),
                                    );
                                }
                                m_indexType = _serde::__private::Some(
                                    match __A::next_value::<IndexType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indices16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indices16) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indices16",
                                        ),
                                    );
                                }
                                m_indices16 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_indices32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_indices32) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indices32",
                                        ),
                                    );
                                }
                                m_indices32 = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexBaseOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexBaseOffset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBaseOffset",
                                        ),
                                    );
                                }
                                m_vertexBaseOffset = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_length => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_length) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("length"),
                                    );
                                }
                                m_length = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_indexType = match m_indexType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indexType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indices16 = match m_indices16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indices16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_indices32 = match m_indices32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "indices32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexBaseOffset = match m_vertexBaseOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBaseOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_length = match m_length {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("length"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkxIndexBuffer {
                        __ptr,
                        parent,
                        m_indexType,
                        m_indices16,
                        m_indices32,
                        m_vertexBaseOffset,
                        m_length,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "indexType",
                "indices16",
                "indices32",
                "vertexBaseOffset",
                "length",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxIndexBuffer",
                FIELDS,
                __hkxIndexBufferVisitor {
                    marker: _serde::__private::PhantomData::<hkxIndexBuffer>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum IndexType {
    #[default]
    INDEX_TYPE_INVALID = 0isize,
    INDEX_TYPE_TRI_LIST = 1isize,
    INDEX_TYPE_TRI_STRIP = 2isize,
    INDEX_TYPE_TRI_FAN = 3isize,
    INDEX_TYPE_MAX_ID = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for IndexType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INDEX_TYPE_INVALID => {
                    __serializer.serialize_field("INDEX_TYPE_INVALID", &0u64)
                }
                Self::INDEX_TYPE_TRI_LIST => {
                    __serializer.serialize_field("INDEX_TYPE_TRI_LIST", &1u64)
                }
                Self::INDEX_TYPE_TRI_STRIP => {
                    __serializer.serialize_field("INDEX_TYPE_TRI_STRIP", &2u64)
                }
                Self::INDEX_TYPE_TRI_FAN => {
                    __serializer.serialize_field("INDEX_TYPE_TRI_FAN", &3u64)
                }
                Self::INDEX_TYPE_MAX_ID => {
                    __serializer.serialize_field("INDEX_TYPE_MAX_ID", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum IndexType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for IndexType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        4i8 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("INDEX_TYPE_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("INDEX_TYPE_TRI_LIST") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("INDEX_TYPE_TRI_STRIP") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("INDEX_TYPE_TRI_FAN") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("INDEX_TYPE_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<IndexType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = IndexType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum IndexType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexType::INDEX_TYPE_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexType::INDEX_TYPE_TRI_LIST)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexType::INDEX_TYPE_TRI_STRIP)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexType::INDEX_TYPE_TRI_FAN)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(IndexType::INDEX_TYPE_MAX_ID)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "INDEX_TYPE_INVALID",
                "INDEX_TYPE_TRI_LIST",
                "INDEX_TYPE_TRI_STRIP",
                "INDEX_TYPE_TRI_FAN",
                "INDEX_TYPE_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "IndexType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<IndexType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
