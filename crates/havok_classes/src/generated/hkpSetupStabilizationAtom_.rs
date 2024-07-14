use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSetupStabilizationAtom`
/// -         version: `1`
/// -       signature: `0xf05d137e`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetupStabilizationAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `enabled`(ctype: `hkBool`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enabled: bool,
    /// # C++ Info
    /// -          name: `maxAngle`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAngle: f32,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkUint8[8]`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_padding: [u8; 8usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSetupStabilizationAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSetupStabilizationAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf05d137e)
        }
    }
    impl _serde::Serialize for hkpSetupStabilizationAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf05d137e)));
            let mut serializer = __serializer
                .serialize_struct("hkpSetupStabilizationAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("enabled", &self.m_enabled)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("maxAngle", &self.m_maxAngle)?;
            serializer.serialize_field("padding", &self.m_padding.as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_enabled,
    m_maxAngle,
    m_padding,
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
            "enabled" => Ok(__Field::m_enabled),
            "maxAngle" => Ok(__Field::m_maxAngle),
            "padding" => Ok(__Field::m_padding),
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
pub(super) struct __hkpSetupStabilizationAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpSetupStabilizationAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSetupStabilizationAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSetupStabilizationAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSetupStabilizationAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSetupStabilizationAtomVisitor<'de> {
    type Value = hkpSetupStabilizationAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpSetupStabilizationAtom")
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
        let mut m_enabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_maxAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<[u8; 8usize]> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_enabled) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("enabled"),
                        );
                    }
                    m_enabled = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_maxAngle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxAngle",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_maxAngle = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_padding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("padding"),
                        );
                    }
                    m_padding = _serde::__private::Some(
                        match __A::next_value::<[u8; 8usize]>(&mut __map) {
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
        let m_enabled = match m_enabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enabled"),
                );
            }
        };
        let m_maxAngle = match m_maxAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxAngle"),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkpSetupStabilizationAtom {
            __ptr,
            parent,
            m_enabled,
            m_maxAngle,
            m_padding,
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
        let parent = __hkpConstraintAtomVisitor::visit_as_parent(&mut __map)?;
        let mut m_enabled: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_maxAngle: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<[u8; 8usize]> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_enabled => {
                        if _serde::__private::Option::is_some(&m_enabled) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enabled",
                                ),
                            );
                        }
                        m_enabled = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxAngle => {
                        if _serde::__private::Option::is_some(&m_maxAngle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxAngle",
                                ),
                            );
                        }
                        m_maxAngle = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_padding => {
                        if _serde::__private::Option::is_some(&m_padding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "padding",
                                ),
                            );
                        }
                        m_padding = _serde::__private::Some(
                            match __A::next_value::<[u8; 8usize]>(&mut __map) {
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
        let m_enabled = match m_enabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enabled"),
                );
            }
        };
        let m_maxAngle = match m_maxAngle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxAngle"),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkpSetupStabilizationAtom {
            __ptr,
            parent,
            m_enabled,
            m_maxAngle,
            m_padding,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSetupStabilizationAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["enabled", "maxAngle", "padding"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSetupStabilizationAtom",
                FIELDS,
                __hkpSetupStabilizationAtomVisitor {
                    marker: _serde::__private::PhantomData::<hkpSetupStabilizationAtom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
