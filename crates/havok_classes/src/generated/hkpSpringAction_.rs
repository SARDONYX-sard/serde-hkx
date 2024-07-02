use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSpringAction`
/// -         version: `0`
/// -       signature: `0x88fc09fa`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSpringAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBinaryAction<'a>,
    /// # C++ Info
    /// -          name: `lastForce`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_lastForce: Vector4,
    /// # C++ Info
    /// -          name: `positionAinA`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_positionAinA: Vector4,
    /// # C++ Info
    /// -          name: `positionBinB`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_positionBinB: Vector4,
    /// # C++ Info
    /// -          name: `restLength`(ctype: `hkReal`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_restLength: f32,
    /// # C++ Info
    /// -          name: `strength`(ctype: `hkReal`)
    /// -        offset:  84(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_strength: f32,
    /// # C++ Info
    /// -          name: `damping`(ctype: `hkReal`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damping: f32,
    /// # C++ Info
    /// -          name: `onCompression`(ctype: `hkBool`)
    /// -        offset:  92(x86)/124(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_onCompression: bool,
    /// # C++ Info
    /// -          name: `onExtension`(ctype: `hkBool`)
    /// -        offset:  93(x86)/125(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_onExtension: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkpSpringAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSpringAction"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2298219002u32)
        }
    }
    impl<'a> __serde::Serialize for hkpSpringAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2298219002u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpSpringAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entityA", &self.parent.m_entityA)?;
            serializer.serialize_field("entityB", &self.parent.m_entityB)?;
            serializer.serialize_field("lastForce", &self.m_lastForce)?;
            serializer.serialize_field("positionAinA", &self.m_positionAinA)?;
            serializer.serialize_field("positionBinB", &self.m_positionBinB)?;
            serializer.serialize_field("restLength", &self.m_restLength)?;
            serializer.serialize_field("strength", &self.m_strength)?;
            serializer.serialize_field("damping", &self.m_damping)?;
            serializer.serialize_field("onCompression", &self.m_onCompression)?;
            serializer.serialize_field("onExtension", &self.m_onExtension)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
