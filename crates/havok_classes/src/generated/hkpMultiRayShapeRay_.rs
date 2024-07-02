use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMultiRayShapeRay`
/// -         version: `0`
/// -       signature: `0xffdc0b65`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMultiRayShapeRay {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `start`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_start: Vector4,
    /// # C++ Info
    /// -          name: `end`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_end: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMultiRayShapeRay {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMultiRayShapeRay"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4292610917u32)
        }
    }
    impl __serde::Serialize for hkpMultiRayShapeRay {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(4292610917u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMultiRayShapeRay", class_meta)?;
            serializer.serialize_field("start", &self.m_start)?;
            serializer.serialize_field("end", &self.m_end)?;
            serializer.end()
        }
    }
};
