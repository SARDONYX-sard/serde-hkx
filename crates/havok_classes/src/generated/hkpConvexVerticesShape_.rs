use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConvexVerticesShape`
/// -         version: `3`
/// -       signature: `0x28726ad8`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexVerticesShape {
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
    /// -          name: `aabbHalfExtents`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbHalfExtents: Vector4,
    /// # C++ Info
    /// -          name: `aabbCenter`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_aabbCenter: Vector4,
    /// # C++ Info
    /// -          name: `rotatedVertices`(ctype: `hkArray<struct hkpConvexVerticesShapeFourVectors>`)
    /// -        offset:  64(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rotatedVertices: Vec<hkpConvexVerticesShapeFourVectors>,
    /// # C++ Info
    /// -          name: `numVertices`(ctype: `hkInt32`)
    /// -        offset:  76(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numVertices: i32,
    /// # C++ Info
    /// -          name: `externalObject`(ctype: `void*`)
    /// -        offset:  80(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_externalObject: Pointer,
    /// # C++ Info
    /// -          name: `getFaceNormals`(ctype: `void*`)
    /// -        offset:  84(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_getFaceNormals: Pointer,
    /// # C++ Info
    /// -          name: `planeEquations`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_planeEquations: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `connectivity`(ctype: `struct hkpConvexVerticesConnectivity*`)
    /// -        offset: 100(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_connectivity: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpConvexVerticesShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConvexVerticesShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(678587096u32)
        }
    }
    impl __serde::Serialize for hkpConvexVerticesShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexVerticesShape", class_meta)?;
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
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("aabbHalfExtents", &self.m_aabbHalfExtents)?;
            serializer.serialize_field("aabbCenter", &self.m_aabbCenter)?;
            serializer
                .serialize_array_meta_field("rotatedVertices", &self.m_rotatedVertices)?;
            serializer.serialize_field("numVertices", &self.m_numVertices)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("externalObject", &self.m_externalObject)?;
            serializer.skip_field("getFaceNormals", &self.m_getFaceNormals)?;
            serializer
                .serialize_array_meta_field("planeEquations", &self.m_planeEquations)?;
            serializer.serialize_field("connectivity", &self.m_connectivity)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field("rotatedVertices", &self.m_rotatedVertices)?;
            serializer.serialize_array_field("planeEquations", &self.m_planeEquations)?;
            serializer.end()
        }
    }
};
