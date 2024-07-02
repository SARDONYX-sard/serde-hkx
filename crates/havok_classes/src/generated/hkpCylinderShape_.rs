use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCylinderShape`
/// -         version: `0`
/// -       signature: `0x3e463c3a`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCylinderShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConvexShape,
    /// # C++ Info
    /// -          name: `cylRadius`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cylRadius: f32,
    /// # C++ Info
    /// -          name: `cylBaseRadiusFactorForHeightFieldCollisions`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_cylBaseRadiusFactorForHeightFieldCollisions: f32,
    /// # C++ Info
    /// -          name: `vertexA`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vertexA: Vector4,
    /// # C++ Info
    /// -          name: `vertexB`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vertexB: Vector4,
    /// # C++ Info
    /// -          name: `perpendicular1`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_perpendicular1: Vector4,
    /// # C++ Info
    /// -          name: `perpendicular2`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_perpendicular2: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCylinderShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCylinderShape"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1044790330u32)
        }
    }
    impl __serde::Serialize for hkpCylinderShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1044790330u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpCylinderShape", class_meta)?;
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
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("radius", &self.parent.m_radius)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("cylRadius", &self.m_cylRadius)?;
            serializer
                .serialize_field(
                    "cylBaseRadiusFactorForHeightFieldCollisions",
                    &self.m_cylBaseRadiusFactorForHeightFieldCollisions,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("vertexA", &self.m_vertexA)?;
            serializer.serialize_field("vertexB", &self.m_vertexB)?;
            serializer.serialize_field("perpendicular1", &self.m_perpendicular1)?;
            serializer.serialize_field("perpendicular2", &self.m_perpendicular2)?;
            serializer.end()
        }
    }
};
