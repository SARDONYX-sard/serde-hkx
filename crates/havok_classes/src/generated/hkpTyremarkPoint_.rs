use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTyremarkPoint`
/// -         version: `0`
/// -       signature: `0x6bb7c5e8`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTyremarkPoint {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pointLeft`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointLeft: Vector4,
    /// # C++ Info
    /// -          name: `pointRight`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointRight: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpTyremarkPoint {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpTyremarkPoint"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1807205864u32)
        }
    }
    impl __serde::Serialize for hkpTyremarkPoint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpTyremarkPoint", class_meta)?;
            serializer.serialize_field("pointLeft", &self.m_pointLeft)?;
            serializer.serialize_field("pointRight", &self.m_pointRight)?;
            serializer.end()
        }
    }
};
