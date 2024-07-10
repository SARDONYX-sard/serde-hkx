use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbDampingModifier`
/// -         version: `1`
/// -       signature: `0x9a040f03`
/// -          size: 160(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbDampingModifier<'a> {
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
    /// -          name: `kP`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_kP: f32,
    /// # C++ Info
    /// -          name: `kI`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_kI: f32,
    /// # C++ Info
    /// -          name: `kD`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_kD: f32,
    /// # C++ Info
    /// -          name: `enableScalarDamping`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 92(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableScalarDamping: bool,
    /// # C++ Info
    /// -          name: `enableVectorDamping`(ctype: `hkBool`)
    /// -        offset:  57(x86)/ 93(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enableVectorDamping: bool,
    /// # C++ Info
    /// -          name: `rawValue`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_rawValue: f32,
    /// # C++ Info
    /// -          name: `dampedValue`(ctype: `hkReal`)
    /// -        offset:  64(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dampedValue: f32,
    /// # C++ Info
    /// -          name: `rawVector`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rawVector: Vector4,
    /// # C++ Info
    /// -          name: `dampedVector`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_dampedVector: Vector4,
    /// # C++ Info
    /// -          name: `vecErrorSum`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vecErrorSum: Vector4,
    /// # C++ Info
    /// -          name: `vecPreviousError`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vecPreviousError: Vector4,
    /// # C++ Info
    /// -          name: `errorSum`(ctype: `hkReal`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_errorSum: f32,
    /// # C++ Info
    /// -          name: `previousError`(ctype: `hkReal`)
    /// -        offset: 148(x86)/180(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_previousError: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbDampingModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbDampingModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9a040f03)
        }
    }
    impl<'a> _serde::Serialize for hkbDampingModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9a040f03)));
            let mut serializer = __serializer
                .serialize_struct("hkbDampingModifier", class_meta)?;
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
            serializer.serialize_field("kP", &self.m_kP)?;
            serializer.serialize_field("kI", &self.m_kI)?;
            serializer.serialize_field("kD", &self.m_kD)?;
            serializer
                .serialize_field("enableScalarDamping", &self.m_enableScalarDamping)?;
            serializer
                .serialize_field("enableVectorDamping", &self.m_enableVectorDamping)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("rawValue", &self.m_rawValue)?;
            serializer.serialize_field("dampedValue", &self.m_dampedValue)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("rawVector", &self.m_rawVector)?;
            serializer.serialize_field("dampedVector", &self.m_dampedVector)?;
            serializer.serialize_field("vecErrorSum", &self.m_vecErrorSum)?;
            serializer.serialize_field("vecPreviousError", &self.m_vecPreviousError)?;
            serializer.serialize_field("errorSum", &self.m_errorSum)?;
            serializer.serialize_field("previousError", &self.m_previousError)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
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
    m_kP,
    m_kI,
    m_kD,
    m_enableScalarDamping,
    m_enableVectorDamping,
    m_rawValue,
    m_dampedValue,
    m_rawVector,
    m_dampedVector,
    m_vecErrorSum,
    m_vecPreviousError,
    m_errorSum,
    m_previousError,
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
            "kP" => Ok(__Field::m_kP),
            "kI" => Ok(__Field::m_kI),
            "kD" => Ok(__Field::m_kD),
            "enableScalarDamping" => Ok(__Field::m_enableScalarDamping),
            "enableVectorDamping" => Ok(__Field::m_enableVectorDamping),
            "rawValue" => Ok(__Field::m_rawValue),
            "dampedValue" => Ok(__Field::m_dampedValue),
            "rawVector" => Ok(__Field::m_rawVector),
            "dampedVector" => Ok(__Field::m_dampedVector),
            "vecErrorSum" => Ok(__Field::m_vecErrorSum),
            "vecPreviousError" => Ok(__Field::m_vecPreviousError),
            "errorSum" => Ok(__Field::m_errorSum),
            "previousError" => Ok(__Field::m_previousError),
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
pub(super) struct __hkbDampingModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbDampingModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbDampingModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbDampingModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbDampingModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbDampingModifierVisitor<'de> {
    type Value = hkbDampingModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbDampingModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_kP: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_kI: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_kD: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_enableScalarDamping: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_enableVectorDamping: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_rawValue: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dampedValue: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_rawVector: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_dampedVector: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vecErrorSum: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vecPreviousError: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_errorSum: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_previousError: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..13usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_kP) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("kP"),
                        );
                    }
                    m_kP = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_kI) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("kI"),
                        );
                    }
                    m_kI = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_kD) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("kD"),
                        );
                    }
                    m_kD = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_enableScalarDamping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableScalarDamping",
                            ),
                        );
                    }
                    m_enableScalarDamping = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_enableVectorDamping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableVectorDamping",
                            ),
                        );
                    }
                    m_enableVectorDamping = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_rawValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rawValue",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_rawValue = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_dampedValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampedValue",
                            ),
                        );
                    }
                    m_dampedValue = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_rawVector) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rawVector",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 8usize)?;
                    m_rawVector = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_dampedVector) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampedVector",
                            ),
                        );
                    }
                    m_dampedVector = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_vecErrorSum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vecErrorSum",
                            ),
                        );
                    }
                    m_vecErrorSum = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_vecPreviousError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vecPreviousError",
                            ),
                        );
                    }
                    m_vecPreviousError = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_errorSum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "errorSum",
                            ),
                        );
                    }
                    m_errorSum = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_previousError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousError",
                            ),
                        );
                    }
                    m_previousError = _serde::__private::Some(
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
        __A::pad(&mut __map, 8usize, 8usize)?;
        let m_kP = match m_kP {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("kP"),
                );
            }
        };
        let m_kI = match m_kI {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("kI"),
                );
            }
        };
        let m_kD = match m_kD {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("kD"),
                );
            }
        };
        let m_enableScalarDamping = match m_enableScalarDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableScalarDamping",
                    ),
                );
            }
        };
        let m_enableVectorDamping = match m_enableVectorDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableVectorDamping",
                    ),
                );
            }
        };
        let m_rawValue = match m_rawValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rawValue"),
                );
            }
        };
        let m_dampedValue = match m_dampedValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dampedValue"),
                );
            }
        };
        let m_rawVector = match m_rawVector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rawVector"),
                );
            }
        };
        let m_dampedVector = match m_dampedVector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dampedVector"),
                );
            }
        };
        let m_vecErrorSum = match m_vecErrorSum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vecErrorSum"),
                );
            }
        };
        let m_vecPreviousError = match m_vecPreviousError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vecPreviousError"),
                );
            }
        };
        let m_errorSum = match m_errorSum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("errorSum"),
                );
            }
        };
        let m_previousError = match m_previousError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("previousError"),
                );
            }
        };
        _serde::__private::Ok(hkbDampingModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_kP,
            m_kI,
            m_kD,
            m_enableScalarDamping,
            m_enableVectorDamping,
            m_rawValue,
            m_dampedValue,
            m_rawVector,
            m_dampedVector,
            m_vecErrorSum,
            m_vecPreviousError,
            m_errorSum,
            m_previousError,
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
        let mut m_kP: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_kI: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_kD: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_enableScalarDamping: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_enableVectorDamping: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_rawValue: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_dampedValue: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_rawVector: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_dampedVector: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vecErrorSum: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vecPreviousError: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_errorSum: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_previousError: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_kP => {
                    if _serde::__private::Option::is_some(&m_kP) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("kP"),
                        );
                    }
                    m_kP = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_kI => {
                    if _serde::__private::Option::is_some(&m_kI) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("kI"),
                        );
                    }
                    m_kI = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_kD => {
                    if _serde::__private::Option::is_some(&m_kD) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("kD"),
                        );
                    }
                    m_kD = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_enableScalarDamping => {
                    if _serde::__private::Option::is_some(&m_enableScalarDamping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableScalarDamping",
                            ),
                        );
                    }
                    m_enableScalarDamping = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_enableVectorDamping => {
                    if _serde::__private::Option::is_some(&m_enableVectorDamping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableVectorDamping",
                            ),
                        );
                    }
                    m_enableVectorDamping = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_rawValue => {
                    if _serde::__private::Option::is_some(&m_rawValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rawValue",
                            ),
                        );
                    }
                    m_rawValue = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_dampedValue => {
                    if _serde::__private::Option::is_some(&m_dampedValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampedValue",
                            ),
                        );
                    }
                    m_dampedValue = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_rawVector => {
                    if _serde::__private::Option::is_some(&m_rawVector) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rawVector",
                            ),
                        );
                    }
                    m_rawVector = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_dampedVector => {
                    if _serde::__private::Option::is_some(&m_dampedVector) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "dampedVector",
                            ),
                        );
                    }
                    m_dampedVector = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_vecErrorSum => {
                    if _serde::__private::Option::is_some(&m_vecErrorSum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vecErrorSum",
                            ),
                        );
                    }
                    m_vecErrorSum = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_vecPreviousError => {
                    if _serde::__private::Option::is_some(&m_vecPreviousError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vecPreviousError",
                            ),
                        );
                    }
                    m_vecPreviousError = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_errorSum => {
                    if _serde::__private::Option::is_some(&m_errorSum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "errorSum",
                            ),
                        );
                    }
                    m_errorSum = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_previousError => {
                    if _serde::__private::Option::is_some(&m_previousError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousError",
                            ),
                        );
                    }
                    m_previousError = _serde::__private::Some(
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
        let m_kP = match m_kP {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("kP"),
                );
            }
        };
        let m_kI = match m_kI {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("kI"),
                );
            }
        };
        let m_kD = match m_kD {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("kD"),
                );
            }
        };
        let m_enableScalarDamping = match m_enableScalarDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableScalarDamping",
                    ),
                );
            }
        };
        let m_enableVectorDamping = match m_enableVectorDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableVectorDamping",
                    ),
                );
            }
        };
        let m_rawValue = match m_rawValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rawValue"),
                );
            }
        };
        let m_dampedValue = match m_dampedValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dampedValue"),
                );
            }
        };
        let m_rawVector = match m_rawVector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rawVector"),
                );
            }
        };
        let m_dampedVector = match m_dampedVector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("dampedVector"),
                );
            }
        };
        let m_vecErrorSum = match m_vecErrorSum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vecErrorSum"),
                );
            }
        };
        let m_vecPreviousError = match m_vecPreviousError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vecPreviousError"),
                );
            }
        };
        let m_errorSum = match m_errorSum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("errorSum"),
                );
            }
        };
        let m_previousError = match m_previousError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("previousError"),
                );
            }
        };
        _serde::__private::Ok(hkbDampingModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_kP,
            m_kI,
            m_kD,
            m_enableScalarDamping,
            m_enableVectorDamping,
            m_rawValue,
            m_dampedValue,
            m_rawVector,
            m_dampedVector,
            m_vecErrorSum,
            m_vecPreviousError,
            m_errorSum,
            m_previousError,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbDampingModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "kP",
                "kI",
                "kD",
                "enableScalarDamping",
                "enableVectorDamping",
                "rawValue",
                "dampedValue",
                "rawVector",
                "dampedVector",
                "vecErrorSum",
                "vecPreviousError",
                "errorSum",
                "previousError",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbDampingModifier",
                FIELDS,
                __hkbDampingModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbDampingModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
