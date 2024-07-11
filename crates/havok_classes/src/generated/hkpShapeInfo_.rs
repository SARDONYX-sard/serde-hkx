use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpShapeInfo`
/// -         version: `0`
/// -       signature: `0xea7f1d08`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpShapeInfo<'a> {
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
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `isHierarchicalCompound`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isHierarchicalCompound: bool,
    /// # C++ Info
    /// -          name: `hkdShapesCollected`(ctype: `hkBool`)
    /// -        offset:  13(x86)/ 25(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hkdShapesCollected: bool,
    /// # C++ Info
    /// -          name: `childShapeNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childShapeNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `childTransforms`(ctype: `hkArray<hkTransform>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_childTransforms: Vec<Transform>,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkTransform`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Transform,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpShapeInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpShapeInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xea7f1d08)
        }
    }
    impl<'a> _serde::Serialize for hkpShapeInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xea7f1d08)));
            let mut serializer = __serializer
                .serialize_struct("hkpShapeInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer
                .serialize_field(
                    "isHierarchicalCompound",
                    &self.m_isHierarchicalCompound,
                )?;
            serializer
                .serialize_field("hkdShapesCollected", &self.m_hkdShapesCollected)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_meta_field("childShapeNames", &self.m_childShapeNames)?;
            serializer
                .serialize_array_meta_field("childTransforms", &self.m_childTransforms)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer
                .serialize_array_field("childShapeNames", &self.m_childShapeNames)?;
            serializer
                .serialize_array_field("childTransforms", &self.m_childTransforms)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_shape,
    m_isHierarchicalCompound,
    m_hkdShapesCollected,
    m_childShapeNames,
    m_childTransforms,
    m_transform,
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
            "shape" => Ok(__Field::m_shape),
            "isHierarchicalCompound" => Ok(__Field::m_isHierarchicalCompound),
            "hkdShapesCollected" => Ok(__Field::m_hkdShapesCollected),
            "childShapeNames" => Ok(__Field::m_childShapeNames),
            "childTransforms" => Ok(__Field::m_childTransforms),
            "transform" => Ok(__Field::m_transform),
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
pub(super) struct __hkpShapeInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpShapeInfo<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpShapeInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpShapeInfo<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpShapeInfo<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpShapeInfoVisitor<'de> {
    type Value = hkpShapeInfo<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpShapeInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_isHierarchicalCompound: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_hkdShapesCollected: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_childShapeNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_childTransforms: _serde::__private::Option<Vec<Transform>> = _serde::__private::None;
        let mut m_transform: _serde::__private::Option<Transform> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_shape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                        );
                    }
                    m_shape = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_isHierarchicalCompound) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isHierarchicalCompound",
                            ),
                        );
                    }
                    m_isHierarchicalCompound = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_hkdShapesCollected) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hkdShapesCollected",
                            ),
                        );
                    }
                    m_hkdShapesCollected = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_childShapeNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childShapeNames",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    m_childShapeNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_childTransforms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childTransforms",
                            ),
                        );
                    }
                    m_childTransforms = _serde::__private::Some(
                        match __A::next_value::<Vec<Transform>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_transform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transform",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    m_transform = _serde::__private::Some(
                        match __A::next_value::<Transform>(&mut __map) {
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
        let m_shape = match m_shape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shape"),
                );
            }
        };
        let m_isHierarchicalCompound = match m_isHierarchicalCompound {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isHierarchicalCompound",
                    ),
                );
            }
        };
        let m_hkdShapesCollected = match m_hkdShapesCollected {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hkdShapesCollected",
                    ),
                );
            }
        };
        let m_childShapeNames = match m_childShapeNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShapeNames"),
                );
            }
        };
        let m_childTransforms = match m_childTransforms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childTransforms"),
                );
            }
        };
        let m_transform = match m_transform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transform"),
                );
            }
        };
        _serde::__private::Ok(hkpShapeInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_shape,
            m_isHierarchicalCompound,
            m_hkdShapesCollected,
            m_childShapeNames,
            m_childTransforms,
            m_transform,
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
        let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_isHierarchicalCompound: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_hkdShapesCollected: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_childShapeNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_childTransforms: _serde::__private::Option<Vec<Transform>> = _serde::__private::None;
        let mut m_transform: _serde::__private::Option<Transform> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_shape => {
                        if _serde::__private::Option::is_some(&m_shape) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                            );
                        }
                        m_shape = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isHierarchicalCompound => {
                        if _serde::__private::Option::is_some(
                            &m_isHierarchicalCompound,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isHierarchicalCompound",
                                ),
                            );
                        }
                        m_isHierarchicalCompound = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_hkdShapesCollected => {
                        if _serde::__private::Option::is_some(&m_hkdShapesCollected) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hkdShapesCollected",
                                ),
                            );
                        }
                        m_hkdShapesCollected = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_childShapeNames => {
                        if _serde::__private::Option::is_some(&m_childShapeNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "childShapeNames",
                                ),
                            );
                        }
                        m_childShapeNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_childTransforms => {
                        if _serde::__private::Option::is_some(&m_childTransforms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "childTransforms",
                                ),
                            );
                        }
                        m_childTransforms = _serde::__private::Some(
                            match __A::next_value::<Vec<Transform>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_transform => {
                        if _serde::__private::Option::is_some(&m_transform) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transform",
                                ),
                            );
                        }
                        m_transform = _serde::__private::Some(
                            match __A::next_value::<Transform>(&mut __map) {
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
        let m_shape = match m_shape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shape"),
                );
            }
        };
        let m_isHierarchicalCompound = match m_isHierarchicalCompound {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isHierarchicalCompound",
                    ),
                );
            }
        };
        let m_hkdShapesCollected = match m_hkdShapesCollected {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hkdShapesCollected",
                    ),
                );
            }
        };
        let m_childShapeNames = match m_childShapeNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShapeNames"),
                );
            }
        };
        let m_childTransforms = match m_childTransforms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childTransforms"),
                );
            }
        };
        let m_transform = match m_transform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transform"),
                );
            }
        };
        _serde::__private::Ok(hkpShapeInfo {
            __ptr,
            parent,
            m_shape,
            m_isHierarchicalCompound,
            m_hkdShapesCollected,
            m_childShapeNames,
            m_childTransforms,
            m_transform,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpShapeInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "shape",
                "isHierarchicalCompound",
                "hkdShapesCollected",
                "childShapeNames",
                "childTransforms",
                "transform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpShapeInfo",
                FIELDS,
                __hkpShapeInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkpShapeInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
