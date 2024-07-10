use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSenseHandleModifier`
/// -         version: `2`
/// -       signature: `0x2a064d99`
/// -          size: 160(x86)/224(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSenseHandleModifier<'a> {
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
    /// -          name: `handle`(ctype: `struct hkbHandle`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:  24(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_handle: hkbHandle,
    /// # C++ Info
    /// -          name: `sensorLocalOffset`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_sensorLocalOffset: Vector4,
    /// # C++ Info
    /// -          name: `ranges`(ctype: `hkArray<struct hkbSenseHandleModifierRange>`)
    /// -        offset:  96(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_ranges: Vec<hkbSenseHandleModifierRange>,
    /// # C++ Info
    /// -          name: `handleOut`(ctype: `struct hkbHandle*`)
    /// -        offset: 108(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_handleOut: Pointer,
    /// # C++ Info
    /// -          name: `handleIn`(ctype: `struct hkbHandle*`)
    /// -        offset: 112(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_handleIn: Pointer,
    /// # C++ Info
    /// -          name: `localFrameName`(ctype: `hkStringPtr`)
    /// -        offset: 116(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrameName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `sensorLocalFrameName`(ctype: `hkStringPtr`)
    /// -        offset: 120(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_sensorLocalFrameName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `minDistance`(ctype: `hkReal`)
    /// -        offset: 124(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minDistance: f32,
    /// # C++ Info
    /// -          name: `maxDistance`(ctype: `hkReal`)
    /// -        offset: 128(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxDistance: f32,
    /// # C++ Info
    /// -          name: `distanceOut`(ctype: `hkReal`)
    /// -        offset: 132(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_distanceOut: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset: 136(x86)/204(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `sensorRagdollBoneIndex`(ctype: `hkInt16`)
    /// -        offset: 140(x86)/208(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_sensorRagdollBoneIndex: i16,
    /// # C++ Info
    /// -          name: `sensorAnimationBoneIndex`(ctype: `hkInt16`)
    /// -        offset: 142(x86)/210(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_sensorAnimationBoneIndex: i16,
    /// # C++ Info
    /// -          name: `sensingMode`(ctype: `enum SensingMode`)
    /// -        offset: 144(x86)/212(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_sensingMode: SensingMode,
    /// # C++ Info
    /// -          name: `extrapolateSensorPosition`(ctype: `hkBool`)
    /// -        offset: 145(x86)/213(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_extrapolateSensorPosition: bool,
    /// # C++ Info
    /// -          name: `keepFirstSensedHandle`(ctype: `hkBool`)
    /// -        offset: 146(x86)/214(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_keepFirstSensedHandle: bool,
    /// # C++ Info
    /// -          name: `foundHandleOut`(ctype: `hkBool`)
    /// -        offset: 147(x86)/215(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_foundHandleOut: bool,
    /// # C++ Info
    /// -          name: `timeSinceLastModify`(ctype: `hkReal`)
    /// -        offset: 148(x86)/216(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeSinceLastModify: f32,
    /// # C++ Info
    /// -          name: `rangeIndexForEventToSendNextUpdate`(ctype: `hkInt32`)
    /// -        offset: 152(x86)/220(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_rangeIndexForEventToSendNextUpdate: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbSenseHandleModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSenseHandleModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2a064d99)
        }
    }
    impl<'a> _serde::Serialize for hkbSenseHandleModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2a064d99)));
            let mut serializer = __serializer
                .serialize_struct("hkbSenseHandleModifier", class_meta)?;
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
            serializer.skip_field("handle", &self.m_handle)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("sensorLocalOffset", &self.m_sensorLocalOffset)?;
            serializer.serialize_array_meta_field("ranges", &self.m_ranges)?;
            serializer.serialize_field("handleOut", &self.m_handleOut)?;
            serializer.serialize_field("handleIn", &self.m_handleIn)?;
            serializer
                .serialize_stringptr_meta_field(
                    "localFrameName",
                    &self.m_localFrameName,
                )?;
            serializer
                .serialize_stringptr_meta_field(
                    "sensorLocalFrameName",
                    &self.m_sensorLocalFrameName,
                )?;
            serializer.serialize_field("minDistance", &self.m_minDistance)?;
            serializer.serialize_field("maxDistance", &self.m_maxDistance)?;
            serializer.serialize_field("distanceOut", &self.m_distanceOut)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer
                .serialize_field(
                    "sensorRagdollBoneIndex",
                    &self.m_sensorRagdollBoneIndex,
                )?;
            serializer
                .serialize_field(
                    "sensorAnimationBoneIndex",
                    &self.m_sensorAnimationBoneIndex,
                )?;
            serializer.serialize_field("sensingMode", &self.m_sensingMode)?;
            serializer
                .serialize_field(
                    "extrapolateSensorPosition",
                    &self.m_extrapolateSensorPosition,
                )?;
            serializer
                .serialize_field(
                    "keepFirstSensedHandle",
                    &self.m_keepFirstSensedHandle,
                )?;
            serializer.serialize_field("foundHandleOut", &self.m_foundHandleOut)?;
            serializer.skip_field("timeSinceLastModify", &self.m_timeSinceLastModify)?;
            serializer
                .skip_field(
                    "rangeIndexForEventToSendNextUpdate",
                    &self.m_rangeIndexForEventToSendNextUpdate,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("ranges", &self.m_ranges)?;
            serializer
                .serialize_stringptr_field("localFrameName", &self.m_localFrameName)?;
            serializer
                .serialize_stringptr_field(
                    "sensorLocalFrameName",
                    &self.m_sensorLocalFrameName,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_handle,
    m_sensorLocalOffset,
    m_ranges,
    m_handleOut,
    m_handleIn,
    m_localFrameName,
    m_sensorLocalFrameName,
    m_minDistance,
    m_maxDistance,
    m_distanceOut,
    m_collisionFilterInfo,
    m_sensorRagdollBoneIndex,
    m_sensorAnimationBoneIndex,
    m_sensingMode,
    m_extrapolateSensorPosition,
    m_keepFirstSensedHandle,
    m_foundHandleOut,
    m_timeSinceLastModify,
    m_rangeIndexForEventToSendNextUpdate,
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
            "sensorLocalOffset" => Ok(__Field::m_sensorLocalOffset),
            "ranges" => Ok(__Field::m_ranges),
            "handleOut" => Ok(__Field::m_handleOut),
            "handleIn" => Ok(__Field::m_handleIn),
            "localFrameName" => Ok(__Field::m_localFrameName),
            "sensorLocalFrameName" => Ok(__Field::m_sensorLocalFrameName),
            "minDistance" => Ok(__Field::m_minDistance),
            "maxDistance" => Ok(__Field::m_maxDistance),
            "distanceOut" => Ok(__Field::m_distanceOut),
            "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
            "sensorRagdollBoneIndex" => Ok(__Field::m_sensorRagdollBoneIndex),
            "sensorAnimationBoneIndex" => Ok(__Field::m_sensorAnimationBoneIndex),
            "sensingMode" => Ok(__Field::m_sensingMode),
            "extrapolateSensorPosition" => Ok(__Field::m_extrapolateSensorPosition),
            "keepFirstSensedHandle" => Ok(__Field::m_keepFirstSensedHandle),
            "foundHandleOut" => Ok(__Field::m_foundHandleOut),
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
pub(super) struct __hkbSenseHandleModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbSenseHandleModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbSenseHandleModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbSenseHandleModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbSenseHandleModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbSenseHandleModifierVisitor<'de> {
    type Value = hkbSenseHandleModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbSenseHandleModifier")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_handle: _serde::__private::Option<hkbHandle> = _serde::__private::None;
        let mut m_sensorLocalOffset: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_ranges: _serde::__private::Option<Vec<hkbSenseHandleModifierRange>> = _serde::__private::None;
        let mut m_handleOut: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_handleIn: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_localFrameName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_sensorLocalFrameName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_minDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_distanceOut: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_sensorRagdollBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_sensorAnimationBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_sensingMode: _serde::__private::Option<SensingMode> = _serde::__private::None;
        let mut m_extrapolateSensorPosition: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_keepFirstSensedHandle: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_foundHandleOut: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_timeSinceLastModify: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_rangeIndexForEventToSendNextUpdate: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..19usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_handle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("handle"),
                        );
                    }
                    m_handle = _serde::__private::Some(
                        match __A::next_value::<hkbHandle>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_sensorLocalOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorLocalOffset",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 0usize)?;
                    m_sensorLocalOffset = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_ranges) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("ranges"),
                        );
                    }
                    m_ranges = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkbSenseHandleModifierRange>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_handleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handleOut",
                            ),
                        );
                    }
                    m_handleOut = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_handleIn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handleIn",
                            ),
                        );
                    }
                    m_handleIn = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_localFrameName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localFrameName",
                            ),
                        );
                    }
                    m_localFrameName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_sensorLocalFrameName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorLocalFrameName",
                            ),
                        );
                    }
                    m_sensorLocalFrameName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_minDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minDistance",
                            ),
                        );
                    }
                    m_minDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_maxDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxDistance",
                            ),
                        );
                    }
                    m_maxDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_distanceOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "distanceOut",
                            ),
                        );
                    }
                    m_distanceOut = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_sensorRagdollBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorRagdollBoneIndex",
                            ),
                        );
                    }
                    m_sensorRagdollBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_sensorAnimationBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorAnimationBoneIndex",
                            ),
                        );
                    }
                    m_sensorAnimationBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
                    if _serde::__private::Option::is_some(&m_sensingMode) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensingMode",
                            ),
                        );
                    }
                    m_sensingMode = _serde::__private::Some(
                        match __A::next_value::<SensingMode>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                14usize => {
                    if _serde::__private::Option::is_some(&m_extrapolateSensorPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "extrapolateSensorPosition",
                            ),
                        );
                    }
                    m_extrapolateSensorPosition = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_keepFirstSensedHandle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keepFirstSensedHandle",
                            ),
                        );
                    }
                    m_keepFirstSensedHandle = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(&m_foundHandleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "foundHandleOut",
                            ),
                        );
                    }
                    m_foundHandleOut = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                17usize => {
                    if _serde::__private::Option::is_some(&m_timeSinceLastModify) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "timeSinceLastModify",
                            ),
                        );
                    }
                    m_timeSinceLastModify = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                18usize => {
                    if _serde::__private::Option::is_some(
                        &m_rangeIndexForEventToSendNextUpdate,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rangeIndexForEventToSendNextUpdate",
                            ),
                        );
                    }
                    m_rangeIndexForEventToSendNextUpdate = _serde::__private::Some(
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
        __A::pad(&mut __map, 4usize, 0usize)?;
        let m_handle = match m_handle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handle"),
                );
            }
        };
        let m_sensorLocalOffset = match m_sensorLocalOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sensorLocalOffset"),
                );
            }
        };
        let m_ranges = match m_ranges {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ranges"),
                );
            }
        };
        let m_handleOut = match m_handleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handleOut"),
                );
            }
        };
        let m_handleIn = match m_handleIn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handleIn"),
                );
            }
        };
        let m_localFrameName = match m_localFrameName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localFrameName"),
                );
            }
        };
        let m_sensorLocalFrameName = match m_sensorLocalFrameName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sensorLocalFrameName",
                    ),
                );
            }
        };
        let m_minDistance = match m_minDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minDistance"),
                );
            }
        };
        let m_maxDistance = match m_maxDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxDistance"),
                );
            }
        };
        let m_distanceOut = match m_distanceOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("distanceOut"),
                );
            }
        };
        let m_collisionFilterInfo = match m_collisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionFilterInfo",
                    ),
                );
            }
        };
        let m_sensorRagdollBoneIndex = match m_sensorRagdollBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sensorRagdollBoneIndex",
                    ),
                );
            }
        };
        let m_sensorAnimationBoneIndex = match m_sensorAnimationBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sensorAnimationBoneIndex",
                    ),
                );
            }
        };
        let m_sensingMode = match m_sensingMode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sensingMode"),
                );
            }
        };
        let m_extrapolateSensorPosition = match m_extrapolateSensorPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "extrapolateSensorPosition",
                    ),
                );
            }
        };
        let m_keepFirstSensedHandle = match m_keepFirstSensedHandle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "keepFirstSensedHandle",
                    ),
                );
            }
        };
        let m_foundHandleOut = match m_foundHandleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("foundHandleOut"),
                );
            }
        };
        let m_timeSinceLastModify = match m_timeSinceLastModify {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "timeSinceLastModify",
                    ),
                );
            }
        };
        let m_rangeIndexForEventToSendNextUpdate = match m_rangeIndexForEventToSendNextUpdate {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "rangeIndexForEventToSendNextUpdate",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbSenseHandleModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_handle,
            m_sensorLocalOffset,
            m_ranges,
            m_handleOut,
            m_handleIn,
            m_localFrameName,
            m_sensorLocalFrameName,
            m_minDistance,
            m_maxDistance,
            m_distanceOut,
            m_collisionFilterInfo,
            m_sensorRagdollBoneIndex,
            m_sensorAnimationBoneIndex,
            m_sensingMode,
            m_extrapolateSensorPosition,
            m_keepFirstSensedHandle,
            m_foundHandleOut,
            m_timeSinceLastModify,
            m_rangeIndexForEventToSendNextUpdate,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_sensorLocalOffset: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_ranges: _serde::__private::Option<Vec<hkbSenseHandleModifierRange>> = _serde::__private::None;
        let mut m_handleOut: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_handleIn: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_localFrameName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_sensorLocalFrameName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_minDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxDistance: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_distanceOut: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_sensorRagdollBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_sensorAnimationBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
        let mut m_sensingMode: _serde::__private::Option<SensingMode> = _serde::__private::None;
        let mut m_extrapolateSensorPosition: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_keepFirstSensedHandle: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_foundHandleOut: _serde::__private::Option<bool> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_sensorLocalOffset => {
                    if _serde::__private::Option::is_some(&m_sensorLocalOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorLocalOffset",
                            ),
                        );
                    }
                    m_sensorLocalOffset = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_ranges => {
                    if _serde::__private::Option::is_some(&m_ranges) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("ranges"),
                        );
                    }
                    m_ranges = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkbSenseHandleModifierRange>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_handleOut => {
                    if _serde::__private::Option::is_some(&m_handleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handleOut",
                            ),
                        );
                    }
                    m_handleOut = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_handleIn => {
                    if _serde::__private::Option::is_some(&m_handleIn) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "handleIn",
                            ),
                        );
                    }
                    m_handleIn = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_localFrameName => {
                    if _serde::__private::Option::is_some(&m_localFrameName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "localFrameName",
                            ),
                        );
                    }
                    m_localFrameName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_sensorLocalFrameName => {
                    if _serde::__private::Option::is_some(&m_sensorLocalFrameName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorLocalFrameName",
                            ),
                        );
                    }
                    m_sensorLocalFrameName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_minDistance => {
                    if _serde::__private::Option::is_some(&m_minDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minDistance",
                            ),
                        );
                    }
                    m_minDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_maxDistance => {
                    if _serde::__private::Option::is_some(&m_maxDistance) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxDistance",
                            ),
                        );
                    }
                    m_maxDistance = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_distanceOut => {
                    if _serde::__private::Option::is_some(&m_distanceOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "distanceOut",
                            ),
                        );
                    }
                    m_distanceOut = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_collisionFilterInfo => {
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_sensorRagdollBoneIndex => {
                    if _serde::__private::Option::is_some(&m_sensorRagdollBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorRagdollBoneIndex",
                            ),
                        );
                    }
                    m_sensorRagdollBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_sensorAnimationBoneIndex => {
                    if _serde::__private::Option::is_some(&m_sensorAnimationBoneIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensorAnimationBoneIndex",
                            ),
                        );
                    }
                    m_sensorAnimationBoneIndex = _serde::__private::Some(
                        match __A::next_value::<i16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_sensingMode => {
                    if _serde::__private::Option::is_some(&m_sensingMode) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sensingMode",
                            ),
                        );
                    }
                    m_sensingMode = _serde::__private::Some(
                        match __A::next_value::<SensingMode>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_extrapolateSensorPosition => {
                    if _serde::__private::Option::is_some(&m_extrapolateSensorPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "extrapolateSensorPosition",
                            ),
                        );
                    }
                    m_extrapolateSensorPosition = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_keepFirstSensedHandle => {
                    if _serde::__private::Option::is_some(&m_keepFirstSensedHandle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keepFirstSensedHandle",
                            ),
                        );
                    }
                    m_keepFirstSensedHandle = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_foundHandleOut => {
                    if _serde::__private::Option::is_some(&m_foundHandleOut) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "foundHandleOut",
                            ),
                        );
                    }
                    m_foundHandleOut = _serde::__private::Some(
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
        let m_sensorLocalOffset = match m_sensorLocalOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sensorLocalOffset"),
                );
            }
        };
        let m_ranges = match m_ranges {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ranges"),
                );
            }
        };
        let m_handleOut = match m_handleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handleOut"),
                );
            }
        };
        let m_handleIn = match m_handleIn {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("handleIn"),
                );
            }
        };
        let m_localFrameName = match m_localFrameName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localFrameName"),
                );
            }
        };
        let m_sensorLocalFrameName = match m_sensorLocalFrameName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sensorLocalFrameName",
                    ),
                );
            }
        };
        let m_minDistance = match m_minDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("minDistance"),
                );
            }
        };
        let m_maxDistance = match m_maxDistance {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxDistance"),
                );
            }
        };
        let m_distanceOut = match m_distanceOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("distanceOut"),
                );
            }
        };
        let m_collisionFilterInfo = match m_collisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "collisionFilterInfo",
                    ),
                );
            }
        };
        let m_sensorRagdollBoneIndex = match m_sensorRagdollBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sensorRagdollBoneIndex",
                    ),
                );
            }
        };
        let m_sensorAnimationBoneIndex = match m_sensorAnimationBoneIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sensorAnimationBoneIndex",
                    ),
                );
            }
        };
        let m_sensingMode = match m_sensingMode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("sensingMode"),
                );
            }
        };
        let m_extrapolateSensorPosition = match m_extrapolateSensorPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "extrapolateSensorPosition",
                    ),
                );
            }
        };
        let m_keepFirstSensedHandle = match m_keepFirstSensedHandle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "keepFirstSensedHandle",
                    ),
                );
            }
        };
        let m_foundHandleOut = match m_foundHandleOut {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("foundHandleOut"),
                );
            }
        };
        _serde::__private::Ok(hkbSenseHandleModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_sensorLocalOffset,
            m_ranges,
            m_handleOut,
            m_handleIn,
            m_localFrameName,
            m_sensorLocalFrameName,
            m_minDistance,
            m_maxDistance,
            m_distanceOut,
            m_collisionFilterInfo,
            m_sensorRagdollBoneIndex,
            m_sensorAnimationBoneIndex,
            m_sensingMode,
            m_extrapolateSensorPosition,
            m_keepFirstSensedHandle,
            m_foundHandleOut,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbSenseHandleModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "handle",
                "sensorLocalOffset",
                "ranges",
                "handleOut",
                "handleIn",
                "localFrameName",
                "sensorLocalFrameName",
                "minDistance",
                "maxDistance",
                "distanceOut",
                "collisionFilterInfo",
                "sensorRagdollBoneIndex",
                "sensorAnimationBoneIndex",
                "sensingMode",
                "extrapolateSensorPosition",
                "keepFirstSensedHandle",
                "foundHandleOut",
                "timeSinceLastModify",
                "rangeIndexForEventToSendNextUpdate",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbSenseHandleModifier",
                FIELDS,
                __hkbSenseHandleModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbSenseHandleModifier>,
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
pub enum SensingMode {
    #[default]
    SENSE_IN_NEARBY_RIGID_BODIES = 0isize,
    SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER = 1isize,
    SENSE_IN_OTHER_CHARACTER_RIGID_BODIES = 2isize,
    SENSE_IN_THIS_CHARACTER_RIGID_BODIES = 3isize,
    SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES = 4isize,
    SENSE_IN_GIVEN_RIGID_BODY = 5isize,
    SENSE_IN_OTHER_CHARACTER_SKELETON = 6isize,
    SENSE_IN_THIS_CHARACTER_SKELETON = 7isize,
    SENSE_IN_GIVEN_CHARACTER_SKELETON = 8isize,
    SENSE_IN_GIVEN_LOCAL_FRAME_GROUP = 9isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SensingMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SENSE_IN_NEARBY_RIGID_BODIES => {
                    __serializer.serialize_field("SENSE_IN_NEARBY_RIGID_BODIES", &0u64)
                }
                Self::SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER => {
                    __serializer
                        .serialize_field(
                            "SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER",
                            &1u64,
                        )
                }
                Self::SENSE_IN_OTHER_CHARACTER_RIGID_BODIES => {
                    __serializer
                        .serialize_field("SENSE_IN_OTHER_CHARACTER_RIGID_BODIES", &2u64)
                }
                Self::SENSE_IN_THIS_CHARACTER_RIGID_BODIES => {
                    __serializer
                        .serialize_field("SENSE_IN_THIS_CHARACTER_RIGID_BODIES", &3u64)
                }
                Self::SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES => {
                    __serializer
                        .serialize_field("SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES", &4u64)
                }
                Self::SENSE_IN_GIVEN_RIGID_BODY => {
                    __serializer.serialize_field("SENSE_IN_GIVEN_RIGID_BODY", &5u64)
                }
                Self::SENSE_IN_OTHER_CHARACTER_SKELETON => {
                    __serializer
                        .serialize_field("SENSE_IN_OTHER_CHARACTER_SKELETON", &6u64)
                }
                Self::SENSE_IN_THIS_CHARACTER_SKELETON => {
                    __serializer
                        .serialize_field("SENSE_IN_THIS_CHARACTER_SKELETON", &7u64)
                }
                Self::SENSE_IN_GIVEN_CHARACTER_SKELETON => {
                    __serializer
                        .serialize_field("SENSE_IN_GIVEN_CHARACTER_SKELETON", &8u64)
                }
                Self::SENSE_IN_GIVEN_LOCAL_FRAME_GROUP => {
                    __serializer
                        .serialize_field("SENSE_IN_GIVEN_LOCAL_FRAME_GROUP", &9u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SensingMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for SensingMode {
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
                                || v
                                    .eq_ignore_ascii_case("SENSE_IN_NEARBY_RIGID_BODIES") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_OTHER_CHARACTER_RIGID_BODIES",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_THIS_CHARACTER_RIGID_BODIES",
                                    ) => _serde::__private::Ok(__Field::__field3),
                            v if v == "4"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES",
                                    ) => _serde::__private::Ok(__Field::__field4),
                            v if v == "5"
                                || v.eq_ignore_ascii_case("SENSE_IN_GIVEN_RIGID_BODY") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_OTHER_CHARACTER_SKELETON",
                                    ) => _serde::__private::Ok(__Field::__field6),
                            v if v == "7"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_THIS_CHARACTER_SKELETON",
                                    ) => _serde::__private::Ok(__Field::__field7),
                            v if v == "8"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_GIVEN_CHARACTER_SKELETON",
                                    ) => _serde::__private::Ok(__Field::__field8),
                            v if v == "9"
                                || v
                                    .eq_ignore_ascii_case(
                                        "SENSE_IN_GIVEN_LOCAL_FRAME_GROUP",
                                    ) => _serde::__private::Ok(__Field::__field9),
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
                marker: _serde::__private::PhantomData<SensingMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SensingMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum SensingMode",
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
                                SensingMode::SENSE_IN_NEARBY_RIGID_BODIES,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_OTHER_CHARACTER_RIGID_BODIES,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_THIS_CHARACTER_RIGID_BODIES,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES,
                            )
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(SensingMode::SENSE_IN_GIVEN_RIGID_BODY)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_OTHER_CHARACTER_SKELETON,
                            )
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_THIS_CHARACTER_SKELETON,
                            )
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_GIVEN_CHARACTER_SKELETON,
                            )
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                SensingMode::SENSE_IN_GIVEN_LOCAL_FRAME_GROUP,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "SENSE_IN_NEARBY_RIGID_BODIES",
                "SENSE_IN_RIGID_BODIES_OUTSIDE_THIS_CHARACTER",
                "SENSE_IN_OTHER_CHARACTER_RIGID_BODIES",
                "SENSE_IN_THIS_CHARACTER_RIGID_BODIES",
                "SENSE_IN_GIVEN_CHARACTER_RIGID_BODIES",
                "SENSE_IN_GIVEN_RIGID_BODY",
                "SENSE_IN_OTHER_CHARACTER_SKELETON",
                "SENSE_IN_THIS_CHARACTER_SKELETON",
                "SENSE_IN_GIVEN_CHARACTER_SKELETON",
                "SENSE_IN_GIVEN_LOCAL_FRAME_GROUP",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "SensingMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SensingMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
