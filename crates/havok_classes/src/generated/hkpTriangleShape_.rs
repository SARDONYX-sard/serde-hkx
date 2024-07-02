use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTriangleShape`
/// -         version: `0`
/// -       signature: `0x95ad1a25`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriangleShape {
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
    /// -          name: `weldingInfo`(ctype: `hkUint16`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_weldingInfo: u16,
    /// # C++ Info
    /// -          name: `weldingType`(ctype: `enum WeldingType`)
    /// -        offset:  22(x86)/ 42(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_weldingType: WeldingType,
    /// # C++ Info
    /// -          name: `isExtruded`(ctype: `hkUint8`)
    /// -        offset:  23(x86)/ 43(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isExtruded: u8,
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
    /// -          name: `vertexC`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_vertexC: Vector4,
    /// # C++ Info
    /// -          name: `extrusion`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_extrusion: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpTriangleShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriangleShape"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2511149605u32)
        }
    }
    impl __serde::Serialize for hkpTriangleShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2511149605u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriangleShape", class_meta)?;
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
            serializer.serialize_field("weldingInfo", &self.m_weldingInfo)?;
            serializer.serialize_field("weldingType", &self.m_weldingType)?;
            serializer.serialize_field("isExtruded", &self.m_isExtruded)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("vertexA", &self.m_vertexA)?;
            serializer.serialize_field("vertexB", &self.m_vertexB)?;
            serializer.serialize_field("vertexC", &self.m_vertexC)?;
            serializer.serialize_field("extrusion", &self.m_extrusion)?;
            serializer.end()
        }
    }
};
