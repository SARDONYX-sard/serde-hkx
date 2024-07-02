use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkRangeInt32Attribute`
/// -         version: `0`
/// -       signature: `0x4846be29`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkRangeInt32Attribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `absmin`(ctype: `hkInt32`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absmin: i32,
    /// # C++ Info
    /// -          name: `absmax`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_absmax: i32,
    /// # C++ Info
    /// -          name: `softmin`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_softmin: i32,
    /// # C++ Info
    /// -          name: `softmax`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_softmax: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkRangeInt32Attribute {
        #[inline]
        fn name(&self) -> &'static str {
            "hkRangeInt32Attribute"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1212595753u32)
        }
    }
    impl __serde::Serialize for hkRangeInt32Attribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1212595753u32)));
            let mut serializer = __serializer
                .serialize_struct("hkRangeInt32Attribute", class_meta)?;
            serializer.serialize_field("absmin", &self.m_absmin)?;
            serializer.serialize_field("absmax", &self.m_absmax)?;
            serializer.serialize_field("softmin", &self.m_softmin)?;
            serializer.serialize_field("softmax", &self.m_softmax)?;
            serializer.end()
        }
    }
};
