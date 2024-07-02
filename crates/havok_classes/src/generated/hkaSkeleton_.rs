use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaSkeleton`
/// -         version: `3`
/// -       signature: `0x366e8220`
/// -          size:  84(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSkeleton<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `parentIndices`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_parentIndices: Vec<i16>,
    /// # C++ Info
    /// -          name: `bones`(ctype: `hkArray<struct hkaBone>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_bones: Vec<hkaBone<'a>>,
    /// # C++ Info
    /// -          name: `referencePose`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_referencePose: Vec<QsTransform>,
    /// # C++ Info
    /// -          name: `referenceFloats`(ctype: `hkArray<hkReal>`)
    /// -        offset:  48(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_referenceFloats: Vec<f32>,
    /// # C++ Info
    /// -          name: `floatSlots`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_floatSlots: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `localFrames`(ctype: `hkArray<struct hkaSkeletonLocalFrameOnBone>`)
    /// -        offset:  72(x86)/104(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_localFrames: Vec<hkaSkeletonLocalFrameOnBone>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkaSkeleton<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSkeleton"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(913211936u32)
        }
    }
    impl<'a> __serde::Serialize for hkaSkeleton<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(913211936u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaSkeleton", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer
                .serialize_array_meta_field("parentIndices", &self.m_parentIndices)?;
            serializer.serialize_array_meta_field("bones", &self.m_bones)?;
            serializer
                .serialize_array_meta_field("referencePose", &self.m_referencePose)?;
            serializer
                .serialize_array_meta_field("referenceFloats", &self.m_referenceFloats)?;
            serializer.serialize_array_meta_field("floatSlots", &self.m_floatSlots)?;
            serializer.serialize_array_meta_field("localFrames", &self.m_localFrames)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("parentIndices", &self.m_parentIndices)?;
            serializer.serialize_array_field("bones", &self.m_bones)?;
            serializer.serialize_array_field("referencePose", &self.m_referencePose)?;
            serializer
                .serialize_array_field("referenceFloats", &self.m_referenceFloats)?;
            serializer.serialize_array_field("floatSlots", &self.m_floatSlots)?;
            serializer.serialize_array_field("localFrames", &self.m_localFrames)?;
            serializer.end()
        }
    }
};
