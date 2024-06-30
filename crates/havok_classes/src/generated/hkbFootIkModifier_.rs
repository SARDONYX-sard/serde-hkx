use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkModifier`
/// -         version: `3`
/// -       signature: `0xed8966c0`
/// -          size: 208(x86)/256(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkModifier<'a> {
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
    /// -          name: `gains`(ctype: `struct hkbFootIkGains`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_gains: hkbFootIkGains,
    /// # C++ Info
    /// -          name: `legs`(ctype: `hkArray<struct hkbFootIkModifierLeg>`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_legs: Vec<hkbFootIkModifierLeg>,
    /// # C++ Info
    /// -          name: `raycastDistanceUp`(ctype: `hkReal`)
    /// -        offset: 104(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceUp: f32,
    /// # C++ Info
    /// -          name: `raycastDistanceDown`(ctype: `hkReal`)
    /// -        offset: 108(x86)/148(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// -          name: `originalGroundHeightMS`(ctype: `hkReal`)
    /// -        offset: 112(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_originalGroundHeightMS: f32,
    /// # C++ Info
    /// -          name: `errorOut`(ctype: `hkReal`)
    /// -        offset: 116(x86)/156(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_errorOut: f32,
    /// # C++ Info
    /// -          name: `errorOutTranslation`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_errorOutTranslation: Vector4,
    /// # C++ Info
    /// -          name: `alignWithGroundRotation`(ctype: `hkQuaternion`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_alignWithGroundRotation: Quaternion,
    /// # C++ Info
    /// -          name: `verticalOffset`(ctype: `hkReal`)
    /// -        offset: 160(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset: 164(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `forwardAlignFraction`(ctype: `hkReal`)
    /// -        offset: 168(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forwardAlignFraction: f32,
    /// # C++ Info
    /// -          name: `sidewaysAlignFraction`(ctype: `hkReal`)
    /// -        offset: 172(x86)/204(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sidewaysAlignFraction: f32,
    /// # C++ Info
    /// -          name: `sidewaysSampleWidth`(ctype: `hkReal`)
    /// -        offset: 176(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sidewaysSampleWidth: f32,
    /// # C++ Info
    /// -          name: `useTrackData`(ctype: `hkBool`)
    /// -        offset: 180(x86)/212(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useTrackData: bool,
    /// # C++ Info
    /// -          name: `lockFeetWhenPlanted`(ctype: `hkBool`)
    /// -        offset: 181(x86)/213(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lockFeetWhenPlanted: bool,
    /// # C++ Info
    /// -          name: `useCharacterUpVector`(ctype: `hkBool`)
    /// -        offset: 182(x86)/214(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useCharacterUpVector: bool,
    /// # C++ Info
    /// -          name: `alignMode`(ctype: `enum AlignMode`)
    /// -        offset: 183(x86)/215(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_alignMode: AlignMode,
    /// # C++ Info
    /// -          name: `internalLegData`(ctype: `hkArray<struct hkbFootIkModifierInternalLegData>`)
    /// -        offset: 184(x86)/216(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_internalLegData: Vec<hkbFootIkModifierInternalLegData>,
    /// # C++ Info
    /// -          name: `prevIsFootIkEnabled`(ctype: `hkReal`)
    /// -        offset: 196(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_prevIsFootIkEnabled: f32,
    /// # C++ Info
    /// -          name: `isSetUp`(ctype: `hkBool`)
    /// -        offset: 200(x86)/236(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isSetUp: bool,
    /// # C++ Info
    /// -          name: `isGroundPositionValid`(ctype: `hkBool`)
    /// -        offset: 201(x86)/237(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isGroundPositionValid: bool,
    /// # C++ Info
    /// -          name: `timeStep`(ctype: `hkReal`)
    /// -        offset: 204(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeStep: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbFootIkModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbFootIkModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3985204928u32)
        }
    }
    impl<'a> __serde::Serialize for hkbFootIkModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkModifier", class_meta)?;
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
            serializer.serialize_field("gains", &self.m_gains)?;
            serializer.serialize_array_meta_field("legs", &self.m_legs)?;
            serializer.serialize_field("raycastDistanceUp", &self.m_raycastDistanceUp)?;
            serializer
                .serialize_field("raycastDistanceDown", &self.m_raycastDistanceDown)?;
            serializer
                .serialize_field(
                    "originalGroundHeightMS",
                    &self.m_originalGroundHeightMS,
                )?;
            serializer.serialize_field("errorOut", &self.m_errorOut)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_field("errorOutTranslation", &self.m_errorOutTranslation)?;
            serializer
                .serialize_field(
                    "alignWithGroundRotation",
                    &self.m_alignWithGroundRotation,
                )?;
            serializer.serialize_field("verticalOffset", &self.m_verticalOffset)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer
                .serialize_field("forwardAlignFraction", &self.m_forwardAlignFraction)?;
            serializer
                .serialize_field(
                    "sidewaysAlignFraction",
                    &self.m_sidewaysAlignFraction,
                )?;
            serializer
                .serialize_field("sidewaysSampleWidth", &self.m_sidewaysSampleWidth)?;
            serializer.serialize_field("useTrackData", &self.m_useTrackData)?;
            serializer
                .serialize_field("lockFeetWhenPlanted", &self.m_lockFeetWhenPlanted)?;
            serializer
                .serialize_field("useCharacterUpVector", &self.m_useCharacterUpVector)?;
            serializer.serialize_field("alignMode", &self.m_alignMode)?;
            serializer
                .skip_array_meta_field("internalLegData", &self.m_internalLegData)?;
            serializer.skip_field("prevIsFootIkEnabled", &self.m_prevIsFootIkEnabled)?;
            serializer.skip_field("isSetUp", &self.m_isSetUp)?;
            serializer
                .skip_field("isGroundPositionValid", &self.m_isGroundPositionValid)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("legs", &self.m_legs)?;
            serializer
                .serialize_array_field("internalLegData", &self.m_internalLegData)?;
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
pub enum AlignMode {
    #[default]
    ALIGN_MODE_FORWARD_RIGHT = 0isize,
    ALIGN_MODE_FORWARD = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for AlignMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ALIGN_MODE_FORWARD_RIGHT => {
                    __serializer.serialize_field("ALIGN_MODE_FORWARD_RIGHT", &0u64)
                }
                Self::ALIGN_MODE_FORWARD => {
                    __serializer.serialize_field("ALIGN_MODE_FORWARD", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum AlignMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for AlignMode {
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("ALIGN_MODE_FORWARD_RIGHT") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("ALIGN_MODE_FORWARD") => {
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
                marker: _serde::__private::PhantomData<AlignMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = AlignMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum AlignMode",
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
                            _serde::__private::Ok(AlignMode::ALIGN_MODE_FORWARD_RIGHT)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(AlignMode::ALIGN_MODE_FORWARD)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ALIGN_MODE_FORWARD_RIGHT",
                "ALIGN_MODE_FORWARD",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "AlignMode",
                VARIANTS,
                _serde::de::ReadEnumSize::Int8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AlignMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
