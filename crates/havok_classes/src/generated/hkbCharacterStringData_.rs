use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterStringData`
/// -         version: `5`
/// -       signature: `0x655b42bc`
/// -          size: 132(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterStringData<'a> {
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
    /// -          name: `deformableSkinNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_deformableSkinNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `rigidSkinNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidSkinNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `animationNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animationNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `animationFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animationFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `characterPropertyNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterPropertyNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `retargetingSkeletonMapperFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_retargetingSkeletonMapperFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `lodNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_lodNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `mirroredSyncPointSubstringsA`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mirroredSyncPointSubstringsA: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `mirroredSyncPointSubstringsB`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset: 104(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_mirroredSyncPointSubstringsB: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset: 116(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `rigName`(ctype: `hkStringPtr`)
    /// -        offset: 120(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rigName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `ragdollName`(ctype: `hkStringPtr`)
    /// -        offset: 124(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_ragdollName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `behaviorFilename`(ctype: `hkStringPtr`)
    /// -        offset: 128(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorFilename: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterStringData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x655b42bc)
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x655b42bc)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "deformableSkinNames",
                    &self.m_deformableSkinNames,
                )?;
            serializer
                .serialize_array_meta_field("rigidSkinNames", &self.m_rigidSkinNames)?;
            serializer
                .serialize_array_meta_field("animationNames", &self.m_animationNames)?;
            serializer
                .serialize_array_meta_field(
                    "animationFilenames",
                    &self.m_animationFilenames,
                )?;
            serializer
                .serialize_array_meta_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                )?;
            serializer
                .serialize_array_meta_field(
                    "retargetingSkeletonMapperFilenames",
                    &self.m_retargetingSkeletonMapperFilenames,
                )?;
            serializer.serialize_array_meta_field("lodNames", &self.m_lodNames)?;
            serializer
                .serialize_array_meta_field(
                    "mirroredSyncPointSubstringsA",
                    &self.m_mirroredSyncPointSubstringsA,
                )?;
            serializer
                .serialize_array_meta_field(
                    "mirroredSyncPointSubstringsB",
                    &self.m_mirroredSyncPointSubstringsB,
                )?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_stringptr_meta_field("rigName", &self.m_rigName)?;
            serializer
                .serialize_stringptr_meta_field("ragdollName", &self.m_ragdollName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "behaviorFilename",
                    &self.m_behaviorFilename,
                )?;
            serializer
                .serialize_array_field(
                    "deformableSkinNames",
                    &self.m_deformableSkinNames,
                )?;
            serializer.serialize_array_field("rigidSkinNames", &self.m_rigidSkinNames)?;
            serializer.serialize_array_field("animationNames", &self.m_animationNames)?;
            serializer
                .serialize_array_field(
                    "animationFilenames",
                    &self.m_animationFilenames,
                )?;
            serializer
                .serialize_array_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                )?;
            serializer
                .serialize_array_field(
                    "retargetingSkeletonMapperFilenames",
                    &self.m_retargetingSkeletonMapperFilenames,
                )?;
            serializer.serialize_array_field("lodNames", &self.m_lodNames)?;
            serializer
                .serialize_array_field(
                    "mirroredSyncPointSubstringsA",
                    &self.m_mirroredSyncPointSubstringsA,
                )?;
            serializer
                .serialize_array_field(
                    "mirroredSyncPointSubstringsB",
                    &self.m_mirroredSyncPointSubstringsB,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_stringptr_field("rigName", &self.m_rigName)?;
            serializer.serialize_stringptr_field("ragdollName", &self.m_ragdollName)?;
            serializer
                .serialize_stringptr_field(
                    "behaviorFilename",
                    &self.m_behaviorFilename,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_deformableSkinNames,
    m_rigidSkinNames,
    m_animationNames,
    m_animationFilenames,
    m_characterPropertyNames,
    m_retargetingSkeletonMapperFilenames,
    m_lodNames,
    m_mirroredSyncPointSubstringsA,
    m_mirroredSyncPointSubstringsB,
    m_name,
    m_rigName,
    m_ragdollName,
    m_behaviorFilename,
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
            "deformableSkinNames" => Ok(__Field::m_deformableSkinNames),
            "rigidSkinNames" => Ok(__Field::m_rigidSkinNames),
            "animationNames" => Ok(__Field::m_animationNames),
            "animationFilenames" => Ok(__Field::m_animationFilenames),
            "characterPropertyNames" => Ok(__Field::m_characterPropertyNames),
            "retargetingSkeletonMapperFilenames" => {
                Ok(__Field::m_retargetingSkeletonMapperFilenames)
            }
            "lodNames" => Ok(__Field::m_lodNames),
            "mirroredSyncPointSubstringsA" => Ok(__Field::m_mirroredSyncPointSubstringsA),
            "mirroredSyncPointSubstringsB" => Ok(__Field::m_mirroredSyncPointSubstringsB),
            "name" => Ok(__Field::m_name),
            "rigName" => Ok(__Field::m_rigName),
            "ragdollName" => Ok(__Field::m_ragdollName),
            "behaviorFilename" => Ok(__Field::m_behaviorFilename),
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
pub(super) struct __hkbCharacterStringDataVisitor<'de> {
    marker: core::marker::PhantomData<hkbCharacterStringData<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbCharacterStringDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbCharacterStringData<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbCharacterStringData<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbCharacterStringDataVisitor<'de> {
    type Value = hkbCharacterStringData<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbCharacterStringData")
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
        let mut m_deformableSkinNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_rigidSkinNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_animationNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_animationFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_characterPropertyNames: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_retargetingSkeletonMapperFilenames: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_lodNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_mirroredSyncPointSubstringsA: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_mirroredSyncPointSubstringsB: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_rigName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_ragdollName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_behaviorFilename: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..13usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_deformableSkinNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "deformableSkinNames",
                            ),
                        );
                    }
                    m_deformableSkinNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_rigidSkinNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rigidSkinNames",
                            ),
                        );
                    }
                    m_rigidSkinNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_animationNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "animationNames",
                            ),
                        );
                    }
                    m_animationNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_animationFilenames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "animationFilenames",
                            ),
                        );
                    }
                    m_animationFilenames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_characterPropertyNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterPropertyNames",
                            ),
                        );
                    }
                    m_characterPropertyNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(
                        &m_retargetingSkeletonMapperFilenames,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "retargetingSkeletonMapperFilenames",
                            ),
                        );
                    }
                    m_retargetingSkeletonMapperFilenames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_lodNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lodNames",
                            ),
                        );
                    }
                    m_lodNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(
                        &m_mirroredSyncPointSubstringsA,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "mirroredSyncPointSubstringsA",
                            ),
                        );
                    }
                    m_mirroredSyncPointSubstringsA = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(
                        &m_mirroredSyncPointSubstringsB,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "mirroredSyncPointSubstringsB",
                            ),
                        );
                    }
                    m_mirroredSyncPointSubstringsB = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_name) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                        );
                    }
                    m_name = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_rigName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("rigName"),
                        );
                    }
                    m_rigName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_ragdollName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ragdollName",
                            ),
                        );
                    }
                    m_ragdollName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_behaviorFilename) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "behaviorFilename",
                            ),
                        );
                    }
                    m_behaviorFilename = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_deformableSkinNames = match m_deformableSkinNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "deformableSkinNames",
                    ),
                );
            }
        };
        let m_rigidSkinNames = match m_rigidSkinNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigidSkinNames"),
                );
            }
        };
        let m_animationNames = match m_animationNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("animationNames"),
                );
            }
        };
        let m_animationFilenames = match m_animationFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "animationFilenames",
                    ),
                );
            }
        };
        let m_characterPropertyNames = match m_characterPropertyNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "characterPropertyNames",
                    ),
                );
            }
        };
        let m_retargetingSkeletonMapperFilenames = match m_retargetingSkeletonMapperFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "retargetingSkeletonMapperFilenames",
                    ),
                );
            }
        };
        let m_lodNames = match m_lodNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lodNames"),
                );
            }
        };
        let m_mirroredSyncPointSubstringsA = match m_mirroredSyncPointSubstringsA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "mirroredSyncPointSubstringsA",
                    ),
                );
            }
        };
        let m_mirroredSyncPointSubstringsB = match m_mirroredSyncPointSubstringsB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "mirroredSyncPointSubstringsB",
                    ),
                );
            }
        };
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_rigName = match m_rigName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigName"),
                );
            }
        };
        let m_ragdollName = match m_ragdollName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ragdollName"),
                );
            }
        };
        let m_behaviorFilename = match m_behaviorFilename {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("behaviorFilename"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterStringData {
            __ptr,
            parent,
            m_deformableSkinNames,
            m_rigidSkinNames,
            m_animationNames,
            m_animationFilenames,
            m_characterPropertyNames,
            m_retargetingSkeletonMapperFilenames,
            m_lodNames,
            m_mirroredSyncPointSubstringsA,
            m_mirroredSyncPointSubstringsB,
            m_name,
            m_rigName,
            m_ragdollName,
            m_behaviorFilename,
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
        let mut m_deformableSkinNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_rigidSkinNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_animationNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_animationFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_characterPropertyNames: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_retargetingSkeletonMapperFilenames: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_lodNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_mirroredSyncPointSubstringsA: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_mirroredSyncPointSubstringsB: _serde::__private::Option<
            Vec<StringPtr<'de>>,
        > = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_rigName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_ragdollName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_behaviorFilename: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for _ in 0..13usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_deformableSkinNames => {
                        if _serde::__private::Option::is_some(&m_deformableSkinNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "deformableSkinNames",
                                ),
                            );
                        }
                        m_deformableSkinNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rigidSkinNames => {
                        if _serde::__private::Option::is_some(&m_rigidSkinNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rigidSkinNames",
                                ),
                            );
                        }
                        m_rigidSkinNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_animationNames => {
                        if _serde::__private::Option::is_some(&m_animationNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "animationNames",
                                ),
                            );
                        }
                        m_animationNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_animationFilenames => {
                        if _serde::__private::Option::is_some(&m_animationFilenames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "animationFilenames",
                                ),
                            );
                        }
                        m_animationFilenames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_characterPropertyNames => {
                        if _serde::__private::Option::is_some(
                            &m_characterPropertyNames,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterPropertyNames",
                                ),
                            );
                        }
                        m_characterPropertyNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_retargetingSkeletonMapperFilenames => {
                        if _serde::__private::Option::is_some(
                            &m_retargetingSkeletonMapperFilenames,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "retargetingSkeletonMapperFilenames",
                                ),
                            );
                        }
                        m_retargetingSkeletonMapperFilenames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_lodNames => {
                        if _serde::__private::Option::is_some(&m_lodNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "lodNames",
                                ),
                            );
                        }
                        m_lodNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_mirroredSyncPointSubstringsA => {
                        if _serde::__private::Option::is_some(
                            &m_mirroredSyncPointSubstringsA,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "mirroredSyncPointSubstringsA",
                                ),
                            );
                        }
                        m_mirroredSyncPointSubstringsA = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_mirroredSyncPointSubstringsB => {
                        if _serde::__private::Option::is_some(
                            &m_mirroredSyncPointSubstringsB,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "mirroredSyncPointSubstringsB",
                                ),
                            );
                        }
                        m_mirroredSyncPointSubstringsB = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_name => {
                        if _serde::__private::Option::is_some(&m_name) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                            );
                        }
                        m_name = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rigName => {
                        if _serde::__private::Option::is_some(&m_rigName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rigName",
                                ),
                            );
                        }
                        m_rigName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_ragdollName => {
                        if _serde::__private::Option::is_some(&m_ragdollName) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ragdollName",
                                ),
                            );
                        }
                        m_ragdollName = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_behaviorFilename => {
                        if _serde::__private::Option::is_some(&m_behaviorFilename) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "behaviorFilename",
                                ),
                            );
                        }
                        m_behaviorFilename = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_deformableSkinNames = match m_deformableSkinNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "deformableSkinNames",
                    ),
                );
            }
        };
        let m_rigidSkinNames = match m_rigidSkinNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigidSkinNames"),
                );
            }
        };
        let m_animationNames = match m_animationNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("animationNames"),
                );
            }
        };
        let m_animationFilenames = match m_animationFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "animationFilenames",
                    ),
                );
            }
        };
        let m_characterPropertyNames = match m_characterPropertyNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "characterPropertyNames",
                    ),
                );
            }
        };
        let m_retargetingSkeletonMapperFilenames = match m_retargetingSkeletonMapperFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "retargetingSkeletonMapperFilenames",
                    ),
                );
            }
        };
        let m_lodNames = match m_lodNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lodNames"),
                );
            }
        };
        let m_mirroredSyncPointSubstringsA = match m_mirroredSyncPointSubstringsA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "mirroredSyncPointSubstringsA",
                    ),
                );
            }
        };
        let m_mirroredSyncPointSubstringsB = match m_mirroredSyncPointSubstringsB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "mirroredSyncPointSubstringsB",
                    ),
                );
            }
        };
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_rigName = match m_rigName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigName"),
                );
            }
        };
        let m_ragdollName = match m_ragdollName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ragdollName"),
                );
            }
        };
        let m_behaviorFilename = match m_behaviorFilename {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("behaviorFilename"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterStringData {
            __ptr,
            parent,
            m_deformableSkinNames,
            m_rigidSkinNames,
            m_animationNames,
            m_animationFilenames,
            m_characterPropertyNames,
            m_retargetingSkeletonMapperFilenames,
            m_lodNames,
            m_mirroredSyncPointSubstringsA,
            m_mirroredSyncPointSubstringsB,
            m_name,
            m_rigName,
            m_ragdollName,
            m_behaviorFilename,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterStringData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "deformableSkinNames",
                "rigidSkinNames",
                "animationNames",
                "animationFilenames",
                "characterPropertyNames",
                "retargetingSkeletonMapperFilenames",
                "lodNames",
                "mirroredSyncPointSubstringsA",
                "mirroredSyncPointSubstringsB",
                "name",
                "rigName",
                "ragdollName",
                "behaviorFilename",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterStringData",
                FIELDS,
                __hkbCharacterStringDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterStringData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
