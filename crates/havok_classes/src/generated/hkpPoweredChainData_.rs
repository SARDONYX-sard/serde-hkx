use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPoweredChainData`
/// -         version: `0`
/// -       signature: `0x38aeafc3`
/// -          size:  64(x86)/ 96(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPoweredChainData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintChainData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpBridgeAtoms`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 24(x86_64)
    ///
    pub m_atoms: hkpBridgeAtoms,
    /// # C++ Info
    /// -          name: `infos`(ctype: `hkArray<struct hkpPoweredChainDataConstraintInfo>`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_infos: Vec<hkpPoweredChainDataConstraintInfo>,
    /// # C++ Info
    /// -          name: `tau`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tau: f32,
    /// # C++ Info
    /// -          name: `damping`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damping: f32,
    /// # C++ Info
    /// -          name: `cfmLinAdd`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmLinAdd: f32,
    /// # C++ Info
    /// -          name: `cfmLinMul`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmLinMul: f32,
    /// # C++ Info
    /// -          name: `cfmAngAdd`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmAngAdd: f32,
    /// # C++ Info
    /// -          name: `cfmAngMul`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cfmAngMul: f32,
    /// # C++ Info
    /// -          name: `maxErrorDistance`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxErrorDistance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPoweredChainData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPoweredChainData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x38aeafc3)
        }
    }
    impl _serde::Serialize for hkpPoweredChainData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x38aeafc3)));
            let mut serializer = __serializer
                .serialize_struct("hkpPoweredChainData", class_meta)?;
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
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_array_meta_field("infos", &self.m_infos)?;
            serializer.serialize_field("tau", &self.m_tau)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer.serialize_field("cfmLinAdd", &self.m_cfmLinAdd)?;
            serializer.serialize_field("cfmLinMul", &self.m_cfmLinMul)?;
            serializer.serialize_field("cfmAngAdd", &self.m_cfmAngAdd)?;
            serializer.serialize_field("cfmAngMul", &self.m_cfmAngMul)?;
            serializer.serialize_field("maxErrorDistance", &self.m_maxErrorDistance)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("infos", &self.m_infos)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_atoms,
    m_infos,
    m_tau,
    m_damping,
    m_cfmLinAdd,
    m_cfmLinMul,
    m_cfmAngAdd,
    m_cfmAngMul,
    m_maxErrorDistance,
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
            "atoms" => Ok(__Field::m_atoms),
            "infos" => Ok(__Field::m_infos),
            "tau" => Ok(__Field::m_tau),
            "damping" => Ok(__Field::m_damping),
            "cfmLinAdd" => Ok(__Field::m_cfmLinAdd),
            "cfmLinMul" => Ok(__Field::m_cfmLinMul),
            "cfmAngAdd" => Ok(__Field::m_cfmAngAdd),
            "cfmAngMul" => Ok(__Field::m_cfmAngMul),
            "maxErrorDistance" => Ok(__Field::m_maxErrorDistance),
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
pub(super) struct __hkpPoweredChainDataVisitor<'de> {
    marker: core::marker::PhantomData<hkpPoweredChainData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPoweredChainDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPoweredChainData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPoweredChainData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPoweredChainDataVisitor<'de> {
    type Value = hkpPoweredChainData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpPoweredChainData")
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
        let mut m_atoms: _serde::__private::Option<hkpBridgeAtoms> = _serde::__private::None;
        let mut m_infos: _serde::__private::Option<
            Vec<hkpPoweredChainDataConstraintInfo>,
        > = _serde::__private::None;
        let mut m_tau: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmLinAdd: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmLinMul: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmAngAdd: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmAngMul: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxErrorDistance: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..9usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_atoms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                        );
                    }
                    m_atoms = _serde::__private::Some(
                        match __A::next_value::<hkpBridgeAtoms>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_infos) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("infos"),
                        );
                    }
                    m_infos = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpPoweredChainDataConstraintInfo>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_tau) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("tau"),
                        );
                    }
                    m_tau = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
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
                4usize => {
                    if _serde::__private::Option::is_some(&m_cfmLinAdd) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cfmLinAdd",
                            ),
                        );
                    }
                    m_cfmLinAdd = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_cfmLinMul) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cfmLinMul",
                            ),
                        );
                    }
                    m_cfmLinMul = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_cfmAngAdd) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cfmAngAdd",
                            ),
                        );
                    }
                    m_cfmAngAdd = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_cfmAngMul) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cfmAngMul",
                            ),
                        );
                    }
                    m_cfmAngMul = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_maxErrorDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxErrorDistance",
                            ),
                        );
                    }
                    m_maxErrorDistance = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_atoms = match m_atoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atoms"),
                );
            }
        };
        let m_infos = match m_infos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("infos"),
                );
            }
        };
        let m_tau = match m_tau {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tau"),
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
        let m_cfmLinAdd = match m_cfmLinAdd {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmLinAdd"),
                );
            }
        };
        let m_cfmLinMul = match m_cfmLinMul {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmLinMul"),
                );
            }
        };
        let m_cfmAngAdd = match m_cfmAngAdd {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmAngAdd"),
                );
            }
        };
        let m_cfmAngMul = match m_cfmAngMul {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmAngMul"),
                );
            }
        };
        let m_maxErrorDistance = match m_maxErrorDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxErrorDistance"),
                );
            }
        };
        _serde::__private::Ok(hkpPoweredChainData {
            __ptr,
            parent,
            m_atoms,
            m_infos,
            m_tau,
            m_damping,
            m_cfmLinAdd,
            m_cfmLinMul,
            m_cfmAngAdd,
            m_cfmAngMul,
            m_maxErrorDistance,
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
        let parent = __hkpConstraintChainDataVisitor::visit_as_parent(&mut __map)?;
        let mut m_atoms: _serde::__private::Option<hkpBridgeAtoms> = _serde::__private::None;
        let mut m_infos: _serde::__private::Option<
            Vec<hkpPoweredChainDataConstraintInfo>,
        > = _serde::__private::None;
        let mut m_tau: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_damping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmLinAdd: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmLinMul: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmAngAdd: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_cfmAngMul: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxErrorDistance: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..9usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_atoms => {
                        if _serde::__private::Option::is_some(&m_atoms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                            );
                        }
                        m_atoms = _serde::__private::Some(
                            match __A::next_value::<hkpBridgeAtoms>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_infos => {
                        if _serde::__private::Option::is_some(&m_infos) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("infos"),
                            );
                        }
                        m_infos = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpPoweredChainDataConstraintInfo>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_tau => {
                        if _serde::__private::Option::is_some(&m_tau) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("tau"),
                            );
                        }
                        m_tau = _serde::__private::Some(
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
                    __Field::m_cfmLinAdd => {
                        if _serde::__private::Option::is_some(&m_cfmLinAdd) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cfmLinAdd",
                                ),
                            );
                        }
                        m_cfmLinAdd = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_cfmLinMul => {
                        if _serde::__private::Option::is_some(&m_cfmLinMul) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cfmLinMul",
                                ),
                            );
                        }
                        m_cfmLinMul = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_cfmAngAdd => {
                        if _serde::__private::Option::is_some(&m_cfmAngAdd) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cfmAngAdd",
                                ),
                            );
                        }
                        m_cfmAngAdd = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_cfmAngMul => {
                        if _serde::__private::Option::is_some(&m_cfmAngMul) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cfmAngMul",
                                ),
                            );
                        }
                        m_cfmAngMul = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxErrorDistance => {
                        if _serde::__private::Option::is_some(&m_maxErrorDistance) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxErrorDistance",
                                ),
                            );
                        }
                        m_maxErrorDistance = _serde::__private::Some(
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
        }
        let m_atoms = match m_atoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atoms"),
                );
            }
        };
        let m_infos = match m_infos {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("infos"),
                );
            }
        };
        let m_tau = match m_tau {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tau"),
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
        let m_cfmLinAdd = match m_cfmLinAdd {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmLinAdd"),
                );
            }
        };
        let m_cfmLinMul = match m_cfmLinMul {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmLinMul"),
                );
            }
        };
        let m_cfmAngAdd = match m_cfmAngAdd {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmAngAdd"),
                );
            }
        };
        let m_cfmAngMul = match m_cfmAngMul {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cfmAngMul"),
                );
            }
        };
        let m_maxErrorDistance = match m_maxErrorDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxErrorDistance"),
                );
            }
        };
        _serde::__private::Ok(hkpPoweredChainData {
            __ptr,
            parent,
            m_atoms,
            m_infos,
            m_tau,
            m_damping,
            m_cfmLinAdd,
            m_cfmLinMul,
            m_cfmAngAdd,
            m_cfmAngMul,
            m_maxErrorDistance,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPoweredChainData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "atoms",
                "infos",
                "tau",
                "damping",
                "cfmLinAdd",
                "cfmLinMul",
                "cfmAngAdd",
                "cfmAngMul",
                "maxErrorDistance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPoweredChainData",
                FIELDS,
                __hkpPoweredChainDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpPoweredChainData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
