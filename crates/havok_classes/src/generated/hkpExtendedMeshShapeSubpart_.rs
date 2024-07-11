use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpExtendedMeshShapeSubpart`
/// -         version: `2`
/// -       signature: `0xf4608207`
/// -          size:  20(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpExtendedMeshShapeSubpart {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum SubpartType`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: SubpartType,
    /// # C++ Info
    /// -          name: `materialIndexStridingType`(ctype: `enum MaterialIndexStridingType`)
    /// -        offset:   1(x86)/  1(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_materialIndexStridingType: MaterialIndexStridingType,
    /// # C++ Info
    /// -          name: `materialStriding`(ctype: `hkInt16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialStriding: i16,
    /// # C++ Info
    /// -          name: `materialIndexBase`(ctype: `void*`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialIndexBase: Pointer,
    /// # C++ Info
    /// -          name: `materialIndexStriding`(ctype: `hkUint16`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_materialIndexStriding: u16,
    /// # C++ Info
    /// -          name: `numMaterials`(ctype: `hkUint16`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_numMaterials: u16,
    /// # C++ Info
    /// -          name: `materialBase`(ctype: `void*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_materialBase: Pointer,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpExtendedMeshShapeSubpart {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpExtendedMeshShapeSubpart"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf4608207)
        }
    }
    impl _serde::Serialize for hkpExtendedMeshShapeSubpart {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf4608207)));
            let mut serializer = __serializer
                .serialize_struct("hkpExtendedMeshShapeSubpart", class_meta)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer
                .serialize_field(
                    "materialIndexStridingType",
                    &self.m_materialIndexStridingType,
                )?;
            serializer.skip_field("materialStriding", &self.m_materialStriding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialIndexBase", &self.m_materialIndexBase)?;
            serializer
                .serialize_field(
                    "materialIndexStriding",
                    &self.m_materialIndexStriding,
                )?;
            serializer.serialize_field("numMaterials", &self.m_numMaterials)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("materialBase", &self.m_materialBase)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_type,
    m_materialIndexStridingType,
    m_materialStriding,
    m_materialIndexBase,
    m_materialIndexStriding,
    m_numMaterials,
    m_materialBase,
    m_userData,
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
            "type" => Ok(__Field::m_type),
            "materialIndexStridingType" => Ok(__Field::m_materialIndexStridingType),
            "materialIndexStriding" => Ok(__Field::m_materialIndexStriding),
            "numMaterials" => Ok(__Field::m_numMaterials),
            "userData" => Ok(__Field::m_userData),
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
pub(super) struct __hkpExtendedMeshShapeSubpartVisitor<'de> {
    marker: core::marker::PhantomData<hkpExtendedMeshShapeSubpart>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpExtendedMeshShapeSubpartVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpExtendedMeshShapeSubpart, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpExtendedMeshShapeSubpart>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpExtendedMeshShapeSubpartVisitor<'de> {
    type Value = hkpExtendedMeshShapeSubpart;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpExtendedMeshShapeSubpart",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_type: _serde::__private::Option<SubpartType> = _serde::__private::None;
        let mut m_materialIndexStridingType: _serde::__private::Option<
            MaterialIndexStridingType,
        > = _serde::__private::None;
        let mut m_materialStriding: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_materialIndexBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_materialIndexStriding: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numMaterials: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_materialBase: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_type) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("type"),
                        );
                    }
                    m_type = _serde::__private::Some(
                        match __A::next_value::<SubpartType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_materialIndexStridingType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndexStridingType",
                            ),
                        );
                    }
                    m_materialIndexStridingType = _serde::__private::Some(
                        match __A::next_value::<MaterialIndexStridingType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_materialStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialStriding",
                            ),
                        );
                    }
                    m_materialStriding = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_materialIndexBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndexBase",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_materialIndexBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_materialIndexStriding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndexStriding",
                            ),
                        );
                    }
                    m_materialIndexStriding = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_numMaterials) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numMaterials",
                            ),
                        );
                    }
                    m_numMaterials = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_materialBase) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialBase",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_materialBase = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_userData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userData",
                            ),
                        );
                    }
                    m_userData = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
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
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_materialIndexStridingType = match m_materialIndexStridingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStridingType",
                    ),
                );
            }
        };
        let m_materialStriding = match m_materialStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialStriding"),
                );
            }
        };
        let m_materialIndexBase = match m_materialIndexBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndexBase"),
                );
            }
        };
        let m_materialIndexStriding = match m_materialIndexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStriding",
                    ),
                );
            }
        };
        let m_numMaterials = match m_numMaterials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numMaterials"),
                );
            }
        };
        let m_materialBase = match m_materialBase {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialBase"),
                );
            }
        };
        let m_userData = match m_userData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShapeSubpart {
            __ptr: __A::class_ptr(&mut __map),
            m_type,
            m_materialIndexStridingType,
            m_materialStriding,
            m_materialIndexBase,
            m_materialIndexStriding,
            m_numMaterials,
            m_materialBase,
            m_userData,
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
        let mut m_type: _serde::__private::Option<SubpartType> = _serde::__private::None;
        let mut m_materialIndexStridingType: _serde::__private::Option<
            MaterialIndexStridingType,
        > = _serde::__private::None;
        let mut m_materialIndexStriding: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numMaterials: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_type => {
                        if _serde::__private::Option::is_some(&m_type) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                            );
                        }
                        m_type = _serde::__private::Some(
                            match __A::next_value::<SubpartType>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_materialIndexStridingType => {
                        if _serde::__private::Option::is_some(
                            &m_materialIndexStridingType,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndexStridingType",
                                ),
                            );
                        }
                        m_materialIndexStridingType = _serde::__private::Some(
                            match __A::next_value::<
                                MaterialIndexStridingType,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_materialIndexStriding => {
                        if _serde::__private::Option::is_some(&m_materialIndexStriding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndexStriding",
                                ),
                            );
                        }
                        m_materialIndexStriding = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numMaterials => {
                        if _serde::__private::Option::is_some(&m_numMaterials) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numMaterials",
                                ),
                            );
                        }
                        m_numMaterials = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userData => {
                        if _serde::__private::Option::is_some(&m_userData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userData",
                                ),
                            );
                        }
                        m_userData = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
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
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_materialIndexStridingType = match m_materialIndexStridingType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStridingType",
                    ),
                );
            }
        };
        let m_materialIndexStriding = match m_materialIndexStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "materialIndexStriding",
                    ),
                );
            }
        };
        let m_numMaterials = match m_numMaterials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numMaterials"),
                );
            }
        };
        let m_userData = match m_userData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData"),
                );
            }
        };
        _serde::__private::Ok(hkpExtendedMeshShapeSubpart {
            __ptr,
            m_type,
            m_materialIndexStridingType,
            m_materialIndexStriding,
            m_numMaterials,
            m_userData,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpExtendedMeshShapeSubpart {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "type",
                "materialIndexStridingType",
                "materialStriding",
                "materialIndexBase",
                "materialIndexStriding",
                "numMaterials",
                "materialBase",
                "userData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpExtendedMeshShapeSubpart",
                FIELDS,
                __hkpExtendedMeshShapeSubpartVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpExtendedMeshShapeSubpart,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
