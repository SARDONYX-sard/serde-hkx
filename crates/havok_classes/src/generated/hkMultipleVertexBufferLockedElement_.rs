use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkMultipleVertexBufferLockedElement`
/// - version: `0`
/// - signature: `0xa0e22afc`
/// - size: `  7`(x86)/`  7`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBufferLockedElement {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `vertexBufferIndex`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_vertexBufferIndex: u8,
    /// # C++ Info
    /// - name: `elementIndex`(ctype: `hkUint8`)
    /// - offset: `  1`(x86)/`  1`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_elementIndex: u8,
    /// # C++ Info
    /// - name: `lockedBufferIndex`(ctype: `hkUint8`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_lockedBufferIndex: u8,
    /// # C++ Info
    /// - name: `vertexFormatIndex`(ctype: `hkUint8`)
    /// - offset: `  3`(x86)/`  3`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_vertexFormatIndex: u8,
    /// # C++ Info
    /// - name: `lockFlags`(ctype: `hkUint8`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_lockFlags: u8,
    /// # C++ Info
    /// - name: `outputBufferIndex`(ctype: `hkUint8`)
    /// - offset: `  5`(x86)/`  5`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_outputBufferIndex: u8,
    /// # C++ Info
    /// - name: `emulatedIndex`(ctype: `hkInt8`)
    /// - offset: `  6`(x86)/`  6`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_emulatedIndex: i8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMultipleVertexBufferLockedElement {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMultipleVertexBufferLockedElement"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa0e22afc)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkMultipleVertexBufferLockedElement {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa0e22afc)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkMultipleVertexBufferLockedElement",
                    class_meta,
                    (7u64, 7u64),
                )?;
            serializer.serialize_field("vertexBufferIndex", &self.m_vertexBufferIndex)?;
            serializer.serialize_field("elementIndex", &self.m_elementIndex)?;
            serializer.serialize_field("lockedBufferIndex", &self.m_lockedBufferIndex)?;
            serializer.serialize_field("vertexFormatIndex", &self.m_vertexFormatIndex)?;
            serializer.serialize_field("lockFlags", &self.m_lockFlags)?;
            serializer.serialize_field("outputBufferIndex", &self.m_outputBufferIndex)?;
            serializer.serialize_field("emulatedIndex", &self.m_emulatedIndex)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMultipleVertexBufferLockedElement {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_vertexBufferIndex,
                m_elementIndex,
                m_lockedBufferIndex,
                m_vertexFormatIndex,
                m_lockFlags,
                m_outputBufferIndex,
                m_emulatedIndex,
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
                        "vertexBufferIndex" => Ok(__Field::m_vertexBufferIndex),
                        "elementIndex" => Ok(__Field::m_elementIndex),
                        "lockedBufferIndex" => Ok(__Field::m_lockedBufferIndex),
                        "vertexFormatIndex" => Ok(__Field::m_vertexFormatIndex),
                        "lockFlags" => Ok(__Field::m_lockFlags),
                        "outputBufferIndex" => Ok(__Field::m_outputBufferIndex),
                        "emulatedIndex" => Ok(__Field::m_emulatedIndex),
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
            struct __hkMultipleVertexBufferLockedElementVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkMultipleVertexBufferLockedElement,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkMultipleVertexBufferLockedElementVisitor<'de> {
                type Value = hkMultipleVertexBufferLockedElement;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkMultipleVertexBufferLockedElement",
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
                    let mut m_vertexBufferIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_elementIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_lockedBufferIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_vertexFormatIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_lockFlags: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_outputBufferIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_emulatedIndex: _serde::__private::Option<i8> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_vertexBufferIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBufferIndex",
                                        ),
                                    );
                                }
                                m_vertexBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_elementIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elementIndex",
                                        ),
                                    );
                                }
                                m_elementIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_lockedBufferIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockedBufferIndex",
                                        ),
                                    );
                                }
                                m_lockedBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_vertexFormatIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexFormatIndex",
                                        ),
                                    );
                                }
                                m_vertexFormatIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_lockFlags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockFlags",
                                        ),
                                    );
                                }
                                m_lockFlags = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_outputBufferIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "outputBufferIndex",
                                        ),
                                    );
                                }
                                m_outputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_emulatedIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "emulatedIndex",
                                        ),
                                    );
                                }
                                m_emulatedIndex = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
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
                    let m_vertexBufferIndex = match m_vertexBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBufferIndex",
                                ),
                            );
                        }
                    };
                    let m_elementIndex = match m_elementIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elementIndex",
                                ),
                            );
                        }
                    };
                    let m_lockedBufferIndex = match m_lockedBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockedBufferIndex",
                                ),
                            );
                        }
                    };
                    let m_vertexFormatIndex = match m_vertexFormatIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexFormatIndex",
                                ),
                            );
                        }
                    };
                    let m_lockFlags = match m_lockFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockFlags",
                                ),
                            );
                        }
                    };
                    let m_outputBufferIndex = match m_outputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIndex",
                                ),
                            );
                        }
                    };
                    let m_emulatedIndex = match m_emulatedIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "emulatedIndex",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkMultipleVertexBufferLockedElement {
                        __ptr,
                        m_vertexBufferIndex,
                        m_elementIndex,
                        m_lockedBufferIndex,
                        m_vertexFormatIndex,
                        m_lockFlags,
                        m_outputBufferIndex,
                        m_emulatedIndex,
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
                    let mut m_vertexBufferIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_elementIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_lockedBufferIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_vertexFormatIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_lockFlags: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_outputBufferIndex: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_emulatedIndex: _serde::__private::Option<i8> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_vertexBufferIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_vertexBufferIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexBufferIndex",
                                        ),
                                    );
                                }
                                m_vertexBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_elementIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_elementIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elementIndex",
                                        ),
                                    );
                                }
                                m_elementIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lockedBufferIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_lockedBufferIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockedBufferIndex",
                                        ),
                                    );
                                }
                                m_lockedBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_vertexFormatIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_vertexFormatIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "vertexFormatIndex",
                                        ),
                                    );
                                }
                                m_vertexFormatIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lockFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lockFlags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockFlags",
                                        ),
                                    );
                                }
                                m_lockFlags = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_outputBufferIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_outputBufferIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "outputBufferIndex",
                                        ),
                                    );
                                }
                                m_outputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_emulatedIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_emulatedIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "emulatedIndex",
                                        ),
                                    );
                                }
                                m_emulatedIndex = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
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
                    let m_vertexBufferIndex = match m_vertexBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexBufferIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_elementIndex = match m_elementIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "elementIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lockedBufferIndex = match m_lockedBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockedBufferIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_vertexFormatIndex = match m_vertexFormatIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "vertexFormatIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lockFlags = match m_lockFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_outputBufferIndex = match m_outputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_emulatedIndex = match m_emulatedIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "emulatedIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkMultipleVertexBufferLockedElement {
                        __ptr,
                        m_vertexBufferIndex,
                        m_elementIndex,
                        m_lockedBufferIndex,
                        m_vertexFormatIndex,
                        m_lockFlags,
                        m_outputBufferIndex,
                        m_emulatedIndex,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "vertexBufferIndex",
                "elementIndex",
                "lockedBufferIndex",
                "vertexFormatIndex",
                "lockFlags",
                "outputBufferIndex",
                "emulatedIndex",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMultipleVertexBufferLockedElement",
                FIELDS,
                __hkMultipleVertexBufferLockedElementVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkMultipleVertexBufferLockedElement,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
