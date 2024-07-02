use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeletonLocalFrameOnBone`
/// -         version: `0`
/// -       signature: `0x52e8043`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeletonLocalFrameOnBone {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `localFrame`(ctype: `struct hkLocalFrame*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrame: Pointer,
    /// # C++ Info
    /// -          name: `boneIndex`(ctype: `hkInt32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_boneIndex: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSkeletonLocalFrameOnBone {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeletonLocalFrameOnBone"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(86933571u32)
        }
    }
    impl _serde::Serialize for hkaSkeletonLocalFrameOnBone {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(86933571u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeletonLocalFrameOnBone", class_meta)?;
            serializer.serialize_field("localFrame", &self.m_localFrame)?;
            serializer.serialize_field("boneIndex", &self.m_boneIndex)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
