use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbDetectCloseToGroundModifier`
/// -         version: `2`
/// -       signature: `0x981687b2`
/// -          size:  72(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbDetectCloseToGroundModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `closeToGroundEvent`(ctype: `struct hkbEventProperty`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_closeToGroundEvent: hkbEventProperty,
    /// # C++ Info
    /// -          name: `closeToGroundHeight`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_closeToGroundHeight: f32,
    /// # C++ Info
    /// -          name: `raycastDistanceDown`(ctype: `hkReal`)
    /// -        offset:  56(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:  60(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `boneIndex`(ctype: `hkInt16`)
    /// -        offset:  64(x86)/108(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_boneIndex: i16,
    /// # C++ Info
    /// -          name: `animBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  66(x86)/110(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_animBoneIndex: i16,
    /// # C++ Info
    /// -          name: `isCloseToGround`(ctype: `hkBool`)
    /// -        offset:  68(x86)/112(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isCloseToGround: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbDetectCloseToGroundModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbDetectCloseToGroundModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2551613362u32)
        }
    }
    impl<'a> __serde::Serialize for hkbDetectCloseToGroundModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbDetectCloseToGroundModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("closeToGroundEvent", &self.m_closeToGroundEvent)?;
            serializer
                .serialize_field("closeToGroundHeight", &self.m_closeToGroundHeight)?;
            serializer
                .serialize_field("raycastDistanceDown", &self.m_raycastDistanceDown)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.serialize_field("boneIndex", &self.m_boneIndex)?;
            serializer.serialize_field("animBoneIndex", &self.m_animBoneIndex)?;
            serializer.skip_field("isCloseToGround", &self.m_isCloseToGround)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
