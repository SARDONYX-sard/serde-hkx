use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkMultipleVertexBuffer`
/// - version: `0`
/// - signature: `0xde3ab602`
/// - size: `324`(x86)/`352`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBuffer<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkMeshVertexBuffer<'a>,
    /// # C++ Info
    /// - name: `vertexFormat`(ctype: `struct hkVertexFormat`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `260`(x86)/`260`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexFormat"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexFormat"))]
    pub m_vertexFormat: hkVertexFormat<'a>,
    /// # C++ Info
    /// - name: `lockedElements`(ctype: `hkArray<struct hkMultipleVertexBufferLockedElement>`)
    /// - offset: `268`(x86)/`280`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lockedElements"))]
    #[cfg_attr(feature = "serde", serde(rename = "lockedElements"))]
    pub m_lockedElements: Vec<hkMultipleVertexBufferLockedElement<'a>>,
    /// # C++ Info
    /// - name: `lockedBuffer`(ctype: `struct hkMemoryMeshVertexBuffer*`)
    /// - offset: `280`(x86)/`296`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lockedBuffer"))]
    #[cfg_attr(feature = "serde", serde(rename = "lockedBuffer"))]
    pub m_lockedBuffer: Pointer<'a>,
    /// # C++ Info
    /// - name: `elementInfos`(ctype: `hkArray<struct hkMultipleVertexBufferElementInfo>`)
    /// - offset: `284`(x86)/`304`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "elementInfos"))]
    #[cfg_attr(feature = "serde", serde(rename = "elementInfos"))]
    pub m_elementInfos: Vec<hkMultipleVertexBufferElementInfo<'a>>,
    /// # C++ Info
    /// - name: `vertexBufferInfos`(ctype: `hkArray<struct hkMultipleVertexBufferVertexBufferInfo>`)
    /// - offset: `296`(x86)/`320`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "vertexBufferInfos"))]
    #[cfg_attr(feature = "serde", serde(rename = "vertexBufferInfos"))]
    pub m_vertexBufferInfos: Vec<hkMultipleVertexBufferVertexBufferInfo<'a>>,
    /// # C++ Info
    /// - name: `numVertices`(ctype: `hkInt32`)
    /// - offset: `308`(x86)/`336`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "numVertices"))]
    #[cfg_attr(feature = "serde", serde(rename = "numVertices"))]
    pub m_numVertices: I32<'a>,
    /// # C++ Info
    /// - name: `isLocked`(ctype: `hkBool`)
    /// - offset: `312`(x86)/`340`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "isLocked"))]
    #[cfg_attr(feature = "serde", serde(rename = "isLocked"))]
    pub m_isLocked: bool,
    /// # C++ Info
    /// - name: `updateCount`(ctype: `hkUint32`)
    /// - offset: `316`(x86)/`344`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "updateCount"))]
    #[cfg_attr(feature = "serde", serde(rename = "updateCount"))]
    pub m_updateCount: U32<'a>,
    /// # C++ Info
    /// - name: `writeLock`(ctype: `hkBool`)
    /// - offset: `320`(x86)/`348`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "writeLock"))]
    #[cfg_attr(feature = "serde", serde(rename = "writeLock"))]
    pub m_writeLock: bool,
    /// # C++ Info
    /// - name: `isSharable`(ctype: `hkBool`)
    /// - offset: `321`(x86)/`349`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "isSharable"))]
    #[cfg_attr(feature = "serde", serde(rename = "isSharable"))]
    pub m_isSharable: bool,
    /// # C++ Info
    /// - name: `constructionComplete`(ctype: `hkBool`)
    /// - offset: `322`(x86)/`350`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "constructionComplete"))]
    #[cfg_attr(feature = "serde", serde(rename = "constructionComplete"))]
    pub m_constructionComplete: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMultipleVertexBuffer<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMultipleVertexBuffer"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xde3ab602)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_vertexFormat.deps_indexes());
            v.extend(
                self
                    .m_lockedElements
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.push(&self.m_lockedBuffer);
            v.extend(
                self
                    .m_elementInfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_vertexBufferInfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkMultipleVertexBuffer<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xde3ab602)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkMultipleVertexBuffer",
                    class_meta,
                    (324u64, 352u64),
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexFormat", &self.m_vertexFormat)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "lockedElements",
                    &self.m_lockedElements,
                    TypeSize::Struct {
                        size_x86: 7u64,
                        size_x86_64: 7u64,
                    },
                )?;
            serializer.serialize_field("lockedBuffer", &self.m_lockedBuffer)?;
            serializer
                .serialize_array_field(
                    "elementInfos",
                    &self.m_elementInfos,
                    TypeSize::Struct {
                        size_x86: 2u64,
                        size_x86_64: 2u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "vertexBufferInfos",
                    &self.m_vertexBufferInfos,
                    TypeSize::Struct {
                        size_x86: 12u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.serialize_field("isLocked", &self.m_isLocked)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("updateCount", &self.m_updateCount)?;
            serializer.serialize_field("writeLock", &self.m_writeLock)?;
            serializer.serialize_field("isSharable", &self.m_isSharable)?;
            serializer
                .serialize_field("constructionComplete", &self.m_constructionComplete)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMultipleVertexBuffer<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_vertexFormat,
                m_lockedElements,
                m_lockedBuffer,
                m_elementInfos,
                m_vertexBufferInfos,
                m_numVertices,
                m_isLocked,
                m_updateCount,
                m_writeLock,
                m_isSharable,
                m_constructionComplete,
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
                        "vertexFormat" => Ok(__Field::m_vertexFormat),
                        "lockedElements" => Ok(__Field::m_lockedElements),
                        "lockedBuffer" => Ok(__Field::m_lockedBuffer),
                        "elementInfos" => Ok(__Field::m_elementInfos),
                        "vertexBufferInfos" => Ok(__Field::m_vertexBufferInfos),
                        "numVertices" => Ok(__Field::m_numVertices),
                        "isLocked" => Ok(__Field::m_isLocked),
                        "updateCount" => Ok(__Field::m_updateCount),
                        "writeLock" => Ok(__Field::m_writeLock),
                        "isSharable" => Ok(__Field::m_isSharable),
                        "constructionComplete" => Ok(__Field::m_constructionComplete),
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
            struct __hkMultipleVertexBufferVisitor<'de> {
                marker: _serde::__private::PhantomData<hkMultipleVertexBuffer<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkMultipleVertexBufferVisitor<'de> {
                type Value = hkMultipleVertexBuffer<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkMultipleVertexBuffer",
                    )
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
                    let mut m_vertexFormat: _serde::__private::Option<hkVertexFormat> = _serde::__private::None;
                    let mut m_lockedElements: _serde::__private::Option<
                        Vec<hkMultipleVertexBufferLockedElement>,
                    > = _serde::__private::None;
                    let mut m_lockedBuffer: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_elementInfos: _serde::__private::Option<
                        Vec<hkMultipleVertexBufferElementInfo>,
                    > = _serde::__private::None;
                    let mut m_vertexBufferInfos: _serde::__private::Option<
                        Vec<hkMultipleVertexBufferVertexBufferInfo>,
                    > = _serde::__private::None;
                    let mut m_numVertices: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_isLocked: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateCount: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_writeLock: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isSharable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_constructionComplete: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_vertexFormat) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexFormat",
                                        ),
                                    );
                                }
                                m_vertexFormat = _serde::__private::Some(
                                    match __A::next_value::<hkVertexFormat>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_lockedElements) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockedElements",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_lockedElements = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMultipleVertexBufferLockedElement>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_lockedBuffer) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockedBuffer",
                                        ),
                                    );
                                }
                                m_lockedBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_elementInfos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elementInfos",
                                        ),
                                    );
                                }
                                m_elementInfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMultipleVertexBufferElementInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_vertexBufferInfos,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBufferInfos",
                                        ),
                                    );
                                }
                                m_vertexBufferInfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMultipleVertexBufferVertexBufferInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_numVertices) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numVertices",
                                        ),
                                    );
                                }
                                m_numVertices = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_isLocked) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isLocked",
                                        ),
                                    );
                                }
                                m_isLocked = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_updateCount) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateCount",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_updateCount = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_writeLock) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "writeLock",
                                        ),
                                    );
                                }
                                m_writeLock = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_isSharable) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isSharable",
                                        ),
                                    );
                                }
                                m_isSharable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_constructionComplete,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constructionComplete",
                                        ),
                                    );
                                }
                                m_constructionComplete = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    let m_vertexFormat = match m_vertexFormat {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexFormat",
                                ),
                            );
                        }
                    };
                    let m_lockedElements = match m_lockedElements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockedElements",
                                ),
                            );
                        }
                    };
                    let m_lockedBuffer = match m_lockedBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockedBuffer",
                                ),
                            );
                        }
                    };
                    let m_elementInfos = match m_elementInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elementInfos",
                                ),
                            );
                        }
                    };
                    let m_vertexBufferInfos = match m_vertexBufferInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBufferInfos",
                                ),
                            );
                        }
                    };
                    let m_numVertices = match m_numVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numVertices",
                                ),
                            );
                        }
                    };
                    let m_isLocked = match m_isLocked {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isLocked"),
                            );
                        }
                    };
                    let m_updateCount = match m_updateCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateCount",
                                ),
                            );
                        }
                    };
                    let m_writeLock = match m_writeLock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "writeLock",
                                ),
                            );
                        }
                    };
                    let m_isSharable = match m_isSharable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isSharable",
                                ),
                            );
                        }
                    };
                    let m_constructionComplete = match m_constructionComplete {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constructionComplete",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkMultipleVertexBuffer {
                        __ptr,
                        parent,
                        m_vertexFormat,
                        m_lockedElements,
                        m_lockedBuffer,
                        m_elementInfos,
                        m_vertexBufferInfos,
                        m_numVertices,
                        m_isLocked,
                        m_updateCount,
                        m_writeLock,
                        m_isSharable,
                        m_constructionComplete,
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
                    let mut m_vertexFormat: _serde::__private::Option<hkVertexFormat> = _serde::__private::None;
                    let mut m_lockedElements: _serde::__private::Option<
                        Vec<hkMultipleVertexBufferLockedElement>,
                    > = _serde::__private::None;
                    let mut m_lockedBuffer: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_elementInfos: _serde::__private::Option<
                        Vec<hkMultipleVertexBufferElementInfo>,
                    > = _serde::__private::None;
                    let mut m_vertexBufferInfos: _serde::__private::Option<
                        Vec<hkMultipleVertexBufferVertexBufferInfo>,
                    > = _serde::__private::None;
                    let mut m_numVertices: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_isLocked: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateCount: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_writeLock: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isSharable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_constructionComplete: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_vertexFormat => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_vertexFormat) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexFormat",
                                        ),
                                    );
                                }
                                m_vertexFormat = _serde::__private::Some(
                                    match __A::next_value::<hkVertexFormat>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lockedElements => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lockedElements) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockedElements",
                                        ),
                                    );
                                }
                                m_lockedElements = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMultipleVertexBufferLockedElement>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lockedBuffer => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lockedBuffer) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockedBuffer",
                                        ),
                                    );
                                }
                                m_lockedBuffer = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_elementInfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_elementInfos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elementInfos",
                                        ),
                                    );
                                }
                                m_elementInfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMultipleVertexBufferElementInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexBufferInfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_vertexBufferInfos,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBufferInfos",
                                        ),
                                    );
                                }
                                m_vertexBufferInfos = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkMultipleVertexBufferVertexBufferInfo>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numVertices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numVertices) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numVertices",
                                        ),
                                    );
                                }
                                m_numVertices = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isLocked => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isLocked) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isLocked",
                                        ),
                                    );
                                }
                                m_isLocked = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_updateCount => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_updateCount) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateCount",
                                        ),
                                    );
                                }
                                m_updateCount = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_writeLock => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_writeLock) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "writeLock",
                                        ),
                                    );
                                }
                                m_writeLock = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isSharable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isSharable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isSharable",
                                        ),
                                    );
                                }
                                m_isSharable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_constructionComplete => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_constructionComplete,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constructionComplete",
                                        ),
                                    );
                                }
                                m_constructionComplete = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_vertexFormat = match m_vertexFormat {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexFormat",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lockedElements = match m_lockedElements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockedElements",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lockedBuffer = match m_lockedBuffer {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockedBuffer",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_elementInfos = match m_elementInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elementInfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexBufferInfos = match m_vertexBufferInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBufferInfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numVertices = match m_numVertices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numVertices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isLocked = match m_isLocked {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isLocked"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_updateCount = match m_updateCount {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateCount",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_writeLock = match m_writeLock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "writeLock",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isSharable = match m_isSharable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isSharable",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constructionComplete = match m_constructionComplete {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constructionComplete",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkMeshVertexBuffer {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkMultipleVertexBuffer {
                        __ptr: __ptr.clone(),
                        parent,
                        m_vertexFormat,
                        m_lockedElements,
                        m_lockedBuffer,
                        m_elementInfos,
                        m_vertexBufferInfos,
                        m_numVertices,
                        m_isLocked,
                        m_updateCount,
                        m_writeLock,
                        m_isSharable,
                        m_constructionComplete,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "vertexFormat",
                "lockedElements",
                "lockedBuffer",
                "elementInfos",
                "vertexBufferInfos",
                "numVertices",
                "isLocked",
                "updateCount",
                "writeLock",
                "isSharable",
                "constructionComplete",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMultipleVertexBuffer",
                FIELDS,
                __hkMultipleVertexBufferVisitor {
                    marker: _serde::__private::PhantomData::<hkMultipleVertexBuffer>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
