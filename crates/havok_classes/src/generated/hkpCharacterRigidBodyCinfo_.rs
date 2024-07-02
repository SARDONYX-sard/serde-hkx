use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCharacterRigidBodyCinfo`
/// -         version: `0`
/// -       signature: `0x892f441`
/// -          size: 112(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCharacterRigidBodyCinfo {
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
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `position`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_position: Vector4,
    /// # C++ Info
    /// -          name: `rotation`(ctype: `hkQuaternion`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_rotation: Quaternion,
    /// # C++ Info
    /// -          name: `mass`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_mass: f32,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_friction: f32,
    /// # C++ Info
    /// -          name: `maxLinearVelocity`(ctype: `hkReal`)
    /// -        offset:  56(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxLinearVelocity: f32,
    /// # C++ Info
    /// -          name: `allowedPenetrationDepth`(ctype: `hkReal`)
    /// -        offset:  60(x86)/ 76(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_allowedPenetrationDepth: f32,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `maxSlope`(ctype: `hkReal`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSlope: f32,
    /// # C++ Info
    /// -          name: `maxForce`(ctype: `hkReal`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxForce: f32,
    /// # C++ Info
    /// -          name: `unweldingHeightOffsetFactor`(ctype: `hkReal`)
    /// -        offset:  88(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_unweldingHeightOffsetFactor: f32,
    /// # C++ Info
    /// -          name: `maxSpeedForSimplexSolver`(ctype: `hkReal`)
    /// -        offset:  92(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxSpeedForSimplexSolver: f32,
    /// # C++ Info
    /// -          name: `supportDistance`(ctype: `hkReal`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_supportDistance: f32,
    /// # C++ Info
    /// -          name: `hardSupportDistance`(ctype: `hkReal`)
    /// -        offset: 100(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_hardSupportDistance: f32,
    /// # C++ Info
    /// -          name: `vdbColor`(ctype: `hkInt32`)
    /// -        offset: 104(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_vdbColor: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCharacterRigidBodyCinfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCharacterRigidBodyCinfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(143848513u32)
        }
    }
    impl __serde::Serialize for hkpCharacterRigidBodyCinfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(143848513u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCharacterRigidBodyCinfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("rotation", &self.m_rotation)?;
            serializer.serialize_field("mass", &self.m_mass)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("maxLinearVelocity", &self.m_maxLinearVelocity)?;
            serializer
                .serialize_field(
                    "allowedPenetrationDepth",
                    &self.m_allowedPenetrationDepth,
                )?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("maxSlope", &self.m_maxSlope)?;
            serializer.serialize_field("maxForce", &self.m_maxForce)?;
            serializer
                .serialize_field(
                    "unweldingHeightOffsetFactor",
                    &self.m_unweldingHeightOffsetFactor,
                )?;
            serializer
                .serialize_field(
                    "maxSpeedForSimplexSolver",
                    &self.m_maxSpeedForSimplexSolver,
                )?;
            serializer.serialize_field("supportDistance", &self.m_supportDistance)?;
            serializer
                .serialize_field("hardSupportDistance", &self.m_hardSupportDistance)?;
            serializer.serialize_field("vdbColor", &self.m_vdbColor)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
