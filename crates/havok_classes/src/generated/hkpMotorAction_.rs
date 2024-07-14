use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMotorAction`
/// -         version: `0`
/// -       signature: `0x8ff131d9`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMotorAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpUnaryAction<'a>,
    /// # C++ Info
    /// -          name: `axis`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_axis: Vector4,
    /// # C++ Info
    /// -          name: `spinRate`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_spinRate: f32,
    /// # C++ Info
    /// -          name: `gain`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_gain: f32,
    /// # C++ Info
    /// -          name: `active`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 88(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_active: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpMotorAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMotorAction"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x8ff131d9)
        }
    }
    impl<'a> _serde::Serialize for hkpMotorAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8ff131d9)));
            let mut serializer = __serializer
                .serialize_struct("hkpMotorAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entity", &self.parent.m_entity)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("axis", &self.m_axis)?;
            serializer.serialize_field("spinRate", &self.m_spinRate)?;
            serializer.serialize_field("gain", &self.m_gain)?;
            serializer.serialize_field("active", &self.m_active)?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_axis,
    m_spinRate,
    m_gain,
    m_active,
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
            "axis" => Ok(__Field::m_axis),
            "spinRate" => Ok(__Field::m_spinRate),
            "gain" => Ok(__Field::m_gain),
            "active" => Ok(__Field::m_active),
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
pub(super) struct __hkpMotorActionVisitor<'de> {
    marker: core::marker::PhantomData<hkpMotorAction<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpMotorActionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpMotorAction<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpMotorAction<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpMotorActionVisitor<'de> {
    type Value = hkpMotorAction<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpMotorAction")
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
        let mut m_axis: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_spinRate: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_active: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_axis) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("axis"),
                        );
                    }
                    m_axis = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_spinRate) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "spinRate",
                            ),
                        );
                    }
                    m_spinRate = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_active) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("active"),
                        );
                    }
                    m_active = _serde::__private::Some(
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
        __A::pad(&mut __map, 7usize, 7usize)?;
        let m_axis = match m_axis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("axis"),
                );
            }
        };
        let m_spinRate = match m_spinRate {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinRate"),
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
        let m_active = match m_active {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("active"),
                );
            }
        };
        _serde::__private::Ok(hkpMotorAction {
            __ptr,
            parent,
            m_axis,
            m_spinRate,
            m_gain,
            m_active,
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
        let parent = __hkpUnaryActionVisitor::visit_as_parent(&mut __map)?;
        let mut m_axis: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_spinRate: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_gain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_active: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_axis => {
                        if _serde::__private::Option::is_some(&m_axis) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("axis"),
                            );
                        }
                        m_axis = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_spinRate => {
                        if _serde::__private::Option::is_some(&m_spinRate) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "spinRate",
                                ),
                            );
                        }
                        m_spinRate = _serde::__private::Some(
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
                    __Field::m_active => {
                        if _serde::__private::Option::is_some(&m_active) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("active"),
                            );
                        }
                        m_active = _serde::__private::Some(
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
        let m_axis = match m_axis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("axis"),
                );
            }
        };
        let m_spinRate = match m_spinRate {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spinRate"),
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
        let m_active = match m_active {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("active"),
                );
            }
        };
        _serde::__private::Ok(hkpMotorAction {
            __ptr,
            parent,
            m_axis,
            m_spinRate,
            m_gain,
            m_active,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMotorAction<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["axis", "spinRate", "gain", "active"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMotorAction",
                FIELDS,
                __hkpMotorActionVisitor {
                    marker: _serde::__private::PhantomData::<hkpMotorAction>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
