use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLinearParametricCurve`
/// -         version: `0`
/// -       signature: `0xd7b3be03`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinearParametricCurve {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpParametricCurve,
    /// # C++ Info
    /// -          name: `smoothingFactor`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_smoothingFactor: f32,
    /// # C++ Info
    /// -          name: `closedLoop`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_closedLoop: bool,
    /// # C++ Info
    /// -          name: `dirNotParallelToTangentAlongWholePath`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_dirNotParallelToTangentAlongWholePath: Vector4,
    /// # C++ Info
    /// -          name: `points`(ctype: `hkArray<hkVector4>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_points: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `distance`(ctype: `hkArray<hkReal>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_distance: Vec<f32>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpLinearParametricCurve {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpLinearParametricCurve"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3618881027u32)
        }
    }
    impl __serde::Serialize for hkpLinearParametricCurve {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpLinearParametricCurve", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("smoothingFactor", &self.m_smoothingFactor)?;
            serializer.serialize_field("closedLoop", &self.m_closedLoop)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer
                .serialize_field(
                    "dirNotParallelToTangentAlongWholePath",
                    &self.m_dirNotParallelToTangentAlongWholePath,
                )?;
            serializer.serialize_array_meta_field("points", &self.m_points)?;
            serializer.serialize_array_meta_field("distance", &self.m_distance)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_array_field("points", &self.m_points)?;
            serializer.serialize_array_field("distance", &self.m_distance)?;
            serializer.end()
        }
    }
};
