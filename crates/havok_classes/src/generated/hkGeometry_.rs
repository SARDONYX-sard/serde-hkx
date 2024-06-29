use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkGeometry`
/// -         version: `0`
/// -       signature: `0x98dd8bdc`
/// -          size:  24(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkGeometry {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `vertices`(ctype: `hkArray<hkVector4>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vertices: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `triangles`(ctype: `hkArray<struct hkGeometryTriangle>`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_triangles: Vec<hkGeometryTriangle>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkGeometry {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkGeometry"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2564656092u32)
        }
    }
    impl __serde::Serialize for hkGeometry {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkGeometry", class_meta)?;
            serializer.serialize_array_meta_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_meta_field("triangles", &self.m_triangles)?;
            serializer.serialize_array_field("vertices", &self.m_vertices)?;
            serializer.serialize_array_field("triangles", &self.m_triangles)?;
            serializer.end()
        }
    }
};
