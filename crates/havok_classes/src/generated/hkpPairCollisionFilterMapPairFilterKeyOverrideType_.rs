use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPairCollisionFilterMapPairFilterKeyOverrideType`
/// -         version: `0`
/// -       signature: `0x36195969`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPairCollisionFilterMapPairFilterKeyOverrideType {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `elem`(ctype: `void*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_elem: Pointer,
    /// # C++ Info
    /// -          name: `numElems`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numElems: i32,
    /// # C++ Info
    /// -          name: `hashMod`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_hashMod: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPairCollisionFilterMapPairFilterKeyOverrideType {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPairCollisionFilterMapPairFilterKeyOverrideType"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x36195969)
        }
    }
    impl _serde::Serialize for hkpPairCollisionFilterMapPairFilterKeyOverrideType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x36195969)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpPairCollisionFilterMapPairFilterKeyOverrideType",
                    class_meta,
                )?;
            serializer.skip_field("elem", &self.m_elem)?;
            serializer.serialize_field("numElems", &self.m_numElems)?;
            serializer.serialize_field("hashMod", &self.m_hashMod)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_elem,
    m_numElems,
    m_hashMod,
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
            "numElems" => Ok(__Field::m_numElems),
            "hashMod" => Ok(__Field::m_hashMod),
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
pub(super) struct __hkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'de> {
    marker: core::marker::PhantomData<
        hkpPairCollisionFilterMapPairFilterKeyOverrideType,
    >,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpPairCollisionFilterMapPairFilterKeyOverrideType,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpPairCollisionFilterMapPairFilterKeyOverrideType,
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
for __hkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor<'de> {
    type Value = hkpPairCollisionFilterMapPairFilterKeyOverrideType;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPairCollisionFilterMapPairFilterKeyOverrideType",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_elem: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_numElems: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_hashMod: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_elem) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("elem"),
                        );
                    }
                    m_elem = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_numElems) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numElems",
                            ),
                        );
                    }
                    m_numElems = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_hashMod) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("hashMod"),
                        );
                    }
                    m_hashMod = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        let m_elem = match m_elem {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("elem"),
                );
            }
        };
        let m_numElems = match m_numElems {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numElems"),
                );
            }
        };
        let m_hashMod = match m_hashMod {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hashMod"),
                );
            }
        };
        _serde::__private::Ok(hkpPairCollisionFilterMapPairFilterKeyOverrideType {
            __ptr,
            m_elem,
            m_numElems,
            m_hashMod,
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
        let mut m_numElems: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_hashMod: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_numElems => {
                        if _serde::__private::Option::is_some(&m_numElems) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numElems",
                                ),
                            );
                        }
                        m_numElems = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_hashMod => {
                        if _serde::__private::Option::is_some(&m_hashMod) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hashMod",
                                ),
                            );
                        }
                        m_hashMod = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
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
        let m_numElems = match m_numElems {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numElems"),
                );
            }
        };
        let m_hashMod = match m_hashMod {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hashMod"),
                );
            }
        };
        _serde::__private::Ok(hkpPairCollisionFilterMapPairFilterKeyOverrideType {
            __ptr,
            m_numElems,
            m_hashMod,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkpPairCollisionFilterMapPairFilterKeyOverrideType {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["elem", "numElems", "hashMod"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPairCollisionFilterMapPairFilterKeyOverrideType",
                FIELDS,
                __hkpPairCollisionFilterMapPairFilterKeyOverrideTypeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPairCollisionFilterMapPairFilterKeyOverrideType,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
