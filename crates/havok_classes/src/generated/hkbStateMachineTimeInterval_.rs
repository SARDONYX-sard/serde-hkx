use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbStateMachineTimeInterval`
/// - version: `0`
/// - signature: `0x60a881e5`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbStateMachineTimeInterval {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `enterEventId`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_enterEventId: i32,
    /// # C++ Info
    /// - name: `exitEventId`(ctype: `hkInt32`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_exitEventId: i32,
    /// # C++ Info
    /// - name: `enterTime`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_enterTime: f32,
    /// # C++ Info
    /// - name: `exitTime`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_exitTime: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbStateMachineTimeInterval {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbStateMachineTimeInterval"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x60a881e5)
        }
    }
    impl _serde::Serialize for hkbStateMachineTimeInterval {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x60a881e5)));
            let mut serializer = __serializer
                .serialize_struct("hkbStateMachineTimeInterval", class_meta)?;
            serializer.serialize_field("enterEventId", &self.m_enterEventId)?;
            serializer.serialize_field("exitEventId", &self.m_exitEventId)?;
            serializer.serialize_field("enterTime", &self.m_enterTime)?;
            serializer.serialize_field("exitTime", &self.m_exitTime)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_enterEventId,
    m_exitEventId,
    m_enterTime,
    m_exitTime,
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
            "enterEventId" => Ok(__Field::m_enterEventId),
            "exitEventId" => Ok(__Field::m_exitEventId),
            "enterTime" => Ok(__Field::m_enterTime),
            "exitTime" => Ok(__Field::m_exitTime),
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
pub(super) struct __hkbStateMachineTimeIntervalVisitor<'de> {
    marker: core::marker::PhantomData<hkbStateMachineTimeInterval>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbStateMachineTimeIntervalVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbStateMachineTimeInterval, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbStateMachineTimeInterval>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbStateMachineTimeIntervalVisitor<'de> {
    type Value = hkbStateMachineTimeInterval;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbStateMachineTimeInterval",
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
        let mut m_enterEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_exitEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_enterTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_exitTime: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_enterEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enterEventId",
                            ),
                        );
                    }
                    m_enterEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_exitEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "exitEventId",
                            ),
                        );
                    }
                    m_exitEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_enterTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enterTime",
                            ),
                        );
                    }
                    m_enterTime = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_exitTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "exitTime",
                            ),
                        );
                    }
                    m_exitTime = _serde::__private::Some(
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
        let m_enterEventId = match m_enterEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enterEventId"),
                );
            }
        };
        let m_exitEventId = match m_exitEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("exitEventId"),
                );
            }
        };
        let m_enterTime = match m_enterTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enterTime"),
                );
            }
        };
        let m_exitTime = match m_exitTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("exitTime"),
                );
            }
        };
        _serde::__private::Ok(hkbStateMachineTimeInterval {
            __ptr,
            m_enterEventId,
            m_exitEventId,
            m_enterTime,
            m_exitTime,
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
        let mut m_enterEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_exitEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_enterTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_exitTime: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_enterEventId => {
                        if _serde::__private::Option::is_some(&m_enterEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enterEventId",
                                ),
                            );
                        }
                        m_enterEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_exitEventId => {
                        if _serde::__private::Option::is_some(&m_exitEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "exitEventId",
                                ),
                            );
                        }
                        m_exitEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enterTime => {
                        if _serde::__private::Option::is_some(&m_enterTime) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enterTime",
                                ),
                            );
                        }
                        m_enterTime = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_exitTime => {
                        if _serde::__private::Option::is_some(&m_exitTime) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "exitTime",
                                ),
                            );
                        }
                        m_exitTime = _serde::__private::Some(
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
        let m_enterEventId = match m_enterEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enterEventId"),
                );
            }
        };
        let m_exitEventId = match m_exitEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("exitEventId"),
                );
            }
        };
        let m_enterTime = match m_enterTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enterTime"),
                );
            }
        };
        let m_exitTime = match m_exitTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("exitTime"),
                );
            }
        };
        _serde::__private::Ok(hkbStateMachineTimeInterval {
            __ptr,
            m_enterEventId,
            m_exitEventId,
            m_enterTime,
            m_exitTime,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbStateMachineTimeInterval {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "enterEventId",
                "exitEventId",
                "enterTime",
                "exitTime",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbStateMachineTimeInterval",
                FIELDS,
                __hkbStateMachineTimeIntervalVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbStateMachineTimeInterval,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
