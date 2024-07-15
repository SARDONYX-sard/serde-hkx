use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbGeneratorSyncInfo`
/// - version: `0`
/// - signature: `0xa3c341f8`
/// - size: ` 80`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGeneratorSyncInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `syncPoints`(ctype: `struct hkbGeneratorSyncInfoSyncPoint[8]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  8`(x86)/` 64`(x86_64)
    pub m_syncPoints: [hkbGeneratorSyncInfoSyncPoint; 8usize],
    /// # C++ Info
    /// - name: `baseFrequency`(ctype: `hkReal`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_baseFrequency: f32,
    /// # C++ Info
    /// - name: `localTime`(ctype: `hkReal`)
    /// - offset: ` 68`(x86)/` 68`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_localTime: f32,
    /// # C++ Info
    /// - name: `playbackSpeed`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_playbackSpeed: f32,
    /// # C++ Info
    /// - name: `numSyncPoints`(ctype: `hkInt8`)
    /// - offset: ` 76`(x86)/` 76`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_numSyncPoints: i8,
    /// # C++ Info
    /// - name: `isCyclic`(ctype: `hkBool`)
    /// - offset: ` 77`(x86)/` 77`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isCyclic: bool,
    /// # C++ Info
    /// - name: `isMirrored`(ctype: `hkBool`)
    /// - offset: ` 78`(x86)/` 78`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isMirrored: bool,
    /// # C++ Info
    /// - name: `isAdditive`(ctype: `hkBool`)
    /// - offset: ` 79`(x86)/` 79`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isAdditive: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbGeneratorSyncInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbGeneratorSyncInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa3c341f8)
        }
    }
    impl _serde::Serialize for hkbGeneratorSyncInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa3c341f8)));
            let mut serializer = __serializer
                .serialize_struct("hkbGeneratorSyncInfo", class_meta)?;
            serializer.serialize_field("syncPoints", &self.m_syncPoints.as_slice())?;
            serializer.pad_field([0u8; 56usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("baseFrequency", &self.m_baseFrequency)?;
            serializer.serialize_field("localTime", &self.m_localTime)?;
            serializer.serialize_field("playbackSpeed", &self.m_playbackSpeed)?;
            serializer.serialize_field("numSyncPoints", &self.m_numSyncPoints)?;
            serializer.serialize_field("isCyclic", &self.m_isCyclic)?;
            serializer.serialize_field("isMirrored", &self.m_isMirrored)?;
            serializer.serialize_field("isAdditive", &self.m_isAdditive)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_syncPoints,
    m_baseFrequency,
    m_localTime,
    m_playbackSpeed,
    m_numSyncPoints,
    m_isCyclic,
    m_isMirrored,
    m_isAdditive,
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
            "syncPoints" => Ok(__Field::m_syncPoints),
            "baseFrequency" => Ok(__Field::m_baseFrequency),
            "localTime" => Ok(__Field::m_localTime),
            "playbackSpeed" => Ok(__Field::m_playbackSpeed),
            "numSyncPoints" => Ok(__Field::m_numSyncPoints),
            "isCyclic" => Ok(__Field::m_isCyclic),
            "isMirrored" => Ok(__Field::m_isMirrored),
            "isAdditive" => Ok(__Field::m_isAdditive),
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
pub(super) struct __hkbGeneratorSyncInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbGeneratorSyncInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbGeneratorSyncInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbGeneratorSyncInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbGeneratorSyncInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbGeneratorSyncInfoVisitor<'de> {
    type Value = hkbGeneratorSyncInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbGeneratorSyncInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_syncPoints: _serde::__private::Option<
            [hkbGeneratorSyncInfoSyncPoint; 8usize],
        > = _serde::__private::None;
        let mut m_baseFrequency: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_localTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_playbackSpeed: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numSyncPoints: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_isCyclic: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isMirrored: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isAdditive: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_syncPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "syncPoints",
                            ),
                        );
                    }
                    m_syncPoints = _serde::__private::Some(
                        match __A::next_value::<
                            [hkbGeneratorSyncInfoSyncPoint; 8usize],
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_baseFrequency) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "baseFrequency",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 56usize, 0usize)?;
                    m_baseFrequency = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_localTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localTime",
                            ),
                        );
                    }
                    m_localTime = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_playbackSpeed) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "playbackSpeed",
                            ),
                        );
                    }
                    m_playbackSpeed = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_numSyncPoints) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numSyncPoints",
                            ),
                        );
                    }
                    m_numSyncPoints = _serde::__private::Some(
                        match __A::next_value::<i8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_isCyclic) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isCyclic",
                            ),
                        );
                    }
                    m_isCyclic = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_isMirrored) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isMirrored",
                            ),
                        );
                    }
                    m_isMirrored = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_isAdditive) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isAdditive",
                            ),
                        );
                    }
                    m_isAdditive = _serde::__private::Some(
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
        let m_syncPoints = match m_syncPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("syncPoints"),
                );
            }
        };
        let m_baseFrequency = match m_baseFrequency {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("baseFrequency"),
                );
            }
        };
        let m_localTime = match m_localTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localTime"),
                );
            }
        };
        let m_playbackSpeed = match m_playbackSpeed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("playbackSpeed"),
                );
            }
        };
        let m_numSyncPoints = match m_numSyncPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numSyncPoints"),
                );
            }
        };
        let m_isCyclic = match m_isCyclic {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isCyclic"),
                );
            }
        };
        let m_isMirrored = match m_isMirrored {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isMirrored"),
                );
            }
        };
        let m_isAdditive = match m_isAdditive {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isAdditive"),
                );
            }
        };
        _serde::__private::Ok(hkbGeneratorSyncInfo {
            __ptr,
            m_syncPoints,
            m_baseFrequency,
            m_localTime,
            m_playbackSpeed,
            m_numSyncPoints,
            m_isCyclic,
            m_isMirrored,
            m_isAdditive,
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
        let mut m_syncPoints: _serde::__private::Option<
            [hkbGeneratorSyncInfoSyncPoint; 8usize],
        > = _serde::__private::None;
        let mut m_baseFrequency: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_localTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_playbackSpeed: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_numSyncPoints: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_isCyclic: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isMirrored: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isAdditive: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_syncPoints => {
                        if _serde::__private::Option::is_some(&m_syncPoints) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "syncPoints",
                                ),
                            );
                        }
                        m_syncPoints = _serde::__private::Some(
                            match __A::next_value::<
                                [hkbGeneratorSyncInfoSyncPoint; 8usize],
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_baseFrequency => {
                        if _serde::__private::Option::is_some(&m_baseFrequency) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "baseFrequency",
                                ),
                            );
                        }
                        m_baseFrequency = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_localTime => {
                        if _serde::__private::Option::is_some(&m_localTime) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "localTime",
                                ),
                            );
                        }
                        m_localTime = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_playbackSpeed => {
                        if _serde::__private::Option::is_some(&m_playbackSpeed) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "playbackSpeed",
                                ),
                            );
                        }
                        m_playbackSpeed = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numSyncPoints => {
                        if _serde::__private::Option::is_some(&m_numSyncPoints) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numSyncPoints",
                                ),
                            );
                        }
                        m_numSyncPoints = _serde::__private::Some(
                            match __A::next_value::<i8>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isCyclic => {
                        if _serde::__private::Option::is_some(&m_isCyclic) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isCyclic",
                                ),
                            );
                        }
                        m_isCyclic = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isMirrored => {
                        if _serde::__private::Option::is_some(&m_isMirrored) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isMirrored",
                                ),
                            );
                        }
                        m_isMirrored = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isAdditive => {
                        if _serde::__private::Option::is_some(&m_isAdditive) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isAdditive",
                                ),
                            );
                        }
                        m_isAdditive = _serde::__private::Some(
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
        let m_syncPoints = match m_syncPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("syncPoints"),
                );
            }
        };
        let m_baseFrequency = match m_baseFrequency {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("baseFrequency"),
                );
            }
        };
        let m_localTime = match m_localTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localTime"),
                );
            }
        };
        let m_playbackSpeed = match m_playbackSpeed {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("playbackSpeed"),
                );
            }
        };
        let m_numSyncPoints = match m_numSyncPoints {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numSyncPoints"),
                );
            }
        };
        let m_isCyclic = match m_isCyclic {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isCyclic"),
                );
            }
        };
        let m_isMirrored = match m_isMirrored {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isMirrored"),
                );
            }
        };
        let m_isAdditive = match m_isAdditive {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isAdditive"),
                );
            }
        };
        _serde::__private::Ok(hkbGeneratorSyncInfo {
            __ptr,
            m_syncPoints,
            m_baseFrequency,
            m_localTime,
            m_playbackSpeed,
            m_numSyncPoints,
            m_isCyclic,
            m_isMirrored,
            m_isAdditive,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbGeneratorSyncInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "syncPoints",
                "baseFrequency",
                "localTime",
                "playbackSpeed",
                "numSyncPoints",
                "isCyclic",
                "isMirrored",
                "isAdditive",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbGeneratorSyncInfo",
                FIELDS,
                __hkbGeneratorSyncInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbGeneratorSyncInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
