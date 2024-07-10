use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSIStateManagerModifierBSiStateData`
/// -         version: `0`
/// -       signature: `0x6b8a15fc`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSIStateManagerModifierBSiStateData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pStateMachine`(ctype: `struct hkbGenerator*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_pStateMachine: Pointer,
    /// # C++ Info
    /// -          name: `StateID`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_StateID: i32,
    /// # C++ Info
    /// -          name: `iStateToSetAs`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_iStateToSetAs: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for BSIStateManagerModifierBSiStateData {
        #[inline]
        fn name(&self) -> &'static str {
            "BSIStateManagerModifierBSiStateData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6b8a15fc)
        }
    }
    impl _serde::Serialize for BSIStateManagerModifierBSiStateData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6b8a15fc)));
            let mut serializer = __serializer
                .serialize_struct("BSIStateManagerModifierBSiStateData", class_meta)?;
            serializer.serialize_field("pStateMachine", &self.m_pStateMachine)?;
            serializer.serialize_field("StateID", &self.m_StateID)?;
            serializer.serialize_field("iStateToSetAs", &self.m_iStateToSetAs)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pStateMachine,
    m_StateID,
    m_iStateToSetAs,
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
            "pStateMachine" => Ok(__Field::m_pStateMachine),
            "StateID" => Ok(__Field::m_StateID),
            "iStateToSetAs" => Ok(__Field::m_iStateToSetAs),
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
pub(super) struct __BSIStateManagerModifierBSiStateDataVisitor<'de> {
    marker: core::marker::PhantomData<BSIStateManagerModifierBSiStateData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSIStateManagerModifierBSiStateDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSIStateManagerModifierBSiStateData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    BSIStateManagerModifierBSiStateData,
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
impl<'de> _serde::de::Visitor<'de>
for __BSIStateManagerModifierBSiStateDataVisitor<'de> {
    type Value = BSIStateManagerModifierBSiStateData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct BSIStateManagerModifierBSiStateData",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_pStateMachine: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_StateID: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_iStateToSetAs: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pStateMachine) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pStateMachine",
                            ),
                        );
                    }
                    m_pStateMachine = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_StateID) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("StateID"),
                        );
                    }
                    m_StateID = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_iStateToSetAs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "iStateToSetAs",
                            ),
                        );
                    }
                    m_iStateToSetAs = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        let m_pStateMachine = match m_pStateMachine {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pStateMachine"),
                );
            }
        };
        let m_StateID = match m_StateID {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("StateID"),
                );
            }
        };
        let m_iStateToSetAs = match m_iStateToSetAs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("iStateToSetAs"),
                );
            }
        };
        _serde::__private::Ok(BSIStateManagerModifierBSiStateData {
            __ptr: __A::class_ptr(&mut __map),
            m_pStateMachine,
            m_StateID,
            m_iStateToSetAs,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_pStateMachine: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_StateID: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_iStateToSetAs: _serde::__private::Option<i32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_pStateMachine => {
                    if _serde::__private::Option::is_some(&m_pStateMachine) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pStateMachine",
                            ),
                        );
                    }
                    m_pStateMachine = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_StateID => {
                    if _serde::__private::Option::is_some(&m_StateID) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("StateID"),
                        );
                    }
                    m_StateID = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_iStateToSetAs => {
                    if _serde::__private::Option::is_some(&m_iStateToSetAs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "iStateToSetAs",
                            ),
                        );
                    }
                    m_iStateToSetAs = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        let m_pStateMachine = match m_pStateMachine {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pStateMachine"),
                );
            }
        };
        let m_StateID = match m_StateID {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("StateID"),
                );
            }
        };
        let m_iStateToSetAs = match m_iStateToSetAs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("iStateToSetAs"),
                );
            }
        };
        _serde::__private::Ok(BSIStateManagerModifierBSiStateData {
            __ptr: __A::class_ptr(&mut __map),
            m_pStateMachine,
            m_StateID,
            m_iStateToSetAs,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSIStateManagerModifierBSiStateData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["pStateMachine", "StateID", "iStateToSetAs"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSIStateManagerModifierBSiStateData",
                FIELDS,
                __BSIStateManagerModifierBSiStateDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSIStateManagerModifierBSiStateData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
