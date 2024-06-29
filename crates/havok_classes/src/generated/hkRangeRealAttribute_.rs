use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkRangeRealAttribute`
/// -         version: `0`
/// -       signature: `0x949db24f`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkRangeRealAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `absmin`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absmin: f32,
    /// # C++ Info
    /// -          name: `absmax`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absmax: f32,
    /// # C++ Info
    /// -          name: `softmin`(ctype: `hkReal`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_softmin: f32,
    /// # C++ Info
    /// -          name: `softmax`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_softmax: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkRangeRealAttribute {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkRangeRealAttribute"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2493362767u32)
        }
    }
    impl __serde::Serialize for hkRangeRealAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkRangeRealAttribute", class_meta)?;
            serializer.serialize_field("absmin", &self.m_absmin)?;
            serializer.serialize_field("absmax", &self.m_absmax)?;
            serializer.serialize_field("softmin", &self.m_softmin)?;
            serializer.serialize_field("softmax", &self.m_softmax)?;
            serializer.end()
        }
    }
};
