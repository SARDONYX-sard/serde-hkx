use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbFootIkModifier`
/// - version: `3`
/// - signature: `0xed8966c0`
/// - size: `208`(x86)/`256`(x86_64)
/// -  vtable: `true`
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
    /// - name: `gains`(ctype: `struct hkbFootIkGains`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_gains: hkbFootIkGains,
    /// # C++ Info
    /// - name: `legs`(ctype: `hkArray<struct hkbFootIkModifierLeg>`)
    /// - offset: ` 92`(x86)/`128`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_legs: Vec<hkbFootIkModifierLeg>,
    /// # C++ Info
    /// - name: `raycastDistanceUp`(ctype: `hkReal`)
    /// - offset: `104`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_raycastDistanceUp: f32,
    /// # C++ Info
    /// - name: `raycastDistanceDown`(ctype: `hkReal`)
    /// - offset: `108`(x86)/`148`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// - name: `originalGroundHeightMS`(ctype: `hkReal`)
    /// - offset: `112`(x86)/`152`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_originalGroundHeightMS: f32,
    /// # C++ Info
    /// - name: `errorOut`(ctype: `hkReal`)
    /// - offset: `116`(x86)/`156`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_errorOut: f32,
    /// # C++ Info
    /// - name: `errorOutTranslation`(ctype: `hkVector4`)
    /// - offset: `128`(x86)/`160`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_errorOutTranslation: Vector4,
    /// # C++ Info
    /// - name: `alignWithGroundRotation`(ctype: `hkQuaternion`)
    /// - offset: `144`(x86)/`176`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_alignWithGroundRotation: Quaternion,
    /// # C++ Info
    /// - name: `verticalOffset`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `164`(x86)/`196`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// - name: `forwardAlignFraction`(ctype: `hkReal`)
    /// - offset: `168`(x86)/`200`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_forwardAlignFraction: f32,
    /// # C++ Info
    /// - name: `sidewaysAlignFraction`(ctype: `hkReal`)
    /// - offset: `172`(x86)/`204`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_sidewaysAlignFraction: f32,
    /// # C++ Info
    /// - name: `sidewaysSampleWidth`(ctype: `hkReal`)
    /// - offset: `176`(x86)/`208`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_sidewaysSampleWidth: f32,
    /// # C++ Info
    /// - name: `useTrackData`(ctype: `hkBool`)
    /// - offset: `180`(x86)/`212`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useTrackData: bool,
    /// # C++ Info
    /// - name: `lockFeetWhenPlanted`(ctype: `hkBool`)
    /// - offset: `181`(x86)/`213`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_lockFeetWhenPlanted: bool,
    /// # C++ Info
    /// - name: `useCharacterUpVector`(ctype: `hkBool`)
    /// - offset: `182`(x86)/`214`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useCharacterUpVector: bool,
    /// # C++ Info
    /// - name: `alignMode`(ctype: `enum AlignMode`)
    /// - offset: `183`(x86)/`215`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_alignMode: AlignMode,
    /// # C++ Info
    /// - name: `internalLegData`(ctype: `hkArray<struct hkbFootIkModifierInternalLegData>`)
    /// - offset: `184`(x86)/`216`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_internalLegData: Vec<hkbFootIkModifierInternalLegData>,
    /// # C++ Info
    /// - name: `prevIsFootIkEnabled`(ctype: `hkReal`)
    /// - offset: `196`(x86)/`232`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_prevIsFootIkEnabled: f32,
    /// # C++ Info
    /// - name: `isSetUp`(ctype: `hkBool`)
    /// - offset: `200`(x86)/`236`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_isSetUp: bool,
    /// # C++ Info
    /// - name: `isGroundPositionValid`(ctype: `hkBool`)
    /// - offset: `201`(x86)/`237`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_isGroundPositionValid: bool,
    /// # C++ Info
    /// - name: `timeStep`(ctype: `hkReal`)
    /// - offset: `204`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_timeStep: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbFootIkModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xed8966c0)
        }
    }
    impl<'a> _serde::Serialize for hkbFootIkModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xed8966c0)));
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_enable,
                m_gains,
                m_legs,
                m_raycastDistanceUp,
                m_raycastDistanceDown,
                m_originalGroundHeightMS,
                m_errorOut,
                m_errorOutTranslation,
                m_alignWithGroundRotation,
                m_verticalOffset,
                m_collisionFilterInfo,
                m_forwardAlignFraction,
                m_sidewaysAlignFraction,
                m_sidewaysSampleWidth,
                m_useTrackData,
                m_lockFeetWhenPlanted,
                m_useCharacterUpVector,
                m_alignMode,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "enable" => Ok(__Field::m_enable),
                        "gains" => Ok(__Field::m_gains),
                        "legs" => Ok(__Field::m_legs),
                        "raycastDistanceUp" => Ok(__Field::m_raycastDistanceUp),
                        "raycastDistanceDown" => Ok(__Field::m_raycastDistanceDown),
                        "originalGroundHeightMS" => Ok(__Field::m_originalGroundHeightMS),
                        "errorOut" => Ok(__Field::m_errorOut),
                        "errorOutTranslation" => Ok(__Field::m_errorOutTranslation),
                        "alignWithGroundRotation" => {
                            Ok(__Field::m_alignWithGroundRotation)
                        }
                        "verticalOffset" => Ok(__Field::m_verticalOffset),
                        "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
                        "forwardAlignFraction" => Ok(__Field::m_forwardAlignFraction),
                        "sidewaysAlignFraction" => Ok(__Field::m_sidewaysAlignFraction),
                        "sidewaysSampleWidth" => Ok(__Field::m_sidewaysSampleWidth),
                        "useTrackData" => Ok(__Field::m_useTrackData),
                        "lockFeetWhenPlanted" => Ok(__Field::m_lockFeetWhenPlanted),
                        "useCharacterUpVector" => Ok(__Field::m_useCharacterUpVector),
                        "alignMode" => Ok(__Field::m_alignMode),
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
            struct __hkbFootIkModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbFootIkModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbFootIkModifierVisitor<'de> {
                type Value = hkbFootIkModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbFootIkModifier",
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
                    let mut m_gains: _serde::__private::Option<hkbFootIkGains> = _serde::__private::None;
                    let mut m_legs: _serde::__private::Option<
                        Vec<hkbFootIkModifierLeg>,
                    > = _serde::__private::None;
                    let mut m_raycastDistanceUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_originalGroundHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_errorOut: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_errorOutTranslation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_alignWithGroundRotation: _serde::__private::Option<
                        Quaternion,
                    > = _serde::__private::None;
                    let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_forwardAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysSampleWidth: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useTrackData: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_lockFeetWhenPlanted: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useCharacterUpVector: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignMode: _serde::__private::Option<AlignMode> = _serde::__private::None;
                    let mut m_internalLegData: _serde::__private::Option<
                        Vec<hkbFootIkModifierInternalLegData>,
                    > = _serde::__private::None;
                    let mut m_prevIsFootIkEnabled: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isSetUp: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isGroundPositionValid: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_timeStep: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..22usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_gains) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("gains"),
                                    );
                                }
                                m_gains = _serde::__private::Some(
                                    match __A::next_value::<hkbFootIkGains>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_legs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("legs"),
                                    );
                                }
                                m_legs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbFootIkModifierLeg>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceUp,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceUp",
                                        ),
                                    );
                                }
                                m_raycastDistanceUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceDown,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceDown",
                                        ),
                                    );
                                }
                                m_raycastDistanceDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_originalGroundHeightMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalGroundHeightMS",
                                        ),
                                    );
                                }
                                m_originalGroundHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_errorOut) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "errorOut",
                                        ),
                                    );
                                }
                                m_errorOut = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_errorOutTranslation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "errorOutTranslation",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 0usize)?;
                                m_errorOutTranslation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_alignWithGroundRotation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignWithGroundRotation",
                                        ),
                                    );
                                }
                                m_alignWithGroundRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_verticalOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalOffset",
                                        ),
                                    );
                                }
                                m_verticalOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
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
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_forwardAlignFraction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardAlignFraction",
                                        ),
                                    );
                                }
                                m_forwardAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysAlignFraction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysAlignFraction",
                                        ),
                                    );
                                }
                                m_sidewaysAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysSampleWidth,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysSampleWidth",
                                        ),
                                    );
                                }
                                m_sidewaysSampleWidth = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_useTrackData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useTrackData",
                                        ),
                                    );
                                }
                                m_useTrackData = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_lockFeetWhenPlanted,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockFeetWhenPlanted",
                                        ),
                                    );
                                }
                                m_lockFeetWhenPlanted = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useCharacterUpVector,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useCharacterUpVector",
                                        ),
                                    );
                                }
                                m_useCharacterUpVector = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_alignMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignMode",
                                        ),
                                    );
                                }
                                m_alignMode = _serde::__private::Some(
                                    match __A::next_value::<AlignMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_internalLegData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "internalLegData",
                                        ),
                                    );
                                }
                                m_internalLegData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbFootIkModifierInternalLegData>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_prevIsFootIkEnabled,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "prevIsFootIkEnabled",
                                        ),
                                    );
                                }
                                m_prevIsFootIkEnabled = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_isSetUp) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isSetUp",
                                        ),
                                    );
                                }
                                m_isSetUp = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_isGroundPositionValid,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isGroundPositionValid",
                                        ),
                                    );
                                }
                                m_isGroundPositionValid = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_timeStep) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeStep",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_timeStep = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 12usize)?;
                    let m_gains = match m_gains {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gains"),
                            );
                        }
                    };
                    let m_legs = match m_legs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("legs"),
                            );
                        }
                    };
                    let m_raycastDistanceUp = match m_raycastDistanceUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceUp",
                                ),
                            );
                        }
                    };
                    let m_raycastDistanceDown = match m_raycastDistanceDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceDown",
                                ),
                            );
                        }
                    };
                    let m_originalGroundHeightMS = match m_originalGroundHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalGroundHeightMS",
                                ),
                            );
                        }
                    };
                    let m_errorOut = match m_errorOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("errorOut"),
                            );
                        }
                    };
                    let m_errorOutTranslation = match m_errorOutTranslation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "errorOutTranslation",
                                ),
                            );
                        }
                    };
                    let m_alignWithGroundRotation = match m_alignWithGroundRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignWithGroundRotation",
                                ),
                            );
                        }
                    };
                    let m_verticalOffset = match m_verticalOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalOffset",
                                ),
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
                    let m_forwardAlignFraction = match m_forwardAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardAlignFraction",
                                ),
                            );
                        }
                    };
                    let m_sidewaysAlignFraction = match m_sidewaysAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysAlignFraction",
                                ),
                            );
                        }
                    };
                    let m_sidewaysSampleWidth = match m_sidewaysSampleWidth {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysSampleWidth",
                                ),
                            );
                        }
                    };
                    let m_useTrackData = match m_useTrackData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useTrackData",
                                ),
                            );
                        }
                    };
                    let m_lockFeetWhenPlanted = match m_lockFeetWhenPlanted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockFeetWhenPlanted",
                                ),
                            );
                        }
                    };
                    let m_useCharacterUpVector = match m_useCharacterUpVector {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useCharacterUpVector",
                                ),
                            );
                        }
                    };
                    let m_alignMode = match m_alignMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignMode",
                                ),
                            );
                        }
                    };
                    let m_internalLegData = match m_internalLegData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "internalLegData",
                                ),
                            );
                        }
                    };
                    let m_prevIsFootIkEnabled = match m_prevIsFootIkEnabled {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "prevIsFootIkEnabled",
                                ),
                            );
                        }
                    };
                    let m_isSetUp = match m_isSetUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isSetUp"),
                            );
                        }
                    };
                    let m_isGroundPositionValid = match m_isGroundPositionValid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isGroundPositionValid",
                                ),
                            );
                        }
                    };
                    let m_timeStep = match m_timeStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("timeStep"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbFootIkModifier {
                        __ptr,
                        parent,
                        m_gains,
                        m_legs,
                        m_raycastDistanceUp,
                        m_raycastDistanceDown,
                        m_originalGroundHeightMS,
                        m_errorOut,
                        m_errorOutTranslation,
                        m_alignWithGroundRotation,
                        m_verticalOffset,
                        m_collisionFilterInfo,
                        m_forwardAlignFraction,
                        m_sidewaysAlignFraction,
                        m_sidewaysSampleWidth,
                        m_useTrackData,
                        m_lockFeetWhenPlanted,
                        m_useCharacterUpVector,
                        m_alignMode,
                        m_internalLegData,
                        m_prevIsFootIkEnabled,
                        m_isSetUp,
                        m_isGroundPositionValid,
                        m_timeStep,
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
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_gains: _serde::__private::Option<hkbFootIkGains> = _serde::__private::None;
                    let mut m_legs: _serde::__private::Option<
                        Vec<hkbFootIkModifierLeg>,
                    > = _serde::__private::None;
                    let mut m_raycastDistanceUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_originalGroundHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_errorOut: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_errorOutTranslation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_alignWithGroundRotation: _serde::__private::Option<
                        Quaternion,
                    > = _serde::__private::None;
                    let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_forwardAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysSampleWidth: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useTrackData: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_lockFeetWhenPlanted: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useCharacterUpVector: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignMode: _serde::__private::Option<AlignMode> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
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
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<u64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_enable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enable) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                                    );
                                }
                                m_enable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_gains => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gains) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("gains"),
                                    );
                                }
                                m_gains = _serde::__private::Some(
                                    match __A::next_value::<hkbFootIkGains>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_legs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_legs) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("legs"),
                                    );
                                }
                                m_legs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbFootIkModifierLeg>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_raycastDistanceUp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceUp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceUp",
                                        ),
                                    );
                                }
                                m_raycastDistanceUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_raycastDistanceDown => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceDown,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceDown",
                                        ),
                                    );
                                }
                                m_raycastDistanceDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_originalGroundHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_originalGroundHeightMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalGroundHeightMS",
                                        ),
                                    );
                                }
                                m_originalGroundHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_errorOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_errorOut) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "errorOut",
                                        ),
                                    );
                                }
                                m_errorOut = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_errorOutTranslation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_errorOutTranslation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "errorOutTranslation",
                                        ),
                                    );
                                }
                                m_errorOutTranslation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_alignWithGroundRotation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_alignWithGroundRotation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignWithGroundRotation",
                                        ),
                                    );
                                }
                                m_alignWithGroundRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_verticalOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_verticalOffset) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalOffset",
                                        ),
                                    );
                                }
                                m_verticalOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionFilterInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_forwardAlignFraction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_forwardAlignFraction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardAlignFraction",
                                        ),
                                    );
                                }
                                m_forwardAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_sidewaysAlignFraction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysAlignFraction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysAlignFraction",
                                        ),
                                    );
                                }
                                m_sidewaysAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_sidewaysSampleWidth => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysSampleWidth,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysSampleWidth",
                                        ),
                                    );
                                }
                                m_sidewaysSampleWidth = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_useTrackData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_useTrackData) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useTrackData",
                                        ),
                                    );
                                }
                                m_useTrackData = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_lockFeetWhenPlanted => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_lockFeetWhenPlanted,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockFeetWhenPlanted",
                                        ),
                                    );
                                }
                                m_lockFeetWhenPlanted = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_useCharacterUpVector => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useCharacterUpVector,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useCharacterUpVector",
                                        ),
                                    );
                                }
                                m_useCharacterUpVector = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_alignMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_alignMode) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignMode",
                                        ),
                                    );
                                }
                                m_alignMode = _serde::__private::Some(
                                    match __A::next_value::<AlignMode>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enable = match m_enable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enable"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_gains = match m_gains {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gains"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_legs = match m_legs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("legs"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_raycastDistanceUp = match m_raycastDistanceUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceUp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_raycastDistanceDown = match m_raycastDistanceDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceDown",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_originalGroundHeightMS = match m_originalGroundHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalGroundHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_errorOut = match m_errorOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("errorOut"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_errorOutTranslation = match m_errorOutTranslation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "errorOutTranslation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_alignWithGroundRotation = match m_alignWithGroundRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignWithGroundRotation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_verticalOffset = match m_verticalOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_forwardAlignFraction = match m_forwardAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardAlignFraction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sidewaysAlignFraction = match m_sidewaysAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysAlignFraction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sidewaysSampleWidth = match m_sidewaysSampleWidth {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysSampleWidth",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useTrackData = match m_useTrackData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useTrackData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lockFeetWhenPlanted = match m_lockFeetWhenPlanted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockFeetWhenPlanted",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useCharacterUpVector = match m_useCharacterUpVector {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useCharacterUpVector",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_alignMode = match m_alignMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignMode",
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
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbModifier {
                        __ptr,
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbFootIkModifier {
                        __ptr,
                        parent,
                        m_gains,
                        m_legs,
                        m_raycastDistanceUp,
                        m_raycastDistanceDown,
                        m_originalGroundHeightMS,
                        m_errorOut,
                        m_errorOutTranslation,
                        m_alignWithGroundRotation,
                        m_verticalOffset,
                        m_collisionFilterInfo,
                        m_forwardAlignFraction,
                        m_sidewaysAlignFraction,
                        m_sidewaysSampleWidth,
                        m_useTrackData,
                        m_lockFeetWhenPlanted,
                        m_useCharacterUpVector,
                        m_alignMode,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "gains",
                "legs",
                "raycastDistanceUp",
                "raycastDistanceDown",
                "originalGroundHeightMS",
                "errorOut",
                "errorOutTranslation",
                "alignWithGroundRotation",
                "verticalOffset",
                "collisionFilterInfo",
                "forwardAlignFraction",
                "sidewaysAlignFraction",
                "sidewaysSampleWidth",
                "useTrackData",
                "lockFeetWhenPlanted",
                "useCharacterUpVector",
                "alignMode",
                "internalLegData",
                "prevIsFootIkEnabled",
                "isSetUp",
                "isGroundPositionValid",
                "timeStep",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbFootIkModifier",
                FIELDS,
                __hkbFootIkModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbFootIkModifier>,
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
                        _serde::de::ReadEnumSize::Int8,
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
                __Visitor {
                    marker: _serde::__private::PhantomData::<AlignMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
