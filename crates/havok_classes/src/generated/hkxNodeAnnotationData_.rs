use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxNodeAnnotationData`
/// -         version: `0`
/// -       signature: `0x433dee92`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxNodeAnnotationData<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `description`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_description: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxNodeAnnotationData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxNodeAnnotationData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x433dee92)
        }
    }
    impl<'a> _serde::Serialize for hkxNodeAnnotationData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x433dee92)));
            let mut serializer = __serializer
                .serialize_struct("hkxNodeAnnotationData", class_meta)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("description", &self.m_description)?;
            serializer.serialize_stringptr_field("description", &self.m_description)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_time,
    m_description,
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
            "time" => Ok(__Field::m_time),
            "description" => Ok(__Field::m_description),
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
pub(super) struct __hkxNodeAnnotationDataVisitor<'de> {
    marker: core::marker::PhantomData<hkxNodeAnnotationData<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxNodeAnnotationDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxNodeAnnotationData<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxNodeAnnotationData<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxNodeAnnotationDataVisitor<'de> {
    type Value = hkxNodeAnnotationData<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxNodeAnnotationData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_description: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_time) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("time"),
                        );
                    }
                    m_time = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_description) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "description",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_description = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_time = match m_time {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("time"),
                );
            }
        };
        let m_description = match m_description {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("description"),
                );
            }
        };
        _serde::__private::Ok(hkxNodeAnnotationData {
            __ptr,
            m_time,
            m_description,
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
        let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_description: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_time => {
                        if _serde::__private::Option::is_some(&m_time) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("time"),
                            );
                        }
                        m_time = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_description => {
                        if _serde::__private::Option::is_some(&m_description) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "description",
                                ),
                            );
                        }
                        m_description = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_time = match m_time {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("time"),
                );
            }
        };
        let m_description = match m_description {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("description"),
                );
            }
        };
        _serde::__private::Ok(hkxNodeAnnotationData {
            __ptr,
            m_time,
            m_description,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxNodeAnnotationData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["time", "description"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxNodeAnnotationData",
                FIELDS,
                __hkxNodeAnnotationDataVisitor {
                    marker: _serde::__private::PhantomData::<hkxNodeAnnotationData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
