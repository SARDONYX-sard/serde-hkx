use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbComputeRotationToTargetModifier`
/// -         version: `0`
/// -       signature: `0x47665f1c`
/// -          size: 160(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbComputeRotationToTargetModifier<'a> {
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
    /// -          name: `rotationOut`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotationOut: Quaternion,
    /// # C++ Info
    /// -          name: `targetPosition`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetPosition: Vector4,
    /// # C++ Info
    /// -          name: `currentPosition`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_currentPosition: Vector4,
    /// # C++ Info
    /// -          name: `currentRotation`(ctype: `hkQuaternion`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_currentRotation: Quaternion,
    /// # C++ Info
    /// -          name: `localAxisOfRotation`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_localAxisOfRotation: Vector4,
    /// # C++ Info
    /// -          name: `localFacingDirection`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_localFacingDirection: Vector4,
    /// # C++ Info
    /// -          name: `resultIsDelta`(ctype: `hkBool`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_resultIsDelta: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbComputeRotationToTargetModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbComputeRotationToTargetModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x47665f1c)
        }
    }
    impl<'a> _serde::Serialize for hkbComputeRotationToTargetModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x47665f1c)));
            let mut serializer = __serializer
                .serialize_struct("hkbComputeRotationToTargetModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("rotationOut", &self.m_rotationOut)?;
            serializer.serialize_field("targetPosition", &self.m_targetPosition)?;
            serializer.serialize_field("currentPosition", &self.m_currentPosition)?;
            serializer.serialize_field("currentRotation", &self.m_currentRotation)?;
            serializer
                .serialize_field("localAxisOfRotation", &self.m_localAxisOfRotation)?;
            serializer
                .serialize_field("localFacingDirection", &self.m_localFacingDirection)?;
            serializer.serialize_field("resultIsDelta", &self.m_resultIsDelta)?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 15usize].as_slice())?;
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
    m_rotationOut,
    m_targetPosition,
    m_currentPosition,
    m_currentRotation,
    m_localAxisOfRotation,
    m_localFacingDirection,
    m_resultIsDelta,
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
            "rotationOut" => Ok(__Field::m_rotationOut),
            "targetPosition" => Ok(__Field::m_targetPosition),
            "currentPosition" => Ok(__Field::m_currentPosition),
            "currentRotation" => Ok(__Field::m_currentRotation),
            "localAxisOfRotation" => Ok(__Field::m_localAxisOfRotation),
            "localFacingDirection" => Ok(__Field::m_localFacingDirection),
            "resultIsDelta" => Ok(__Field::m_resultIsDelta),
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
pub(super) struct __hkbComputeRotationToTargetModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbComputeRotationToTargetModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbComputeRotationToTargetModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbComputeRotationToTargetModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbComputeRotationToTargetModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkbComputeRotationToTargetModifierVisitor<'de> {
    type Value = hkbComputeRotationToTargetModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbComputeRotationToTargetModifier",
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
        let parent = __A::next_value(&mut __map)?;
        let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_targetPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_currentPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_currentRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_localAxisOfRotation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_localFacingDirection: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_resultIsDelta: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_rotationOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotationOut",
                            ),
                        );
                    }
                    m_rotationOut = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_targetPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "targetPosition",
                            ),
                        );
                    }
                    m_targetPosition = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_currentPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentPosition",
                            ),
                        );
                    }
                    m_currentPosition = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_currentRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "currentRotation",
                            ),
                        );
                    }
                    m_currentRotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_localAxisOfRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localAxisOfRotation",
                            ),
                        );
                    }
                    m_localAxisOfRotation = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_localFacingDirection) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localFacingDirection",
                            ),
                        );
                    }
                    m_localFacingDirection = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_resultIsDelta) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "resultIsDelta",
                            ),
                        );
                    }
                    m_resultIsDelta = _serde::__private::Some(
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
        __A::pad(&mut __map, 15usize, 15usize)?;
        let m_rotationOut = match m_rotationOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationOut"),
                );
            }
        };
        let m_targetPosition = match m_targetPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetPosition"),
                );
            }
        };
        let m_currentPosition = match m_currentPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentPosition"),
                );
            }
        };
        let m_currentRotation = match m_currentRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentRotation"),
                );
            }
        };
        let m_localAxisOfRotation = match m_localAxisOfRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "localAxisOfRotation",
                    ),
                );
            }
        };
        let m_localFacingDirection = match m_localFacingDirection {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "localFacingDirection",
                    ),
                );
            }
        };
        let m_resultIsDelta = match m_resultIsDelta {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("resultIsDelta"),
                );
            }
        };
        _serde::__private::Ok(hkbComputeRotationToTargetModifier {
            __ptr,
            parent,
            m_rotationOut,
            m_targetPosition,
            m_currentPosition,
            m_currentRotation,
            m_localAxisOfRotation,
            m_localFacingDirection,
            m_resultIsDelta,
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
        let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_targetPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_currentPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_currentRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_localAxisOfRotation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_localFacingDirection: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_resultIsDelta: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_rotationOut => {
                        if _serde::__private::Option::is_some(&m_rotationOut) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotationOut",
                                ),
                            );
                        }
                        m_rotationOut = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_targetPosition => {
                        if _serde::__private::Option::is_some(&m_targetPosition) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "targetPosition",
                                ),
                            );
                        }
                        m_targetPosition = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_currentPosition => {
                        if _serde::__private::Option::is_some(&m_currentPosition) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "currentPosition",
                                ),
                            );
                        }
                        m_currentPosition = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_currentRotation => {
                        if _serde::__private::Option::is_some(&m_currentRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "currentRotation",
                                ),
                            );
                        }
                        m_currentRotation = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_localAxisOfRotation => {
                        if _serde::__private::Option::is_some(&m_localAxisOfRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "localAxisOfRotation",
                                ),
                            );
                        }
                        m_localAxisOfRotation = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_localFacingDirection => {
                        if _serde::__private::Option::is_some(&m_localFacingDirection) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "localFacingDirection",
                                ),
                            );
                        }
                        m_localFacingDirection = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_resultIsDelta => {
                        if _serde::__private::Option::is_some(&m_resultIsDelta) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "resultIsDelta",
                                ),
                            );
                        }
                        m_resultIsDelta = _serde::__private::Some(
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
        let m_rotationOut = match m_rotationOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationOut"),
                );
            }
        };
        let m_targetPosition = match m_targetPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetPosition"),
                );
            }
        };
        let m_currentPosition = match m_currentPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentPosition"),
                );
            }
        };
        let m_currentRotation = match m_currentRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("currentRotation"),
                );
            }
        };
        let m_localAxisOfRotation = match m_localAxisOfRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "localAxisOfRotation",
                    ),
                );
            }
        };
        let m_localFacingDirection = match m_localFacingDirection {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "localFacingDirection",
                    ),
                );
            }
        };
        let m_resultIsDelta = match m_resultIsDelta {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("resultIsDelta"),
                );
            }
        };
        _serde::__private::Ok(hkbComputeRotationToTargetModifier {
            __ptr,
            parent,
            m_rotationOut,
            m_targetPosition,
            m_currentPosition,
            m_currentRotation,
            m_localAxisOfRotation,
            m_localFacingDirection,
            m_resultIsDelta,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbComputeRotationToTargetModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "rotationOut",
                "targetPosition",
                "currentPosition",
                "currentRotation",
                "localAxisOfRotation",
                "localFacingDirection",
                "resultIsDelta",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbComputeRotationToTargetModifier",
                FIELDS,
                __hkbComputeRotationToTargetModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbComputeRotationToTargetModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
