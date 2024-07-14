use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSIsActiveModifier`
/// -         version: `1`
/// -       signature: `0xb0fde45a`
/// -          size:  56(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSIsActiveModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `bIsActive0`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive0: bool,
    /// # C++ Info
    /// -          name: `bInvertActive0`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive0: bool,
    /// # C++ Info
    /// -          name: `bIsActive1`(ctype: `hkBool`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive1: bool,
    /// # C++ Info
    /// -          name: `bInvertActive1`(ctype: `hkBool`)
    /// -        offset:  47(x86)/ 83(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive1: bool,
    /// # C++ Info
    /// -          name: `bIsActive2`(ctype: `hkBool`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive2: bool,
    /// # C++ Info
    /// -          name: `bInvertActive2`(ctype: `hkBool`)
    /// -        offset:  49(x86)/ 85(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive2: bool,
    /// # C++ Info
    /// -          name: `bIsActive3`(ctype: `hkBool`)
    /// -        offset:  50(x86)/ 86(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive3: bool,
    /// # C++ Info
    /// -          name: `bInvertActive3`(ctype: `hkBool`)
    /// -        offset:  51(x86)/ 87(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive3: bool,
    /// # C++ Info
    /// -          name: `bIsActive4`(ctype: `hkBool`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bIsActive4: bool,
    /// # C++ Info
    /// -          name: `bInvertActive4`(ctype: `hkBool`)
    /// -        offset:  53(x86)/ 89(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bInvertActive4: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSIsActiveModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSIsActiveModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb0fde45a)
        }
    }
    impl<'a> _serde::Serialize for BSIsActiveModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb0fde45a)));
            let mut serializer = __serializer
                .serialize_struct("BSIsActiveModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bIsActive0", &self.m_bIsActive0)?;
            serializer.serialize_field("bInvertActive0", &self.m_bInvertActive0)?;
            serializer.serialize_field("bIsActive1", &self.m_bIsActive1)?;
            serializer.serialize_field("bInvertActive1", &self.m_bInvertActive1)?;
            serializer.serialize_field("bIsActive2", &self.m_bIsActive2)?;
            serializer.serialize_field("bInvertActive2", &self.m_bInvertActive2)?;
            serializer.serialize_field("bIsActive3", &self.m_bIsActive3)?;
            serializer.serialize_field("bInvertActive3", &self.m_bInvertActive3)?;
            serializer.serialize_field("bIsActive4", &self.m_bIsActive4)?;
            serializer.serialize_field("bInvertActive4", &self.m_bInvertActive4)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_bIsActive0,
    m_bInvertActive0,
    m_bIsActive1,
    m_bInvertActive1,
    m_bIsActive2,
    m_bInvertActive2,
    m_bIsActive3,
    m_bInvertActive3,
    m_bIsActive4,
    m_bInvertActive4,
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
            "bIsActive0" => Ok(__Field::m_bIsActive0),
            "bInvertActive0" => Ok(__Field::m_bInvertActive0),
            "bIsActive1" => Ok(__Field::m_bIsActive1),
            "bInvertActive1" => Ok(__Field::m_bInvertActive1),
            "bIsActive2" => Ok(__Field::m_bIsActive2),
            "bInvertActive2" => Ok(__Field::m_bInvertActive2),
            "bIsActive3" => Ok(__Field::m_bIsActive3),
            "bInvertActive3" => Ok(__Field::m_bInvertActive3),
            "bIsActive4" => Ok(__Field::m_bIsActive4),
            "bInvertActive4" => Ok(__Field::m_bInvertActive4),
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
pub(super) struct __BSIsActiveModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSIsActiveModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSIsActiveModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSIsActiveModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSIsActiveModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSIsActiveModifierVisitor<'de> {
    type Value = BSIsActiveModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSIsActiveModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_bIsActive0: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive0: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive4: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive4: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..10usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_bIsActive0) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bIsActive0",
                            ),
                        );
                    }
                    m_bIsActive0 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_bInvertActive0) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bInvertActive0",
                            ),
                        );
                    }
                    m_bInvertActive0 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_bIsActive1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bIsActive1",
                            ),
                        );
                    }
                    m_bIsActive1 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_bInvertActive1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bInvertActive1",
                            ),
                        );
                    }
                    m_bInvertActive1 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_bIsActive2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bIsActive2",
                            ),
                        );
                    }
                    m_bIsActive2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_bInvertActive2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bInvertActive2",
                            ),
                        );
                    }
                    m_bInvertActive2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_bIsActive3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bIsActive3",
                            ),
                        );
                    }
                    m_bIsActive3 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_bInvertActive3) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bInvertActive3",
                            ),
                        );
                    }
                    m_bInvertActive3 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_bIsActive4) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bIsActive4",
                            ),
                        );
                    }
                    m_bIsActive4 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_bInvertActive4) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bInvertActive4",
                            ),
                        );
                    }
                    m_bInvertActive4 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
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
        __A::pad(&mut __map, 2usize, 6usize)?;
        let m_bIsActive0 = match m_bIsActive0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive0"),
                );
            }
        };
        let m_bInvertActive0 = match m_bInvertActive0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive0"),
                );
            }
        };
        let m_bIsActive1 = match m_bIsActive1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive1"),
                );
            }
        };
        let m_bInvertActive1 = match m_bInvertActive1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive1"),
                );
            }
        };
        let m_bIsActive2 = match m_bIsActive2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive2"),
                );
            }
        };
        let m_bInvertActive2 = match m_bInvertActive2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive2"),
                );
            }
        };
        let m_bIsActive3 = match m_bIsActive3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive3"),
                );
            }
        };
        let m_bInvertActive3 = match m_bInvertActive3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive3"),
                );
            }
        };
        let m_bIsActive4 = match m_bIsActive4 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive4"),
                );
            }
        };
        let m_bInvertActive4 = match m_bInvertActive4 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive4"),
                );
            }
        };
        _serde::__private::Ok(BSIsActiveModifier {
            __ptr,
            parent,
            m_bIsActive0,
            m_bInvertActive0,
            m_bIsActive1,
            m_bInvertActive1,
            m_bIsActive2,
            m_bInvertActive2,
            m_bIsActive3,
            m_bInvertActive3,
            m_bIsActive4,
            m_bInvertActive4,
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
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_bIsActive0: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive0: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive1: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive3: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bIsActive4: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bInvertActive4: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..10usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_bIsActive0 => {
                        if _serde::__private::Option::is_some(&m_bIsActive0) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bIsActive0",
                                ),
                            );
                        }
                        m_bIsActive0 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bInvertActive0 => {
                        if _serde::__private::Option::is_some(&m_bInvertActive0) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bInvertActive0",
                                ),
                            );
                        }
                        m_bInvertActive0 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bIsActive1 => {
                        if _serde::__private::Option::is_some(&m_bIsActive1) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bIsActive1",
                                ),
                            );
                        }
                        m_bIsActive1 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bInvertActive1 => {
                        if _serde::__private::Option::is_some(&m_bInvertActive1) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bInvertActive1",
                                ),
                            );
                        }
                        m_bInvertActive1 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bIsActive2 => {
                        if _serde::__private::Option::is_some(&m_bIsActive2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bIsActive2",
                                ),
                            );
                        }
                        m_bIsActive2 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bInvertActive2 => {
                        if _serde::__private::Option::is_some(&m_bInvertActive2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bInvertActive2",
                                ),
                            );
                        }
                        m_bInvertActive2 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bIsActive3 => {
                        if _serde::__private::Option::is_some(&m_bIsActive3) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bIsActive3",
                                ),
                            );
                        }
                        m_bIsActive3 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bInvertActive3 => {
                        if _serde::__private::Option::is_some(&m_bInvertActive3) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bInvertActive3",
                                ),
                            );
                        }
                        m_bInvertActive3 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bIsActive4 => {
                        if _serde::__private::Option::is_some(&m_bIsActive4) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bIsActive4",
                                ),
                            );
                        }
                        m_bIsActive4 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_bInvertActive4 => {
                        if _serde::__private::Option::is_some(&m_bInvertActive4) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bInvertActive4",
                                ),
                            );
                        }
                        m_bInvertActive4 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        let m_bIsActive0 = match m_bIsActive0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive0"),
                );
            }
        };
        let m_bInvertActive0 = match m_bInvertActive0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive0"),
                );
            }
        };
        let m_bIsActive1 = match m_bIsActive1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive1"),
                );
            }
        };
        let m_bInvertActive1 = match m_bInvertActive1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive1"),
                );
            }
        };
        let m_bIsActive2 = match m_bIsActive2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive2"),
                );
            }
        };
        let m_bInvertActive2 = match m_bInvertActive2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive2"),
                );
            }
        };
        let m_bIsActive3 = match m_bIsActive3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive3"),
                );
            }
        };
        let m_bInvertActive3 = match m_bInvertActive3 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive3"),
                );
            }
        };
        let m_bIsActive4 = match m_bIsActive4 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bIsActive4"),
                );
            }
        };
        let m_bInvertActive4 = match m_bInvertActive4 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bInvertActive4"),
                );
            }
        };
        _serde::__private::Ok(BSIsActiveModifier {
            __ptr,
            parent,
            m_bIsActive0,
            m_bInvertActive0,
            m_bIsActive1,
            m_bInvertActive1,
            m_bIsActive2,
            m_bInvertActive2,
            m_bIsActive3,
            m_bInvertActive3,
            m_bIsActive4,
            m_bInvertActive4,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSIsActiveModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "bIsActive0",
                "bInvertActive0",
                "bIsActive1",
                "bInvertActive1",
                "bIsActive2",
                "bInvertActive2",
                "bIsActive3",
                "bInvertActive3",
                "bIsActive4",
                "bInvertActive4",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSIsActiveModifier",
                FIELDS,
                __BSIsActiveModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSIsActiveModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
