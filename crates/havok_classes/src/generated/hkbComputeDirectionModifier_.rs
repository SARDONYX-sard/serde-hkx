use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbComputeDirectionModifier`
/// -         version: `0`
/// -       signature: `0xdf358bd3`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbComputeDirectionModifier<'a> {
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
    /// -          name: `pointIn`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointIn: Vector4,
    /// # C++ Info
    /// -          name: `pointOut`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointOut: Vector4,
    /// # C++ Info
    /// -          name: `groundAngleOut`(ctype: `hkReal`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_groundAngleOut: f32,
    /// # C++ Info
    /// -          name: `upAngleOut`(ctype: `hkReal`)
    /// -        offset:  84(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_upAngleOut: f32,
    /// # C++ Info
    /// -          name: `verticalOffset`(ctype: `hkReal`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// -          name: `reverseGroundAngle`(ctype: `hkBool`)
    /// -        offset:  92(x86)/124(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_reverseGroundAngle: bool,
    /// # C++ Info
    /// -          name: `reverseUpAngle`(ctype: `hkBool`)
    /// -        offset:  93(x86)/125(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_reverseUpAngle: bool,
    /// # C++ Info
    /// -          name: `projectPoint`(ctype: `hkBool`)
    /// -        offset:  94(x86)/126(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_projectPoint: bool,
    /// # C++ Info
    /// -          name: `normalizePoint`(ctype: `hkBool`)
    /// -        offset:  95(x86)/127(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_normalizePoint: bool,
    /// # C++ Info
    /// -          name: `computeOnlyOnce`(ctype: `hkBool`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computeOnlyOnce: bool,
    /// # C++ Info
    /// -          name: `computedOutput`(ctype: `hkBool`)
    /// -        offset:  97(x86)/129(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computedOutput: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbComputeDirectionModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbComputeDirectionModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xdf358bd3)
        }
    }
    impl<'a> _serde::Serialize for hkbComputeDirectionModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xdf358bd3)));
            let mut serializer = __serializer
                .serialize_struct("hkbComputeDirectionModifier", class_meta)?;
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
            serializer.serialize_field("pointIn", &self.m_pointIn)?;
            serializer.serialize_field("pointOut", &self.m_pointOut)?;
            serializer.serialize_field("groundAngleOut", &self.m_groundAngleOut)?;
            serializer.serialize_field("upAngleOut", &self.m_upAngleOut)?;
            serializer.serialize_field("verticalOffset", &self.m_verticalOffset)?;
            serializer
                .serialize_field("reverseGroundAngle", &self.m_reverseGroundAngle)?;
            serializer.serialize_field("reverseUpAngle", &self.m_reverseUpAngle)?;
            serializer.serialize_field("projectPoint", &self.m_projectPoint)?;
            serializer.serialize_field("normalizePoint", &self.m_normalizePoint)?;
            serializer.serialize_field("computeOnlyOnce", &self.m_computeOnlyOnce)?;
            serializer.serialize_field("computedOutput", &self.m_computedOutput)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
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
    m_pointIn,
    m_pointOut,
    m_groundAngleOut,
    m_upAngleOut,
    m_verticalOffset,
    m_reverseGroundAngle,
    m_reverseUpAngle,
    m_projectPoint,
    m_normalizePoint,
    m_computeOnlyOnce,
    m_computedOutput,
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
            "pointIn" => Ok(__Field::m_pointIn),
            "pointOut" => Ok(__Field::m_pointOut),
            "groundAngleOut" => Ok(__Field::m_groundAngleOut),
            "upAngleOut" => Ok(__Field::m_upAngleOut),
            "verticalOffset" => Ok(__Field::m_verticalOffset),
            "reverseGroundAngle" => Ok(__Field::m_reverseGroundAngle),
            "reverseUpAngle" => Ok(__Field::m_reverseUpAngle),
            "projectPoint" => Ok(__Field::m_projectPoint),
            "normalizePoint" => Ok(__Field::m_normalizePoint),
            "computeOnlyOnce" => Ok(__Field::m_computeOnlyOnce),
            "computedOutput" => Ok(__Field::m_computedOutput),
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
pub(super) struct __hkbComputeDirectionModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbComputeDirectionModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbComputeDirectionModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbComputeDirectionModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbComputeDirectionModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkbComputeDirectionModifierVisitor<'de> {
    type Value = hkbComputeDirectionModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbComputeDirectionModifier",
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
        let mut m_pointIn: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_pointOut: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_groundAngleOut: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_upAngleOut: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_reverseGroundAngle: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_reverseUpAngle: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_projectPoint: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_normalizePoint: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computeOnlyOnce: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computedOutput: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..11usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pointIn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pointIn"),
                        );
                    }
                    m_pointIn = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_pointOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pointOut",
                            ),
                        );
                    }
                    m_pointOut = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_groundAngleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "groundAngleOut",
                            ),
                        );
                    }
                    m_groundAngleOut = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_upAngleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "upAngleOut",
                            ),
                        );
                    }
                    m_upAngleOut = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_verticalOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalOffset",
                            ),
                        );
                    }
                    m_verticalOffset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_reverseGroundAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reverseGroundAngle",
                            ),
                        );
                    }
                    m_reverseGroundAngle = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_reverseUpAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reverseUpAngle",
                            ),
                        );
                    }
                    m_reverseUpAngle = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_projectPoint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "projectPoint",
                            ),
                        );
                    }
                    m_projectPoint = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_normalizePoint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "normalizePoint",
                            ),
                        );
                    }
                    m_normalizePoint = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_computeOnlyOnce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "computeOnlyOnce",
                            ),
                        );
                    }
                    m_computeOnlyOnce = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_computedOutput) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "computedOutput",
                            ),
                        );
                    }
                    m_computedOutput = _serde::__private::Some(
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
        __A::pad(&mut __map, 14usize, 14usize)?;
        let m_pointIn = match m_pointIn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pointIn"),
                );
            }
        };
        let m_pointOut = match m_pointOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pointOut"),
                );
            }
        };
        let m_groundAngleOut = match m_groundAngleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("groundAngleOut"),
                );
            }
        };
        let m_upAngleOut = match m_upAngleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("upAngleOut"),
                );
            }
        };
        let m_verticalOffset = match m_verticalOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("verticalOffset"),
                );
            }
        };
        let m_reverseGroundAngle = match m_reverseGroundAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "reverseGroundAngle",
                    ),
                );
            }
        };
        let m_reverseUpAngle = match m_reverseUpAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reverseUpAngle"),
                );
            }
        };
        let m_projectPoint = match m_projectPoint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("projectPoint"),
                );
            }
        };
        let m_normalizePoint = match m_normalizePoint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("normalizePoint"),
                );
            }
        };
        let m_computeOnlyOnce = match m_computeOnlyOnce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computeOnlyOnce"),
                );
            }
        };
        let m_computedOutput = match m_computedOutput {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computedOutput"),
                );
            }
        };
        _serde::__private::Ok(hkbComputeDirectionModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_pointIn,
            m_pointOut,
            m_groundAngleOut,
            m_upAngleOut,
            m_verticalOffset,
            m_reverseGroundAngle,
            m_reverseUpAngle,
            m_projectPoint,
            m_normalizePoint,
            m_computeOnlyOnce,
            m_computedOutput,
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
        let mut m_pointIn: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_pointOut: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_groundAngleOut: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_upAngleOut: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_reverseGroundAngle: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_reverseUpAngle: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_projectPoint: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_normalizePoint: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computeOnlyOnce: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computedOutput: _serde::__private::Option<bool> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_pointIn => {
                    if _serde::__private::Option::is_some(&m_pointIn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pointIn"),
                        );
                    }
                    m_pointIn = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_pointOut => {
                    if _serde::__private::Option::is_some(&m_pointOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pointOut",
                            ),
                        );
                    }
                    m_pointOut = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_groundAngleOut => {
                    if _serde::__private::Option::is_some(&m_groundAngleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "groundAngleOut",
                            ),
                        );
                    }
                    m_groundAngleOut = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_upAngleOut => {
                    if _serde::__private::Option::is_some(&m_upAngleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "upAngleOut",
                            ),
                        );
                    }
                    m_upAngleOut = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_verticalOffset => {
                    if _serde::__private::Option::is_some(&m_verticalOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalOffset",
                            ),
                        );
                    }
                    m_verticalOffset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_reverseGroundAngle => {
                    if _serde::__private::Option::is_some(&m_reverseGroundAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reverseGroundAngle",
                            ),
                        );
                    }
                    m_reverseGroundAngle = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_reverseUpAngle => {
                    if _serde::__private::Option::is_some(&m_reverseUpAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reverseUpAngle",
                            ),
                        );
                    }
                    m_reverseUpAngle = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_projectPoint => {
                    if _serde::__private::Option::is_some(&m_projectPoint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "projectPoint",
                            ),
                        );
                    }
                    m_projectPoint = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_normalizePoint => {
                    if _serde::__private::Option::is_some(&m_normalizePoint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "normalizePoint",
                            ),
                        );
                    }
                    m_normalizePoint = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_computeOnlyOnce => {
                    if _serde::__private::Option::is_some(&m_computeOnlyOnce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "computeOnlyOnce",
                            ),
                        );
                    }
                    m_computeOnlyOnce = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_computedOutput => {
                    if _serde::__private::Option::is_some(&m_computedOutput) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "computedOutput",
                            ),
                        );
                    }
                    m_computedOutput = _serde::__private::Some(
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
        let m_pointIn = match m_pointIn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pointIn"),
                );
            }
        };
        let m_pointOut = match m_pointOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pointOut"),
                );
            }
        };
        let m_groundAngleOut = match m_groundAngleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("groundAngleOut"),
                );
            }
        };
        let m_upAngleOut = match m_upAngleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("upAngleOut"),
                );
            }
        };
        let m_verticalOffset = match m_verticalOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("verticalOffset"),
                );
            }
        };
        let m_reverseGroundAngle = match m_reverseGroundAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "reverseGroundAngle",
                    ),
                );
            }
        };
        let m_reverseUpAngle = match m_reverseUpAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reverseUpAngle"),
                );
            }
        };
        let m_projectPoint = match m_projectPoint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("projectPoint"),
                );
            }
        };
        let m_normalizePoint = match m_normalizePoint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("normalizePoint"),
                );
            }
        };
        let m_computeOnlyOnce = match m_computeOnlyOnce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computeOnlyOnce"),
                );
            }
        };
        let m_computedOutput = match m_computedOutput {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computedOutput"),
                );
            }
        };
        _serde::__private::Ok(hkbComputeDirectionModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_pointIn,
            m_pointOut,
            m_groundAngleOut,
            m_upAngleOut,
            m_verticalOffset,
            m_reverseGroundAngle,
            m_reverseUpAngle,
            m_projectPoint,
            m_normalizePoint,
            m_computeOnlyOnce,
            m_computedOutput,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbComputeDirectionModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "pointIn",
                "pointOut",
                "groundAngleOut",
                "upAngleOut",
                "verticalOffset",
                "reverseGroundAngle",
                "reverseUpAngle",
                "projectPoint",
                "normalizePoint",
                "computeOnlyOnce",
                "computedOutput",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbComputeDirectionModifier",
                FIELDS,
                __hkbComputeDirectionModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbComputeDirectionModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
