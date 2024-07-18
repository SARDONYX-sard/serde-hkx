use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbSequence`
/// - version: `0`
/// - signature: `0x43182ca3`
/// - size: `168`(x86)/`248`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSequence<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `eventSequencedData`(ctype: `hkArray<hkbEventSequencedData*>`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_eventSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// - name: `realVariableSequencedData`(ctype: `hkArray<hkbRealVariableSequencedData*>`)
    /// - offset: ` 56`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_realVariableSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// - name: `boolVariableSequencedData`(ctype: `hkArray<hkbBoolVariableSequencedData*>`)
    /// - offset: ` 68`(x86)/`112`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_boolVariableSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// - name: `intVariableSequencedData`(ctype: `hkArray<hkbIntVariableSequencedData*>`)
    /// - offset: ` 80`(x86)/`128`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_intVariableSequencedData: Vec<Pointer>,
    /// # C++ Info
    /// - name: `enableEventId`(ctype: `hkInt32`)
    /// - offset: ` 92`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_enableEventId: i32,
    /// # C++ Info
    /// - name: `disableEventId`(ctype: `hkInt32`)
    /// - offset: ` 96`(x86)/`148`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_disableEventId: i32,
    /// # C++ Info
    /// - name: `stringData`(ctype: `struct hkbSequenceStringData*`)
    /// - offset: `100`(x86)/`152`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_stringData: Pointer,
    /// # C++ Info
    /// - name: `variableIdMap`(ctype: `void*`)
    /// - offset: `104`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_variableIdMap: Pointer,
    /// # C++ Info
    /// - name: `eventIdMap`(ctype: `void*`)
    /// - offset: `108`(x86)/`168`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_eventIdMap: Pointer,
    /// # C++ Info
    /// - name: `nextSampleEvents`(ctype: `hkArray<void>`)
    /// - offset: `112`(x86)/`176`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_nextSampleEvents: Vec<()>,
    /// # C++ Info
    /// - name: `nextSampleReals`(ctype: `hkArray<void>`)
    /// - offset: `124`(x86)/`192`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_nextSampleReals: Vec<()>,
    /// # C++ Info
    /// - name: `nextSampleBools`(ctype: `hkArray<void>`)
    /// - offset: `136`(x86)/`208`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_nextSampleBools: Vec<()>,
    /// # C++ Info
    /// - name: `nextSampleInts`(ctype: `hkArray<void>`)
    /// - offset: `148`(x86)/`224`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_nextSampleInts: Vec<()>,
    /// # C++ Info
    /// - name: `time`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_time: f32,
    /// # C++ Info
    /// - name: `isEnabled`(ctype: `hkBool`)
    /// - offset: `164`(x86)/`244`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_isEnabled: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbSequence<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSequence"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x43182ca3)
        }
    }
    impl<'a> _serde::Serialize for hkbSequence<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x43182ca3)));
            let mut serializer = __serializer
                .serialize_struct("hkbSequence", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "eventSequencedData",
                    &self.m_eventSequencedData,
                )?;
            serializer
                .serialize_array_meta_field(
                    "realVariableSequencedData",
                    &self.m_realVariableSequencedData,
                )?;
            serializer
                .serialize_array_meta_field(
                    "boolVariableSequencedData",
                    &self.m_boolVariableSequencedData,
                )?;
            serializer
                .serialize_array_meta_field(
                    "intVariableSequencedData",
                    &self.m_intVariableSequencedData,
                )?;
            serializer.serialize_field("enableEventId", &self.m_enableEventId)?;
            serializer.serialize_field("disableEventId", &self.m_disableEventId)?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer.skip_field("variableIdMap", &self.m_variableIdMap)?;
            serializer.skip_field("eventIdMap", &self.m_eventIdMap)?;
            serializer
                .skip_array_meta_field("nextSampleEvents", &self.m_nextSampleEvents)?;
            serializer
                .skip_array_meta_field("nextSampleReals", &self.m_nextSampleReals)?;
            serializer
                .skip_array_meta_field("nextSampleBools", &self.m_nextSampleBools)?;
            serializer.skip_array_meta_field("nextSampleInts", &self.m_nextSampleInts)?;
            serializer.skip_field("time", &self.m_time)?;
            serializer.skip_field("isEnabled", &self.m_isEnabled)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field(
                    "eventSequencedData",
                    &self.m_eventSequencedData,
                )?;
            serializer
                .serialize_array_field(
                    "realVariableSequencedData",
                    &self.m_realVariableSequencedData,
                )?;
            serializer
                .serialize_array_field(
                    "boolVariableSequencedData",
                    &self.m_boolVariableSequencedData,
                )?;
            serializer
                .serialize_array_field(
                    "intVariableSequencedData",
                    &self.m_intVariableSequencedData,
                )?;
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
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_eventSequencedData,
    m_realVariableSequencedData,
    m_boolVariableSequencedData,
    m_intVariableSequencedData,
    m_enableEventId,
    m_disableEventId,
    m_stringData,
    m_variableIdMap,
    m_eventIdMap,
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
            "eventSequencedData" => Ok(__Field::m_eventSequencedData),
            "realVariableSequencedData" => Ok(__Field::m_realVariableSequencedData),
            "boolVariableSequencedData" => Ok(__Field::m_boolVariableSequencedData),
            "intVariableSequencedData" => Ok(__Field::m_intVariableSequencedData),
            "enableEventId" => Ok(__Field::m_enableEventId),
            "disableEventId" => Ok(__Field::m_disableEventId),
            "stringData" => Ok(__Field::m_stringData),
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
pub(super) struct __hkbSequenceVisitor<'de> {
    marker: core::marker::PhantomData<hkbSequence<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbSequenceVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbSequence<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbSequence<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbSequenceVisitor<'de> {
    type Value = hkbSequence<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbSequence")
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
        let mut m_eventSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_realVariableSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_boolVariableSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_intVariableSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_enableEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_disableEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_variableIdMap: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_eventIdMap: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_nextSampleEvents: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_nextSampleReals: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_nextSampleBools: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_nextSampleInts: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_isEnabled: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..15usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_eventSequencedData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eventSequencedData",
                            ),
                        );
                    }
                    m_eventSequencedData = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_realVariableSequencedData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "realVariableSequencedData",
                            ),
                        );
                    }
                    m_realVariableSequencedData = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_boolVariableSequencedData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "boolVariableSequencedData",
                            ),
                        );
                    }
                    m_boolVariableSequencedData = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_intVariableSequencedData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "intVariableSequencedData",
                            ),
                        );
                    }
                    m_intVariableSequencedData = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_enableEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableEventId",
                            ),
                        );
                    }
                    m_enableEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_disableEventId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "disableEventId",
                            ),
                        );
                    }
                    m_disableEventId = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_stringData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "stringData",
                            ),
                        );
                    }
                    m_stringData = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_variableIdMap) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "variableIdMap",
                            ),
                        );
                    }
                    m_variableIdMap = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_eventIdMap) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eventIdMap",
                            ),
                        );
                    }
                    m_eventIdMap = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_nextSampleEvents) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nextSampleEvents",
                            ),
                        );
                    }
                    m_nextSampleEvents = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_nextSampleReals) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nextSampleReals",
                            ),
                        );
                    }
                    m_nextSampleReals = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_nextSampleBools) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nextSampleBools",
                            ),
                        );
                    }
                    m_nextSampleBools = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_nextSampleInts) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nextSampleInts",
                            ),
                        );
                    }
                    m_nextSampleInts = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
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
                14usize => {
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
        let m_eventSequencedData = match m_eventSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "eventSequencedData",
                    ),
                );
            }
        };
        let m_realVariableSequencedData = match m_realVariableSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "realVariableSequencedData",
                    ),
                );
            }
        };
        let m_boolVariableSequencedData = match m_boolVariableSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "boolVariableSequencedData",
                    ),
                );
            }
        };
        let m_intVariableSequencedData = match m_intVariableSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "intVariableSequencedData",
                    ),
                );
            }
        };
        let m_enableEventId = match m_enableEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enableEventId"),
                );
            }
        };
        let m_disableEventId = match m_disableEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("disableEventId"),
                );
            }
        };
        let m_stringData = match m_stringData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stringData"),
                );
            }
        };
        let m_variableIdMap = match m_variableIdMap {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("variableIdMap"),
                );
            }
        };
        let m_eventIdMap = match m_eventIdMap {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("eventIdMap"),
                );
            }
        };
        let m_nextSampleEvents = match m_nextSampleEvents {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nextSampleEvents"),
                );
            }
        };
        let m_nextSampleReals = match m_nextSampleReals {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nextSampleReals"),
                );
            }
        };
        let m_nextSampleBools = match m_nextSampleBools {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nextSampleBools"),
                );
            }
        };
        let m_nextSampleInts = match m_nextSampleInts {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nextSampleInts"),
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
                    <__A::Error as _serde::de::Error>::missing_field("isEnabled"),
                );
            }
        };
        _serde::__private::Ok(hkbSequence {
            __ptr,
            parent,
            m_eventSequencedData,
            m_realVariableSequencedData,
            m_boolVariableSequencedData,
            m_intVariableSequencedData,
            m_enableEventId,
            m_disableEventId,
            m_stringData,
            m_variableIdMap,
            m_eventIdMap,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_eventSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_realVariableSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_boolVariableSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_intVariableSequencedData: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_enableEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_disableEventId: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..7usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_eventSequencedData => {
                        if _serde::__private::Option::is_some(&m_eventSequencedData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "eventSequencedData",
                                ),
                            );
                        }
                        m_eventSequencedData = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_realVariableSequencedData => {
                        if _serde::__private::Option::is_some(
                            &m_realVariableSequencedData,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "realVariableSequencedData",
                                ),
                            );
                        }
                        m_realVariableSequencedData = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_boolVariableSequencedData => {
                        if _serde::__private::Option::is_some(
                            &m_boolVariableSequencedData,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "boolVariableSequencedData",
                                ),
                            );
                        }
                        m_boolVariableSequencedData = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_intVariableSequencedData => {
                        if _serde::__private::Option::is_some(
                            &m_intVariableSequencedData,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "intVariableSequencedData",
                                ),
                            );
                        }
                        m_intVariableSequencedData = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_enableEventId => {
                        if _serde::__private::Option::is_some(&m_enableEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enableEventId",
                                ),
                            );
                        }
                        m_enableEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_disableEventId => {
                        if _serde::__private::Option::is_some(&m_disableEventId) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "disableEventId",
                                ),
                            );
                        }
                        m_disableEventId = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_stringData => {
                        if _serde::__private::Option::is_some(&m_stringData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "stringData",
                                ),
                            );
                        }
                        m_stringData = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
        let m_eventSequencedData = match m_eventSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "eventSequencedData",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_realVariableSequencedData = match m_realVariableSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "realVariableSequencedData",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_boolVariableSequencedData = match m_boolVariableSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "boolVariableSequencedData",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_intVariableSequencedData = match m_intVariableSequencedData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "intVariableSequencedData",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_enableEventId = match m_enableEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enableEventId"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_disableEventId = match m_disableEventId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("disableEventId"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_stringData = match m_stringData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("stringData"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkbSequence {
            __ptr,
            parent,
            m_eventSequencedData,
            m_realVariableSequencedData,
            m_boolVariableSequencedData,
            m_intVariableSequencedData,
            m_enableEventId,
            m_disableEventId,
            m_stringData,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbSequence<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "eventSequencedData",
                "realVariableSequencedData",
                "boolVariableSequencedData",
                "intVariableSequencedData",
                "enableEventId",
                "disableEventId",
                "stringData",
                "variableIdMap",
                "eventIdMap",
                "nextSampleEvents",
                "nextSampleReals",
                "nextSampleBools",
                "nextSampleInts",
                "time",
                "isEnabled",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbSequence",
                FIELDS,
                __hkbSequenceVisitor {
                    marker: _serde::__private::PhantomData::<hkbSequence>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
