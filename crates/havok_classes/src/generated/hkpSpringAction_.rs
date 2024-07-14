use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSpringAction`
/// -         version: `0`
/// -       signature: `0x88fc09fa`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSpringAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBinaryAction<'a>,
    /// # C++ Info
    /// -          name: `lastForce`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_lastForce: Vector4,
    /// # C++ Info
    /// -          name: `positionAinA`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_positionAinA: Vector4,
    /// # C++ Info
    /// -          name: `positionBinB`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_positionBinB: Vector4,
    /// # C++ Info
    /// -          name: `restLength`(ctype: `hkReal`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_restLength: f32,
    /// # C++ Info
    /// -          name: `strength`(ctype: `hkReal`)
    /// -        offset:  84(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_strength: f32,
    /// # C++ Info
    /// -          name: `damping`(ctype: `hkReal`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damping: f32,
    /// # C++ Info
    /// -          name: `onCompression`(ctype: `hkBool`)
    /// -        offset:  92(x86)/124(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_onCompression: bool,
    /// # C++ Info
    /// -          name: `onExtension`(ctype: `hkBool`)
    /// -        offset:  93(x86)/125(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_onExtension: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpSpringAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSpringAction"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x88fc09fa)
        }
    }
    impl<'a> _serde::Serialize for hkpSpringAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x88fc09fa)));
            let mut serializer = __serializer
                .serialize_struct("hkpSpringAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entityA", &self.parent.m_entityA)?;
            serializer.serialize_field("entityB", &self.parent.m_entityB)?;
            serializer.serialize_field("lastForce", &self.m_lastForce)?;
            serializer.serialize_field("positionAinA", &self.m_positionAinA)?;
            serializer.serialize_field("positionBinB", &self.m_positionBinB)?;
            serializer.serialize_field("restLength", &self.m_restLength)?;
            serializer.serialize_field("strength", &self.m_strength)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer.serialize_field("onCompression", &self.m_onCompression)?;
            serializer.serialize_field("onExtension", &self.m_onExtension)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_lastForce,
    m_positionAinA,
    m_positionBinB,
    m_restLength,
    m_strength,
    m_damping,
    m_onCompression,
    m_onExtension,
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
            "lastForce" => Ok(__Field::m_lastForce),
            "positionAinA" => Ok(__Field::m_positionAinA),
            "positionBinB" => Ok(__Field::m_positionBinB),
            "restLength" => Ok(__Field::m_restLength),
            "strength" => Ok(__Field::m_strength),
            "damping" => Ok(__Field::m_damping),
            "onCompression" => Ok(__Field::m_onCompression),
            "onExtension" => Ok(__Field::m_onExtension),
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
pub(super) struct __hkpSpringActionVisitor<'de> {
    marker: core::marker::PhantomData<hkpSpringAction<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSpringActionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSpringAction<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSpringAction<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSpringActionVisitor<'de> {
    type Value = hkpSpringAction<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpSpringAction")
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
        let mut m_lastForce: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_positionAinA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_positionBinB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_restLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_strength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onCompression: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_onExtension: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_lastForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "lastForce",
                            ),
                        );
                    }
                    m_lastForce = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_positionAinA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "positionAinA",
                            ),
                        );
                    }
                    m_positionAinA = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_positionBinB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "positionBinB",
                            ),
                        );
                    }
                    m_positionBinB = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_restLength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "restLength",
                            ),
                        );
                    }
                    m_restLength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_strength) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "strength",
                            ),
                        );
                    }
                    m_strength = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_damping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("damping"),
                        );
                    }
                    m_damping = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_onCompression) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "onCompression",
                            ),
                        );
                    }
                    m_onCompression = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_onExtension) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "onExtension",
                            ),
                        );
                    }
                    m_onExtension = _serde::__private::Some(
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
        __A::pad(&mut __map, 2usize, 2usize)?;
        let m_lastForce = match m_lastForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lastForce"),
                );
            }
        };
        let m_positionAinA = match m_positionAinA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("positionAinA"),
                );
            }
        };
        let m_positionBinB = match m_positionBinB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("positionBinB"),
                );
            }
        };
        let m_restLength = match m_restLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("restLength"),
                );
            }
        };
        let m_strength = match m_strength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("strength"),
                );
            }
        };
        let m_damping = match m_damping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("damping"),
                );
            }
        };
        let m_onCompression = match m_onCompression {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onCompression"),
                );
            }
        };
        let m_onExtension = match m_onExtension {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onExtension"),
                );
            }
        };
        _serde::__private::Ok(hkpSpringAction {
            __ptr,
            parent,
            m_lastForce,
            m_positionAinA,
            m_positionBinB,
            m_restLength,
            m_strength,
            m_damping,
            m_onCompression,
            m_onExtension,
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
        let parent = __hkpBinaryActionVisitor::visit_as_parent(&mut __map)?;
        let mut m_lastForce: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_positionAinA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_positionBinB: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_restLength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_strength: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_onCompression: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_onExtension: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_lastForce => {
                        if _serde::__private::Option::is_some(&m_lastForce) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "lastForce",
                                ),
                            );
                        }
                        m_lastForce = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_positionAinA => {
                        if _serde::__private::Option::is_some(&m_positionAinA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "positionAinA",
                                ),
                            );
                        }
                        m_positionAinA = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_positionBinB => {
                        if _serde::__private::Option::is_some(&m_positionBinB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "positionBinB",
                                ),
                            );
                        }
                        m_positionBinB = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_restLength => {
                        if _serde::__private::Option::is_some(&m_restLength) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "restLength",
                                ),
                            );
                        }
                        m_restLength = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_strength => {
                        if _serde::__private::Option::is_some(&m_strength) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "strength",
                                ),
                            );
                        }
                        m_strength = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_damping => {
                        if _serde::__private::Option::is_some(&m_damping) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "damping",
                                ),
                            );
                        }
                        m_damping = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_onCompression => {
                        if _serde::__private::Option::is_some(&m_onCompression) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "onCompression",
                                ),
                            );
                        }
                        m_onCompression = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_onExtension => {
                        if _serde::__private::Option::is_some(&m_onExtension) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "onExtension",
                                ),
                            );
                        }
                        m_onExtension = _serde::__private::Some(
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
        let m_lastForce = match m_lastForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("lastForce"),
                );
            }
        };
        let m_positionAinA = match m_positionAinA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("positionAinA"),
                );
            }
        };
        let m_positionBinB = match m_positionBinB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("positionBinB"),
                );
            }
        };
        let m_restLength = match m_restLength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("restLength"),
                );
            }
        };
        let m_strength = match m_strength {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("strength"),
                );
            }
        };
        let m_damping = match m_damping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("damping"),
                );
            }
        };
        let m_onCompression = match m_onCompression {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onCompression"),
                );
            }
        };
        let m_onExtension = match m_onExtension {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("onExtension"),
                );
            }
        };
        _serde::__private::Ok(hkpSpringAction {
            __ptr,
            parent,
            m_lastForce,
            m_positionAinA,
            m_positionBinB,
            m_restLength,
            m_strength,
            m_damping,
            m_onCompression,
            m_onExtension,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSpringAction<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "lastForce",
                "positionAinA",
                "positionBinB",
                "restLength",
                "strength",
                "damping",
                "onCompression",
                "onExtension",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSpringAction",
                FIELDS,
                __hkpSpringActionVisitor {
                    marker: _serde::__private::PhantomData::<hkpSpringAction>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
