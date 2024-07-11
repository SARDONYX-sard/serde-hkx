use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbExtractRagdollPoseModifier`
/// -         version: `1`
/// -       signature: `0x804dcbab`
/// -          size:  52(x86)/ 88(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExtractRagdollPoseModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `poseMatchingBone0`(ctype: `hkInt16`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone0: i16,
    /// # C++ Info
    /// -          name: `poseMatchingBone1`(ctype: `hkInt16`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone1: i16,
    /// # C++ Info
    /// -          name: `poseMatchingBone2`(ctype: `hkInt16`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_poseMatchingBone2: i16,
    /// # C++ Info
    /// -          name: `enableComputeWorldFromModel`(ctype: `hkBool`)
    /// -        offset:  50(x86)/ 86(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableComputeWorldFromModel: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbExtractRagdollPoseModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbExtractRagdollPoseModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x804dcbab)
        }
    }
    impl<'a> _serde::Serialize for hkbExtractRagdollPoseModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x804dcbab)));
            let mut serializer = __serializer
                .serialize_struct("hkbExtractRagdollPoseModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("poseMatchingBone0", &self.m_poseMatchingBone0)?;
            serializer.serialize_field("poseMatchingBone1", &self.m_poseMatchingBone1)?;
            serializer.serialize_field("poseMatchingBone2", &self.m_poseMatchingBone2)?;
            serializer
                .serialize_field(
                    "enableComputeWorldFromModel",
                    &self.m_enableComputeWorldFromModel,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_poseMatchingBone0,
    m_poseMatchingBone1,
    m_poseMatchingBone2,
    m_enableComputeWorldFromModel,
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
            "poseMatchingBone0" => Ok(__Field::m_poseMatchingBone0),
            "poseMatchingBone1" => Ok(__Field::m_poseMatchingBone1),
            "poseMatchingBone2" => Ok(__Field::m_poseMatchingBone2),
            "enableComputeWorldFromModel" => Ok(__Field::m_enableComputeWorldFromModel),
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
pub(super) struct __hkbExtractRagdollPoseModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbExtractRagdollPoseModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbExtractRagdollPoseModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbExtractRagdollPoseModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbExtractRagdollPoseModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkbExtractRagdollPoseModifierVisitor<'de> {
    type Value = hkbExtractRagdollPoseModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbExtractRagdollPoseModifier",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_poseMatchingBone0: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_poseMatchingBone1: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_poseMatchingBone2: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_enableComputeWorldFromModel: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_poseMatchingBone0) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "poseMatchingBone0",
                            ),
                        );
                    }
                    m_poseMatchingBone0 = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_poseMatchingBone1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "poseMatchingBone1",
                            ),
                        );
                    }
                    m_poseMatchingBone1 = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_poseMatchingBone2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "poseMatchingBone2",
                            ),
                        );
                    }
                    m_poseMatchingBone2 = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(
                        &m_enableComputeWorldFromModel,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableComputeWorldFromModel",
                            ),
                        );
                    }
                    m_enableComputeWorldFromModel = _serde::__private::Some(
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
        let m_poseMatchingBone0 = match m_poseMatchingBone0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseMatchingBone0"),
                );
            }
        };
        let m_poseMatchingBone1 = match m_poseMatchingBone1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseMatchingBone1"),
                );
            }
        };
        let m_poseMatchingBone2 = match m_poseMatchingBone2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseMatchingBone2"),
                );
            }
        };
        let m_enableComputeWorldFromModel = match m_enableComputeWorldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableComputeWorldFromModel",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbExtractRagdollPoseModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_poseMatchingBone0,
            m_poseMatchingBone1,
            m_poseMatchingBone2,
            m_enableComputeWorldFromModel,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_poseMatchingBone0: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_poseMatchingBone1: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_poseMatchingBone2: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_enableComputeWorldFromModel: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_poseMatchingBone0 => {
                        if _serde::__private::Option::is_some(&m_poseMatchingBone0) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "poseMatchingBone0",
                                ),
                            );
                        }
                        m_poseMatchingBone0 = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_poseMatchingBone1 => {
                        if _serde::__private::Option::is_some(&m_poseMatchingBone1) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "poseMatchingBone1",
                                ),
                            );
                        }
                        m_poseMatchingBone1 = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_poseMatchingBone2 => {
                        if _serde::__private::Option::is_some(&m_poseMatchingBone2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "poseMatchingBone2",
                                ),
                            );
                        }
                        m_poseMatchingBone2 = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enableComputeWorldFromModel => {
                        if _serde::__private::Option::is_some(
                            &m_enableComputeWorldFromModel,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enableComputeWorldFromModel",
                                ),
                            );
                        }
                        m_enableComputeWorldFromModel = _serde::__private::Some(
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
        }
        let m_poseMatchingBone0 = match m_poseMatchingBone0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseMatchingBone0"),
                );
            }
        };
        let m_poseMatchingBone1 = match m_poseMatchingBone1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseMatchingBone1"),
                );
            }
        };
        let m_poseMatchingBone2 = match m_poseMatchingBone2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("poseMatchingBone2"),
                );
            }
        };
        let m_enableComputeWorldFromModel = match m_enableComputeWorldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableComputeWorldFromModel",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbExtractRagdollPoseModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_poseMatchingBone0,
            m_poseMatchingBone1,
            m_poseMatchingBone2,
            m_enableComputeWorldFromModel,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbExtractRagdollPoseModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "poseMatchingBone0",
                "poseMatchingBone1",
                "poseMatchingBone2",
                "enableComputeWorldFromModel",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbExtractRagdollPoseModifier",
                FIELDS,
                __hkbExtractRagdollPoseModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbExtractRagdollPoseModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
