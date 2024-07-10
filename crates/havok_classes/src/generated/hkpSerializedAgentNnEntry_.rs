use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedAgentNnEntry`
/// -         version: `0`
/// -       signature: `0x49ec7de3`
/// -          size: 320(x86)/368(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedAgentNnEntry {
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
    /// -          name: `bodyA`(ctype: `struct hkpEntity*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyA: Pointer,
    /// # C++ Info
    /// -          name: `bodyB`(ctype: `struct hkpEntity*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyB: Pointer,
    /// # C++ Info
    /// -          name: `bodyAId`(ctype: `hkUlong`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyAId: u64,
    /// # C++ Info
    /// -          name: `bodyBId`(ctype: `hkUlong`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyBId: u64,
    /// # C++ Info
    /// -          name: `useEntityIds`(ctype: `hkBool`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useEntityIds: bool,
    /// # C++ Info
    /// -          name: `agentType`(ctype: `enum SerializedAgentType`)
    /// -        offset:  25(x86)/ 49(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_agentType: SerializedAgentType,
    /// # C++ Info
    /// -          name: `atom`(ctype: `struct hkpSimpleContactConstraintAtom`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_atom: hkpSimpleContactConstraintAtom,
    /// # C++ Info
    /// -          name: `propertiesStream`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_propertiesStream: Vec<u8>,
    /// # C++ Info
    /// -          name: `contactPoints`(ctype: `hkArray<struct hkContactPoint>`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_contactPoints: Vec<hkContactPoint>,
    /// # C++ Info
    /// -          name: `cpIdMgr`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 104(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_cpIdMgr: Vec<u8>,
    /// # C++ Info
    /// -          name: `nnEntryData`(ctype: `hkUint8[160]`)
    /// -        offset: 116(x86)/160(x86_64)
    /// -     type_size: 160(x86)/160(x86_64)
    ///
    #[cfg_attr(feature = "serde", serde_as(as = "[_; 160]"))]
    #[educe(Default = [0;160usize])]
    pub m_nnEntryData: [u8; 160usize],
    /// # C++ Info
    /// -          name: `trackInfo`(ctype: `struct hkpSerializedTrack1nInfo`)
    /// -        offset: 276(x86)/320(x86_64)
    /// -     type_size:  24(x86)/ 32(x86_64)
    ///
    pub m_trackInfo: hkpSerializedTrack1nInfo,
    /// # C++ Info
    /// -          name: `endianCheckBuffer`(ctype: `hkUint8[4]`)
    /// -        offset: 300(x86)/352(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_endianCheckBuffer: [u8; 4usize],
    /// # C++ Info
    /// -          name: `version`(ctype: `hkUint32`)
    /// -        offset: 304(x86)/356(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_version: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSerializedAgentNnEntry {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSerializedAgentNnEntry"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x49ec7de3)
        }
    }
    impl _serde::Serialize for hkpSerializedAgentNnEntry {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x49ec7de3)));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedAgentNnEntry", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bodyA", &self.m_bodyA)?;
            serializer.serialize_field("bodyB", &self.m_bodyB)?;
            serializer.serialize_field("bodyAId", &self.m_bodyAId)?;
            serializer.serialize_field("bodyBId", &self.m_bodyBId)?;
            serializer.serialize_field("useEntityIds", &self.m_useEntityIds)?;
            serializer.serialize_field("agentType", &self.m_agentType)?;
            serializer.pad_field([0u8; 6usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("atom", &self.m_atom)?;
            serializer
                .serialize_array_meta_field(
                    "propertiesStream",
                    &self.m_propertiesStream,
                )?;
            serializer
                .serialize_array_meta_field("contactPoints", &self.m_contactPoints)?;
            serializer.serialize_array_meta_field("cpIdMgr", &self.m_cpIdMgr)?;
            serializer.serialize_field("nnEntryData", &self.m_nnEntryData.as_slice())?;
            serializer.serialize_field("trackInfo", &self.m_trackInfo)?;
            serializer
                .serialize_field(
                    "endianCheckBuffer",
                    &self.m_endianCheckBuffer.as_slice(),
                )?;
            serializer.serialize_field("version", &self.m_version)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field("propertiesStream", &self.m_propertiesStream)?;
            serializer.serialize_array_field("contactPoints", &self.m_contactPoints)?;
            serializer.serialize_array_field("cpIdMgr", &self.m_cpIdMgr)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_bodyA,
    m_bodyB,
    m_bodyAId,
    m_bodyBId,
    m_useEntityIds,
    m_agentType,
    m_atom,
    m_propertiesStream,
    m_contactPoints,
    m_cpIdMgr,
    m_nnEntryData,
    m_trackInfo,
    m_endianCheckBuffer,
    m_version,
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
            "bodyA" => Ok(__Field::m_bodyA),
            "bodyB" => Ok(__Field::m_bodyB),
            "bodyAId" => Ok(__Field::m_bodyAId),
            "bodyBId" => Ok(__Field::m_bodyBId),
            "useEntityIds" => Ok(__Field::m_useEntityIds),
            "agentType" => Ok(__Field::m_agentType),
            "atom" => Ok(__Field::m_atom),
            "propertiesStream" => Ok(__Field::m_propertiesStream),
            "contactPoints" => Ok(__Field::m_contactPoints),
            "cpIdMgr" => Ok(__Field::m_cpIdMgr),
            "nnEntryData" => Ok(__Field::m_nnEntryData),
            "trackInfo" => Ok(__Field::m_trackInfo),
            "endianCheckBuffer" => Ok(__Field::m_endianCheckBuffer),
            "version" => Ok(__Field::m_version),
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
pub(super) struct __hkpSerializedAgentNnEntryVisitor<'de> {
    marker: core::marker::PhantomData<hkpSerializedAgentNnEntry>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSerializedAgentNnEntryVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSerializedAgentNnEntry, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSerializedAgentNnEntry>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSerializedAgentNnEntryVisitor<'de> {
    type Value = hkpSerializedAgentNnEntry;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpSerializedAgentNnEntry")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_bodyA: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_bodyB: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_bodyAId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_bodyBId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_useEntityIds: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_agentType: _serde::__private::Option<SerializedAgentType> = _serde::__private::None;
        let mut m_atom: _serde::__private::Option<hkpSimpleContactConstraintAtom> = _serde::__private::None;
        let mut m_propertiesStream: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_contactPoints: _serde::__private::Option<Vec<hkContactPoint>> = _serde::__private::None;
        let mut m_cpIdMgr: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_nnEntryData: _serde::__private::Option<[u8; 160usize]> = _serde::__private::None;
        let mut m_trackInfo: _serde::__private::Option<hkpSerializedTrack1nInfo> = _serde::__private::None;
        let mut m_endianCheckBuffer: _serde::__private::Option<[u8; 4usize]> = _serde::__private::None;
        let mut m_version: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..14usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_bodyA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyA"),
                        );
                    }
                    m_bodyA = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_bodyB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyB"),
                        );
                    }
                    m_bodyB = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_bodyAId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyAId"),
                        );
                    }
                    m_bodyAId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_bodyBId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyBId"),
                        );
                    }
                    m_bodyBId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_useEntityIds) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "useEntityIds",
                            ),
                        );
                    }
                    m_useEntityIds = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_agentType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "agentType",
                            ),
                        );
                    }
                    m_agentType = _serde::__private::Some(
                        match __A::next_value::<SerializedAgentType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_atom) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("atom"),
                        );
                    }
                    __A::pad(&mut __map, 6usize, 14usize)?;
                    m_atom = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSimpleContactConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_propertiesStream) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "propertiesStream",
                            ),
                        );
                    }
                    m_propertiesStream = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_contactPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactPoints",
                            ),
                        );
                    }
                    m_contactPoints = _serde::__private::Some(
                        match __A::next_value::<Vec<hkContactPoint>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_cpIdMgr) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("cpIdMgr"),
                        );
                    }
                    m_cpIdMgr = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_nnEntryData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nnEntryData",
                            ),
                        );
                    }
                    m_nnEntryData = _serde::__private::Some(
                        match __A::next_value::<[u8; 160usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_trackInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "trackInfo",
                            ),
                        );
                    }
                    m_trackInfo = _serde::__private::Some(
                        match __A::next_value::<hkpSerializedTrack1nInfo>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_endianCheckBuffer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endianCheckBuffer",
                            ),
                        );
                    }
                    m_endianCheckBuffer = _serde::__private::Some(
                        match __A::next_value::<[u8; 4usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_version) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("version"),
                        );
                    }
                    m_version = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        __A::pad(&mut __map, 12usize, 8usize)?;
        let m_bodyA = match m_bodyA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyA"),
                );
            }
        };
        let m_bodyB = match m_bodyB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyB"),
                );
            }
        };
        let m_bodyAId = match m_bodyAId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyAId"),
                );
            }
        };
        let m_bodyBId = match m_bodyBId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyBId"),
                );
            }
        };
        let m_useEntityIds = match m_useEntityIds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("useEntityIds"),
                );
            }
        };
        let m_agentType = match m_agentType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("agentType"),
                );
            }
        };
        let m_atom = match m_atom {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atom"),
                );
            }
        };
        let m_propertiesStream = match m_propertiesStream {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("propertiesStream"),
                );
            }
        };
        let m_contactPoints = match m_contactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactPoints"),
                );
            }
        };
        let m_cpIdMgr = match m_cpIdMgr {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cpIdMgr"),
                );
            }
        };
        let m_nnEntryData = match m_nnEntryData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nnEntryData"),
                );
            }
        };
        let m_trackInfo = match m_trackInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("trackInfo"),
                );
            }
        };
        let m_endianCheckBuffer = match m_endianCheckBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endianCheckBuffer"),
                );
            }
        };
        let m_version = match m_version {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("version"),
                );
            }
        };
        _serde::__private::Ok(hkpSerializedAgentNnEntry {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_bodyA,
            m_bodyB,
            m_bodyAId,
            m_bodyBId,
            m_useEntityIds,
            m_agentType,
            m_atom,
            m_propertiesStream,
            m_contactPoints,
            m_cpIdMgr,
            m_nnEntryData,
            m_trackInfo,
            m_endianCheckBuffer,
            m_version,
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
        let mut m_bodyA: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_bodyB: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_bodyAId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_bodyBId: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_useEntityIds: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_agentType: _serde::__private::Option<SerializedAgentType> = _serde::__private::None;
        let mut m_atom: _serde::__private::Option<hkpSimpleContactConstraintAtom> = _serde::__private::None;
        let mut m_propertiesStream: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_contactPoints: _serde::__private::Option<Vec<hkContactPoint>> = _serde::__private::None;
        let mut m_cpIdMgr: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_nnEntryData: _serde::__private::Option<[u8; 160usize]> = _serde::__private::None;
        let mut m_trackInfo: _serde::__private::Option<hkpSerializedTrack1nInfo> = _serde::__private::None;
        let mut m_endianCheckBuffer: _serde::__private::Option<[u8; 4usize]> = _serde::__private::None;
        let mut m_version: _serde::__private::Option<u32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_bodyA => {
                    if _serde::__private::Option::is_some(&m_bodyA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyA"),
                        );
                    }
                    m_bodyA = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bodyB => {
                    if _serde::__private::Option::is_some(&m_bodyB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyB"),
                        );
                    }
                    m_bodyB = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bodyAId => {
                    if _serde::__private::Option::is_some(&m_bodyAId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyAId"),
                        );
                    }
                    m_bodyAId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_bodyBId => {
                    if _serde::__private::Option::is_some(&m_bodyBId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bodyBId"),
                        );
                    }
                    m_bodyBId = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_useEntityIds => {
                    if _serde::__private::Option::is_some(&m_useEntityIds) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "useEntityIds",
                            ),
                        );
                    }
                    m_useEntityIds = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_agentType => {
                    if _serde::__private::Option::is_some(&m_agentType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "agentType",
                            ),
                        );
                    }
                    m_agentType = _serde::__private::Some(
                        match __A::next_value::<SerializedAgentType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_atom => {
                    if _serde::__private::Option::is_some(&m_atom) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("atom"),
                        );
                    }
                    m_atom = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSimpleContactConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_propertiesStream => {
                    if _serde::__private::Option::is_some(&m_propertiesStream) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "propertiesStream",
                            ),
                        );
                    }
                    m_propertiesStream = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_contactPoints => {
                    if _serde::__private::Option::is_some(&m_contactPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactPoints",
                            ),
                        );
                    }
                    m_contactPoints = _serde::__private::Some(
                        match __A::next_value::<Vec<hkContactPoint>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_cpIdMgr => {
                    if _serde::__private::Option::is_some(&m_cpIdMgr) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("cpIdMgr"),
                        );
                    }
                    m_cpIdMgr = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_nnEntryData => {
                    if _serde::__private::Option::is_some(&m_nnEntryData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nnEntryData",
                            ),
                        );
                    }
                    m_nnEntryData = _serde::__private::Some(
                        match __A::next_value::<[u8; 160usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_trackInfo => {
                    if _serde::__private::Option::is_some(&m_trackInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "trackInfo",
                            ),
                        );
                    }
                    m_trackInfo = _serde::__private::Some(
                        match __A::next_value::<hkpSerializedTrack1nInfo>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_endianCheckBuffer => {
                    if _serde::__private::Option::is_some(&m_endianCheckBuffer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endianCheckBuffer",
                            ),
                        );
                    }
                    m_endianCheckBuffer = _serde::__private::Some(
                        match __A::next_value::<[u8; 4usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_version => {
                    if _serde::__private::Option::is_some(&m_version) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("version"),
                        );
                    }
                    m_version = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        let m_bodyA = match m_bodyA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyA"),
                );
            }
        };
        let m_bodyB = match m_bodyB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyB"),
                );
            }
        };
        let m_bodyAId = match m_bodyAId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyAId"),
                );
            }
        };
        let m_bodyBId = match m_bodyBId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bodyBId"),
                );
            }
        };
        let m_useEntityIds = match m_useEntityIds {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("useEntityIds"),
                );
            }
        };
        let m_agentType = match m_agentType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("agentType"),
                );
            }
        };
        let m_atom = match m_atom {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atom"),
                );
            }
        };
        let m_propertiesStream = match m_propertiesStream {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("propertiesStream"),
                );
            }
        };
        let m_contactPoints = match m_contactPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactPoints"),
                );
            }
        };
        let m_cpIdMgr = match m_cpIdMgr {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cpIdMgr"),
                );
            }
        };
        let m_nnEntryData = match m_nnEntryData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nnEntryData"),
                );
            }
        };
        let m_trackInfo = match m_trackInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("trackInfo"),
                );
            }
        };
        let m_endianCheckBuffer = match m_endianCheckBuffer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endianCheckBuffer"),
                );
            }
        };
        let m_version = match m_version {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("version"),
                );
            }
        };
        _serde::__private::Ok(hkpSerializedAgentNnEntry {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_bodyA,
            m_bodyB,
            m_bodyAId,
            m_bodyBId,
            m_useEntityIds,
            m_agentType,
            m_atom,
            m_propertiesStream,
            m_contactPoints,
            m_cpIdMgr,
            m_nnEntryData,
            m_trackInfo,
            m_endianCheckBuffer,
            m_version,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSerializedAgentNnEntry {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "bodyA",
                "bodyB",
                "bodyAId",
                "bodyBId",
                "useEntityIds",
                "agentType",
                "atom",
                "propertiesStream",
                "contactPoints",
                "cpIdMgr",
                "nnEntryData",
                "trackInfo",
                "endianCheckBuffer",
                "version",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSerializedAgentNnEntry",
                FIELDS,
                __hkpSerializedAgentNnEntryVisitor {
                    marker: _serde::__private::PhantomData::<hkpSerializedAgentNnEntry>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum SerializedAgentType {
    #[default]
    INVALID_AGENT_TYPE = 0isize,
    BOX_BOX_AGENT3 = 1isize,
    CAPSULE_TRIANGLE_AGENT3 = 2isize,
    PRED_GSK_AGENT3 = 3isize,
    PRED_GSK_CYLINDER_AGENT3 = 4isize,
    CONVEX_LIST_AGENT3 = 5isize,
    LIST_AGENT3 = 6isize,
    BV_TREE_AGENT3 = 7isize,
    COLLECTION_COLLECTION_AGENT3 = 8isize,
    COLLECTION_AGENT3 = 9isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SerializedAgentType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INVALID_AGENT_TYPE => {
                    __serializer.serialize_field("INVALID_AGENT_TYPE", &0u64)
                }
                Self::BOX_BOX_AGENT3 => {
                    __serializer.serialize_field("BOX_BOX_AGENT3", &1u64)
                }
                Self::CAPSULE_TRIANGLE_AGENT3 => {
                    __serializer.serialize_field("CAPSULE_TRIANGLE_AGENT3", &2u64)
                }
                Self::PRED_GSK_AGENT3 => {
                    __serializer.serialize_field("PRED_GSK_AGENT3", &3u64)
                }
                Self::PRED_GSK_CYLINDER_AGENT3 => {
                    __serializer.serialize_field("PRED_GSK_CYLINDER_AGENT3", &4u64)
                }
                Self::CONVEX_LIST_AGENT3 => {
                    __serializer.serialize_field("CONVEX_LIST_AGENT3", &5u64)
                }
                Self::LIST_AGENT3 => __serializer.serialize_field("LIST_AGENT3", &6u64),
                Self::BV_TREE_AGENT3 => {
                    __serializer.serialize_field("BV_TREE_AGENT3", &7u64)
                }
                Self::COLLECTION_COLLECTION_AGENT3 => {
                    __serializer.serialize_field("COLLECTION_COLLECTION_AGENT3", &8u64)
                }
                Self::COLLECTION_AGENT3 => {
                    __serializer.serialize_field("COLLECTION_AGENT3", &9u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SerializedAgentType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SerializedAgentType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        4i8 => _serde::__private::Ok(__Field::__field4),
                        5i8 => _serde::__private::Ok(__Field::__field5),
                        6i8 => _serde::__private::Ok(__Field::__field6),
                        7i8 => _serde::__private::Ok(__Field::__field7),
                        8i8 => _serde::__private::Ok(__Field::__field8),
                        9i8 => _serde::__private::Ok(__Field::__field9),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("INVALID_AGENT_TYPE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("BOX_BOX_AGENT3") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("CAPSULE_TRIANGLE_AGENT3") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("PRED_GSK_AGENT3") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("PRED_GSK_CYLINDER_AGENT3") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("CONVEX_LIST_AGENT3") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("LIST_AGENT3") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7" || v.eq_ignore_ascii_case("BV_TREE_AGENT3") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8"
                                || v
                                    .eq_ignore_ascii_case("COLLECTION_COLLECTION_AGENT3") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9"
                                || v.eq_ignore_ascii_case("COLLECTION_AGENT3") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SerializedAgentType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SerializedAgentType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SerializedAgentType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SerializedAgentType::INVALID_AGENT_TYPE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SerializedAgentType::BOX_BOX_AGENT3)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SerializedAgentType::CAPSULE_TRIANGLE_AGENT3,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SerializedAgentType::PRED_GSK_AGENT3)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SerializedAgentType::PRED_GSK_CYLINDER_AGENT3,
                            )
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SerializedAgentType::CONVEX_LIST_AGENT3,
                            )
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SerializedAgentType::LIST_AGENT3)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SerializedAgentType::BV_TREE_AGENT3)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SerializedAgentType::COLLECTION_COLLECTION_AGENT3,
                            )
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SerializedAgentType::COLLECTION_AGENT3)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "INVALID_AGENT_TYPE",
                "BOX_BOX_AGENT3",
                "CAPSULE_TRIANGLE_AGENT3",
                "PRED_GSK_AGENT3",
                "PRED_GSK_CYLINDER_AGENT3",
                "CONVEX_LIST_AGENT3",
                "LIST_AGENT3",
                "BV_TREE_AGENT3",
                "COLLECTION_COLLECTION_AGENT3",
                "COLLECTION_AGENT3",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SerializedAgentType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SerializedAgentType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
