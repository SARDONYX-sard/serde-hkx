use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterSkinInfo`
/// -         version: `2`
/// -       signature: `0x180d900d`
/// -          size:  40(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterSkinInfo {
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
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `deformableSkins`(ctype: `hkArray<hkUint64>`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_deformableSkins: Vec<u64>,
    /// # C++ Info
    /// -          name: `rigidSkins`(ctype: `hkArray<hkUint64>`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidSkins: Vec<u64>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterSkinInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterSkinInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x180d900d)
        }
    }
    impl _serde::Serialize for hkbCharacterSkinInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x180d900d)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterSkinInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer
                .serialize_array_meta_field("deformableSkins", &self.m_deformableSkins)?;
            serializer.serialize_array_meta_field("rigidSkins", &self.m_rigidSkins)?;
            serializer
                .serialize_array_field("deformableSkins", &self.m_deformableSkins)?;
            serializer.serialize_array_field("rigidSkins", &self.m_rigidSkins)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_characterId,
    m_deformableSkins,
    m_rigidSkins,
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
            "characterId" => Ok(__Field::m_characterId),
            "deformableSkins" => Ok(__Field::m_deformableSkins),
            "rigidSkins" => Ok(__Field::m_rigidSkins),
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
pub(super) struct __hkbCharacterSkinInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbCharacterSkinInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbCharacterSkinInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbCharacterSkinInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbCharacterSkinInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbCharacterSkinInfoVisitor<'de> {
    type Value = hkbCharacterSkinInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbCharacterSkinInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_deformableSkins: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
        let mut m_rigidSkins: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_characterId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterId",
                            ),
                        );
                    }
                    m_characterId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_deformableSkins) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "deformableSkins",
                            ),
                        );
                    }
                    m_deformableSkins = _serde::__private::Some(
                        match __A::next_value::<Vec<u64>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_rigidSkins) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rigidSkins",
                            ),
                        );
                    }
                    m_rigidSkins = _serde::__private::Some(
                        match __A::next_value::<Vec<u64>>(&mut __map) {
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
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_deformableSkins = match m_deformableSkins {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("deformableSkins"),
                );
            }
        };
        let m_rigidSkins = match m_rigidSkins {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigidSkins"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterSkinInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_characterId,
            m_deformableSkins,
            m_rigidSkins,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_deformableSkins: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
        let mut m_rigidSkins: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_characterId => {
                        if _serde::__private::Option::is_some(&m_characterId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterId",
                                ),
                            );
                        }
                        m_characterId = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_deformableSkins => {
                        if _serde::__private::Option::is_some(&m_deformableSkins) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "deformableSkins",
                                ),
                            );
                        }
                        m_deformableSkins = _serde::__private::Some(
                            match __A::next_value::<Vec<u64>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rigidSkins => {
                        if _serde::__private::Option::is_some(&m_rigidSkins) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rigidSkins",
                                ),
                            );
                        }
                        m_rigidSkins = _serde::__private::Some(
                            match __A::next_value::<Vec<u64>>(&mut __map) {
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
        let m_characterId = match m_characterId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterId"),
                );
            }
        };
        let m_deformableSkins = match m_deformableSkins {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("deformableSkins"),
                );
            }
        };
        let m_rigidSkins = match m_rigidSkins {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigidSkins"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterSkinInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_characterId,
            m_deformableSkins,
            m_rigidSkins,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterSkinInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["characterId", "deformableSkins", "rigidSkins"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterSkinInfo",
                FIELDS,
                __hkbCharacterSkinInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterSkinInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
