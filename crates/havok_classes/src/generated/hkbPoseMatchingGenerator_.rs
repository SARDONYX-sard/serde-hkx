use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbPoseMatchingGenerator`
/// -         version: `2`
/// -       signature: `0x29e271b4`
/// -          size: 208(x86)/240(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoseMatchingGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbBlenderGenerator<'a>,
    /// # C++ Info
    /// -          name: `worldFromModelRotation`(ctype: `hkQuaternion`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_worldFromModelRotation: Quaternion,
    /// # C++ Info
    /// -          name: `blendSpeed`(ctype: `hkReal`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_blendSpeed: f32,
    /// # C++ Info
    /// -          name: `minSpeedToSwitch`(ctype: `hkReal`)
    /// -        offset: 148(x86)/180(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minSpeedToSwitch: f32,
    /// # C++ Info
    /// -          name: `minSwitchTimeNoError`(ctype: `hkReal`)
    /// -        offset: 152(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minSwitchTimeNoError: f32,
    /// # C++ Info
    /// -          name: `minSwitchTimeFullError`(ctype: `hkReal`)
    /// -        offset: 156(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minSwitchTimeFullError: f32,
    /// # C++ Info
    /// -          name: `startPlayingEventId`(ctype: `hkInt32`)
    /// -        offset: 160(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startPlayingEventId: i32,
    /// # C++ Info
    /// -          name: `startMatchingEventId`(ctype: `hkInt32`)
    /// -        offset: 164(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startMatchingEventId: i32,
    /// # C++ Info
    /// -          name: `rootBoneIndex`(ctype: `hkInt16`)
    /// -        offset: 168(x86)/200(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_rootBoneIndex: i16,
    /// # C++ Info
    /// -          name: `otherBoneIndex`(ctype: `hkInt16`)
    /// -        offset: 170(x86)/202(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_otherBoneIndex: i16,
    /// # C++ Info
    /// -          name: `anotherBoneIndex`(ctype: `hkInt16`)
    /// -        offset: 172(x86)/204(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_anotherBoneIndex: i16,
    /// # C++ Info
    /// -          name: `pelvisIndex`(ctype: `hkInt16`)
    /// -        offset: 174(x86)/206(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_pelvisIndex: i16,
    /// # C++ Info
    /// -          name: `mode`(ctype: `enum Mode`)
    /// -        offset: 176(x86)/208(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_mode: Mode,
    /// # C++ Info
    /// -          name: `currentMatch`(ctype: `hkInt32`)
    /// -        offset: 180(x86)/212(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_currentMatch: i32,
    /// # C++ Info
    /// -          name: `bestMatch`(ctype: `hkInt32`)
    /// -        offset: 184(x86)/216(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bestMatch: i32,
    /// # C++ Info
    /// -          name: `timeSinceBetterMatch`(ctype: `hkReal`)
    /// -        offset: 188(x86)/220(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeSinceBetterMatch: f32,
    /// # C++ Info
    /// -          name: `error`(ctype: `hkReal`)
    /// -        offset: 192(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_error: f32,
    /// # C++ Info
    /// -          name: `resetCurrentMatchLocalTime`(ctype: `hkBool`)
    /// -        offset: 196(x86)/228(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_resetCurrentMatchLocalTime: bool,
    /// # C++ Info
    /// -          name: `poseMatchingUtility`(ctype: `void*`)
    /// -        offset: 200(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_poseMatchingUtility: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbPoseMatchingGenerator<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbPoseMatchingGenerator"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(702706100u32)
        }
    }
    impl<'a> __serde::Serialize for hkbPoseMatchingGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbPoseMatchingGenerator", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer
                .skip_field("padNode", &self.parent.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "referencePoseWeightThreshold",
                    &self.parent.m_referencePoseWeightThreshold,
                )?;
            serializer.serialize_field("blendParameter", &self.parent.m_blendParameter)?;
            serializer
                .serialize_field(
                    "minCyclicBlendParameter",
                    &self.parent.m_minCyclicBlendParameter,
                )?;
            serializer
                .serialize_field(
                    "maxCyclicBlendParameter",
                    &self.parent.m_maxCyclicBlendParameter,
                )?;
            serializer
                .serialize_field(
                    "indexOfSyncMasterChild",
                    &self.parent.m_indexOfSyncMasterChild,
                )?;
            serializer.serialize_field("flags", &self.parent.m_flags)?;
            serializer
                .serialize_field("subtractLastChild", &self.parent.m_subtractLastChild)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_meta_field("children", &self.parent.m_children)?;
            serializer
                .skip_array_meta_field(
                    "childrenInternalStates",
                    &self.parent.m_childrenInternalStates,
                )?;
            serializer
                .skip_array_meta_field("sortedChildren", &self.parent.m_sortedChildren)?;
            serializer
                .skip_field("endIntervalWeight", &self.parent.m_endIntervalWeight)?;
            serializer
                .skip_field("numActiveChildren", &self.parent.m_numActiveChildren)?;
            serializer
                .skip_field("beginIntervalIndex", &self.parent.m_beginIntervalIndex)?;
            serializer.skip_field("endIntervalIndex", &self.parent.m_endIntervalIndex)?;
            serializer.skip_field("initSync", &self.parent.m_initSync)?;
            serializer
                .skip_field("doSubtractiveBlend", &self.parent.m_doSubtractiveBlend)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field(
                    "worldFromModelRotation",
                    &self.m_worldFromModelRotation,
                )?;
            serializer.serialize_field("blendSpeed", &self.m_blendSpeed)?;
            serializer.serialize_field("minSpeedToSwitch", &self.m_minSpeedToSwitch)?;
            serializer
                .serialize_field("minSwitchTimeNoError", &self.m_minSwitchTimeNoError)?;
            serializer
                .serialize_field(
                    "minSwitchTimeFullError",
                    &self.m_minSwitchTimeFullError,
                )?;
            serializer
                .serialize_field("startPlayingEventId", &self.m_startPlayingEventId)?;
            serializer
                .serialize_field("startMatchingEventId", &self.m_startMatchingEventId)?;
            serializer.serialize_field("rootBoneIndex", &self.m_rootBoneIndex)?;
            serializer.serialize_field("otherBoneIndex", &self.m_otherBoneIndex)?;
            serializer.serialize_field("anotherBoneIndex", &self.m_anotherBoneIndex)?;
            serializer.serialize_field("pelvisIndex", &self.m_pelvisIndex)?;
            serializer.serialize_field("mode", &self.m_mode)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("currentMatch", &self.m_currentMatch)?;
            serializer.skip_field("bestMatch", &self.m_bestMatch)?;
            serializer.skip_field("timeSinceBetterMatch", &self.m_timeSinceBetterMatch)?;
            serializer.skip_field("error", &self.m_error)?;
            serializer
                .skip_field(
                    "resetCurrentMatchLocalTime",
                    &self.m_resetCurrentMatchLocalTime,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("poseMatchingUtility", &self.m_poseMatchingUtility)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer.serialize_array_field("children", &self.parent.m_children)?;
            serializer
                .serialize_array_field(
                    "childrenInternalStates",
                    &self.parent.m_childrenInternalStates,
                )?;
            serializer
                .serialize_array_field("sortedChildren", &self.parent.m_sortedChildren)?;
            serializer.end()
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
pub enum Mode {
    #[default]
    MODE_MATCH = 0isize,
    MODE_PLAY = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Mode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MODE_MATCH => __serializer.serialize_field("MODE_MATCH", &0u64),
                Self::MODE_PLAY => __serializer.serialize_field("MODE_PLAY", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum Mode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for Mode {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                            v if v == "0" || v.eq_ignore_ascii_case("MODE_MATCH") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("MODE_PLAY") => {
                                _serde::__private::Ok(__Field::__field1)
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
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Mode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Mode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Mode")
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
                            _serde::__private::Ok(Mode::MODE_MATCH)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Mode::MODE_PLAY)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["MODE_MATCH", "MODE_PLAY"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Mode",
                VARIANTS,
                _serde::de::ReadEnumSize::Int8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Mode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
