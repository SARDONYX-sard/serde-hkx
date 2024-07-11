use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEventsFromRangeModifierInternalState`
/// -         version: `0`
/// -       signature: `0xcc47b48d`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEventsFromRangeModifierInternalState {
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
    /// -          name: `wasActiveInPreviousFrame`(ctype: `hkArray<hkBool>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wasActiveInPreviousFrame: Vec<bool>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbEventsFromRangeModifierInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEventsFromRangeModifierInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcc47b48d)
        }
    }
    impl _serde::Serialize for hkbEventsFromRangeModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xcc47b48d)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbEventsFromRangeModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "wasActiveInPreviousFrame",
                    &self.m_wasActiveInPreviousFrame,
                )?;
            serializer
                .serialize_array_field(
                    "wasActiveInPreviousFrame",
                    &self.m_wasActiveInPreviousFrame,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_wasActiveInPreviousFrame,
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
            "wasActiveInPreviousFrame" => Ok(__Field::m_wasActiveInPreviousFrame),
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
pub(super) struct __hkbEventsFromRangeModifierInternalStateVisitor<'de> {
    marker: core::marker::PhantomData<hkbEventsFromRangeModifierInternalState>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbEventsFromRangeModifierInternalStateVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbEventsFromRangeModifierInternalState, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbEventsFromRangeModifierInternalState,
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
for __hkbEventsFromRangeModifierInternalStateVisitor<'de> {
    type Value = hkbEventsFromRangeModifierInternalState;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbEventsFromRangeModifierInternalState",
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
        let mut m_wasActiveInPreviousFrame: _serde::__private::Option<Vec<bool>> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_wasActiveInPreviousFrame) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wasActiveInPreviousFrame",
                            ),
                        );
                    }
                    m_wasActiveInPreviousFrame = _serde::__private::Some(
                        match __A::next_value::<Vec<bool>>(&mut __map) {
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
        let m_wasActiveInPreviousFrame = match m_wasActiveInPreviousFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wasActiveInPreviousFrame",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbEventsFromRangeModifierInternalState {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wasActiveInPreviousFrame,
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
        let mut m_wasActiveInPreviousFrame: _serde::__private::Option<Vec<bool>> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_wasActiveInPreviousFrame => {
                        if _serde::__private::Option::is_some(
                            &m_wasActiveInPreviousFrame,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wasActiveInPreviousFrame",
                                ),
                            );
                        }
                        m_wasActiveInPreviousFrame = _serde::__private::Some(
                            match __A::next_value::<Vec<bool>>(&mut __map) {
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
        let m_wasActiveInPreviousFrame = match m_wasActiveInPreviousFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wasActiveInPreviousFrame",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbEventsFromRangeModifierInternalState {
            __ptr,
            parent,
            m_wasActiveInPreviousFrame,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbEventsFromRangeModifierInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["wasActiveInPreviousFrame"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbEventsFromRangeModifierInternalState",
                FIELDS,
                __hkbEventsFromRangeModifierInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbEventsFromRangeModifierInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
