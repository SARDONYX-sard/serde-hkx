use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSequenceStringData`
/// -         version: `0`
/// -       signature: `0x6a5094e3`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSequenceStringData<'a> {
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
    /// -          name: `eventNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `variableNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_variableNames: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbSequenceStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSequenceStringData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6a5094e3)
        }
    }
    impl<'a> _serde::Serialize for hkbSequenceStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6a5094e3)));
            let mut serializer = __serializer
                .serialize_struct("hkbSequenceStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("eventNames", &self.m_eventNames)?;
            serializer
                .serialize_array_meta_field("variableNames", &self.m_variableNames)?;
            serializer.serialize_array_field("eventNames", &self.m_eventNames)?;
            serializer.serialize_array_field("variableNames", &self.m_variableNames)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_eventNames,
    m_variableNames,
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
            "eventNames" => Ok(__Field::m_eventNames),
            "variableNames" => Ok(__Field::m_variableNames),
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
pub(super) struct __hkbSequenceStringDataVisitor<'de> {
    marker: core::marker::PhantomData<hkbSequenceStringData<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbSequenceStringDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbSequenceStringData<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbSequenceStringData<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbSequenceStringDataVisitor<'de> {
    type Value = hkbSequenceStringData<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbSequenceStringData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_eventNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_variableNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_eventNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eventNames",
                            ),
                        );
                    }
                    m_eventNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_variableNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "variableNames",
                            ),
                        );
                    }
                    m_variableNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
        let m_eventNames = match m_eventNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventNames"),
                );
            }
        };
        let m_variableNames = match m_variableNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableNames"),
                );
            }
        };
        _serde::__private::Ok(hkbSequenceStringData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_eventNames,
            m_variableNames,
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
        let mut m_eventNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        let mut m_variableNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_eventNames => {
                        if _serde::__private::Option::is_some(&m_eventNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "eventNames",
                                ),
                            );
                        }
                        m_eventNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_variableNames => {
                        if _serde::__private::Option::is_some(&m_variableNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "variableNames",
                                ),
                            );
                        }
                        m_variableNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
        let m_eventNames = match m_eventNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventNames"),
                );
            }
        };
        let m_variableNames = match m_variableNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableNames"),
                );
            }
        };
        _serde::__private::Ok(hkbSequenceStringData {
            __ptr,
            parent,
            m_eventNames,
            m_variableNames,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbSequenceStringData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["eventNames", "variableNames"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbSequenceStringData",
                FIELDS,
                __hkbSequenceStringDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbSequenceStringData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
