use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterAddedInfo`
/// -         version: `0`
/// -       signature: `0x3544e182`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterAddedInfo<'a> {
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
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `instanceName`(ctype: `hkStringPtr`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_instanceName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `templateName`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_templateName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `fullPathToProject`(ctype: `hkStringPtr`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_fullPathToProject: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeleton: Pointer,
    /// # C++ Info
    /// -          name: `worldFromModel`(ctype: `hkQsTransform`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// -          name: `poseModelSpace`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_poseModelSpace: Vec<QsTransform>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterAddedInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterAddedInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3544e182)
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterAddedInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3544e182)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterAddedInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer
                .serialize_stringptr_meta_field("instanceName", &self.m_instanceName)?;
            serializer
                .serialize_stringptr_meta_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer
                .serialize_array_meta_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_stringptr_field("instanceName", &self.m_instanceName)?;
            serializer.serialize_stringptr_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_array_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_characterId,
    m_instanceName,
    m_templateName,
    m_fullPathToProject,
    m_skeleton,
    m_worldFromModel,
    m_poseModelSpace,
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
            "characterId" => Ok(__Field::m_characterId),
            "instanceName" => Ok(__Field::m_instanceName),
            "templateName" => Ok(__Field::m_templateName),
            "fullPathToProject" => Ok(__Field::m_fullPathToProject),
            "skeleton" => Ok(__Field::m_skeleton),
            "worldFromModel" => Ok(__Field::m_worldFromModel),
            "poseModelSpace" => Ok(__Field::m_poseModelSpace),
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
pub(super) struct __hkbCharacterAddedInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbCharacterAddedInfo<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbCharacterAddedInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbCharacterAddedInfo<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbCharacterAddedInfo<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbCharacterAddedInfoVisitor<'de> {
    type Value = hkbCharacterAddedInfo<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbCharacterAddedInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_instanceName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_templateName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_fullPathToProject: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_skeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_poseModelSpace: _serde::__private::Option<Vec<QsTransform>> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_characterId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterId",
                            ),
                        );
                    }
                    m_characterId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_instanceName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "instanceName",
                            ),
                        );
                    }
                    m_instanceName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_templateName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "templateName",
                            ),
                        );
                    }
                    m_templateName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_fullPathToProject) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fullPathToProject",
                            ),
                        );
                    }
                    m_fullPathToProject = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_skeleton) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "skeleton",
                            ),
                        );
                    }
                    m_skeleton = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_worldFromModel) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "worldFromModel",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 8usize)?;
                    m_worldFromModel = _serde::__private::Some(
                        match __A::next_value::<QsTransform>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_poseModelSpace) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "poseModelSpace",
                            ),
                        );
                    }
                    m_poseModelSpace = _serde::__private::Some(
                        match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
        __A::pad(&mut __map, 4usize, 0usize)?;
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_instanceName = match m_instanceName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("instanceName"),
                );
            }
        };
        let m_templateName = match m_templateName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("templateName"),
                );
            }
        };
        let m_fullPathToProject = match m_fullPathToProject {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fullPathToProject"),
                );
            }
        };
        let m_skeleton = match m_skeleton {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skeleton"),
                );
            }
        };
        let m_worldFromModel = match m_worldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("worldFromModel"),
                );
            }
        };
        let m_poseModelSpace = match m_poseModelSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseModelSpace"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterAddedInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_characterId,
            m_instanceName,
            m_templateName,
            m_fullPathToProject,
            m_skeleton,
            m_worldFromModel,
            m_poseModelSpace,
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
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_instanceName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_templateName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_fullPathToProject: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_skeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_poseModelSpace: _serde::__private::Option<Vec<QsTransform>> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_characterId => {
                        if _serde::__private::Option::is_some(&m_characterId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterId",
                                ),
                            );
                        }
                        m_characterId = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_instanceName => {
                        if _serde::__private::Option::is_some(&m_instanceName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "instanceName",
                                ),
                            );
                        }
                        m_instanceName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_templateName => {
                        if _serde::__private::Option::is_some(&m_templateName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "templateName",
                                ),
                            );
                        }
                        m_templateName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fullPathToProject => {
                        if _serde::__private::Option::is_some(&m_fullPathToProject) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fullPathToProject",
                                ),
                            );
                        }
                        m_fullPathToProject = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_skeleton => {
                        if _serde::__private::Option::is_some(&m_skeleton) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "skeleton",
                                ),
                            );
                        }
                        m_skeleton = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_worldFromModel => {
                        if _serde::__private::Option::is_some(&m_worldFromModel) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "worldFromModel",
                                ),
                            );
                        }
                        m_worldFromModel = _serde::__private::Some(
                            match __A::next_value::<QsTransform>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_poseModelSpace => {
                        if _serde::__private::Option::is_some(&m_poseModelSpace) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "poseModelSpace",
                                ),
                            );
                        }
                        m_poseModelSpace = _serde::__private::Some(
                            match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_instanceName = match m_instanceName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("instanceName"),
                );
            }
        };
        let m_templateName = match m_templateName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("templateName"),
                );
            }
        };
        let m_fullPathToProject = match m_fullPathToProject {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fullPathToProject"),
                );
            }
        };
        let m_skeleton = match m_skeleton {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("skeleton"),
                );
            }
        };
        let m_worldFromModel = match m_worldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("worldFromModel"),
                );
            }
        };
        let m_poseModelSpace = match m_poseModelSpace {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseModelSpace"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterAddedInfo {
            __ptr,
            parent,
            m_characterId,
            m_instanceName,
            m_templateName,
            m_fullPathToProject,
            m_skeleton,
            m_worldFromModel,
            m_poseModelSpace,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterAddedInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "characterId",
                "instanceName",
                "templateName",
                "fullPathToProject",
                "skeleton",
                "worldFromModel",
                "poseModelSpace",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterAddedInfo",
                FIELDS,
                __hkbCharacterAddedInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterAddedInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
