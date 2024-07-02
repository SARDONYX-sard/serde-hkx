use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMotionState`
/// -         version: `1`
/// -       signature: `0x5797386e`
/// -          size: 176(x86)/176(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMotionState {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkTransform`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Transform,
    /// # C++ Info
    /// -          name: `sweptTransform`(ctype: `struct hkSweptTransform`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:  80(x86)/ 80(x86_64)
    ///
    pub m_sweptTransform: hkSweptTransform,
    /// # C++ Info
    /// -          name: `deltaAngle`(ctype: `hkVector4`)
    /// -        offset: 144(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_deltaAngle: Vector4,
    /// # C++ Info
    /// -          name: `objectRadius`(ctype: `hkReal`)
    /// -        offset: 160(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_objectRadius: f32,
    /// # C++ Info
    /// -          name: `linearDamping`(ctype: `hkHalf`)
    /// -        offset: 164(x86)/164(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_linearDamping: f16,
    /// # C++ Info
    /// -          name: `angularDamping`(ctype: `hkHalf`)
    /// -        offset: 166(x86)/166(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_angularDamping: f16,
    /// # C++ Info
    /// -          name: `timeFactor`(ctype: `hkHalf`)
    /// -        offset: 168(x86)/168(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_timeFactor: f16,
    /// # C++ Info
    /// -          name: `maxLinearVelocity`(ctype: `hkUint8`)
    /// -        offset: 170(x86)/170(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_maxLinearVelocity: u8,
    /// # C++ Info
    /// -          name: `maxAngularVelocity`(ctype: `hkUint8`)
    /// -        offset: 171(x86)/171(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_maxAngularVelocity: u8,
    /// # C++ Info
    /// -          name: `deactivationClass`(ctype: `hkUint8`)
    /// -        offset: 172(x86)/172(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_deactivationClass: u8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMotionState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMotionState"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1469528174u32)
        }
    }
    impl __serde::Serialize for hkMotionState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1469528174u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMotionState", class_meta)?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.serialize_field("sweptTransform", &self.m_sweptTransform)?;
            serializer.serialize_field("deltaAngle", &self.m_deltaAngle)?;
            serializer.serialize_field("objectRadius", &self.m_objectRadius)?;
            serializer.serialize_field("linearDamping", &self.m_linearDamping)?;
            serializer.serialize_field("angularDamping", &self.m_angularDamping)?;
            serializer.serialize_field("timeFactor", &self.m_timeFactor)?;
            serializer.serialize_field("maxLinearVelocity", &self.m_maxLinearVelocity)?;
            serializer
                .serialize_field("maxAngularVelocity", &self.m_maxAngularVelocity)?;
            serializer.serialize_field("deactivationClass", &self.m_deactivationClass)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
