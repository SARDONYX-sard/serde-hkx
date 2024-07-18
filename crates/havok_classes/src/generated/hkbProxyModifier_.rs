use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbProxyModifier`
/// - version: `1`
/// - signature: `0x8a41554f`
/// - size: `256`(x86)/`288`(x86_64)
/// -  vtable: `true`
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
    /// - name: `proxyInfo`(ctype: `struct hkbProxyModifierProxyInfo`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 80`(x86)/` 80`(x86_64)
    pub m_proxyInfo: hkbProxyModifierProxyInfo,
    /// # C++ Info
    /// - name: `linearVelocity`(ctype: `hkVector4`)
    /// - offset: `128`(x86)/`160`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// - name: `horizontalGain`(ctype: `hkReal`)
    /// - offset: `144`(x86)/`176`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_horizontalGain: f32,
    /// # C++ Info
    /// - name: `verticalGain`(ctype: `hkReal`)
    /// - offset: `148`(x86)/`180`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalGain: f32,
    /// # C++ Info
    /// - name: `maxHorizontalSeparation`(ctype: `hkReal`)
    /// - offset: `152`(x86)/`184`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxHorizontalSeparation: f32,
    /// # C++ Info
    /// - name: `maxVerticalSeparation`(ctype: `hkReal`)
    /// - offset: `156`(x86)/`188`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxVerticalSeparation: f32,
    /// # C++ Info
    /// - name: `verticalDisplacementError`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalDisplacementError: f32,
    /// # C++ Info
    /// - name: `verticalDisplacementErrorGain`(ctype: `hkReal`)
    /// - offset: `164`(x86)/`196`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_verticalDisplacementErrorGain: f32,
    /// # C++ Info
    /// - name: `maxVerticalDisplacement`(ctype: `hkReal`)
    /// - offset: `168`(x86)/`200`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxVerticalDisplacement: f32,
    /// # C++ Info
    /// - name: `minVerticalDisplacement`(ctype: `hkReal`)
    /// - offset: `172`(x86)/`204`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_minVerticalDisplacement: f32,
    /// # C++ Info
    /// - name: `capsuleHeight`(ctype: `hkReal`)
    /// - offset: `176`(x86)/`208`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_capsuleHeight: f32,
    /// # C++ Info
    /// - name: `capsuleRadius`(ctype: `hkReal`)
    /// - offset: `180`(x86)/`212`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_capsuleRadius: f32,
    /// # C++ Info
    /// - name: `maxSlopeForRotation`(ctype: `hkReal`)
    /// - offset: `184`(x86)/`216`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxSlopeForRotation: f32,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: `188`(x86)/`220`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// - name: `phantomType`(ctype: `enum PhantomType`)
    /// - offset: `192`(x86)/`224`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_phantomType: PhantomType,
    /// # C++ Info
    /// - name: `linearVelocityMode`(ctype: `enum LinearVelocityMode`)
    /// - offset: `193`(x86)/`225`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_linearVelocityMode: LinearVelocityMode,
    /// # C++ Info
    /// - name: `ignoreIncomingRotation`(ctype: `hkBool`)
    /// - offset: `194`(x86)/`226`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_ignoreIncomingRotation: bool,
    /// # C++ Info
    /// - name: `ignoreCollisionDuringRotation`(ctype: `hkBool`)
    /// - offset: `195`(x86)/`227`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_ignoreCollisionDuringRotation: bool,
    /// # C++ Info
    /// - name: `ignoreIncomingTranslation`(ctype: `hkBool`)
    /// - offset: `196`(x86)/`228`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_ignoreIncomingTranslation: bool,
    /// # C++ Info
    /// - name: `includeDownwardMomentum`(ctype: `hkBool`)
    /// - offset: `197`(x86)/`229`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_includeDownwardMomentum: bool,
    /// # C++ Info
    /// - name: `followWorldFromModel`(ctype: `hkBool`)
    /// - offset: `198`(x86)/`230`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_followWorldFromModel: bool,
    /// # C++ Info
    /// - name: `isTouchingGround`(ctype: `hkBool`)
    /// - offset: `199`(x86)/`231`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_isTouchingGround: bool,
    /// # C++ Info
    /// - name: `characterProxy`(ctype: `void*`)
    /// - offset: `200`(x86)/`232`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_characterProxy: Pointer,
    /// # C++ Info
    /// - name: `phantom`(ctype: `void*`)
    /// - offset: `204`(x86)/`240`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_phantom: Pointer,
    /// # C++ Info
    /// - name: `phantomShape`(ctype: `void*`)
    /// - offset: `208`(x86)/`248`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_phantomShape: Pointer,
    /// # C++ Info
    /// - name: `horizontalDisplacement`(ctype: `hkVector4`)
    /// - offset: `224`(x86)/`256`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_horizontalDisplacement: Vector4,
    /// # C++ Info
    /// - name: `verticalDisplacement`(ctype: `hkReal`)
    /// - offset: `240`(x86)/`272`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_verticalDisplacement: f32,
    /// # C++ Info
    /// - name: `timestep`(ctype: `hkReal`)
    /// - offset: `244`(x86)/`276`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_timestep: f32,
    /// # C++ Info
    /// - name: `previousFrameFollowWorldFromModel`(ctype: `hkBool`)
    /// - offset: `248`(x86)/`280`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
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
            _serde::__private::Signature::new(0x8a41554f)
        }
    }
    impl<'a> _serde::Serialize for hkbProxyModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8a41554f)));
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
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_proxyInfo,
    m_linearVelocity,
    m_horizontalGain,
    m_verticalGain,
    m_maxHorizontalSeparation,
    m_maxVerticalSeparation,
    m_verticalDisplacementError,
    m_verticalDisplacementErrorGain,
    m_maxVerticalDisplacement,
    m_minVerticalDisplacement,
    m_capsuleHeight,
    m_capsuleRadius,
    m_maxSlopeForRotation,
    m_collisionFilterInfo,
    m_phantomType,
    m_linearVelocityMode,
    m_ignoreIncomingRotation,
    m_ignoreCollisionDuringRotation,
    m_ignoreIncomingTranslation,
    m_includeDownwardMomentum,
    m_followWorldFromModel,
    m_isTouchingGround,
    m_characterProxy,
    m_phantom,
    m_phantomShape,
    m_horizontalDisplacement,
    m_verticalDisplacement,
    m_timestep,
    m_previousFrameFollowWorldFromModel,
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
            "proxyInfo" => Ok(__Field::m_proxyInfo),
            "linearVelocity" => Ok(__Field::m_linearVelocity),
            "horizontalGain" => Ok(__Field::m_horizontalGain),
            "verticalGain" => Ok(__Field::m_verticalGain),
            "maxHorizontalSeparation" => Ok(__Field::m_maxHorizontalSeparation),
            "maxVerticalSeparation" => Ok(__Field::m_maxVerticalSeparation),
            "verticalDisplacementError" => Ok(__Field::m_verticalDisplacementError),
            "verticalDisplacementErrorGain" => {
                Ok(__Field::m_verticalDisplacementErrorGain)
            }
            "maxVerticalDisplacement" => Ok(__Field::m_maxVerticalDisplacement),
            "minVerticalDisplacement" => Ok(__Field::m_minVerticalDisplacement),
            "capsuleHeight" => Ok(__Field::m_capsuleHeight),
            "capsuleRadius" => Ok(__Field::m_capsuleRadius),
            "maxSlopeForRotation" => Ok(__Field::m_maxSlopeForRotation),
            "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
            "phantomType" => Ok(__Field::m_phantomType),
            "linearVelocityMode" => Ok(__Field::m_linearVelocityMode),
            "ignoreIncomingRotation" => Ok(__Field::m_ignoreIncomingRotation),
            "ignoreCollisionDuringRotation" => {
                Ok(__Field::m_ignoreCollisionDuringRotation)
            }
            "ignoreIncomingTranslation" => Ok(__Field::m_ignoreIncomingTranslation),
            "includeDownwardMomentum" => Ok(__Field::m_includeDownwardMomentum),
            "followWorldFromModel" => Ok(__Field::m_followWorldFromModel),
            "isTouchingGround" => Ok(__Field::m_isTouchingGround),
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
pub(super) struct __hkbProxyModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbProxyModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbProxyModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbProxyModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbProxyModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbProxyModifierVisitor<'de> {
    type Value = hkbProxyModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbProxyModifier")
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
        let mut m_proxyInfo: _serde::__private::Option<hkbProxyModifierProxyInfo> = _serde::__private::None;
        let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_horizontalGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxHorizontalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVerticalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalDisplacementError: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalDisplacementErrorGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVerticalDisplacement: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minVerticalDisplacement: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_capsuleHeight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_capsuleRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSlopeForRotation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_phantomType: _serde::__private::Option<PhantomType> = _serde::__private::None;
        let mut m_linearVelocityMode: _serde::__private::Option<LinearVelocityMode> = _serde::__private::None;
        let mut m_ignoreIncomingRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ignoreCollisionDuringRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ignoreIncomingTranslation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_includeDownwardMomentum: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_followWorldFromModel: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isTouchingGround: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_characterProxy: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_phantom: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_phantomShape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_horizontalDisplacement: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_verticalDisplacement: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_timestep: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_previousFrameFollowWorldFromModel: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..29usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_proxyInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "proxyInfo",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 0usize)?;
                    m_proxyInfo = _serde::__private::Some(
                        match __A::next_value::<hkbProxyModifierProxyInfo>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_linearVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "linearVelocity",
                            ),
                        );
                    }
                    m_linearVelocity = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_horizontalGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "horizontalGain",
                            ),
                        );
                    }
                    m_horizontalGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_verticalGain) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalGain",
                            ),
                        );
                    }
                    m_verticalGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_maxHorizontalSeparation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxHorizontalSeparation",
                            ),
                        );
                    }
                    m_maxHorizontalSeparation = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_maxVerticalSeparation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxVerticalSeparation",
                            ),
                        );
                    }
                    m_maxVerticalSeparation = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_verticalDisplacementError) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalDisplacementError",
                            ),
                        );
                    }
                    m_verticalDisplacementError = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(
                        &m_verticalDisplacementErrorGain,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalDisplacementErrorGain",
                            ),
                        );
                    }
                    m_verticalDisplacementErrorGain = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_maxVerticalDisplacement) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxVerticalDisplacement",
                            ),
                        );
                    }
                    m_maxVerticalDisplacement = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_minVerticalDisplacement) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "minVerticalDisplacement",
                            ),
                        );
                    }
                    m_minVerticalDisplacement = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_capsuleHeight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "capsuleHeight",
                            ),
                        );
                    }
                    m_capsuleHeight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_capsuleRadius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "capsuleRadius",
                            ),
                        );
                    }
                    m_capsuleRadius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                12usize => {
                    if _serde::__private::Option::is_some(&m_maxSlopeForRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxSlopeForRotation",
                            ),
                        );
                    }
                    m_maxSlopeForRotation = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                13usize => {
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
                14usize => {
                    if _serde::__private::Option::is_some(&m_phantomType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "phantomType",
                            ),
                        );
                    }
                    m_phantomType = _serde::__private::Some(
                        match __A::next_value::<PhantomType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_linearVelocityMode) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "linearVelocityMode",
                            ),
                        );
                    }
                    m_linearVelocityMode = _serde::__private::Some(
                        match __A::next_value::<LinearVelocityMode>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                16usize => {
                    if _serde::__private::Option::is_some(&m_ignoreIncomingRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ignoreIncomingRotation",
                            ),
                        );
                    }
                    m_ignoreIncomingRotation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                17usize => {
                    if _serde::__private::Option::is_some(
                        &m_ignoreCollisionDuringRotation,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ignoreCollisionDuringRotation",
                            ),
                        );
                    }
                    m_ignoreCollisionDuringRotation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                18usize => {
                    if _serde::__private::Option::is_some(&m_ignoreIncomingTranslation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ignoreIncomingTranslation",
                            ),
                        );
                    }
                    m_ignoreIncomingTranslation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                19usize => {
                    if _serde::__private::Option::is_some(&m_includeDownwardMomentum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "includeDownwardMomentum",
                            ),
                        );
                    }
                    m_includeDownwardMomentum = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                20usize => {
                    if _serde::__private::Option::is_some(&m_followWorldFromModel) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "followWorldFromModel",
                            ),
                        );
                    }
                    m_followWorldFromModel = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                21usize => {
                    if _serde::__private::Option::is_some(&m_isTouchingGround) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isTouchingGround",
                            ),
                        );
                    }
                    m_isTouchingGround = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                22usize => {
                    if _serde::__private::Option::is_some(&m_characterProxy) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterProxy",
                            ),
                        );
                    }
                    m_characterProxy = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                23usize => {
                    if _serde::__private::Option::is_some(&m_phantom) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("phantom"),
                        );
                    }
                    m_phantom = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                24usize => {
                    if _serde::__private::Option::is_some(&m_phantomShape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "phantomShape",
                            ),
                        );
                    }
                    m_phantomShape = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                25usize => {
                    if _serde::__private::Option::is_some(&m_horizontalDisplacement) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "horizontalDisplacement",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 0usize)?;
                    m_horizontalDisplacement = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                26usize => {
                    if _serde::__private::Option::is_some(&m_verticalDisplacement) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "verticalDisplacement",
                            ),
                        );
                    }
                    m_verticalDisplacement = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                27usize => {
                    if _serde::__private::Option::is_some(&m_timestep) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "timestep",
                            ),
                        );
                    }
                    m_timestep = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                28usize => {
                    if _serde::__private::Option::is_some(
                        &m_previousFrameFollowWorldFromModel,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "previousFrameFollowWorldFromModel",
                            ),
                        );
                    }
                    m_previousFrameFollowWorldFromModel = _serde::__private::Some(
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
        __A::pad(&mut __map, 7usize, 7usize)?;
        let m_proxyInfo = match m_proxyInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("proxyInfo"),
                );
            }
        };
        let m_linearVelocity = match m_linearVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("linearVelocity"),
                );
            }
        };
        let m_horizontalGain = match m_horizontalGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("horizontalGain"),
                );
            }
        };
        let m_verticalGain = match m_verticalGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("verticalGain"),
                );
            }
        };
        let m_maxHorizontalSeparation = match m_maxHorizontalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxHorizontalSeparation",
                    ),
                );
            }
        };
        let m_maxVerticalSeparation = match m_maxVerticalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVerticalSeparation",
                    ),
                );
            }
        };
        let m_verticalDisplacementError = match m_verticalDisplacementError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "verticalDisplacementError",
                    ),
                );
            }
        };
        let m_verticalDisplacementErrorGain = match m_verticalDisplacementErrorGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "verticalDisplacementErrorGain",
                    ),
                );
            }
        };
        let m_maxVerticalDisplacement = match m_maxVerticalDisplacement {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVerticalDisplacement",
                    ),
                );
            }
        };
        let m_minVerticalDisplacement = match m_minVerticalDisplacement {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minVerticalDisplacement",
                    ),
                );
            }
        };
        let m_capsuleHeight = match m_capsuleHeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("capsuleHeight"),
                );
            }
        };
        let m_capsuleRadius = match m_capsuleRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("capsuleRadius"),
                );
            }
        };
        let m_maxSlopeForRotation = match m_maxSlopeForRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxSlopeForRotation",
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
        let m_phantomType = match m_phantomType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("phantomType"),
                );
            }
        };
        let m_linearVelocityMode = match m_linearVelocityMode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "linearVelocityMode",
                    ),
                );
            }
        };
        let m_ignoreIncomingRotation = match m_ignoreIncomingRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ignoreIncomingRotation",
                    ),
                );
            }
        };
        let m_ignoreCollisionDuringRotation = match m_ignoreCollisionDuringRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ignoreCollisionDuringRotation",
                    ),
                );
            }
        };
        let m_ignoreIncomingTranslation = match m_ignoreIncomingTranslation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ignoreIncomingTranslation",
                    ),
                );
            }
        };
        let m_includeDownwardMomentum = match m_includeDownwardMomentum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "includeDownwardMomentum",
                    ),
                );
            }
        };
        let m_followWorldFromModel = match m_followWorldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "followWorldFromModel",
                    ),
                );
            }
        };
        let m_isTouchingGround = match m_isTouchingGround {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isTouchingGround"),
                );
            }
        };
        let m_characterProxy = match m_characterProxy {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("characterProxy"),
                );
            }
        };
        let m_phantom = match m_phantom {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("phantom"),
                );
            }
        };
        let m_phantomShape = match m_phantomShape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("phantomShape"),
                );
            }
        };
        let m_horizontalDisplacement = match m_horizontalDisplacement {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "horizontalDisplacement",
                    ),
                );
            }
        };
        let m_verticalDisplacement = match m_verticalDisplacement {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "verticalDisplacement",
                    ),
                );
            }
        };
        let m_timestep = match m_timestep {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("timestep"),
                );
            }
        };
        let m_previousFrameFollowWorldFromModel = match m_previousFrameFollowWorldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "previousFrameFollowWorldFromModel",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbProxyModifier {
            __ptr,
            parent,
            m_proxyInfo,
            m_linearVelocity,
            m_horizontalGain,
            m_verticalGain,
            m_maxHorizontalSeparation,
            m_maxVerticalSeparation,
            m_verticalDisplacementError,
            m_verticalDisplacementErrorGain,
            m_maxVerticalDisplacement,
            m_minVerticalDisplacement,
            m_capsuleHeight,
            m_capsuleRadius,
            m_maxSlopeForRotation,
            m_collisionFilterInfo,
            m_phantomType,
            m_linearVelocityMode,
            m_ignoreIncomingRotation,
            m_ignoreCollisionDuringRotation,
            m_ignoreIncomingTranslation,
            m_includeDownwardMomentum,
            m_followWorldFromModel,
            m_isTouchingGround,
            m_characterProxy,
            m_phantom,
            m_phantomShape,
            m_horizontalDisplacement,
            m_verticalDisplacement,
            m_timestep,
            m_previousFrameFollowWorldFromModel,
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
        let mut m_proxyInfo: _serde::__private::Option<hkbProxyModifierProxyInfo> = _serde::__private::None;
        let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_horizontalGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxHorizontalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVerticalSeparation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalDisplacementError: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalDisplacementErrorGain: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxVerticalDisplacement: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_minVerticalDisplacement: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_capsuleHeight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_capsuleRadius: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxSlopeForRotation: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_phantomType: _serde::__private::Option<PhantomType> = _serde::__private::None;
        let mut m_linearVelocityMode: _serde::__private::Option<LinearVelocityMode> = _serde::__private::None;
        let mut m_ignoreIncomingRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ignoreCollisionDuringRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_ignoreIncomingTranslation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_includeDownwardMomentum: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_followWorldFromModel: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isTouchingGround: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..22usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_proxyInfo => {
                        if _serde::__private::Option::is_some(&m_proxyInfo) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "proxyInfo",
                                ),
                            );
                        }
                        m_proxyInfo = _serde::__private::Some(
                            match __A::next_value::<
                                hkbProxyModifierProxyInfo,
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
                    __Field::m_linearVelocity => {
                        if _serde::__private::Option::is_some(&m_linearVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "linearVelocity",
                                ),
                            );
                        }
                        m_linearVelocity = _serde::__private::Some(
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
                    __Field::m_horizontalGain => {
                        if _serde::__private::Option::is_some(&m_horizontalGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "horizontalGain",
                                ),
                            );
                        }
                        m_horizontalGain = _serde::__private::Some(
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
                    __Field::m_verticalGain => {
                        if _serde::__private::Option::is_some(&m_verticalGain) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "verticalGain",
                                ),
                            );
                        }
                        m_verticalGain = _serde::__private::Some(
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
                    __Field::m_maxHorizontalSeparation => {
                        if _serde::__private::Option::is_some(
                            &m_maxHorizontalSeparation,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxHorizontalSeparation",
                                ),
                            );
                        }
                        m_maxHorizontalSeparation = _serde::__private::Some(
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
                    __Field::m_maxVerticalSeparation => {
                        if _serde::__private::Option::is_some(&m_maxVerticalSeparation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxVerticalSeparation",
                                ),
                            );
                        }
                        m_maxVerticalSeparation = _serde::__private::Some(
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
                    __Field::m_verticalDisplacementError => {
                        if _serde::__private::Option::is_some(
                            &m_verticalDisplacementError,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "verticalDisplacementError",
                                ),
                            );
                        }
                        m_verticalDisplacementError = _serde::__private::Some(
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
                    __Field::m_verticalDisplacementErrorGain => {
                        if _serde::__private::Option::is_some(
                            &m_verticalDisplacementErrorGain,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "verticalDisplacementErrorGain",
                                ),
                            );
                        }
                        m_verticalDisplacementErrorGain = _serde::__private::Some(
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
                    __Field::m_maxVerticalDisplacement => {
                        if _serde::__private::Option::is_some(
                            &m_maxVerticalDisplacement,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxVerticalDisplacement",
                                ),
                            );
                        }
                        m_maxVerticalDisplacement = _serde::__private::Some(
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
                    __Field::m_minVerticalDisplacement => {
                        if _serde::__private::Option::is_some(
                            &m_minVerticalDisplacement,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "minVerticalDisplacement",
                                ),
                            );
                        }
                        m_minVerticalDisplacement = _serde::__private::Some(
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
                    __Field::m_capsuleHeight => {
                        if _serde::__private::Option::is_some(&m_capsuleHeight) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "capsuleHeight",
                                ),
                            );
                        }
                        m_capsuleHeight = _serde::__private::Some(
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
                    __Field::m_capsuleRadius => {
                        if _serde::__private::Option::is_some(&m_capsuleRadius) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "capsuleRadius",
                                ),
                            );
                        }
                        m_capsuleRadius = _serde::__private::Some(
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
                    __Field::m_maxSlopeForRotation => {
                        if _serde::__private::Option::is_some(&m_maxSlopeForRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxSlopeForRotation",
                                ),
                            );
                        }
                        m_maxSlopeForRotation = _serde::__private::Some(
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
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_phantomType => {
                        if _serde::__private::Option::is_some(&m_phantomType) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "phantomType",
                                ),
                            );
                        }
                        m_phantomType = _serde::__private::Some(
                            match __A::next_value::<PhantomType>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_linearVelocityMode => {
                        if _serde::__private::Option::is_some(&m_linearVelocityMode) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "linearVelocityMode",
                                ),
                            );
                        }
                        m_linearVelocityMode = _serde::__private::Some(
                            match __A::next_value::<LinearVelocityMode>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_ignoreIncomingRotation => {
                        if _serde::__private::Option::is_some(
                            &m_ignoreIncomingRotation,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ignoreIncomingRotation",
                                ),
                            );
                        }
                        m_ignoreIncomingRotation = _serde::__private::Some(
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
                    __Field::m_ignoreCollisionDuringRotation => {
                        if _serde::__private::Option::is_some(
                            &m_ignoreCollisionDuringRotation,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ignoreCollisionDuringRotation",
                                ),
                            );
                        }
                        m_ignoreCollisionDuringRotation = _serde::__private::Some(
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
                    __Field::m_ignoreIncomingTranslation => {
                        if _serde::__private::Option::is_some(
                            &m_ignoreIncomingTranslation,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ignoreIncomingTranslation",
                                ),
                            );
                        }
                        m_ignoreIncomingTranslation = _serde::__private::Some(
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
                    __Field::m_includeDownwardMomentum => {
                        if _serde::__private::Option::is_some(
                            &m_includeDownwardMomentum,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "includeDownwardMomentum",
                                ),
                            );
                        }
                        m_includeDownwardMomentum = _serde::__private::Some(
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
                    __Field::m_followWorldFromModel => {
                        if _serde::__private::Option::is_some(&m_followWorldFromModel) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "followWorldFromModel",
                                ),
                            );
                        }
                        m_followWorldFromModel = _serde::__private::Some(
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
                    __Field::m_isTouchingGround => {
                        if _serde::__private::Option::is_some(&m_isTouchingGround) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isTouchingGround",
                                ),
                            );
                        }
                        m_isTouchingGround = _serde::__private::Some(
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
                    _ => {}
                }
            }
        }
        let m_proxyInfo = match m_proxyInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("proxyInfo"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_linearVelocity = match m_linearVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("linearVelocity"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_horizontalGain = match m_horizontalGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("horizontalGain"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_verticalGain = match m_verticalGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("verticalGain"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxHorizontalSeparation = match m_maxHorizontalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxHorizontalSeparation",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxVerticalSeparation = match m_maxVerticalSeparation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVerticalSeparation",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_verticalDisplacementError = match m_verticalDisplacementError {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "verticalDisplacementError",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_verticalDisplacementErrorGain = match m_verticalDisplacementErrorGain {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "verticalDisplacementErrorGain",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxVerticalDisplacement = match m_maxVerticalDisplacement {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxVerticalDisplacement",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_minVerticalDisplacement = match m_minVerticalDisplacement {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "minVerticalDisplacement",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_capsuleHeight = match m_capsuleHeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("capsuleHeight"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_capsuleRadius = match m_capsuleRadius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("capsuleRadius"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxSlopeForRotation = match m_maxSlopeForRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxSlopeForRotation",
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
        let m_phantomType = match m_phantomType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("phantomType"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_linearVelocityMode = match m_linearVelocityMode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "linearVelocityMode",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_ignoreIncomingRotation = match m_ignoreIncomingRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ignoreIncomingRotation",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_ignoreCollisionDuringRotation = match m_ignoreCollisionDuringRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ignoreCollisionDuringRotation",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_ignoreIncomingTranslation = match m_ignoreIncomingTranslation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "ignoreIncomingTranslation",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_includeDownwardMomentum = match m_includeDownwardMomentum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "includeDownwardMomentum",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_followWorldFromModel = match m_followWorldFromModel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "followWorldFromModel",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_isTouchingGround = match m_isTouchingGround {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isTouchingGround"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkbProxyModifier {
            __ptr,
            parent,
            m_proxyInfo,
            m_linearVelocity,
            m_horizontalGain,
            m_verticalGain,
            m_maxHorizontalSeparation,
            m_maxVerticalSeparation,
            m_verticalDisplacementError,
            m_verticalDisplacementErrorGain,
            m_maxVerticalDisplacement,
            m_minVerticalDisplacement,
            m_capsuleHeight,
            m_capsuleRadius,
            m_maxSlopeForRotation,
            m_collisionFilterInfo,
            m_phantomType,
            m_linearVelocityMode,
            m_ignoreIncomingRotation,
            m_ignoreCollisionDuringRotation,
            m_ignoreIncomingTranslation,
            m_includeDownwardMomentum,
            m_followWorldFromModel,
            m_isTouchingGround,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbProxyModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "proxyInfo",
                "linearVelocity",
                "horizontalGain",
                "verticalGain",
                "maxHorizontalSeparation",
                "maxVerticalSeparation",
                "verticalDisplacementError",
                "verticalDisplacementErrorGain",
                "maxVerticalDisplacement",
                "minVerticalDisplacement",
                "capsuleHeight",
                "capsuleRadius",
                "maxSlopeForRotation",
                "collisionFilterInfo",
                "phantomType",
                "linearVelocityMode",
                "ignoreIncomingRotation",
                "ignoreCollisionDuringRotation",
                "ignoreIncomingTranslation",
                "includeDownwardMomentum",
                "followWorldFromModel",
                "isTouchingGround",
                "characterProxy",
                "phantom",
                "phantomShape",
                "horizontalDisplacement",
                "verticalDisplacement",
                "timestep",
                "previousFrameFollowWorldFromModel",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbProxyModifier",
                FIELDS,
                __hkbProxyModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbProxyModifier>,
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
