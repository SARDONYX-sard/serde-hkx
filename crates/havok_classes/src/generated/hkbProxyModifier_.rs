use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbProxyModifier`
/// -         version: `1`
/// -       signature: `0x8a41554f`
/// -          size: 256(x86)/288(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbProxyModifier<'a> {
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
    /// -          name: `proxyInfo`(ctype: `struct hkbProxyModifierProxyInfo`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  80(x86)/ 80(x86_64)
    ///
    pub m_proxyInfo: hkbProxyModifierProxyInfo,
    /// # C++ Info
    /// -          name: `linearVelocity`(ctype: `hkVector4`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// -          name: `horizontalGain`(ctype: `hkReal`)
    /// -        offset: 144(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_horizontalGain: f32,
    /// # C++ Info
    /// -          name: `verticalGain`(ctype: `hkReal`)
    /// -        offset: 148(x86)/180(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalGain: f32,
    /// # C++ Info
    /// -          name: `maxHorizontalSeparation`(ctype: `hkReal`)
    /// -        offset: 152(x86)/184(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxHorizontalSeparation: f32,
    /// # C++ Info
    /// -          name: `maxVerticalSeparation`(ctype: `hkReal`)
    /// -        offset: 156(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxVerticalSeparation: f32,
    /// # C++ Info
    /// -          name: `verticalDisplacementError`(ctype: `hkReal`)
    /// -        offset: 160(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalDisplacementError: f32,
    /// # C++ Info
    /// -          name: `verticalDisplacementErrorGain`(ctype: `hkReal`)
    /// -        offset: 164(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalDisplacementErrorGain: f32,
    /// # C++ Info
    /// -          name: `maxVerticalDisplacement`(ctype: `hkReal`)
    /// -        offset: 168(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxVerticalDisplacement: f32,
    /// # C++ Info
    /// -          name: `minVerticalDisplacement`(ctype: `hkReal`)
    /// -        offset: 172(x86)/204(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minVerticalDisplacement: f32,
    /// # C++ Info
    /// -          name: `capsuleHeight`(ctype: `hkReal`)
    /// -        offset: 176(x86)/208(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_capsuleHeight: f32,
    /// # C++ Info
    /// -          name: `capsuleRadius`(ctype: `hkReal`)
    /// -        offset: 180(x86)/212(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_capsuleRadius: f32,
    /// # C++ Info
    /// -          name: `maxSlopeForRotation`(ctype: `hkReal`)
    /// -        offset: 184(x86)/216(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSlopeForRotation: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset: 188(x86)/220(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `phantomType`(ctype: `enum PhantomType`)
    /// -        offset: 192(x86)/224(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_phantomType: PhantomType,
    /// # C++ Info
    /// -          name: `linearVelocityMode`(ctype: `enum LinearVelocityMode`)
    /// -        offset: 193(x86)/225(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_linearVelocityMode: LinearVelocityMode,
    /// # C++ Info
    /// -          name: `ignoreIncomingRotation`(ctype: `hkBool`)
    /// -        offset: 194(x86)/226(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_ignoreIncomingRotation: bool,
    /// # C++ Info
    /// -          name: `ignoreCollisionDuringRotation`(ctype: `hkBool`)
    /// -        offset: 195(x86)/227(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_ignoreCollisionDuringRotation: bool,
    /// # C++ Info
    /// -          name: `ignoreIncomingTranslation`(ctype: `hkBool`)
    /// -        offset: 196(x86)/228(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_ignoreIncomingTranslation: bool,
    /// # C++ Info
    /// -          name: `includeDownwardMomentum`(ctype: `hkBool`)
    /// -        offset: 197(x86)/229(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_includeDownwardMomentum: bool,
    /// # C++ Info
    /// -          name: `followWorldFromModel`(ctype: `hkBool`)
    /// -        offset: 198(x86)/230(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_followWorldFromModel: bool,
    /// # C++ Info
    /// -          name: `isTouchingGround`(ctype: `hkBool`)
    /// -        offset: 199(x86)/231(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isTouchingGround: bool,
    /// # C++ Info
    /// -          name: `characterProxy`(ctype: `void*`)
    /// -        offset: 200(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_characterProxy: Pointer,
    /// # C++ Info
    /// -          name: `phantom`(ctype: `void*`)
    /// -        offset: 204(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_phantom: Pointer,
    /// # C++ Info
    /// -          name: `phantomShape`(ctype: `void*`)
    /// -        offset: 208(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_phantomShape: Pointer,
    /// # C++ Info
    /// -          name: `horizontalDisplacement`(ctype: `hkVector4`)
    /// -        offset: 224(x86)/256(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_horizontalDisplacement: Vector4,
    /// # C++ Info
    /// -          name: `verticalDisplacement`(ctype: `hkReal`)
    /// -        offset: 240(x86)/272(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_verticalDisplacement: f32,
    /// # C++ Info
    /// -          name: `timestep`(ctype: `hkReal`)
    /// -        offset: 244(x86)/276(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timestep: f32,
    /// # C++ Info
    /// -          name: `previousFrameFollowWorldFromModel`(ctype: `hkBool`)
    /// -        offset: 248(x86)/280(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_previousFrameFollowWorldFromModel: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbProxyModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbProxyModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2319537487u32)
        }
    }
    impl<'a> _serde::Serialize for hkbProxyModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2319537487u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbProxyModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("proxyInfo", &self.m_proxyInfo)?;
            serializer.serialize_field("linearVelocity", &self.m_linearVelocity)?;
            serializer.serialize_field("horizontalGain", &self.m_horizontalGain)?;
            serializer.serialize_field("verticalGain", &self.m_verticalGain)?;
            serializer
                .serialize_field(
                    "maxHorizontalSeparation",
                    &self.m_maxHorizontalSeparation,
                )?;
            serializer
                .serialize_field(
                    "maxVerticalSeparation",
                    &self.m_maxVerticalSeparation,
                )?;
            serializer
                .serialize_field(
                    "verticalDisplacementError",
                    &self.m_verticalDisplacementError,
                )?;
            serializer
                .serialize_field(
                    "verticalDisplacementErrorGain",
                    &self.m_verticalDisplacementErrorGain,
                )?;
            serializer
                .serialize_field(
                    "maxVerticalDisplacement",
                    &self.m_maxVerticalDisplacement,
                )?;
            serializer
                .serialize_field(
                    "minVerticalDisplacement",
                    &self.m_minVerticalDisplacement,
                )?;
            serializer.serialize_field("capsuleHeight", &self.m_capsuleHeight)?;
            serializer.serialize_field("capsuleRadius", &self.m_capsuleRadius)?;
            serializer
                .serialize_field("maxSlopeForRotation", &self.m_maxSlopeForRotation)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.serialize_field("phantomType", &self.m_phantomType)?;
            serializer
                .serialize_field("linearVelocityMode", &self.m_linearVelocityMode)?;
            serializer
                .serialize_field(
                    "ignoreIncomingRotation",
                    &self.m_ignoreIncomingRotation,
                )?;
            serializer
                .serialize_field(
                    "ignoreCollisionDuringRotation",
                    &self.m_ignoreCollisionDuringRotation,
                )?;
            serializer
                .serialize_field(
                    "ignoreIncomingTranslation",
                    &self.m_ignoreIncomingTranslation,
                )?;
            serializer
                .serialize_field(
                    "includeDownwardMomentum",
                    &self.m_includeDownwardMomentum,
                )?;
            serializer
                .serialize_field("followWorldFromModel", &self.m_followWorldFromModel)?;
            serializer.serialize_field("isTouchingGround", &self.m_isTouchingGround)?;
            serializer.skip_field("characterProxy", &self.m_characterProxy)?;
            serializer.skip_field("phantom", &self.m_phantom)?;
            serializer.skip_field("phantomShape", &self.m_phantomShape)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .skip_field("horizontalDisplacement", &self.m_horizontalDisplacement)?;
            serializer.skip_field("verticalDisplacement", &self.m_verticalDisplacement)?;
            serializer.skip_field("timestep", &self.m_timestep)?;
            serializer
                .skip_field(
                    "previousFrameFollowWorldFromModel",
                    &self.m_previousFrameFollowWorldFromModel,
                )?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
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
pub enum PhantomType {
    #[default]
    PHANTOM_TYPE_SIMPLE = 0isize,
    PHANTOM_TYPE_CACHING = 1isize,
}
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
pub enum LinearVelocityMode {
    #[default]
    LINEAR_VELOCITY_MODE_WORLD = 0isize,
    LINEAR_VELOCITY_MODE_MODEL = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for PhantomType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::PHANTOM_TYPE_SIMPLE => {
                    __serializer.serialize_field("PHANTOM_TYPE_SIMPLE", &0u64)
                }
                Self::PHANTOM_TYPE_CACHING => {
                    __serializer.serialize_field("PHANTOM_TYPE_CACHING", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum PhantomType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for LinearVelocityMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::LINEAR_VELOCITY_MODE_WORLD => {
                    __serializer.serialize_field("LINEAR_VELOCITY_MODE_WORLD", &0u64)
                }
                Self::LINEAR_VELOCITY_MODE_MODEL => {
                    __serializer.serialize_field("LINEAR_VELOCITY_MODE_MODEL", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum LinearVelocityMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for PhantomType {
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
                                || v.eq_ignore_ascii_case("PHANTOM_TYPE_SIMPLE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("PHANTOM_TYPE_CACHING") => {
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<PhantomType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PhantomType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum PhantomType",
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
                            _serde::__private::Ok(PhantomType::PHANTOM_TYPE_SIMPLE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(PhantomType::PHANTOM_TYPE_CACHING)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "PHANTOM_TYPE_SIMPLE",
                "PHANTOM_TYPE_CACHING",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "PhantomType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PhantomType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for LinearVelocityMode {
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
                                || v.eq_ignore_ascii_case("LINEAR_VELOCITY_MODE_WORLD") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("LINEAR_VELOCITY_MODE_MODEL") => {
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<LinearVelocityMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = LinearVelocityMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum LinearVelocityMode",
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
                                LinearVelocityMode::LINEAR_VELOCITY_MODE_WORLD,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                LinearVelocityMode::LINEAR_VELOCITY_MODE_MODEL,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "LINEAR_VELOCITY_MODE_WORLD",
                "LINEAR_VELOCITY_MODE_MODEL",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "LinearVelocityMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<LinearVelocityMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
