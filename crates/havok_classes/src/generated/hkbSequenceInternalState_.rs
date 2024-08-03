use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbSequenceInternalState`
/// - version: `0`
/// - signature: `0x419b9a05`
/// - size: ` 64`(x86)/` 88`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSequenceInternalState {
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
    /// - name: `nextSampleEvents`(ctype: `hkArray<hkInt32>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nextSampleEvents: Vec<i32>,
    /// # C++ Info
    /// - name: `nextSampleReals`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nextSampleReals: Vec<i32>,
    /// # C++ Info
    /// - name: `nextSampleBools`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nextSampleBools: Vec<i32>,
    /// # C++ Info
    /// - name: `nextSampleInts`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_nextSampleInts: Vec<i32>,
    /// # C++ Info
    /// - name: `time`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_time: f32,
    /// # C++ Info
    /// - name: `isEnabled`(ctype: `hkBool`)
    /// - offset: ` 60`(x86)/` 84`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isEnabled: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbSequenceInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSequenceInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x419b9a05)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkbSequenceInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x419b9a05)));
            let mut serializer = __serializer
                .serialize_struct("hkbSequenceInternalState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "nextSampleEvents",
                    &self.m_nextSampleEvents,
                )?;
            serializer
                .serialize_array_meta_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .serialize_array_meta_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer
                .serialize_array_meta_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.serialize_field("time", &self.m_time)?;
            serializer.serialize_field("isEnabled", &self.m_isEnabled)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_array_field("nextSampleEvents", &self.m_nextSampleEvents)?;
            serializer
                .serialize_array_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .serialize_array_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer.serialize_array_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbSequenceInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_nextSampleEvents,
                m_nextSampleReals,
                m_nextSampleBools,
                m_nextSampleInts,
                m_time,
                m_isEnabled,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "nextSampleEvents" => Ok(__Field::m_nextSampleEvents),
                        "nextSampleReals" => Ok(__Field::m_nextSampleReals),
                        "nextSampleBools" => Ok(__Field::m_nextSampleBools),
                        "nextSampleInts" => Ok(__Field::m_nextSampleInts),
                        "time" => Ok(__Field::m_time),
                        "isEnabled" => Ok(__Field::m_isEnabled),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkbSequenceInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbSequenceInternalState>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbSequenceInternalStateVisitor<'de> {
                type Value = hkbSequenceInternalState;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbSequenceInternalState",
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
                    let mut m_nextSampleEvents: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_nextSampleReals: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_nextSampleBools: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_nextSampleInts: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_nextSampleEvents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleEvents",
                                        ),
                                    );
                                }
                                m_nextSampleEvents = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_nextSampleReals) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleReals",
                                        ),
                                    );
                                }
                                m_nextSampleReals = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_nextSampleBools) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleBools",
                                        ),
                                    );
                                }
                                m_nextSampleBools = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_nextSampleInts) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleInts",
                                        ),
                                    );
                                }
                                m_nextSampleInts = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_time) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("time"),
                                    );
                                }
                                m_time = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_isEnabled) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isEnabled",
                                        ),
                                    );
                                }
                                m_isEnabled = _serde::__private::Some(
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
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    let m_nextSampleEvents = match m_nextSampleEvents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleEvents",
                                ),
                            );
                        }
                    };
                    let m_nextSampleReals = match m_nextSampleReals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleReals",
                                ),
                            );
                        }
                    };
                    let m_nextSampleBools = match m_nextSampleBools {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleBools",
                                ),
                            );
                        }
                    };
                    let m_nextSampleInts = match m_nextSampleInts {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleInts",
                                ),
                            );
                        }
                    };
                    let m_time = match m_time {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("time"),
                            );
                        }
                    };
                    let m_isEnabled = match m_isEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isEnabled",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbSequenceInternalState {
                        __ptr,
                        parent,
                        m_nextSampleEvents,
                        m_nextSampleReals,
                        m_nextSampleBools,
                        m_nextSampleInts,
                        m_time,
                        m_isEnabled,
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
                    let mut m_nextSampleEvents: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_nextSampleReals: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_nextSampleBools: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_nextSampleInts: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isEnabled: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_nextSampleEvents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nextSampleEvents) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleEvents",
                                        ),
                                    );
                                }
                                m_nextSampleEvents = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nextSampleReals => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nextSampleReals) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleReals",
                                        ),
                                    );
                                }
                                m_nextSampleReals = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nextSampleBools => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nextSampleBools) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleBools",
                                        ),
                                    );
                                }
                                m_nextSampleBools = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nextSampleInts => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nextSampleInts) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextSampleInts",
                                        ),
                                    );
                                }
                                m_nextSampleInts = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_time => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_time) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("time"),
                                    );
                                }
                                m_time = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isEnabled => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isEnabled) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isEnabled",
                                        ),
                                    );
                                }
                                m_isEnabled = _serde::__private::Some(
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
                    let m_nextSampleEvents = match m_nextSampleEvents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleEvents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nextSampleReals = match m_nextSampleReals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleReals",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nextSampleBools = match m_nextSampleBools {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleBools",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nextSampleInts = match m_nextSampleInts {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextSampleInts",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_time = match m_time {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("time"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isEnabled = match m_isEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isEnabled",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbSequenceInternalState {
                        __ptr,
                        parent,
                        m_nextSampleEvents,
                        m_nextSampleReals,
                        m_nextSampleBools,
                        m_nextSampleInts,
                        m_time,
                        m_isEnabled,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "nextSampleEvents",
                "nextSampleReals",
                "nextSampleBools",
                "nextSampleInts",
                "time",
                "isEnabled",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbSequenceInternalState",
                FIELDS,
                __hkbSequenceInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<hkbSequenceInternalState>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
