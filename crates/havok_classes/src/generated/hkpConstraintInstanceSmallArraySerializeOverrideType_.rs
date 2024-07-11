use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConstraintInstanceSmallArraySerializeOverrideType`
/// -         version: `1`
/// -       signature: `0xee3c2aec`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintInstanceSmallArraySerializeOverrideType {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `data`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_data: Pointer,
    /// # C++ Info
    /// -          name: `size`(ctype: `hkUint16`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_size: u16,
    /// # C++ Info
    /// -          name: `capacityAndFlags`(ctype: `hkUint16`)
    /// -        offset:   6(x86)/ 10(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_capacityAndFlags: u16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConstraintInstanceSmallArraySerializeOverrideType {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintInstanceSmallArraySerializeOverrideType"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xee3c2aec)
        }
    }
    impl _serde::Serialize for hkpConstraintInstanceSmallArraySerializeOverrideType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xee3c2aec)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpConstraintInstanceSmallArraySerializeOverrideType",
                    class_meta,
                )?;
            serializer.skip_field("data", &self.m_data)?;
            serializer.serialize_field("size", &self.m_size)?;
            serializer.serialize_field("capacityAndFlags", &self.m_capacityAndFlags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_data,
    m_size,
    m_capacityAndFlags,
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
            "size" => Ok(__Field::m_size),
            "capacityAndFlags" => Ok(__Field::m_capacityAndFlags),
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
pub(super) struct __hkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'de> {
    marker: core::marker::PhantomData<
        hkpConstraintInstanceSmallArraySerializeOverrideType,
    >,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpConstraintInstanceSmallArraySerializeOverrideType,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpConstraintInstanceSmallArraySerializeOverrideType,
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
for __hkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor<'de> {
    type Value = hkpConstraintInstanceSmallArraySerializeOverrideType;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpConstraintInstanceSmallArraySerializeOverrideType",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_data: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_size: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_capacityAndFlags: _serde::__private::Option<u16> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                        );
                    }
                    m_data = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_size) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("size"),
                        );
                    }
                    m_size = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_capacityAndFlags) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "capacityAndFlags",
                            ),
                        );
                    }
                    m_capacityAndFlags = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        let m_size = match m_size {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("size"),
                );
            }
        };
        let m_capacityAndFlags = match m_capacityAndFlags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("capacityAndFlags"),
                );
            }
        };
        _serde::__private::Ok(hkpConstraintInstanceSmallArraySerializeOverrideType {
            __ptr: __A::class_ptr(&mut __map),
            m_data,
            m_size,
            m_capacityAndFlags,
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
        let mut m_size: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_capacityAndFlags: _serde::__private::Option<u16> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_size => {
                        if _serde::__private::Option::is_some(&m_size) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("size"),
                            );
                        }
                        m_size = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_capacityAndFlags => {
                        if _serde::__private::Option::is_some(&m_capacityAndFlags) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "capacityAndFlags",
                                ),
                            );
                        }
                        m_capacityAndFlags = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
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
        let m_size = match m_size {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("size"),
                );
            }
        };
        let m_capacityAndFlags = match m_capacityAndFlags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("capacityAndFlags"),
                );
            }
        };
        _serde::__private::Ok(hkpConstraintInstanceSmallArraySerializeOverrideType {
            __ptr,
            m_size,
            m_capacityAndFlags,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkpConstraintInstanceSmallArraySerializeOverrideType {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["data", "size", "capacityAndFlags"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConstraintInstanceSmallArraySerializeOverrideType",
                FIELDS,
                __hkpConstraintInstanceSmallArraySerializeOverrideTypeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpConstraintInstanceSmallArraySerializeOverrideType,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
