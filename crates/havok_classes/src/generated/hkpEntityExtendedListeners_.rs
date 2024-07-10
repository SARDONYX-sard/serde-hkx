use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpEntityExtendedListeners`
/// -         version: `0`
/// -       signature: `0xf557023c`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntityExtendedListeners {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `activationListeners`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_activationListeners: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `entityListeners`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_entityListeners: hkpEntitySmallArraySerializeOverrideType,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpEntityExtendedListeners {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpEntityExtendedListeners"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf557023c)
        }
    }
    impl _serde::Serialize for hkpEntityExtendedListeners {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf557023c)));
            let mut serializer = __serializer
                .serialize_struct("hkpEntityExtendedListeners", class_meta)?;
            serializer.skip_field("activationListeners", &self.m_activationListeners)?;
            serializer.skip_field("entityListeners", &self.m_entityListeners)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_activationListeners,
    m_entityListeners,
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
pub(super) struct __hkpEntityExtendedListenersVisitor<'de> {
    marker: core::marker::PhantomData<hkpEntityExtendedListeners>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpEntityExtendedListenersVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpEntityExtendedListeners, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpEntityExtendedListeners>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpEntityExtendedListenersVisitor<'de> {
    type Value = hkpEntityExtendedListeners;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpEntityExtendedListeners")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_activationListeners: _serde::__private::Option<
            hkpEntitySmallArraySerializeOverrideType,
        > = _serde::__private::None;
        let mut m_entityListeners: _serde::__private::Option<
            hkpEntitySmallArraySerializeOverrideType,
        > = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_activationListeners) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "activationListeners",
                            ),
                        );
                    }
                    m_activationListeners = _serde::__private::Some(
                        match __A::next_value::<
                            hkpEntitySmallArraySerializeOverrideType,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_entityListeners) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "entityListeners",
                            ),
                        );
                    }
                    m_entityListeners = _serde::__private::Some(
                        match __A::next_value::<
                            hkpEntitySmallArraySerializeOverrideType,
                        >(&mut __map) {
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
        let m_activationListeners = match m_activationListeners {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "activationListeners",
                    ),
                );
            }
        };
        let m_entityListeners = match m_entityListeners {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("entityListeners"),
                );
            }
        };
        _serde::__private::Ok(hkpEntityExtendedListeners {
            __ptr: __A::class_ptr(&mut __map),
            m_activationListeners,
            m_entityListeners,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                _ => {}
            }
        }
        _serde::__private::Ok(hkpEntityExtendedListeners {
            __ptr: __A::class_ptr(&mut __map),
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpEntityExtendedListeners {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["activationListeners", "entityListeners"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpEntityExtendedListeners",
                FIELDS,
                __hkpEntityExtendedListenersVisitor {
                    marker: _serde::__private::PhantomData::<hkpEntityExtendedListeners>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
