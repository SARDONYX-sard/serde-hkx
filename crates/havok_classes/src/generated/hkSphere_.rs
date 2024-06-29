use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkSphere`
/// -         version: `0`
/// -       signature: `0x143dff99`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkSphere {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pos`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pos: Vector4,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkSphere {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkSphere"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(339607449u32)
        }
    }
    impl __serde::Serialize for hkSphere {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkSphere", class_meta)?;
            serializer.serialize_field("pos", &self.m_pos)?;
            serializer.end()
        }
    }
};
