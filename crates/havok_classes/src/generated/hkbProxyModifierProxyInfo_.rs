use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbProxyModifierProxyInfo`
/// -         version: `0`
/// -       signature: `0x39de637e`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbProxyModifierProxyInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `dynamicFriction`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dynamicFriction: f32,
    /// # C++ Info
    /// -          name: `staticFriction`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticFriction: f32,
    /// # C++ Info
    /// -          name: `keepContactTolerance`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keepContactTolerance: f32,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `keepDistance`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keepDistance: f32,
    /// # C++ Info
    /// -          name: `contactAngleSensitivity`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contactAngleSensitivity: f32,
    /// # C++ Info
    /// -          name: `userPlanes`(ctype: `hkUint32`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_userPlanes: u32,
    /// # C++ Info
    /// -          name: `maxCharacterSpeedForSolver`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCharacterSpeedForSolver: f32,
    /// # C++ Info
    /// -          name: `characterStrength`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_characterStrength: f32,
    /// # C++ Info
    /// -          name: `characterMass`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_characterMass: f32,
    /// # C++ Info
    /// -          name: `maxSlope`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSlope: f32,
    /// # C++ Info
    /// -          name: `penetrationRecoverySpeed`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_penetrationRecoverySpeed: f32,
    /// # C++ Info
    /// -          name: `maxCastIterations`(ctype: `hkInt32`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCastIterations: i32,
    /// # C++ Info
    /// -          name: `refreshManifoldInCheckSupport`(ctype: `hkBool`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_refreshManifoldInCheckSupport: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbProxyModifierProxyInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbProxyModifierProxyInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(970875774u32)
        }
    }
    impl __serde::Serialize for hkbProxyModifierProxyInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(970875774u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbProxyModifierProxyInfo", class_meta)?;
            serializer.serialize_field("dynamicFriction", &self.m_dynamicFriction)?;
            serializer.serialize_field("staticFriction", &self.m_staticFriction)?;
            serializer
                .serialize_field("keepContactTolerance", &self.m_keepContactTolerance)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("keepDistance", &self.m_keepDistance)?;
            serializer
                .serialize_field(
                    "contactAngleSensitivity",
                    &self.m_contactAngleSensitivity,
                )?;
            serializer.serialize_field("userPlanes", &self.m_userPlanes)?;
            serializer
                .serialize_field(
                    "maxCharacterSpeedForSolver",
                    &self.m_maxCharacterSpeedForSolver,
                )?;
            serializer.serialize_field("characterStrength", &self.m_characterStrength)?;
            serializer.serialize_field("characterMass", &self.m_characterMass)?;
            serializer.serialize_field("maxSlope", &self.m_maxSlope)?;
            serializer
                .serialize_field(
                    "penetrationRecoverySpeed",
                    &self.m_penetrationRecoverySpeed,
                )?;
            serializer.serialize_field("maxCastIterations", &self.m_maxCastIterations)?;
            serializer
                .serialize_field(
                    "refreshManifoldInCheckSupport",
                    &self.m_refreshManifoldInCheckSupport,
                )?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
