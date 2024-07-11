use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMeshSection`
/// -         version: `1`
/// -       signature: `0xe2286cf8`
/// -          size:  40(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMeshSection {
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
    /// -          name: `vertexBuffer`(ctype: `struct hkxVertexBuffer*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_vertexBuffer: Pointer,
    /// # C++ Info
    /// -          name: `indexBuffers`(ctype: `hkArray<hkxIndexBuffer*>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_indexBuffers: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `material`(ctype: `struct hkxMaterial*`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_material: Pointer,
    /// # C++ Info
    /// -          name: `userChannels`(ctype: `hkArray<hkReferencedObject*>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_userChannels: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxMeshSection {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMeshSection"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe2286cf8)
        }
    }
    impl _serde::Serialize for hkxMeshSection {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe2286cf8)));
            let mut serializer = __serializer
                .serialize_struct("hkxMeshSection", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexBuffer", &self.m_vertexBuffer)?;
            serializer.serialize_array_meta_field("indexBuffers", &self.m_indexBuffers)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.serialize_array_meta_field("userChannels", &self.m_userChannels)?;
            serializer.serialize_array_field("indexBuffers", &self.m_indexBuffers)?;
            serializer.serialize_array_field("userChannels", &self.m_userChannels)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vertexBuffer,
    m_indexBuffers,
    m_material,
    m_userChannels,
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
            "indexBuffers" => Ok(__Field::m_indexBuffers),
            "material" => Ok(__Field::m_material),
            "userChannels" => Ok(__Field::m_userChannels),
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
pub(super) struct __hkxMeshSectionVisitor<'de> {
    marker: core::marker::PhantomData<hkxMeshSection>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxMeshSectionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxMeshSection, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxMeshSection>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxMeshSectionVisitor<'de> {
    type Value = hkxMeshSection;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxMeshSection")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_vertexBuffer: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_indexBuffers: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_material: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_userChannels: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..4usize {
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
                    if _serde::__private::Option::is_some(&m_indexBuffers) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "indexBuffers",
                            ),
                        );
                    }
                    m_indexBuffers = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_material) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "material",
                            ),
                        );
                    }
                    m_material = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_userChannels) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userChannels",
                            ),
                        );
                    }
                    m_userChannels = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_indexBuffers = match m_indexBuffers {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexBuffers"),
                );
            }
        };
        let m_material = match m_material {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("material"),
                );
            }
        };
        let m_userChannels = match m_userChannels {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userChannels"),
                );
            }
        };
        _serde::__private::Ok(hkxMeshSection {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_vertexBuffer,
            m_indexBuffers,
            m_material,
            m_userChannels,
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
        let mut m_vertexBuffer: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_indexBuffers: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_material: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_userChannels: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
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
                    __Field::m_indexBuffers => {
                        if _serde::__private::Option::is_some(&m_indexBuffers) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "indexBuffers",
                                ),
                            );
                        }
                        m_indexBuffers = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_material => {
                        if _serde::__private::Option::is_some(&m_material) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "material",
                                ),
                            );
                        }
                        m_material = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userChannels => {
                        if _serde::__private::Option::is_some(&m_userChannels) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userChannels",
                                ),
                            );
                        }
                        m_userChannels = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_vertexBuffer = match m_vertexBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vertexBuffer"),
                );
            }
        };
        let m_indexBuffers = match m_indexBuffers {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("indexBuffers"),
                );
            }
        };
        let m_material = match m_material {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("material"),
                );
            }
        };
        let m_userChannels = match m_userChannels {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userChannels"),
                );
            }
        };
        _serde::__private::Ok(hkxMeshSection {
            __ptr,
            parent,
            m_vertexBuffer,
            m_indexBuffers,
            m_material,
            m_userChannels,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxMeshSection {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "vertexBuffer",
                "indexBuffers",
                "material",
                "userChannels",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxMeshSection",
                FIELDS,
                __hkxMeshSectionVisitor {
                    marker: _serde::__private::PhantomData::<hkxMeshSection>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
