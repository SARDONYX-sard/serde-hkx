use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedSubTrack1nInfo`
/// -         version: `0`
/// -       signature: `0x10155a`
/// -          size:  32(x86)/ 40(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedSubTrack1nInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpSerializedTrack1nInfo,
    /// # C++ Info
    /// -          name: `sectorIndex`(ctype: `hkInt32`)
    /// -        offset:  24(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sectorIndex: i32,
    /// # C++ Info
    /// -          name: `offsetInSector`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_offsetInSector: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSerializedSubTrack1nInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSerializedSubTrack1nInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x10155a)
        }
    }
    impl _serde::Serialize for hkpSerializedSubTrack1nInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x10155a)));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedSubTrack1nInfo", class_meta)?;
            serializer.serialize_array_meta_field("sectors", &self.parent.m_sectors)?;
            serializer
                .serialize_array_meta_field("subTracks", &self.parent.m_subTracks)?;
            serializer.serialize_field("sectorIndex", &self.m_sectorIndex)?;
            serializer.serialize_field("offsetInSector", &self.m_offsetInSector)?;
            serializer.serialize_array_field("sectors", &self.parent.m_sectors)?;
            serializer.serialize_array_field("subTracks", &self.parent.m_subTracks)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_sectorIndex,
    m_offsetInSector,
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
            "sectorIndex" => Ok(__Field::m_sectorIndex),
            "offsetInSector" => Ok(__Field::m_offsetInSector),
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
pub(super) struct __hkpSerializedSubTrack1nInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpSerializedSubTrack1nInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSerializedSubTrack1nInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSerializedSubTrack1nInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSerializedSubTrack1nInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSerializedSubTrack1nInfoVisitor<'de> {
    type Value = hkpSerializedSubTrack1nInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpSerializedSubTrack1nInfo",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_sectorIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_offsetInSector: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_sectorIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sectorIndex",
                            ),
                        );
                    }
                    m_sectorIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_offsetInSector) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "offsetInSector",
                            ),
                        );
                    }
                    m_offsetInSector = _serde::__private::Some(
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
        let m_sectorIndex = match m_sectorIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sectorIndex"),
                );
            }
        };
        let m_offsetInSector = match m_offsetInSector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offsetInSector"),
                );
            }
        };
        _serde::__private::Ok(hkpSerializedSubTrack1nInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_sectorIndex,
            m_offsetInSector,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpSerializedTrack1nInfoVisitor::visit_as_parent(&mut __map)?;
        let mut m_sectorIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_offsetInSector: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_sectorIndex => {
                        if _serde::__private::Option::is_some(&m_sectorIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sectorIndex",
                                ),
                            );
                        }
                        m_sectorIndex = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_offsetInSector => {
                        if _serde::__private::Option::is_some(&m_offsetInSector) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "offsetInSector",
                                ),
                            );
                        }
                        m_offsetInSector = _serde::__private::Some(
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
        let m_sectorIndex = match m_sectorIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sectorIndex"),
                );
            }
        };
        let m_offsetInSector = match m_offsetInSector {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offsetInSector"),
                );
            }
        };
        _serde::__private::Ok(hkpSerializedSubTrack1nInfo {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_sectorIndex,
            m_offsetInSector,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSerializedSubTrack1nInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["sectorIndex", "offsetInSector"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSerializedSubTrack1nInfo",
                FIELDS,
                __hkpSerializedSubTrack1nInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpSerializedSubTrack1nInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
