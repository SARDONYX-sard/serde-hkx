use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbTransformVectorModifier`
/// -         version: `0`
/// -       signature: `0xf93e0e24`
/// -          size: 128(x86)/160(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTransformVectorModifier<'a> {
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
    /// -          name: `rotation`(ctype: `hkQuaternion`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation: Quaternion,
    /// # C++ Info
    /// -          name: `translation`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_translation: Vector4,
    /// # C++ Info
    /// -          name: `vectorIn`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vectorIn: Vector4,
    /// # C++ Info
    /// -          name: `vectorOut`(ctype: `hkVector4`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vectorOut: Vector4,
    /// # C++ Info
    /// -          name: `rotateOnly`(ctype: `hkBool`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_rotateOnly: bool,
    /// # C++ Info
    /// -          name: `inverse`(ctype: `hkBool`)
    /// -        offset: 113(x86)/145(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_inverse: bool,
    /// # C++ Info
    /// -          name: `computeOnActivate`(ctype: `hkBool`)
    /// -        offset: 114(x86)/146(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computeOnActivate: bool,
    /// # C++ Info
    /// -          name: `computeOnModify`(ctype: `hkBool`)
    /// -        offset: 115(x86)/147(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computeOnModify: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbTransformVectorModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbTransformVectorModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf93e0e24)
        }
    }
    impl<'a> _serde::Serialize for hkbTransformVectorModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf93e0e24)));
            let mut serializer = __serializer
                .serialize_struct("hkbTransformVectorModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("rotation", &self.m_rotation)?;
            serializer.serialize_field("translation", &self.m_translation)?;
            serializer.serialize_field("vectorIn", &self.m_vectorIn)?;
            serializer.serialize_field("vectorOut", &self.m_vectorOut)?;
            serializer.serialize_field("rotateOnly", &self.m_rotateOnly)?;
            serializer.serialize_field("inverse", &self.m_inverse)?;
            serializer.serialize_field("computeOnActivate", &self.m_computeOnActivate)?;
            serializer.serialize_field("computeOnModify", &self.m_computeOnModify)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
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
    m_rotation,
    m_translation,
    m_vectorIn,
    m_vectorOut,
    m_rotateOnly,
    m_inverse,
    m_computeOnActivate,
    m_computeOnModify,
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
            "rotation" => Ok(__Field::m_rotation),
            "translation" => Ok(__Field::m_translation),
            "vectorIn" => Ok(__Field::m_vectorIn),
            "vectorOut" => Ok(__Field::m_vectorOut),
            "rotateOnly" => Ok(__Field::m_rotateOnly),
            "inverse" => Ok(__Field::m_inverse),
            "computeOnActivate" => Ok(__Field::m_computeOnActivate),
            "computeOnModify" => Ok(__Field::m_computeOnModify),
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
pub(super) struct __hkbTransformVectorModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbTransformVectorModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbTransformVectorModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbTransformVectorModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbTransformVectorModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkbTransformVectorModifierVisitor<'de> {
    type Value = hkbTransformVectorModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbTransformVectorModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::parent_value(&mut __map)?;
        let mut m_rotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_translation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vectorIn: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vectorOut: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotateOnly: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_inverse: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computeOnActivate: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computeOnModify: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_rotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotation",
                            ),
                        );
                    }
                    m_rotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_translation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "translation",
                            ),
                        );
                    }
                    m_translation = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_vectorIn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vectorIn",
                            ),
                        );
                    }
                    m_vectorIn = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_vectorOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "vectorOut",
                            ),
                        );
                    }
                    m_vectorOut = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_rotateOnly) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotateOnly",
                            ),
                        );
                    }
                    m_rotateOnly = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_inverse) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("inverse"),
                        );
                    }
                    m_inverse = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_computeOnActivate) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "computeOnActivate",
                            ),
                        );
                    }
                    m_computeOnActivate = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_computeOnModify) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "computeOnModify",
                            ),
                        );
                    }
                    m_computeOnModify = _serde::__private::Some(
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
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_rotation = match m_rotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotation"),
                );
            }
        };
        let m_translation = match m_translation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translation"),
                );
            }
        };
        let m_vectorIn = match m_vectorIn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorIn"),
                );
            }
        };
        let m_vectorOut = match m_vectorOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorOut"),
                );
            }
        };
        let m_rotateOnly = match m_rotateOnly {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotateOnly"),
                );
            }
        };
        let m_inverse = match m_inverse {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("inverse"),
                );
            }
        };
        let m_computeOnActivate = match m_computeOnActivate {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computeOnActivate"),
                );
            }
        };
        let m_computeOnModify = match m_computeOnModify {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computeOnModify"),
                );
            }
        };
        _serde::__private::Ok(hkbTransformVectorModifier {
            __ptr,
            parent,
            m_rotation,
            m_translation,
            m_vectorIn,
            m_vectorOut,
            m_rotateOnly,
            m_inverse,
            m_computeOnActivate,
            m_computeOnModify,
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
        let mut m_rotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_translation: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vectorIn: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_vectorOut: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_rotateOnly: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_inverse: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computeOnActivate: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_computeOnModify: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_rotation => {
                        if _serde::__private::Option::is_some(&m_rotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotation",
                                ),
                            );
                        }
                        m_rotation = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_translation => {
                        if _serde::__private::Option::is_some(&m_translation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "translation",
                                ),
                            );
                        }
                        m_translation = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_vectorIn => {
                        if _serde::__private::Option::is_some(&m_vectorIn) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vectorIn",
                                ),
                            );
                        }
                        m_vectorIn = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_vectorOut => {
                        if _serde::__private::Option::is_some(&m_vectorOut) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vectorOut",
                                ),
                            );
                        }
                        m_vectorOut = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rotateOnly => {
                        if _serde::__private::Option::is_some(&m_rotateOnly) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotateOnly",
                                ),
                            );
                        }
                        m_rotateOnly = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_inverse => {
                        if _serde::__private::Option::is_some(&m_inverse) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "inverse",
                                ),
                            );
                        }
                        m_inverse = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_computeOnActivate => {
                        if _serde::__private::Option::is_some(&m_computeOnActivate) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "computeOnActivate",
                                ),
                            );
                        }
                        m_computeOnActivate = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_computeOnModify => {
                        if _serde::__private::Option::is_some(&m_computeOnModify) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "computeOnModify",
                                ),
                            );
                        }
                        m_computeOnModify = _serde::__private::Some(
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
        let m_rotation = match m_rotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotation"),
                );
            }
        };
        let m_translation = match m_translation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translation"),
                );
            }
        };
        let m_vectorIn = match m_vectorIn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorIn"),
                );
            }
        };
        let m_vectorOut = match m_vectorOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectorOut"),
                );
            }
        };
        let m_rotateOnly = match m_rotateOnly {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotateOnly"),
                );
            }
        };
        let m_inverse = match m_inverse {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("inverse"),
                );
            }
        };
        let m_computeOnActivate = match m_computeOnActivate {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computeOnActivate"),
                );
            }
        };
        let m_computeOnModify = match m_computeOnModify {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("computeOnModify"),
                );
            }
        };
        _serde::__private::Ok(hkbTransformVectorModifier {
            __ptr,
            parent,
            m_rotation,
            m_translation,
            m_vectorIn,
            m_vectorOut,
            m_rotateOnly,
            m_inverse,
            m_computeOnActivate,
            m_computeOnModify,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbTransformVectorModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "rotation",
                "translation",
                "vectorIn",
                "vectorOut",
                "rotateOnly",
                "inverse",
                "computeOnActivate",
                "computeOnModify",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbTransformVectorModifier",
                FIELDS,
                __hkbTransformVectorModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbTransformVectorModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
