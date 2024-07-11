use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSInterpValueModifier`
/// -         version: `0`
/// -       signature: `0x29adc802`
/// -          size:  64(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSInterpValueModifier<'a> {
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
    /// -          name: `source`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_source: f32,
    /// # C++ Info
    /// -          name: `target`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_target: f32,
    /// # C++ Info
    /// -          name: `result`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_result: f32,
    /// # C++ Info
    /// -          name: `gain`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_gain: f32,
    /// # C++ Info
    /// -          name: `timeStep`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeStep: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSInterpValueModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSInterpValueModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x29adc802)
        }
    }
    impl<'a> _serde::Serialize for BSInterpValueModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x29adc802)));
            let mut serializer = __serializer
                .serialize_struct("BSInterpValueModifier", class_meta)?;
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
            serializer.serialize_field("source", &self.m_source)?;
            serializer.serialize_field("target", &self.m_target)?;
            serializer.serialize_field("result", &self.m_result)?;
            serializer.serialize_field("gain", &self.m_gain)?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
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
    m_source,
    m_target,
    m_result,
    m_gain,
    m_timeStep,
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
            "source" => Ok(__Field::m_source),
            "target" => Ok(__Field::m_target),
            "result" => Ok(__Field::m_result),
            "gain" => Ok(__Field::m_gain),
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
pub(super) struct __BSInterpValueModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSInterpValueModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSInterpValueModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSInterpValueModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSInterpValueModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSInterpValueModifierVisitor<'de> {
    type Value = BSInterpValueModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSInterpValueModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_source: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_target: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_result: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_source) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("source"),
                        );
                    }
                    m_source = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_target) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("target"),
                        );
                    }
                    m_target = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_result) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("result"),
                        );
                    }
                    m_result = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
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
                4usize => {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_source = match m_source {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("source"),
                );
            }
        };
        let m_target = match m_target {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("target"),
                );
            }
        };
        let m_result = match m_result {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("result"),
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
        let m_timeStep = match m_timeStep {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("timeStep"),
                );
            }
        };
        _serde::__private::Ok(BSInterpValueModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_source,
            m_target,
            m_result,
            m_gain,
            m_timeStep,
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
        let mut m_source: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_target: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_result: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_source => {
                        if _serde::__private::Option::is_some(&m_source) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("source"),
                            );
                        }
                        m_source = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_target => {
                        if _serde::__private::Option::is_some(&m_target) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("target"),
                            );
                        }
                        m_target = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_result => {
                        if _serde::__private::Option::is_some(&m_result) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("result"),
                            );
                        }
                        m_result = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
                    _ => {}
                }
            }
        }
        let m_source = match m_source {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("source"),
                );
            }
        };
        let m_target = match m_target {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("target"),
                );
            }
        };
        let m_result = match m_result {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("result"),
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
        _serde::__private::Ok(BSInterpValueModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_source,
            m_target,
            m_result,
            m_gain,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSInterpValueModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["source", "target", "result", "gain", "timeStep"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSInterpValueModifier",
                FIELDS,
                __BSInterpValueModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSInterpValueModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
