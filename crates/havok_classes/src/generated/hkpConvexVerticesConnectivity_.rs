use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConvexVerticesConnectivity`
/// -         version: `0`
/// -       signature: `0x63d38e9c`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexVerticesConnectivity {
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
    /// -          name: `vertexIndices`(ctype: `hkArray<hkUint16>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertexIndices: Vec<u16>,
    /// # C++ Info
    /// -          name: `numVerticesPerFace`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_numVerticesPerFace: Vec<u8>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConvexVerticesConnectivity {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConvexVerticesConnectivity"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x63d38e9c)
        }
    }
    impl _serde::Serialize for hkpConvexVerticesConnectivity {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x63d38e9c)));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexVerticesConnectivity", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("vertexIndices", &self.m_vertexIndices)?;
            serializer
                .serialize_array_meta_field(
                    "numVerticesPerFace",
                    &self.m_numVerticesPerFace,
                )?;
            serializer.serialize_array_field("vertexIndices", &self.m_vertexIndices)?;
            serializer
                .serialize_array_field(
                    "numVerticesPerFace",
                    &self.m_numVerticesPerFace,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vertexIndices,
    m_numVerticesPerFace,
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
            "vertexIndices" => Ok(__Field::m_vertexIndices),
            "numVerticesPerFace" => Ok(__Field::m_numVerticesPerFace),
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
pub(super) struct __hkpConvexVerticesConnectivityVisitor<'de> {
    marker: core::marker::PhantomData<hkpConvexVerticesConnectivity>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpConvexVerticesConnectivityVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpConvexVerticesConnectivity, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpConvexVerticesConnectivity>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpConvexVerticesConnectivityVisitor<'de> {
    type Value = hkpConvexVerticesConnectivity;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpConvexVerticesConnectivity",
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
        let mut m_vertexIndices: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_numVerticesPerFace: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vertexIndices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vertexIndices",
                            ),
                        );
                    }
                    m_vertexIndices = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_numVerticesPerFace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numVerticesPerFace",
                            ),
                        );
                    }
                    m_numVerticesPerFace = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
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
        let m_vertexIndices = match m_vertexIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexIndices"),
                );
            }
        };
        let m_numVerticesPerFace = match m_numVerticesPerFace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numVerticesPerFace",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpConvexVerticesConnectivity {
            __ptr,
            parent,
            m_vertexIndices,
            m_numVerticesPerFace,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_vertexIndices: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_numVerticesPerFace: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_vertexIndices => {
                        if _serde::__private::Option::is_some(&m_vertexIndices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vertexIndices",
                                ),
                            );
                        }
                        m_vertexIndices = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numVerticesPerFace => {
                        if _serde::__private::Option::is_some(&m_numVerticesPerFace) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numVerticesPerFace",
                                ),
                            );
                        }
                        m_numVerticesPerFace = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
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
        }
        let m_vertexIndices = match m_vertexIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexIndices"),
                );
            }
        };
        let m_numVerticesPerFace = match m_numVerticesPerFace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numVerticesPerFace",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpConvexVerticesConnectivity {
            __ptr,
            parent,
            m_vertexIndices,
            m_numVerticesPerFace,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConvexVerticesConnectivity {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["vertexIndices", "numVerticesPerFace"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConvexVerticesConnectivity",
                FIELDS,
                __hkpConvexVerticesConnectivityVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpConvexVerticesConnectivity,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
