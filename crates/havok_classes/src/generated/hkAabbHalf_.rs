use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkAabbHalf`
/// -         version: `0`
/// -       signature: `0x1d716a17`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkAabbHalf {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkUint16[6]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_data: [u16; 6usize],
    /// # C++ Info
    /// -          name: `extras`(ctype: `hkUint16[2]`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_extras: [u16; 2usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkAabbHalf {
        #[inline]
        fn name(&self) -> &'static str {
            "hkAabbHalf"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(493971991u32)
        }
    }
    impl __serde::Serialize for hkAabbHalf {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(493971991u32)));
            let mut serializer = __serializer
                .serialize_struct("hkAabbHalf", class_meta)?;
            serializer.serialize_field("data", &self.m_data.as_slice())?;
            serializer.serialize_field("extras", &self.m_extras.as_slice())?;
            serializer.end()
        }
    }
};
