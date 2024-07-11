use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEventDrivenModifier`
/// -         version: `0`
/// -       signature: `0x7ed3f44e`
/// -          size:  60(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEventDrivenModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifierWrapper<'a>,
    /// # C++ Info
    /// -          name: `activateEventId`(ctype: `hkInt32`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_activateEventId: i32,
    /// # C++ Info
    /// -          name: `deactivateEventId`(ctype: `hkInt32`)
    /// -        offset:  52(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_deactivateEventId: i32,
    /// # C++ Info
    /// -          name: `activeByDefault`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_activeByDefault: bool,
    /// # C++ Info
    /// -          name: `isActive`(ctype: `hkBool`)
    /// -        offset:  57(x86)/ 97(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isActive: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbEventDrivenModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEventDrivenModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7ed3f44e)
        }
    }
    impl<'a> _serde::Serialize for hkbEventDrivenModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7ed3f44e)));
            let mut serializer = __serializer
                .serialize_struct("hkbEventDrivenModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer
                .skip_field("padNode", &self.parent.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.parent.m_enable)?;
            serializer
                .skip_field(
                    "padModifier",
                    &self.parent.parent.m_padModifier.as_slice(),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("modifier", &self.parent.m_modifier)?;
            serializer.serialize_field("activateEventId", &self.m_activateEventId)?;
            serializer.serialize_field("deactivateEventId", &self.m_deactivateEventId)?;
            serializer.serialize_field("activeByDefault", &self.m_activeByDefault)?;
            serializer.skip_field("isActive", &self.m_isActive)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_activateEventId,
    m_deactivateEventId,
    m_activeByDefault,
    m_isActive,
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
            "activateEventId" => Ok(__Field::m_activateEventId),
            "deactivateEventId" => Ok(__Field::m_deactivateEventId),
            "activeByDefault" => Ok(__Field::m_activeByDefault),
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
pub(super) struct __hkbEventDrivenModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbEventDrivenModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbEventDrivenModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbEventDrivenModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbEventDrivenModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbEventDrivenModifierVisitor<'de> {
    type Value = hkbEventDrivenModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbEventDrivenModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_activateEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_deactivateEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_activeByDefault: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isActive: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_activateEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "activateEventId",
                            ),
                        );
                    }
                    m_activateEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_deactivateEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "deactivateEventId",
                            ),
                        );
                    }
                    m_deactivateEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_activeByDefault) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "activeByDefault",
                            ),
                        );
                    }
                    m_activeByDefault = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_isActive) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isActive",
                            ),
                        );
                    }
                    m_isActive = _serde::__private::Some(
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
        __A::pad(&mut __map, 2usize, 6usize)?;
        let m_activateEventId = match m_activateEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activateEventId"),
                );
            }
        };
        let m_deactivateEventId = match m_deactivateEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("deactivateEventId"),
                );
            }
        };
        let m_activeByDefault = match m_activeByDefault {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activeByDefault"),
                );
            }
        };
        let m_isActive = match m_isActive {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isActive"),
                );
            }
        };
        _serde::__private::Ok(hkbEventDrivenModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_activateEventId,
            m_deactivateEventId,
            m_activeByDefault,
            m_isActive,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkbModifierWrapperVisitor::visit_as_parent(&mut __map)?;
        let mut m_activateEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_deactivateEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_activeByDefault: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_activateEventId => {
                        if _serde::__private::Option::is_some(&m_activateEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "activateEventId",
                                ),
                            );
                        }
                        m_activateEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_deactivateEventId => {
                        if _serde::__private::Option::is_some(&m_deactivateEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "deactivateEventId",
                                ),
                            );
                        }
                        m_deactivateEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_activeByDefault => {
                        if _serde::__private::Option::is_some(&m_activeByDefault) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "activeByDefault",
                                ),
                            );
                        }
                        m_activeByDefault = _serde::__private::Some(
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
        let m_activateEventId = match m_activateEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activateEventId"),
                );
            }
        };
        let m_deactivateEventId = match m_deactivateEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("deactivateEventId"),
                );
            }
        };
        let m_activeByDefault = match m_activeByDefault {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("activeByDefault"),
                );
            }
        };
        _serde::__private::Ok(hkbEventDrivenModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_activateEventId,
            m_deactivateEventId,
            m_activeByDefault,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbEventDrivenModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "activateEventId",
                "deactivateEventId",
                "activeByDefault",
                "isActive",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbEventDrivenModifier",
                FIELDS,
                __hkbEventDrivenModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbEventDrivenModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
