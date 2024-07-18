use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpSimpleContactConstraintAtom`
/// - version: `0`
/// - signature: `0x920df11a`
/// - size: ` 48`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleContactConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// - name: `sizeOfAllAtoms`(ctype: `hkUint16`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_sizeOfAllAtoms: u16,
    /// # C++ Info
    /// - name: `numContactPoints`(ctype: `hkUint16`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_numContactPoints: u16,
    /// # C++ Info
    /// - name: `numReservedContactPoints`(ctype: `hkUint16`)
    /// - offset: `  6`(x86)/`  6`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_numReservedContactPoints: u16,
    /// # C++ Info
    /// - name: `numUserDatasForBodyA`(ctype: `hkUint8`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_numUserDatasForBodyA: u8,
    /// # C++ Info
    /// - name: `numUserDatasForBodyB`(ctype: `hkUint8`)
    /// - offset: `  9`(x86)/`  9`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_numUserDatasForBodyB: u8,
    /// # C++ Info
    /// - name: `contactPointPropertiesStriding`(ctype: `hkUint8`)
    /// - offset: ` 10`(x86)/` 10`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_contactPointPropertiesStriding: u8,
    /// # C++ Info
    /// - name: `maxNumContactPoints`(ctype: `hkUint16`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_maxNumContactPoints: u16,
    /// # C++ Info
    /// - name: `info`(ctype: `struct hkpSimpleContactConstraintDataInfo`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 32`(x86)/` 32`(x86_64)
    /// - flags: `ALIGN_16`
    pub m_info: hkpSimpleContactConstraintDataInfo,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSimpleContactConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSimpleContactConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x920df11a)
        }
    }
    impl _serde::Serialize for hkpSimpleContactConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x920df11a)));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleContactConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("sizeOfAllAtoms", &self.m_sizeOfAllAtoms)?;
            serializer.serialize_field("numContactPoints", &self.m_numContactPoints)?;
            serializer
                .serialize_field(
                    "numReservedContactPoints",
                    &self.m_numReservedContactPoints,
                )?;
            serializer
                .serialize_field("numUserDatasForBodyA", &self.m_numUserDatasForBodyA)?;
            serializer
                .serialize_field("numUserDatasForBodyB", &self.m_numUserDatasForBodyB)?;
            serializer
                .serialize_field(
                    "contactPointPropertiesStriding",
                    &self.m_contactPointPropertiesStriding,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("maxNumContactPoints", &self.m_maxNumContactPoints)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("info", &self.m_info)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_sizeOfAllAtoms,
    m_numContactPoints,
    m_numReservedContactPoints,
    m_numUserDatasForBodyA,
    m_numUserDatasForBodyB,
    m_contactPointPropertiesStriding,
    m_maxNumContactPoints,
    m_info,
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
            "sizeOfAllAtoms" => Ok(__Field::m_sizeOfAllAtoms),
            "numContactPoints" => Ok(__Field::m_numContactPoints),
            "numReservedContactPoints" => Ok(__Field::m_numReservedContactPoints),
            "numUserDatasForBodyA" => Ok(__Field::m_numUserDatasForBodyA),
            "numUserDatasForBodyB" => Ok(__Field::m_numUserDatasForBodyB),
            "contactPointPropertiesStriding" => {
                Ok(__Field::m_contactPointPropertiesStriding)
            }
            "maxNumContactPoints" => Ok(__Field::m_maxNumContactPoints),
            "info" => Ok(__Field::m_info),
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
pub(super) struct __hkpSimpleContactConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpSimpleContactConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSimpleContactConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSimpleContactConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSimpleContactConstraintAtom>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSimpleContactConstraintAtomVisitor<'de> {
    type Value = hkpSimpleContactConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpSimpleContactConstraintAtom",
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
        let parent = __A::parent_value(&mut __map)?;
        let mut m_sizeOfAllAtoms: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numContactPoints: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numReservedContactPoints: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numUserDatasForBodyA: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_numUserDatasForBodyB: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_contactPointPropertiesStriding: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxNumContactPoints: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_info: _serde::__private::Option<hkpSimpleContactConstraintDataInfo> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_sizeOfAllAtoms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sizeOfAllAtoms",
                            ),
                        );
                    }
                    m_sizeOfAllAtoms = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_numContactPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numContactPoints",
                            ),
                        );
                    }
                    m_numContactPoints = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_numReservedContactPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numReservedContactPoints",
                            ),
                        );
                    }
                    m_numReservedContactPoints = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_numUserDatasForBodyA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numUserDatasForBodyA",
                            ),
                        );
                    }
                    m_numUserDatasForBodyA = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_numUserDatasForBodyB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numUserDatasForBodyB",
                            ),
                        );
                    }
                    m_numUserDatasForBodyB = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(
                        &m_contactPointPropertiesStriding,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactPointPropertiesStriding",
                            ),
                        );
                    }
                    m_contactPointPropertiesStriding = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_maxNumContactPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxNumContactPoints",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_maxNumContactPoints = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_info) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("info"),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 2usize)?;
                    m_info = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSimpleContactConstraintDataInfo,
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
        let m_sizeOfAllAtoms = match m_sizeOfAllAtoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sizeOfAllAtoms"),
                );
            }
        };
        let m_numContactPoints = match m_numContactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numContactPoints"),
                );
            }
        };
        let m_numReservedContactPoints = match m_numReservedContactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numReservedContactPoints",
                    ),
                );
            }
        };
        let m_numUserDatasForBodyA = match m_numUserDatasForBodyA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numUserDatasForBodyA",
                    ),
                );
            }
        };
        let m_numUserDatasForBodyB = match m_numUserDatasForBodyB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numUserDatasForBodyB",
                    ),
                );
            }
        };
        let m_contactPointPropertiesStriding = match m_contactPointPropertiesStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contactPointPropertiesStriding",
                    ),
                );
            }
        };
        let m_maxNumContactPoints = match m_maxNumContactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxNumContactPoints",
                    ),
                );
            }
        };
        let m_info = match m_info {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("info"),
                );
            }
        };
        _serde::__private::Ok(hkpSimpleContactConstraintAtom {
            __ptr,
            parent,
            m_sizeOfAllAtoms,
            m_numContactPoints,
            m_numReservedContactPoints,
            m_numUserDatasForBodyA,
            m_numUserDatasForBodyB,
            m_contactPointPropertiesStriding,
            m_maxNumContactPoints,
            m_info,
        })
    }
    #[allow(clippy::manual_unwrap_or_default)]
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpConstraintAtomVisitor::visit_as_parent(&mut __map)?;
        let mut m_sizeOfAllAtoms: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numContactPoints: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numReservedContactPoints: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_numUserDatasForBodyA: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_numUserDatasForBodyB: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_contactPointPropertiesStriding: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_maxNumContactPoints: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_info: _serde::__private::Option<hkpSimpleContactConstraintDataInfo> = _serde::__private::None;
        for _ in 0..8usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_sizeOfAllAtoms => {
                        if _serde::__private::Option::is_some(&m_sizeOfAllAtoms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sizeOfAllAtoms",
                                ),
                            );
                        }
                        m_sizeOfAllAtoms = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_numContactPoints => {
                        if _serde::__private::Option::is_some(&m_numContactPoints) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numContactPoints",
                                ),
                            );
                        }
                        m_numContactPoints = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_numReservedContactPoints => {
                        if _serde::__private::Option::is_some(
                            &m_numReservedContactPoints,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numReservedContactPoints",
                                ),
                            );
                        }
                        m_numReservedContactPoints = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_numUserDatasForBodyA => {
                        if _serde::__private::Option::is_some(&m_numUserDatasForBodyA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numUserDatasForBodyA",
                                ),
                            );
                        }
                        m_numUserDatasForBodyA = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_numUserDatasForBodyB => {
                        if _serde::__private::Option::is_some(&m_numUserDatasForBodyB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numUserDatasForBodyB",
                                ),
                            );
                        }
                        m_numUserDatasForBodyB = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_contactPointPropertiesStriding => {
                        if _serde::__private::Option::is_some(
                            &m_contactPointPropertiesStriding,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contactPointPropertiesStriding",
                                ),
                            );
                        }
                        m_contactPointPropertiesStriding = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_maxNumContactPoints => {
                        if _serde::__private::Option::is_some(&m_maxNumContactPoints) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxNumContactPoints",
                                ),
                            );
                        }
                        m_maxNumContactPoints = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_info => {
                        if _serde::__private::Option::is_some(&m_info) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("info"),
                            );
                        }
                        m_info = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSimpleContactConstraintDataInfo,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_sizeOfAllAtoms = match m_sizeOfAllAtoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sizeOfAllAtoms"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_numContactPoints = match m_numContactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numContactPoints"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_numReservedContactPoints = match m_numReservedContactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numReservedContactPoints",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_numUserDatasForBodyA = match m_numUserDatasForBodyA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numUserDatasForBodyA",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_numUserDatasForBodyB = match m_numUserDatasForBodyB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numUserDatasForBodyB",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_contactPointPropertiesStriding = match m_contactPointPropertiesStriding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contactPointPropertiesStriding",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxNumContactPoints = match m_maxNumContactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxNumContactPoints",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_info = match m_info {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("info"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpSimpleContactConstraintAtom {
            __ptr,
            parent,
            m_sizeOfAllAtoms,
            m_numContactPoints,
            m_numReservedContactPoints,
            m_numUserDatasForBodyA,
            m_numUserDatasForBodyB,
            m_contactPointPropertiesStriding,
            m_maxNumContactPoints,
            m_info,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSimpleContactConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "sizeOfAllAtoms",
                "numContactPoints",
                "numReservedContactPoints",
                "numUserDatasForBodyA",
                "numUserDatasForBodyB",
                "contactPointPropertiesStriding",
                "maxNumContactPoints",
                "info",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSimpleContactConstraintAtom",
                FIELDS,
                __hkpSimpleContactConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpSimpleContactConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
