use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandIkControlsModifierHand`
/// -         version: `0`
/// -       signature: `0x9c72e9e3`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkControlsModifierHand {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `controlData`(ctype: `struct hkbHandIkControlData`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  80(x86)/ 96(x86_64)
    ///
    pub m_controlData: hkbHandIkControlData,
    /// # C++ Info
    /// -          name: `handIndex`(ctype: `hkInt32`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_handIndex: i32,
    /// # C++ Info
    /// -          name: `enable`(ctype: `hkBool`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enable: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbHandIkControlsModifierHand {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbHandIkControlsModifierHand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9c72e9e3)
        }
    }
    impl _serde::Serialize for hkbHandIkControlsModifierHand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9c72e9e3)));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkControlsModifierHand", class_meta)?;
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_field("handIndex", &self.m_handIndex)?;
            serializer.serialize_field("enable", &self.m_enable)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_controlData,
    m_handIndex,
    m_enable,
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
            "controlData" => Ok(__Field::m_controlData),
            "handIndex" => Ok(__Field::m_handIndex),
            "enable" => Ok(__Field::m_enable),
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
pub(super) struct __hkbHandIkControlsModifierHandVisitor<'de> {
    marker: core::marker::PhantomData<hkbHandIkControlsModifierHand>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbHandIkControlsModifierHandVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbHandIkControlsModifierHand, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbHandIkControlsModifierHand>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbHandIkControlsModifierHandVisitor<'de> {
    type Value = hkbHandIkControlsModifierHand;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbHandIkControlsModifierHand",
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
        let mut m_controlData: _serde::__private::Option<hkbHandIkControlData> = _serde::__private::None;
        let mut m_handIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_controlData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "controlData",
                            ),
                        );
                    }
                    m_controlData = _serde::__private::Some(
                        match __A::next_value::<hkbHandIkControlData>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_handIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handIndex",
                            ),
                        );
                    }
                    m_handIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_enable) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                        );
                    }
                    m_enable = _serde::__private::Some(
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
        __A::pad(&mut __map, 11usize, 11usize)?;
        let m_controlData = match m_controlData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("controlData"),
                );
            }
        };
        let m_handIndex = match m_handIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handIndex"),
                );
            }
        };
        let m_enable = match m_enable {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enable"),
                );
            }
        };
        _serde::__private::Ok(hkbHandIkControlsModifierHand {
            __ptr,
            m_controlData,
            m_handIndex,
            m_enable,
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
        let mut m_controlData: _serde::__private::Option<hkbHandIkControlData> = _serde::__private::None;
        let mut m_handIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_controlData => {
                        if _serde::__private::Option::is_some(&m_controlData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "controlData",
                                ),
                            );
                        }
                        m_controlData = _serde::__private::Some(
                            match __A::next_value::<hkbHandIkControlData>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_handIndex => {
                        if _serde::__private::Option::is_some(&m_handIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "handIndex",
                                ),
                            );
                        }
                        m_handIndex = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enable => {
                        if _serde::__private::Option::is_some(&m_enable) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                            );
                        }
                        m_enable = _serde::__private::Some(
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
        let m_controlData = match m_controlData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("controlData"),
                );
            }
        };
        let m_handIndex = match m_handIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handIndex"),
                );
            }
        };
        let m_enable = match m_enable {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enable"),
                );
            }
        };
        _serde::__private::Ok(hkbHandIkControlsModifierHand {
            __ptr,
            m_controlData,
            m_handIndex,
            m_enable,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbHandIkControlsModifierHand {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["controlData", "handIndex", "enable"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbHandIkControlsModifierHand",
                FIELDS,
                __hkbHandIkControlsModifierHandVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbHandIkControlsModifierHand,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
