use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCharacterProxyCinfo`
/// -         version: `1`
/// -       signature: `0x586d97b2`
/// -          size: 144(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCharacterProxyCinfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCharacterControllerCinfo,
    /// # C++ Info
    /// -          name: `position`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_position: Vector4,
    /// # C++ Info
    /// -          name: `velocity`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_velocity: Vector4,
    /// # C++ Info
    /// -          name: `dynamicFriction`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_dynamicFriction: f32,
    /// # C++ Info
    /// -          name: `staticFriction`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_staticFriction: f32,
    /// # C++ Info
    /// -          name: `keepContactTolerance`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keepContactTolerance: f32,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `extraUpStaticFriction`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extraUpStaticFriction: f32,
    /// # C++ Info
    /// -          name: `extraDownStaticFriction`(ctype: `hkReal`)
    /// -        offset:  84(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extraDownStaticFriction: f32,
    /// # C++ Info
    /// -          name: `shapePhantom`(ctype: `struct hkpShapePhantom*`)
    /// -        offset:  88(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shapePhantom: Pointer,
    /// # C++ Info
    /// -          name: `keepDistance`(ctype: `hkReal`)
    /// -        offset:  92(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keepDistance: f32,
    /// # C++ Info
    /// -          name: `contactAngleSensitivity`(ctype: `hkReal`)
    /// -        offset:  96(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_contactAngleSensitivity: f32,
    /// # C++ Info
    /// -          name: `userPlanes`(ctype: `hkUint32`)
    /// -        offset: 100(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_userPlanes: u32,
    /// # C++ Info
    /// -          name: `maxCharacterSpeedForSolver`(ctype: `hkReal`)
    /// -        offset: 104(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCharacterSpeedForSolver: f32,
    /// # C++ Info
    /// -          name: `characterStrength`(ctype: `hkReal`)
    /// -        offset: 108(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_characterStrength: f32,
    /// # C++ Info
    /// -          name: `characterMass`(ctype: `hkReal`)
    /// -        offset: 112(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_characterMass: f32,
    /// # C++ Info
    /// -          name: `maxSlope`(ctype: `hkReal`)
    /// -        offset: 116(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSlope: f32,
    /// # C++ Info
    /// -          name: `penetrationRecoverySpeed`(ctype: `hkReal`)
    /// -        offset: 120(x86)/124(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_penetrationRecoverySpeed: f32,
    /// # C++ Info
    /// -          name: `maxCastIterations`(ctype: `hkInt32`)
    /// -        offset: 124(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxCastIterations: i32,
    /// # C++ Info
    /// -          name: `refreshManifoldInCheckSupport`(ctype: `hkBool`)
    /// -        offset: 128(x86)/132(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_refreshManifoldInCheckSupport: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCharacterProxyCinfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCharacterProxyCinfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1483577266u32)
        }
    }
    impl __serde::Serialize for hkpCharacterProxyCinfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1483577266u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCharacterProxyCinfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("velocity", &self.m_velocity)?;
            serializer.serialize_field("dynamicFriction", &self.m_dynamicFriction)?;
            serializer.serialize_field("staticFriction", &self.m_staticFriction)?;
            serializer
                .serialize_field("keepContactTolerance", &self.m_keepContactTolerance)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer
                .serialize_field(
                    "extraUpStaticFriction",
                    &self.m_extraUpStaticFriction,
                )?;
            serializer
                .serialize_field(
                    "extraDownStaticFriction",
                    &self.m_extraDownStaticFriction,
                )?;
            serializer.serialize_field("shapePhantom", &self.m_shapePhantom)?;
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
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
