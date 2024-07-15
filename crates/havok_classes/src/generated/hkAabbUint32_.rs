use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkAabbUint32`
/// - version: `0`
/// - signature: `0x11e7c11`
/// - size: ` 32`(x86)/` 32`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkAabbUint32 {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `min`(ctype: `hkUint32[3]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    /// - flags: `ALIGN_16`
    pub m_min: [u32; 3usize],
    /// # C++ Info
    /// - name: `expansionMin`(ctype: `hkUint8[3]`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  3`(x86)/`  3`(x86_64)
    pub m_expansionMin: [u8; 3usize],
    /// # C++ Info
    /// - name: `expansionShift`(ctype: `hkUint8`)
    /// - offset: ` 15`(x86)/` 15`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_expansionShift: u8,
    /// # C++ Info
    /// - name: `max`(ctype: `hkUint32[3]`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_max: [u32; 3usize],
    /// # C++ Info
    /// - name: `expansionMax`(ctype: `hkUint8[3]`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  3`(x86)/`  3`(x86_64)
    pub m_expansionMax: [u8; 3usize],
    /// # C++ Info
    /// - name: `shapeKeyByte`(ctype: `hkUint8`)
    /// - offset: ` 31`(x86)/` 31`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_shapeKeyByte: u8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkAabbUint32 {
        #[inline]
        fn name(&self) -> &'static str {
            "hkAabbUint32"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x11e7c11)
        }
    }
    impl _serde::Serialize for hkAabbUint32 {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x11e7c11)));
            let mut serializer = __serializer
                .serialize_struct("hkAabbUint32", class_meta)?;
            serializer.serialize_field("min", &self.m_min.as_slice())?;
            serializer.serialize_field("expansionMin", &self.m_expansionMin.as_slice())?;
            serializer.serialize_field("expansionShift", &self.m_expansionShift)?;
            serializer.serialize_field("max", &self.m_max.as_slice())?;
            serializer.serialize_field("expansionMax", &self.m_expansionMax.as_slice())?;
            serializer.serialize_field("shapeKeyByte", &self.m_shapeKeyByte)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_min,
    m_expansionMin,
    m_expansionShift,
    m_max,
    m_expansionMax,
    m_shapeKeyByte,
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
            "min" => Ok(__Field::m_min),
            "expansionMin" => Ok(__Field::m_expansionMin),
            "expansionShift" => Ok(__Field::m_expansionShift),
            "max" => Ok(__Field::m_max),
            "expansionMax" => Ok(__Field::m_expansionMax),
            "shapeKeyByte" => Ok(__Field::m_shapeKeyByte),
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
pub(super) struct __hkAabbUint32Visitor<'de> {
    marker: core::marker::PhantomData<hkAabbUint32>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkAabbUint32Visitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkAabbUint32, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkAabbUint32>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkAabbUint32Visitor<'de> {
    type Value = hkAabbUint32;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkAabbUint32")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_min: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
        let mut m_expansionMin: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
        let mut m_expansionShift: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_max: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
        let mut m_expansionMax: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
        let mut m_shapeKeyByte: _serde::__private::Option<u8> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_min) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("min"),
                        );
                    }
                    m_min = _serde::__private::Some(
                        match __A::next_value::<[u32; 3usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_expansionMin) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "expansionMin",
                            ),
                        );
                    }
                    m_expansionMin = _serde::__private::Some(
                        match __A::next_value::<[u8; 3usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_expansionShift) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "expansionShift",
                            ),
                        );
                    }
                    m_expansionShift = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_max) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("max"),
                        );
                    }
                    m_max = _serde::__private::Some(
                        match __A::next_value::<[u32; 3usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_expansionMax) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "expansionMax",
                            ),
                        );
                    }
                    m_expansionMax = _serde::__private::Some(
                        match __A::next_value::<[u8; 3usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_shapeKeyByte) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "shapeKeyByte",
                            ),
                        );
                    }
                    m_shapeKeyByte = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
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
        let m_min = match m_min {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("min"),
                );
            }
        };
        let m_expansionMin = match m_expansionMin {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("expansionMin"),
                );
            }
        };
        let m_expansionShift = match m_expansionShift {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("expansionShift"),
                );
            }
        };
        let m_max = match m_max {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("max"),
                );
            }
        };
        let m_expansionMax = match m_expansionMax {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("expansionMax"),
                );
            }
        };
        let m_shapeKeyByte = match m_shapeKeyByte {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapeKeyByte"),
                );
            }
        };
        _serde::__private::Ok(hkAabbUint32 {
            __ptr,
            m_min,
            m_expansionMin,
            m_expansionShift,
            m_max,
            m_expansionMax,
            m_shapeKeyByte,
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
        let mut m_min: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
        let mut m_expansionMin: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
        let mut m_expansionShift: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_max: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
        let mut m_expansionMax: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
        let mut m_shapeKeyByte: _serde::__private::Option<u8> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_min => {
                        if _serde::__private::Option::is_some(&m_min) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("min"),
                            );
                        }
                        m_min = _serde::__private::Some(
                            match __A::next_value::<[u32; 3usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_expansionMin => {
                        if _serde::__private::Option::is_some(&m_expansionMin) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "expansionMin",
                                ),
                            );
                        }
                        m_expansionMin = _serde::__private::Some(
                            match __A::next_value::<[u8; 3usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_expansionShift => {
                        if _serde::__private::Option::is_some(&m_expansionShift) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "expansionShift",
                                ),
                            );
                        }
                        m_expansionShift = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_max => {
                        if _serde::__private::Option::is_some(&m_max) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("max"),
                            );
                        }
                        m_max = _serde::__private::Some(
                            match __A::next_value::<[u32; 3usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_expansionMax => {
                        if _serde::__private::Option::is_some(&m_expansionMax) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "expansionMax",
                                ),
                            );
                        }
                        m_expansionMax = _serde::__private::Some(
                            match __A::next_value::<[u8; 3usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_shapeKeyByte => {
                        if _serde::__private::Option::is_some(&m_shapeKeyByte) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "shapeKeyByte",
                                ),
                            );
                        }
                        m_shapeKeyByte = _serde::__private::Some(
                            match __A::next_value::<u8>(&mut __map) {
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
        let m_min = match m_min {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("min"),
                );
            }
        };
        let m_expansionMin = match m_expansionMin {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("expansionMin"),
                );
            }
        };
        let m_expansionShift = match m_expansionShift {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("expansionShift"),
                );
            }
        };
        let m_max = match m_max {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("max"),
                );
            }
        };
        let m_expansionMax = match m_expansionMax {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("expansionMax"),
                );
            }
        };
        let m_shapeKeyByte = match m_shapeKeyByte {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapeKeyByte"),
                );
            }
        };
        _serde::__private::Ok(hkAabbUint32 {
            __ptr,
            m_min,
            m_expansionMin,
            m_expansionShift,
            m_max,
            m_expansionMax,
            m_shapeKeyByte,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkAabbUint32 {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "min",
                "expansionMin",
                "expansionShift",
                "max",
                "expansionMax",
                "shapeKeyByte",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkAabbUint32",
                FIELDS,
                __hkAabbUint32Visitor {
                    marker: _serde::__private::PhantomData::<hkAabbUint32>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
