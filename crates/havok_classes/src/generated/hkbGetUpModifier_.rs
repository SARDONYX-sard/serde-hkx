use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbGetUpModifier`
/// -         version: `2`
/// -       signature: `0x61cb7ac0`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbGetUpModifier<'a> {
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
    /// -          name: `groundNormal`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_groundNormal: Vector4,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `alignWithGroundDuration`(ctype: `hkReal`)
    /// -        offset:  68(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_alignWithGroundDuration: f32,
    /// # C++ Info
    /// -          name: `rootBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  72(x86)/104(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_rootBoneIndex: i16,
    /// # C++ Info
    /// -          name: `otherBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  74(x86)/106(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_otherBoneIndex: i16,
    /// # C++ Info
    /// -          name: `anotherBoneIndex`(ctype: `hkInt16`)
    /// -        offset:  76(x86)/108(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_anotherBoneIndex: i16,
    /// # C++ Info
    /// -          name: `timeSinceBegin`(ctype: `hkReal`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeSinceBegin: f32,
    /// # C++ Info
    /// -          name: `timeStep`(ctype: `hkReal`)
    /// -        offset:  84(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timeStep: f32,
    /// # C++ Info
    /// -          name: `initNextModify`(ctype: `hkBool`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_initNextModify: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbGetUpModifier<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbGetUpModifier"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1640725184u32)
        }
    }
    impl<'a> __serde::Serialize for hkbGetUpModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbGetUpModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("groundNormal", &self.m_groundNormal)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "alignWithGroundDuration",
                    &self.m_alignWithGroundDuration,
                )?;
            serializer.serialize_field("rootBoneIndex", &self.m_rootBoneIndex)?;
            serializer.serialize_field("otherBoneIndex", &self.m_otherBoneIndex)?;
            serializer.serialize_field("anotherBoneIndex", &self.m_anotherBoneIndex)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.skip_field("timeSinceBegin", &self.m_timeSinceBegin)?;
            serializer.skip_field("timeStep", &self.m_timeStep)?;
            serializer.skip_field("initNextModify", &self.m_initNextModify)?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 7usize].as_slice())?;
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
