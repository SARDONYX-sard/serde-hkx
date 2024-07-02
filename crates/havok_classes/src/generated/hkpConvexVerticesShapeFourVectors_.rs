use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConvexVerticesShapeFourVectors`
/// -         version: `0`
/// -       signature: `0x3d80c5bf`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexVerticesShapeFourVectors {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `x`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_x: Vector4,
    /// # C++ Info
    /// -          name: `y`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_y: Vector4,
    /// # C++ Info
    /// -          name: `z`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_z: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConvexVerticesShapeFourVectors {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConvexVerticesShapeFourVectors"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1031849407u32)
        }
    }
    impl _serde::Serialize for hkpConvexVerticesShapeFourVectors {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1031849407u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexVerticesShapeFourVectors", class_meta)?;
            serializer.serialize_field("x", &self.m_x)?;
            serializer.serialize_field("y", &self.m_y)?;
            serializer.serialize_field("z", &self.m_z)?;
            serializer.end()
        }
    }
};
