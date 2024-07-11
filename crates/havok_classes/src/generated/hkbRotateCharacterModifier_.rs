use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbRotateCharacterModifier`
/// -         version: `1`
/// -       signature: `0x877ebc0b`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbRotateCharacterModifier<'a> {
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
    /// -          name: `degreesPerSecond`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_degreesPerSecond: f32,
    /// # C++ Info
    /// -          name: `speedMultiplier`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_speedMultiplier: f32,
    /// # C++ Info
    /// -          name: `axisOfRotation`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_axisOfRotation: Vector4,
    /// # C++ Info
    /// -          name: `angle`(ctype: `hkReal`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_angle: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbRotateCharacterModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbRotateCharacterModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x877ebc0b)
        }
    }
    impl<'a> _serde::Serialize for hkbRotateCharacterModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x877ebc0b)));
            let mut serializer = __serializer
                .serialize_struct("hkbRotateCharacterModifier", class_meta)?;
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
            serializer.serialize_field("degreesPerSecond", &self.m_degreesPerSecond)?;
            serializer.serialize_field("speedMultiplier", &self.m_speedMultiplier)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("axisOfRotation", &self.m_axisOfRotation)?;
            serializer.skip_field("angle", &self.m_angle)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
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
    m_degreesPerSecond,
    m_speedMultiplier,
    m_axisOfRotation,
    m_angle,
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
            "degreesPerSecond" => Ok(__Field::m_degreesPerSecond),
            "speedMultiplier" => Ok(__Field::m_speedMultiplier),
            "axisOfRotation" => Ok(__Field::m_axisOfRotation),
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
pub(super) struct __hkbRotateCharacterModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbRotateCharacterModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbRotateCharacterModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbRotateCharacterModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbRotateCharacterModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkbRotateCharacterModifierVisitor<'de> {
    type Value = hkbRotateCharacterModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbRotateCharacterModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_degreesPerSecond: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_speedMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_axisOfRotation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_angle: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_degreesPerSecond) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "degreesPerSecond",
                            ),
                        );
                    }
                    m_degreesPerSecond = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_speedMultiplier) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "speedMultiplier",
                            ),
                        );
                    }
                    m_speedMultiplier = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_axisOfRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "axisOfRotation",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 8usize)?;
                    m_axisOfRotation = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_angle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("angle"),
                        );
                    }
                    m_angle = _serde::__private::Some(
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
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_degreesPerSecond = match m_degreesPerSecond {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("degreesPerSecond"),
                );
            }
        };
        let m_speedMultiplier = match m_speedMultiplier {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("speedMultiplier"),
                );
            }
        };
        let m_axisOfRotation = match m_axisOfRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("axisOfRotation"),
                );
            }
        };
        let m_angle = match m_angle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("angle"),
                );
            }
        };
        _serde::__private::Ok(hkbRotateCharacterModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_degreesPerSecond,
            m_speedMultiplier,
            m_axisOfRotation,
            m_angle,
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
        let mut m_degreesPerSecond: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_speedMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_axisOfRotation: _serde::__private::Option<Vector4> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_degreesPerSecond => {
                        if _serde::__private::Option::is_some(&m_degreesPerSecond) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "degreesPerSecond",
                                ),
                            );
                        }
                        m_degreesPerSecond = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_speedMultiplier => {
                        if _serde::__private::Option::is_some(&m_speedMultiplier) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "speedMultiplier",
                                ),
                            );
                        }
                        m_speedMultiplier = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_axisOfRotation => {
                        if _serde::__private::Option::is_some(&m_axisOfRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "axisOfRotation",
                                ),
                            );
                        }
                        m_axisOfRotation = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
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
        let m_degreesPerSecond = match m_degreesPerSecond {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("degreesPerSecond"),
                );
            }
        };
        let m_speedMultiplier = match m_speedMultiplier {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("speedMultiplier"),
                );
            }
        };
        let m_axisOfRotation = match m_axisOfRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("axisOfRotation"),
                );
            }
        };
        _serde::__private::Ok(hkbRotateCharacterModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_degreesPerSecond,
            m_speedMultiplier,
            m_axisOfRotation,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbRotateCharacterModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "degreesPerSecond",
                "speedMultiplier",
                "axisOfRotation",
                "angle",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbRotateCharacterModifier",
                FIELDS,
                __hkbRotateCharacterModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbRotateCharacterModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
