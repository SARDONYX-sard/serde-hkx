use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSLimbIKModifier`
/// -         version: `0`
/// -       signature: `0x8ea971e5`
/// -          size:  76(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSLimbIKModifier<'a> {
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
    /// -          name: `limitAngleDegrees`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_limitAngleDegrees: f32,
    /// # C++ Info
    /// -          name: `currentAngle`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_currentAngle: f32,
    /// # C++ Info
    /// -          name: `startBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_startBoneIndex: i16,
    /// # C++ Info
    /// -          name: `endBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  54(x86)/ 90(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_endBoneIndex: i16,
    /// # C++ Info
    /// -          name: `gain`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_gain: f32,
    /// # C++ Info
    /// -          name: `boneRadius`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_boneRadius: f32,
    /// # C++ Info
    /// -          name: `castOffset`(ctype: `hkReal`)
    /// -        offset:  64(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_castOffset: f32,
    /// # C++ Info
    /// -          name: `timeStep`(ctype: `hkReal`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeStep: f32,
    /// # C++ Info
    /// -          name: `pSkeletonMemory`(ctype: `void*`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pSkeletonMemory: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSLimbIKModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSLimbIKModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x8ea971e5)
        }
    }
    impl<'a> _serde::Serialize for BSLimbIKModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8ea971e5)));
            let mut serializer = __serializer
                .serialize_struct("BSLimbIKModifier", class_meta)?;
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
            serializer.serialize_field("limitAngleDegrees", &self.m_limitAngleDegrees)?;
            serializer.skip_field("currentAngle", &self.m_currentAngle)?;
            serializer.serialize_field("startBoneIndex", &self.m_startBoneIndex)?;
            serializer.serialize_field("endBoneIndex", &self.m_endBoneIndex)?;
            serializer.serialize_field("gain", &self.m_gain)?;
            serializer.serialize_field("boneRadius", &self.m_boneRadius)?;
            serializer.serialize_field("castOffset", &self.m_castOffset)?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("pSkeletonMemory", &self.m_pSkeletonMemory)?;
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
    m_limitAngleDegrees,
    m_currentAngle,
    m_startBoneIndex,
    m_endBoneIndex,
    m_gain,
    m_boneRadius,
    m_castOffset,
    m_timeStep,
    m_pSkeletonMemory,
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
            "limitAngleDegrees" => Ok(__Field::m_limitAngleDegrees),
            "startBoneIndex" => Ok(__Field::m_startBoneIndex),
            "endBoneIndex" => Ok(__Field::m_endBoneIndex),
            "gain" => Ok(__Field::m_gain),
            "boneRadius" => Ok(__Field::m_boneRadius),
            "castOffset" => Ok(__Field::m_castOffset),
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
pub(super) struct __BSLimbIKModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSLimbIKModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSLimbIKModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSLimbIKModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSLimbIKModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSLimbIKModifierVisitor<'de> {
    type Value = BSLimbIKModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSLimbIKModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_currentAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_boneRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_castOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_pSkeletonMemory: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..9usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_limitAngleDegrees) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitAngleDegrees",
                            ),
                        );
                    }
                    m_limitAngleDegrees = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_currentAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentAngle",
                            ),
                        );
                    }
                    m_currentAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_startBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startBoneIndex",
                            ),
                        );
                    }
                    m_startBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_endBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endBoneIndex",
                            ),
                        );
                    }
                    m_endBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_gain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("gain"),
                        );
                    }
                    m_gain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_boneRadius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "boneRadius",
                            ),
                        );
                    }
                    m_boneRadius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_castOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "castOffset",
                            ),
                        );
                    }
                    m_castOffset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_timeStep) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "timeStep",
                            ),
                        );
                    }
                    m_timeStep = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_pSkeletonMemory) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pSkeletonMemory",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_pSkeletonMemory = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
        let m_limitAngleDegrees = match m_limitAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
                );
            }
        };
        let m_currentAngle = match m_currentAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentAngle"),
                );
            }
        };
        let m_startBoneIndex = match m_startBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneIndex"),
                );
            }
        };
        let m_endBoneIndex = match m_endBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneIndex"),
                );
            }
        };
        let m_gain = match m_gain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("gain"),
                );
            }
        };
        let m_boneRadius = match m_boneRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneRadius"),
                );
            }
        };
        let m_castOffset = match m_castOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("castOffset"),
                );
            }
        };
        let m_timeStep = match m_timeStep {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("timeStep"),
                );
            }
        };
        let m_pSkeletonMemory = match m_pSkeletonMemory {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pSkeletonMemory"),
                );
            }
        };
        _serde::__private::Ok(BSLimbIKModifier {
            __ptr,
            parent,
            m_limitAngleDegrees,
            m_currentAngle,
            m_startBoneIndex,
            m_endBoneIndex,
            m_gain,
            m_boneRadius,
            m_castOffset,
            m_timeStep,
            m_pSkeletonMemory,
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
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_limitAngleDegrees: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_endBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_boneRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_castOffset: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_limitAngleDegrees => {
                        if _serde::__private::Option::is_some(&m_limitAngleDegrees) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitAngleDegrees",
                                ),
                            );
                        }
                        m_limitAngleDegrees = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_startBoneIndex => {
                        if _serde::__private::Option::is_some(&m_startBoneIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "startBoneIndex",
                                ),
                            );
                        }
                        m_startBoneIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_endBoneIndex => {
                        if _serde::__private::Option::is_some(&m_endBoneIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endBoneIndex",
                                ),
                            );
                        }
                        m_endBoneIndex = _serde::__private::Some(
                            match __A::next_value::<i16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_gain => {
                        if _serde::__private::Option::is_some(&m_gain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("gain"),
                            );
                        }
                        m_gain = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_boneRadius => {
                        if _serde::__private::Option::is_some(&m_boneRadius) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "boneRadius",
                                ),
                            );
                        }
                        m_boneRadius = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_castOffset => {
                        if _serde::__private::Option::is_some(&m_castOffset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "castOffset",
                                ),
                            );
                        }
                        m_castOffset = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        let m_limitAngleDegrees = match m_limitAngleDegrees {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitAngleDegrees"),
                );
            }
        };
        let m_startBoneIndex = match m_startBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startBoneIndex"),
                );
            }
        };
        let m_endBoneIndex = match m_endBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endBoneIndex"),
                );
            }
        };
        let m_gain = match m_gain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("gain"),
                );
            }
        };
        let m_boneRadius = match m_boneRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneRadius"),
                );
            }
        };
        let m_castOffset = match m_castOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("castOffset"),
                );
            }
        };
        _serde::__private::Ok(BSLimbIKModifier {
            __ptr,
            parent,
            m_limitAngleDegrees,
            m_startBoneIndex,
            m_endBoneIndex,
            m_gain,
            m_boneRadius,
            m_castOffset,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSLimbIKModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "limitAngleDegrees",
                "currentAngle",
                "startBoneIndex",
                "endBoneIndex",
                "gain",
                "boneRadius",
                "castOffset",
                "timeStep",
                "pSkeletonMemory",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSLimbIKModifier",
                FIELDS,
                __BSLimbIKModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSLimbIKModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
