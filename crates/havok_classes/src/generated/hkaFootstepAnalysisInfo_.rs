use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkaFootstepAnalysisInfo`
/// -         version: `1`
/// -       signature: `0x824faf75`
/// -          size: 152(x86)/208(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaFootstepAnalysisInfo {
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
    /// -          name: `name`(ctype: `hkArray<hkChar>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_name: Vec<char>,
    /// # C++ Info
    /// -          name: `nameStrike`(ctype: `hkArray<hkChar>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nameStrike: Vec<char>,
    /// # C++ Info
    /// -          name: `nameLift`(ctype: `hkArray<hkChar>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nameLift: Vec<char>,
    /// # C++ Info
    /// -          name: `nameLock`(ctype: `hkArray<hkChar>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nameLock: Vec<char>,
    /// # C++ Info
    /// -          name: `nameUnlock`(ctype: `hkArray<hkChar>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nameUnlock: Vec<char>,
    /// # C++ Info
    /// -          name: `minPos`(ctype: `hkArray<hkReal>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_minPos: Vec<f32>,
    /// # C++ Info
    /// -          name: `maxPos`(ctype: `hkArray<hkReal>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_maxPos: Vec<f32>,
    /// # C++ Info
    /// -          name: `minVel`(ctype: `hkArray<hkReal>`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_minVel: Vec<f32>,
    /// # C++ Info
    /// -          name: `maxVel`(ctype: `hkArray<hkReal>`)
    /// -        offset: 104(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_maxVel: Vec<f32>,
    /// # C++ Info
    /// -          name: `allBonesDown`(ctype: `hkArray<hkReal>`)
    /// -        offset: 116(x86)/160(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_allBonesDown: Vec<f32>,
    /// # C++ Info
    /// -          name: `anyBonesDown`(ctype: `hkArray<hkReal>`)
    /// -        offset: 128(x86)/176(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_anyBonesDown: Vec<f32>,
    /// # C++ Info
    /// -          name: `posTol`(ctype: `hkReal`)
    /// -        offset: 140(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_posTol: f32,
    /// # C++ Info
    /// -          name: `velTol`(ctype: `hkReal`)
    /// -        offset: 144(x86)/196(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_velTol: f32,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset: 148(x86)/200(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkaFootstepAnalysisInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaFootstepAnalysisInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2186260341u32)
        }
    }
    impl __serde::Serialize for hkaFootstepAnalysisInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(2186260341u32)));
            let mut serializer = __serializer
                .serialize_struct("hkaFootstepAnalysisInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("name", &self.m_name)?;
            serializer.serialize_array_meta_field("nameStrike", &self.m_nameStrike)?;
            serializer.serialize_array_meta_field("nameLift", &self.m_nameLift)?;
            serializer.serialize_array_meta_field("nameLock", &self.m_nameLock)?;
            serializer.serialize_array_meta_field("nameUnlock", &self.m_nameUnlock)?;
            serializer.serialize_array_meta_field("minPos", &self.m_minPos)?;
            serializer.serialize_array_meta_field("maxPos", &self.m_maxPos)?;
            serializer.serialize_array_meta_field("minVel", &self.m_minVel)?;
            serializer.serialize_array_meta_field("maxVel", &self.m_maxVel)?;
            serializer.serialize_array_meta_field("allBonesDown", &self.m_allBonesDown)?;
            serializer.serialize_array_meta_field("anyBonesDown", &self.m_anyBonesDown)?;
            serializer.serialize_field("posTol", &self.m_posTol)?;
            serializer.serialize_field("velTol", &self.m_velTol)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("name", &self.m_name)?;
            serializer.serialize_array_field("nameStrike", &self.m_nameStrike)?;
            serializer.serialize_array_field("nameLift", &self.m_nameLift)?;
            serializer.serialize_array_field("nameLock", &self.m_nameLock)?;
            serializer.serialize_array_field("nameUnlock", &self.m_nameUnlock)?;
            serializer.serialize_array_field("minPos", &self.m_minPos)?;
            serializer.serialize_array_field("maxPos", &self.m_maxPos)?;
            serializer.serialize_array_field("minVel", &self.m_minVel)?;
            serializer.serialize_array_field("maxVel", &self.m_maxVel)?;
            serializer.serialize_array_field("allBonesDown", &self.m_allBonesDown)?;
            serializer.serialize_array_field("anyBonesDown", &self.m_anyBonesDown)?;
            serializer.end()
        }
    }
};
