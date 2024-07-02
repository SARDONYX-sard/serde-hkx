use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMoppCodeCodeInfo`
/// -         version: `0`
/// -       signature: `0xd8fdbb08`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMoppCodeCodeInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `offset`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_offset: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpMoppCodeCodeInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMoppCodeCodeInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3640507144u32)
        }
    }
    impl __serde::Serialize for hkpMoppCodeCodeInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3640507144u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMoppCodeCodeInfo", class_meta)?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.end()
        }
    }
};
