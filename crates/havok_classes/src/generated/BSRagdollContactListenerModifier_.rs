use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSRagdollContactListenerModifier`
/// -         version: `0`
/// -       signature: `0x8003d8ce`
/// -          size:  76(x86)/136(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSRagdollContactListenerModifier<'a> {
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
    /// -          name: `contactEvent`(ctype: `struct hkbEventProperty`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_contactEvent: hkbEventProperty,
    /// # C++ Info
    /// -          name: `bones`(ctype: `struct hkbBoneIndexArray*`)
    /// -        offset:  56(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bones: Pointer,
    /// # C++ Info
    /// -          name: `throwEvent`(ctype: `hkBool`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_throwEvent: bool,
    /// # C++ Info
    /// -          name: `ragdollRigidBodies`(ctype: `hkArray<void*>`)
    /// -        offset:  64(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ragdollRigidBodies: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSRagdollContactListenerModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSRagdollContactListenerModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x8003d8ce)
        }
    }
    impl<'a> _serde::Serialize for BSRagdollContactListenerModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8003d8ce)));
            let mut serializer = __serializer
                .serialize_struct("BSRagdollContactListenerModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("contactEvent", &self.m_contactEvent)?;
            serializer.serialize_field("bones", &self.m_bones)?;
            serializer.skip_field("throwEvent", &self.m_throwEvent)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .skip_array_meta_field(
                    "ragdollRigidBodies",
                    &self.m_ragdollRigidBodies,
                )?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field(
                    "ragdollRigidBodies",
                    &self.m_ragdollRigidBodies,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_contactEvent,
    m_bones,
    m_throwEvent,
    m_ragdollRigidBodies,
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
            "contactEvent" => Ok(__Field::m_contactEvent),
            "bones" => Ok(__Field::m_bones),
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
pub(super) struct __BSRagdollContactListenerModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSRagdollContactListenerModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSRagdollContactListenerModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSRagdollContactListenerModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    BSRagdollContactListenerModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __BSRagdollContactListenerModifierVisitor<'de> {
    type Value = BSRagdollContactListenerModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct BSRagdollContactListenerModifier",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_contactEvent: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bones: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_throwEvent: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ragdollRigidBodies: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_contactEvent) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactEvent",
                            ),
                        );
                    }
                    m_contactEvent = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_bones) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                        );
                    }
                    m_bones = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_throwEvent) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "throwEvent",
                            ),
                        );
                    }
                    m_throwEvent = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_ragdollRigidBodies) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ragdollRigidBodies",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 7usize)?;
                    m_ragdollRigidBodies = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_contactEvent = match m_contactEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactEvent"),
                );
            }
        };
        let m_bones = match m_bones {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bones"),
                );
            }
        };
        let m_throwEvent = match m_throwEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("throwEvent"),
                );
            }
        };
        let m_ragdollRigidBodies = match m_ragdollRigidBodies {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ragdollRigidBodies",
                    ),
                );
            }
        };
        _serde::__private::Ok(BSRagdollContactListenerModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_contactEvent,
            m_bones,
            m_throwEvent,
            m_ragdollRigidBodies,
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
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_contactEvent: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_bones: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_contactEvent => {
                        if _serde::__private::Option::is_some(&m_contactEvent) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contactEvent",
                                ),
                            );
                        }
                        m_contactEvent = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bones => {
                        if _serde::__private::Option::is_some(&m_bones) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                            );
                        }
                        m_bones = _serde::__private::Some(
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
        let m_contactEvent = match m_contactEvent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactEvent"),
                );
            }
        };
        let m_bones = match m_bones {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bones"),
                );
            }
        };
        _serde::__private::Ok(BSRagdollContactListenerModifier {
            __ptr,
            parent,
            m_contactEvent,
            m_bones,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSRagdollContactListenerModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "contactEvent",
                "bones",
                "throwEvent",
                "ragdollRigidBodies",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSRagdollContactListenerModifier",
                FIELDS,
                __BSRagdollContactListenerModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSRagdollContactListenerModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
