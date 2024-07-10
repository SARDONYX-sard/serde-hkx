use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAttributeModifierAssignment`
/// -         version: `0`
/// -       signature: `0x48b8ad52`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAttributeModifierAssignment {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `attributeIndex`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_attributeIndex: i32,
    /// # C++ Info
    /// -          name: `attributeValue`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_attributeValue: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbAttributeModifierAssignment {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbAttributeModifierAssignment"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x48b8ad52)
        }
    }
    impl _serde::Serialize for hkbAttributeModifierAssignment {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x48b8ad52)));
            let mut serializer = __serializer
                .serialize_struct("hkbAttributeModifierAssignment", class_meta)?;
            serializer.serialize_field("attributeIndex", &self.m_attributeIndex)?;
            serializer.serialize_field("attributeValue", &self.m_attributeValue)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_attributeIndex,
    m_attributeValue,
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
            "attributeIndex" => Ok(__Field::m_attributeIndex),
            "attributeValue" => Ok(__Field::m_attributeValue),
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
pub(super) struct __hkbAttributeModifierAssignmentVisitor<'de> {
    marker: core::marker::PhantomData<hkbAttributeModifierAssignment>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbAttributeModifierAssignmentVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbAttributeModifierAssignment, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbAttributeModifierAssignment>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbAttributeModifierAssignmentVisitor<'de> {
    type Value = hkbAttributeModifierAssignment;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbAttributeModifierAssignment",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_attributeIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_attributeValue: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_attributeIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attributeIndex",
                            ),
                        );
                    }
                    m_attributeIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_attributeValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attributeValue",
                            ),
                        );
                    }
                    m_attributeValue = _serde::__private::Some(
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
        let m_attributeIndex = match m_attributeIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attributeIndex"),
                );
            }
        };
        let m_attributeValue = match m_attributeValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attributeValue"),
                );
            }
        };
        _serde::__private::Ok(hkbAttributeModifierAssignment {
            __ptr: __A::class_ptr(&mut __map),
            m_attributeIndex,
            m_attributeValue,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_attributeIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_attributeValue: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_attributeIndex => {
                    if _serde::__private::Option::is_some(&m_attributeIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attributeIndex",
                            ),
                        );
                    }
                    m_attributeIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_attributeValue => {
                    if _serde::__private::Option::is_some(&m_attributeValue) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attributeValue",
                            ),
                        );
                    }
                    m_attributeValue = _serde::__private::Some(
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
        let m_attributeIndex = match m_attributeIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attributeIndex"),
                );
            }
        };
        let m_attributeValue = match m_attributeValue {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attributeValue"),
                );
            }
        };
        _serde::__private::Ok(hkbAttributeModifierAssignment {
            __ptr: __A::class_ptr(&mut __map),
            m_attributeIndex,
            m_attributeValue,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbAttributeModifierAssignment {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["attributeIndex", "attributeValue"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbAttributeModifierAssignment",
                FIELDS,
                __hkbAttributeModifierAssignmentVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbAttributeModifierAssignment,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
