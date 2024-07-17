use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpAgent1nSector`
/// - version: `0`
/// - signature: `0x626e55a`
/// - size: `512`(x86)/`512`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpAgent1nSector {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `bytesAllocated`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_bytesAllocated: u32,
    /// # C++ Info
    /// - name: `pad0`(ctype: `hkUint32`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pad0: u32,
    /// # C++ Info
    /// - name: `pad1`(ctype: `hkUint32`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pad1: u32,
    /// # C++ Info
    /// - name: `pad2`(ctype: `hkUint32`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pad2: u32,
    /// # C++ Info
    /// - name: `data`(ctype: `hkUint8[496]`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `496`(x86)/`496`(x86_64)
    #[cfg_attr(feature = "serde", serde_as(as = "[_; 496]"))]
    #[educe(Default = [0;496usize])]
    pub m_data: [u8; 496usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpAgent1nSector {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpAgent1nSector"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x626e55a)
        }
    }
    impl _serde::Serialize for hkpAgent1nSector {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x626e55a)));
            let mut serializer = __serializer
                .serialize_struct("hkpAgent1nSector", class_meta)?;
            serializer.serialize_field("bytesAllocated", &self.m_bytesAllocated)?;
            serializer.serialize_field("pad0", &self.m_pad0)?;
            serializer.serialize_field("pad1", &self.m_pad1)?;
            serializer.serialize_field("pad2", &self.m_pad2)?;
            serializer.serialize_fixed_array_field("data", self.m_data.as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_bytesAllocated,
    m_pad0,
    m_pad1,
    m_pad2,
    m_data,
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
            "bytesAllocated" => Ok(__Field::m_bytesAllocated),
            "pad0" => Ok(__Field::m_pad0),
            "pad1" => Ok(__Field::m_pad1),
            "pad2" => Ok(__Field::m_pad2),
            "data" => Ok(__Field::m_data),
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
pub(super) struct __hkpAgent1nSectorVisitor<'de> {
    marker: core::marker::PhantomData<hkpAgent1nSector>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpAgent1nSectorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpAgent1nSector, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpAgent1nSector>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpAgent1nSectorVisitor<'de> {
    type Value = hkpAgent1nSector;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpAgent1nSector")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_bytesAllocated: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_pad0: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_pad1: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_pad2: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_data: _serde::__private::Option<[u8; 496usize]> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_bytesAllocated) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bytesAllocated",
                            ),
                        );
                    }
                    m_bytesAllocated = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_pad0) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pad0"),
                        );
                    }
                    m_pad0 = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_pad1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pad1"),
                        );
                    }
                    m_pad1 = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_pad2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pad2"),
                        );
                    }
                    m_pad2 = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                        );
                    }
                    m_data = _serde::__private::Some(
                        match __A::next_value::<[u8; 496usize]>(&mut __map) {
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
        let m_bytesAllocated = match m_bytesAllocated {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bytesAllocated"),
                );
            }
        };
        let m_pad0 = match m_pad0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad0"),
                );
            }
        };
        let m_pad1 = match m_pad1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad1"),
                );
            }
        };
        let m_pad2 = match m_pad2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad2"),
                );
            }
        };
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        _serde::__private::Ok(hkpAgent1nSector {
            __ptr,
            m_bytesAllocated,
            m_pad0,
            m_pad1,
            m_pad2,
            m_data,
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
        let mut m_bytesAllocated: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_pad0: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_pad1: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_pad2: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_data: _serde::__private::Option<[u8; 496usize]> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_bytesAllocated => {
                        if _serde::__private::Option::is_some(&m_bytesAllocated) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bytesAllocated",
                                ),
                            );
                        }
                        m_bytesAllocated = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pad0 => {
                        if _serde::__private::Option::is_some(&m_pad0) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("pad0"),
                            );
                        }
                        m_pad0 = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pad1 => {
                        if _serde::__private::Option::is_some(&m_pad1) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("pad1"),
                            );
                        }
                        m_pad1 = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pad2 => {
                        if _serde::__private::Option::is_some(&m_pad2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("pad2"),
                            );
                        }
                        m_pad2 = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_data => {
                        if _serde::__private::Option::is_some(&m_data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                            );
                        }
                        m_data = _serde::__private::Some(
                            match __A::next_value::<[u8; 496usize]>(&mut __map) {
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
        let m_bytesAllocated = match m_bytesAllocated {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bytesAllocated"),
                );
            }
        };
        let m_pad0 = match m_pad0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad0"),
                );
            }
        };
        let m_pad1 = match m_pad1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad1"),
                );
            }
        };
        let m_pad2 = match m_pad2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad2"),
                );
            }
        };
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        _serde::__private::Ok(hkpAgent1nSector {
            __ptr,
            m_bytesAllocated,
            m_pad0,
            m_pad1,
            m_pad2,
            m_data,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpAgent1nSector {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["bytesAllocated", "pad0", "pad1", "pad2", "data"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpAgent1nSector",
                FIELDS,
                __hkpAgent1nSectorVisitor {
                    marker: _serde::__private::PhantomData::<hkpAgent1nSector>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
