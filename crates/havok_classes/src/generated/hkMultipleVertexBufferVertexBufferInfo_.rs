use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMultipleVertexBufferVertexBufferInfo`
/// -         version: `0`
/// -       signature: `0xdafbe0e6`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMultipleVertexBufferVertexBufferInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertexBuffer`(ctype: `struct hkMeshVertexBuffer*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexBuffer: Pointer,
    /// # C++ Info
    /// -          name: `lockedVertices`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_lockedVertices: Pointer,
    /// # C++ Info
    /// -          name: `isLocked`(ctype: `hkBool`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isLocked: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMultipleVertexBufferVertexBufferInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMultipleVertexBufferVertexBufferInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xdafbe0e6)
        }
    }
    impl _serde::Serialize for hkMultipleVertexBufferVertexBufferInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xdafbe0e6)));
            let mut serializer = __serializer
                .serialize_struct("hkMultipleVertexBufferVertexBufferInfo", class_meta)?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer.skip_field("lockedVertices", &self.m_lockedVertices)?;
            serializer.serialize_field("isLocked", &self.m_isLocked)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vertexBuffer,
    m_lockedVertices,
    m_isLocked,
    __ignore,
}
struct __FieldVisitor;
impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
    type Value = __Field;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
    where
        __E: _serde::de::Error,
    {
        match __value {
            "vertexBuffer" => Ok(__Field::m_vertexBuffer),
            "isLocked" => Ok(__Field::m_isLocked),
            _ => Ok(__Field::__ignore),
        }
    }
}
impl<'de> _serde::Deserialize<'de> for __Field {
    #[inline]
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkMultipleVertexBufferVertexBufferInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkMultipleVertexBufferVertexBufferInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkMultipleVertexBufferVertexBufferInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkMultipleVertexBufferVertexBufferInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkMultipleVertexBufferVertexBufferInfo,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de>
for __hkMultipleVertexBufferVertexBufferInfoVisitor<'de> {
    type Value = hkMultipleVertexBufferVertexBufferInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkMultipleVertexBufferVertexBufferInfo",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_vertexBuffer: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_lockedVertices: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_isLocked: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vertexBuffer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexBuffer",
                            ),
                        );
                    }
                    m_vertexBuffer = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_lockedVertices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lockedVertices",
                            ),
                        );
                    }
                    m_lockedVertices = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 3usize, 7usize)?;
        let m_vertexBuffer = match m_vertexBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexBuffer"),
                );
            }
        };
        let m_lockedVertices = match m_lockedVertices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lockedVertices"),
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
        _serde::__private::Ok(hkMultipleVertexBufferVertexBufferInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_vertexBuffer,
            m_lockedVertices,
            m_isLocked,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_vertexBuffer: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_isLocked: _serde::__private::Option<bool> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_vertexBuffer => {
                    if _serde::__private::Option::is_some(&m_vertexBuffer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexBuffer",
                            ),
                        );
                    }
                    m_vertexBuffer = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_isLocked => {
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
                _ => {}
            }
        }
        let m_vertexBuffer = match m_vertexBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexBuffer"),
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
        _serde::__private::Ok(hkMultipleVertexBufferVertexBufferInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_vertexBuffer,
            m_isLocked,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMultipleVertexBufferVertexBufferInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["vertexBuffer", "lockedVertices", "isLocked"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMultipleVertexBufferVertexBufferInfo",
                FIELDS,
                __hkMultipleVertexBufferVertexBufferInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkMultipleVertexBufferVertexBufferInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
