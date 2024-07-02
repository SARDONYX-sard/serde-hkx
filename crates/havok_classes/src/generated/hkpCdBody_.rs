use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCdBody`
/// -         version: `1`
/// -       signature: `0x54a4b841`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCdBody {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `shapeKey`(ctype: `hkUint32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_shapeKey: u32,
    /// # C++ Info
    /// -          name: `motion`(ctype: `void*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_motion: Pointer,
    /// # C++ Info
    /// -          name: `parent`(ctype: `struct hkpCdBody*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_parent: Pointer,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpCdBody {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCdBody"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1420081217u32)
        }
    }
    impl __serde::Serialize for hkpCdBody {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(1420081217u32)));
            let mut serializer = __serializer.serialize_struct("hkpCdBody", class_meta)?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer.serialize_field("shapeKey", &self.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("motion", &self.m_motion)?;
            serializer.skip_field("parent", &self.m_parent)?;
            serializer.end()
        }
    }
};
