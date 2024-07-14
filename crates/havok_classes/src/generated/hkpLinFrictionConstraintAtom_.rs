use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLinFrictionConstraintAtom`
/// -         version: `0`
/// -       signature: `0x3e94ef7c`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinFrictionConstraintAtom {
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
    /// -          name: `isEnabled`(ctype: `hkUint8`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isEnabled: u8,
    /// # C++ Info
    /// -          name: `frictionAxis`(ctype: `hkUint8`)
    /// -        offset:   3(x86)/  3(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_frictionAxis: u8,
    /// # C++ Info
    /// -          name: `maxFrictionForce`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxFrictionForce: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpLinFrictionConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpLinFrictionConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3e94ef7c)
        }
    }
    impl _serde::Serialize for hkpLinFrictionConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3e94ef7c)));
            let mut serializer = __serializer
                .serialize_struct("hkpLinFrictionConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.serialize_field("frictionAxis", &self.m_frictionAxis)?;
            serializer.serialize_field("maxFrictionForce", &self.m_maxFrictionForce)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_isEnabled,
    m_frictionAxis,
    m_maxFrictionForce,
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
            "isEnabled" => Ok(__Field::m_isEnabled),
            "frictionAxis" => Ok(__Field::m_frictionAxis),
            "maxFrictionForce" => Ok(__Field::m_maxFrictionForce),
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
pub(super) struct __hkpLinFrictionConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpLinFrictionConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpLinFrictionConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpLinFrictionConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpLinFrictionConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpLinFrictionConstraintAtomVisitor<'de> {
    type Value = hkpLinFrictionConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpLinFrictionConstraintAtom",
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
        let mut m_isEnabled: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_frictionAxis: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxFrictionForce: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_isEnabled) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isEnabled",
                            ),
                        );
                    }
                    m_isEnabled = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_frictionAxis) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "frictionAxis",
                            ),
                        );
                    }
                    m_frictionAxis = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_maxFrictionForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFrictionForce",
                            ),
                        );
                    }
                    m_maxFrictionForce = _serde::__private::Some(
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
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        let m_frictionAxis = match m_frictionAxis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("frictionAxis"),
                );
            }
        };
        let m_maxFrictionForce = match m_maxFrictionForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFrictionForce"),
                );
            }
        };
        _serde::__private::Ok(hkpLinFrictionConstraintAtom {
            __ptr,
            parent,
            m_isEnabled,
            m_frictionAxis,
            m_maxFrictionForce,
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
        let mut m_isEnabled: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_frictionAxis: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxFrictionForce: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_isEnabled => {
                        if _serde::__private::Option::is_some(&m_isEnabled) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isEnabled",
                                ),
                            );
                        }
                        m_isEnabled = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_frictionAxis => {
                        if _serde::__private::Option::is_some(&m_frictionAxis) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "frictionAxis",
                                ),
                            );
                        }
                        m_frictionAxis = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxFrictionForce => {
                        if _serde::__private::Option::is_some(&m_maxFrictionForce) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxFrictionForce",
                                ),
                            );
                        }
                        m_maxFrictionForce = _serde::__private::Some(
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
        let m_isEnabled = match m_isEnabled {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        let m_frictionAxis = match m_frictionAxis {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("frictionAxis"),
                );
            }
        };
        let m_maxFrictionForce = match m_maxFrictionForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFrictionForce"),
                );
            }
        };
        _serde::__private::Ok(hkpLinFrictionConstraintAtom {
            __ptr,
            parent,
            m_isEnabled,
            m_frictionAxis,
            m_maxFrictionForce,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpLinFrictionConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["isEnabled", "frictionAxis", "maxFrictionForce"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpLinFrictionConstraintAtom",
                FIELDS,
                __hkpLinFrictionConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpLinFrictionConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
