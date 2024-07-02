use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClipGenerator`
/// -         version: `2`
/// -       signature: `0x333b85b9`
/// -          size: 208(x86)/272(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClipGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `animationName`(ctype: `hkStringPtr`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animationName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `triggers`(ctype: `struct hkbClipTriggerArray*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_triggers: Pointer,
    /// # C++ Info
    /// -          name: `cropStartAmountLocalTime`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cropStartAmountLocalTime: f32,
    /// # C++ Info
    /// -          name: `cropEndAmountLocalTime`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 92(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cropEndAmountLocalTime: f32,
    /// # C++ Info
    /// -          name: `startTime`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startTime: f32,
    /// # C++ Info
    /// -          name: `playbackSpeed`(ctype: `hkReal`)
    /// -        offset:  60(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_playbackSpeed: f32,
    /// # C++ Info
    /// -          name: `enforcedDuration`(ctype: `hkReal`)
    /// -        offset:  64(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_enforcedDuration: f32,
    /// # C++ Info
    /// -          name: `userControlledTimeFraction`(ctype: `hkReal`)
    /// -        offset:  68(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_userControlledTimeFraction: f32,
    /// # C++ Info
    /// -          name: `animationBindingIndex`(ctype: `hkInt16`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_animationBindingIndex: i16,
    /// # C++ Info
    /// -          name: `mode`(ctype: `enum PlaybackMode`)
    /// -        offset:  74(x86)/114(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_mode: PlaybackMode,
    /// # C++ Info
    /// -          name: `flags`(ctype: `hkInt8`)
    /// -        offset:  75(x86)/115(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_flags: i8,
    /// # C++ Info
    /// -          name: `animDatas`(ctype: `hkArray<void>`)
    /// -        offset:  76(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_animDatas: Vec<()>,
    /// # C++ Info
    /// -          name: `animationControl`(ctype: `void*`)
    /// -        offset:  88(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_animationControl: Pointer,
    /// # C++ Info
    /// -          name: `originalTriggers`(ctype: `void*`)
    /// -        offset:  92(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_originalTriggers: Pointer,
    /// # C++ Info
    /// -          name: `mapperData`(ctype: `void*`)
    /// -        offset:  96(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_mapperData: Pointer,
    /// # C++ Info
    /// -          name: `binding`(ctype: `void*`)
    /// -        offset: 100(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_binding: Pointer,
    /// # C++ Info
    /// -          name: `mirroredAnimation`(ctype: `void*`)
    /// -        offset: 104(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_mirroredAnimation: Pointer,
    /// # C++ Info
    /// -          name: `extractedMotion`(ctype: `hkQsTransform`)
    /// -        offset: 112(x86)/176(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_extractedMotion: QsTransform,
    /// # C++ Info
    /// -          name: `echos`(ctype: `hkArray<void>`)
    /// -        offset: 160(x86)/224(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_echos: Vec<()>,
    /// # C++ Info
    /// -          name: `localTime`(ctype: `hkReal`)
    /// -        offset: 172(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_localTime: f32,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset: 176(x86)/244(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_time: f32,
    /// # C++ Info
    /// -          name: `previousUserControlledTimeFraction`(ctype: `hkReal`)
    /// -        offset: 180(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_previousUserControlledTimeFraction: f32,
    /// # C++ Info
    /// -          name: `bufferSize`(ctype: `hkInt32`)
    /// -        offset: 184(x86)/252(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bufferSize: i32,
    /// # C++ Info
    /// -          name: `echoBufferSize`(ctype: `hkInt32`)
    /// -        offset: 188(x86)/256(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_echoBufferSize: i32,
    /// # C++ Info
    /// -          name: `atEnd`(ctype: `hkBool`)
    /// -        offset: 192(x86)/260(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_atEnd: bool,
    /// # C++ Info
    /// -          name: `ignoreStartTime`(ctype: `hkBool`)
    /// -        offset: 193(x86)/261(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ignoreStartTime: bool,
    /// # C++ Info
    /// -          name: `pingPongBackward`(ctype: `hkBool`)
    /// -        offset: 194(x86)/262(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pingPongBackward: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbClipGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClipGenerator"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(859538873u32)
        }
    }
    impl<'a> __serde::Serialize for hkbClipGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(859538873u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbClipGenerator", class_meta)?;
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
            serializer
                .serialize_stringptr_meta_field("animationName", &self.m_animationName)?;
            serializer.serialize_field("triggers", &self.m_triggers)?;
            serializer
                .serialize_field(
                    "cropStartAmountLocalTime",
                    &self.m_cropStartAmountLocalTime,
                )?;
            serializer
                .serialize_field(
                    "cropEndAmountLocalTime",
                    &self.m_cropEndAmountLocalTime,
                )?;
            serializer.serialize_field("startTime", &self.m_startTime)?;
            serializer.serialize_field("playbackSpeed", &self.m_playbackSpeed)?;
            serializer.serialize_field("enforcedDuration", &self.m_enforcedDuration)?;
            serializer
                .serialize_field(
                    "userControlledTimeFraction",
                    &self.m_userControlledTimeFraction,
                )?;
            serializer
                .serialize_field(
                    "animationBindingIndex",
                    &self.m_animationBindingIndex,
                )?;
            serializer.serialize_field("mode", &self.m_mode)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("animDatas", &self.m_animDatas)?;
            serializer.skip_field("animationControl", &self.m_animationControl)?;
            serializer.skip_field("originalTriggers", &self.m_originalTriggers)?;
            serializer.skip_field("mapperData", &self.m_mapperData)?;
            serializer.skip_field("binding", &self.m_binding)?;
            serializer.skip_field("mirroredAnimation", &self.m_mirroredAnimation)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.skip_field("extractedMotion", &self.m_extractedMotion)?;
            serializer.skip_array_meta_field("echos", &self.m_echos)?;
            serializer.skip_field("localTime", &self.m_localTime)?;
            serializer.skip_field("time", &self.m_time)?;
            serializer
                .skip_field(
                    "previousUserControlledTimeFraction",
                    &self.m_previousUserControlledTimeFraction,
                )?;
            serializer.skip_field("bufferSize", &self.m_bufferSize)?;
            serializer.skip_field("echoBufferSize", &self.m_echoBufferSize)?;
            serializer.skip_field("atEnd", &self.m_atEnd)?;
            serializer.skip_field("ignoreStartTime", &self.m_ignoreStartTime)?;
            serializer.skip_field("pingPongBackward", &self.m_pingPongBackward)?;
            serializer.pad_field([0u8; 13usize].as_slice(), [0u8; 9usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_stringptr_field("animationName", &self.m_animationName)?;
            serializer.serialize_array_field("animDatas", &self.m_animDatas)?;
            serializer.serialize_array_field("echos", &self.m_echos)?;
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
pub enum PlaybackMode {
    #[default]
    MODE_SINGLE_PLAY = 0isize,
    MODE_LOOPING = 1isize,
    MODE_USER_CONTROLLED = 2isize,
    MODE_PING_PONG = 3isize,
    MODE_COUNT = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for PlaybackMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MODE_SINGLE_PLAY => {
                    __serializer.serialize_field("MODE_SINGLE_PLAY", &0u64)
                }
                Self::MODE_LOOPING => __serializer.serialize_field("MODE_LOOPING", &1u64),
                Self::MODE_USER_CONTROLLED => {
                    __serializer.serialize_field("MODE_USER_CONTROLLED", &2u64)
                }
                Self::MODE_PING_PONG => {
                    __serializer.serialize_field("MODE_PING_PONG", &3u64)
                }
                Self::MODE_COUNT => __serializer.serialize_field("MODE_COUNT", &4u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum PlaybackMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for PlaybackMode {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4",
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
                                || v.eq_ignore_ascii_case("MODE_SINGLE_PLAY") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("MODE_LOOPING") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("MODE_USER_CONTROLLED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("MODE_PING_PONG") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("MODE_COUNT") => {
                                _serde::__private::Ok(__Field::__field4)
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
                marker: _serde::__private::PhantomData<PlaybackMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PlaybackMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum PlaybackMode",
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
                            _serde::__private::Ok(PlaybackMode::MODE_SINGLE_PLAY)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(PlaybackMode::MODE_LOOPING)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(PlaybackMode::MODE_USER_CONTROLLED)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(PlaybackMode::MODE_PING_PONG)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(PlaybackMode::MODE_COUNT)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MODE_SINGLE_PLAY",
                "MODE_LOOPING",
                "MODE_USER_CONTROLLED",
                "MODE_PING_PONG",
                "MODE_COUNT",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "PlaybackMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PlaybackMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
