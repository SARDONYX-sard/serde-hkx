use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedTrack1nInfo`
/// -         version: `0`
/// -       signature: `0xf12d48d9`
/// -          size:  24(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedTrack1nInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `sectors`(ctype: `hkArray<hkpAgent1nSector*>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_sectors: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `subTracks`(ctype: `hkArray<hkpSerializedSubTrack1nInfo*>`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_subTracks: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSerializedTrack1nInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSerializedTrack1nInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf12d48d9)
        }
    }
    impl _serde::Serialize for hkpSerializedTrack1nInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf12d48d9)));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedTrack1nInfo", class_meta)?;
            serializer.serialize_array_meta_field("sectors", &self.m_sectors)?;
            serializer.serialize_array_meta_field("subTracks", &self.m_subTracks)?;
            serializer.serialize_array_field("sectors", &self.m_sectors)?;
            serializer.serialize_array_field("subTracks", &self.m_subTracks)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_sectors,
    m_subTracks,
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
            "sectors" => Ok(__Field::m_sectors),
            "subTracks" => Ok(__Field::m_subTracks),
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
pub(super) struct __hkpSerializedTrack1nInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpSerializedTrack1nInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSerializedTrack1nInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSerializedTrack1nInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSerializedTrack1nInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSerializedTrack1nInfoVisitor<'de> {
    type Value = hkpSerializedTrack1nInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpSerializedTrack1nInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_sectors: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_subTracks: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_sectors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("sectors"),
                        );
                    }
                    m_sectors = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_subTracks) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "subTracks",
                            ),
                        );
                    }
                    m_subTracks = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_sectors = match m_sectors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sectors"),
                );
            }
        };
        let m_subTracks = match m_subTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("subTracks"),
                );
            }
        };
        _serde::__private::Ok(hkpSerializedTrack1nInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_sectors,
            m_subTracks,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_sectors: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_subTracks: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_sectors => {
                        if _serde::__private::Option::is_some(&m_sectors) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sectors",
                                ),
                            );
                        }
                        m_sectors = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_subTracks => {
                        if _serde::__private::Option::is_some(&m_subTracks) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "subTracks",
                                ),
                            );
                        }
                        m_subTracks = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_sectors = match m_sectors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sectors"),
                );
            }
        };
        let m_subTracks = match m_subTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("subTracks"),
                );
            }
        };
        _serde::__private::Ok(hkpSerializedTrack1nInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_sectors,
            m_subTracks,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSerializedTrack1nInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["sectors", "subTracks"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSerializedTrack1nInfo",
                FIELDS,
                __hkpSerializedTrack1nInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkpSerializedTrack1nInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
