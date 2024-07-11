use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGetWorldFromModelModifierInternalState`
/// -         version: `0`
/// -       signature: `0xa92ed39f`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGetWorldFromModelModifierInternalState {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `translationOut`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translationOut: Vector4,
    /// # C++ Info
    /// -          name: `rotationOut`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotationOut: Quaternion,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbGetWorldFromModelModifierInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbGetWorldFromModelModifierInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa92ed39f)
        }
    }
    impl _serde::Serialize for hkbGetWorldFromModelModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa92ed39f)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbGetWorldFromModelModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("translationOut", &self.m_translationOut)?;
            serializer.serialize_field("rotationOut", &self.m_rotationOut)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_translationOut,
    m_rotationOut,
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
            "translationOut" => Ok(__Field::m_translationOut),
            "rotationOut" => Ok(__Field::m_rotationOut),
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
pub(super) struct __hkbGetWorldFromModelModifierInternalStateVisitor<'de> {
    marker: core::marker::PhantomData<hkbGetWorldFromModelModifierInternalState>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbGetWorldFromModelModifierInternalStateVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbGetWorldFromModelModifierInternalState, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbGetWorldFromModelModifierInternalState,
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
for __hkbGetWorldFromModelModifierInternalStateVisitor<'de> {
    type Value = hkbGetWorldFromModelModifierInternalState;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbGetWorldFromModelModifierInternalState",
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
        let mut m_translationOut: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_translationOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "translationOut",
                            ),
                        );
                    }
                    m_translationOut = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_rotationOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotationOut",
                            ),
                        );
                    }
                    m_rotationOut = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
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
        let m_translationOut = match m_translationOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translationOut"),
                );
            }
        };
        let m_rotationOut = match m_rotationOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationOut"),
                );
            }
        };
        _serde::__private::Ok(hkbGetWorldFromModelModifierInternalState {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_translationOut,
            m_rotationOut,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_translationOut: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_translationOut => {
                        if _serde::__private::Option::is_some(&m_translationOut) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "translationOut",
                                ),
                            );
                        }
                        m_translationOut = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rotationOut => {
                        if _serde::__private::Option::is_some(&m_rotationOut) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotationOut",
                                ),
                            );
                        }
                        m_rotationOut = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
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
        let m_translationOut = match m_translationOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translationOut"),
                );
            }
        };
        let m_rotationOut = match m_rotationOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationOut"),
                );
            }
        };
        _serde::__private::Ok(hkbGetWorldFromModelModifierInternalState {
            __ptr,
            parent,
            m_translationOut,
            m_rotationOut,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbGetWorldFromModelModifierInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["translationOut", "rotationOut"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbGetWorldFromModelModifierInternalState",
                FIELDS,
                __hkbGetWorldFromModelModifierInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbGetWorldFromModelModifierInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
