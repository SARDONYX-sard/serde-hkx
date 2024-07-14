use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainMapperLinkInfo`
/// -         version: `0`
/// -       signature: `0xcf071a1b`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainMapperLinkInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `firstTargetIdx`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_firstTargetIdx: i32,
    /// # C++ Info
    /// -          name: `numTargets`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numTargets: i32,
    /// # C++ Info
    /// -          name: `limitConstraint`(ctype: `struct hkpConstraintInstance*`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_limitConstraint: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPoweredChainMapperLinkInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPoweredChainMapperLinkInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcf071a1b)
        }
    }
    impl _serde::Serialize for hkpPoweredChainMapperLinkInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xcf071a1b)));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainMapperLinkInfo", class_meta)?;
            serializer.serialize_field("firstTargetIdx", &self.m_firstTargetIdx)?;
            serializer.serialize_field("numTargets", &self.m_numTargets)?;
            serializer.serialize_field("limitConstraint", &self.m_limitConstraint)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_firstTargetIdx,
    m_numTargets,
    m_limitConstraint,
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
            "firstTargetIdx" => Ok(__Field::m_firstTargetIdx),
            "numTargets" => Ok(__Field::m_numTargets),
            "limitConstraint" => Ok(__Field::m_limitConstraint),
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
pub(super) struct __hkpPoweredChainMapperLinkInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpPoweredChainMapperLinkInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPoweredChainMapperLinkInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPoweredChainMapperLinkInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPoweredChainMapperLinkInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPoweredChainMapperLinkInfoVisitor<'de> {
    type Value = hkpPoweredChainMapperLinkInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPoweredChainMapperLinkInfo",
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
        let mut m_firstTargetIdx: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numTargets: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_limitConstraint: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_firstTargetIdx) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "firstTargetIdx",
                            ),
                        );
                    }
                    m_firstTargetIdx = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_numTargets) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numTargets",
                            ),
                        );
                    }
                    m_numTargets = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_limitConstraint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitConstraint",
                            ),
                        );
                    }
                    m_limitConstraint = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
        let m_firstTargetIdx = match m_firstTargetIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("firstTargetIdx"),
                );
            }
        };
        let m_numTargets = match m_numTargets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numTargets"),
                );
            }
        };
        let m_limitConstraint = match m_limitConstraint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitConstraint"),
                );
            }
        };
        _serde::__private::Ok(hkpPoweredChainMapperLinkInfo {
            __ptr,
            m_firstTargetIdx,
            m_numTargets,
            m_limitConstraint,
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
        let mut m_firstTargetIdx: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numTargets: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_limitConstraint: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_firstTargetIdx => {
                        if _serde::__private::Option::is_some(&m_firstTargetIdx) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "firstTargetIdx",
                                ),
                            );
                        }
                        m_firstTargetIdx = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numTargets => {
                        if _serde::__private::Option::is_some(&m_numTargets) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numTargets",
                                ),
                            );
                        }
                        m_numTargets = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_limitConstraint => {
                        if _serde::__private::Option::is_some(&m_limitConstraint) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "limitConstraint",
                                ),
                            );
                        }
                        m_limitConstraint = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
        let m_firstTargetIdx = match m_firstTargetIdx {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("firstTargetIdx"),
                );
            }
        };
        let m_numTargets = match m_numTargets {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numTargets"),
                );
            }
        };
        let m_limitConstraint = match m_limitConstraint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("limitConstraint"),
                );
            }
        };
        _serde::__private::Ok(hkpPoweredChainMapperLinkInfo {
            __ptr,
            m_firstTargetIdx,
            m_numTargets,
            m_limitConstraint,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPoweredChainMapperLinkInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["firstTargetIdx", "numTargets", "limitConstraint"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPoweredChainMapperLinkInfo",
                FIELDS,
                __hkpPoweredChainMapperLinkInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPoweredChainMapperLinkInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
