use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintChainInstance`
/// -         version: `0`
/// -       signature: `0x7a490753`
/// -          size:  72(x86)/136(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintChainInstance<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintInstance<'a>,
    /// # C++ Info
    /// -          name: `chainedEntities`(ctype: `hkArray<hkpEntity*>`)
    /// -        offset:  56(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_chainedEntities: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `action`(ctype: `struct hkpConstraintChainInstanceAction*`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_action: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpConstraintChainInstance<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintChainInstance"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7a490753)
        }
    }
    impl<'a> _serde::Serialize for hkpConstraintChainInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7a490753)));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintChainInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("owner", &self.parent.m_owner)?;
            serializer.serialize_field("data", &self.parent.m_data)?;
            serializer
                .serialize_field(
                    "constraintModifiers",
                    &self.parent.m_constraintModifiers,
                )?;
            serializer.serialize_field("entities", &self.parent.m_entities.as_slice())?;
            serializer.serialize_field("priority", &self.parent.m_priority)?;
            serializer.serialize_field("wantRuntime", &self.parent.m_wantRuntime)?;
            serializer
                .serialize_field(
                    "destructionRemapInfo",
                    &self.parent.m_destructionRemapInfo,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.skip_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.skip_field("internal", &self.parent.m_internal)?;
            serializer.skip_field("uid", &self.parent.m_uid)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("chainedEntities", &self.m_chainedEntities)?;
            serializer.serialize_field("action", &self.m_action)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer
                .serialize_array_field("chainedEntities", &self.m_chainedEntities)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_chainedEntities,
    m_action,
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
            "chainedEntities" => Ok(__Field::m_chainedEntities),
            "action" => Ok(__Field::m_action),
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
pub(super) struct __hkpConstraintChainInstanceVisitor<'de> {
    marker: core::marker::PhantomData<hkpConstraintChainInstance<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpConstraintChainInstanceVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpConstraintChainInstance<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpConstraintChainInstance<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkpConstraintChainInstanceVisitor<'de> {
    type Value = hkpConstraintChainInstance<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpConstraintChainInstance")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_chainedEntities: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_action: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_chainedEntities) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "chainedEntities",
                            ),
                        );
                    }
                    m_chainedEntities = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_action) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("action"),
                        );
                    }
                    m_action = _serde::__private::Some(
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
        let m_chainedEntities = match m_chainedEntities {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("chainedEntities"),
                );
            }
        };
        let m_action = match m_action {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("action"),
                );
            }
        };
        _serde::__private::Ok(hkpConstraintChainInstance {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_chainedEntities,
            m_action,
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
        let parent = __hkpConstraintInstanceVisitor::visit_as_parent(&mut __map)?;
        let mut m_chainedEntities: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_action: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_chainedEntities => {
                        if _serde::__private::Option::is_some(&m_chainedEntities) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "chainedEntities",
                                ),
                            );
                        }
                        m_chainedEntities = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_action => {
                        if _serde::__private::Option::is_some(&m_action) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("action"),
                            );
                        }
                        m_action = _serde::__private::Some(
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
        let m_chainedEntities = match m_chainedEntities {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("chainedEntities"),
                );
            }
        };
        let m_action = match m_action {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("action"),
                );
            }
        };
        _serde::__private::Ok(hkpConstraintChainInstance {
            __ptr,
            parent,
            m_chainedEntities,
            m_action,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConstraintChainInstance<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["chainedEntities", "action"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConstraintChainInstance",
                FIELDS,
                __hkpConstraintChainInstanceVisitor {
                    marker: _serde::__private::PhantomData::<hkpConstraintChainInstance>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
