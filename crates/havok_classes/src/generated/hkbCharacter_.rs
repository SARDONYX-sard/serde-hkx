use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacter`
/// -         version: `2`
/// -       signature: `0x3088a5c5`
/// -          size:  88(x86)/160(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacter<'a> {
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
    /// -          name: `nearbyCharacters`(ctype: `hkArray<hkbCharacter*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_nearbyCharacters: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `currentLod`(ctype: `hkInt16`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_currentLod: i16,
    /// # C++ Info
    /// -          name: `numTracksInLod`(ctype: `hkInt16`)
    /// -        offset:  22(x86)/ 34(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numTracksInLod: i16,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `ragdollDriver`(ctype: `void*`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ragdollDriver: Pointer,
    /// # C++ Info
    /// -          name: `characterControllerDriver`(ctype: `void*`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_characterControllerDriver: Pointer,
    /// # C++ Info
    /// -          name: `footIkDriver`(ctype: `void*`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_footIkDriver: Pointer,
    /// # C++ Info
    /// -          name: `handIkDriver`(ctype: `void*`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_handIkDriver: Pointer,
    /// # C++ Info
    /// -          name: `setup`(ctype: `struct hkbCharacterSetup*`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_setup: Pointer,
    /// # C++ Info
    /// -          name: `behaviorGraph`(ctype: `struct hkbBehaviorGraph*`)
    /// -        offset:  48(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorGraph: Pointer,
    /// # C++ Info
    /// -          name: `projectData`(ctype: `struct hkbProjectData*`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_projectData: Pointer,
    /// # C++ Info
    /// -          name: `animationBindingSet`(ctype: `void*`)
    /// -        offset:  56(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_animationBindingSet: Pointer,
    /// # C++ Info
    /// -          name: `raycastInterface`(ctype: `void*`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_raycastInterface: Pointer,
    /// # C++ Info
    /// -          name: `world`(ctype: `void*`)
    /// -        offset:  64(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `NOT_OWNED|SERIALIZE_IGNORED`
    ///
    pub m_world: Pointer,
    /// # C++ Info
    /// -          name: `eventQueue`(ctype: `void*`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_eventQueue: Pointer,
    /// # C++ Info
    /// -          name: `worldFromModel`(ctype: `void*`)
    /// -        offset:  72(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_worldFromModel: Pointer,
    /// # C++ Info
    /// -          name: `poseLocal`(ctype: `hkSimpleArray<void>`)
    /// -        offset:  76(x86)/144(x86_64)
    /// -     type_size:   8(x86)/ 12(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_poseLocal: Vec<()>,
    /// # C++ Info
    /// -          name: `deleteWorldFromModel`(ctype: `hkBool`)
    /// -        offset:  84(x86)/156(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_deleteWorldFromModel: bool,
    /// # C++ Info
    /// -          name: `deletePoseLocal`(ctype: `hkBool`)
    /// -        offset:  85(x86)/157(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_deletePoseLocal: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbCharacter<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacter"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(814261701u32)
        }
    }
    impl<'a> __serde::Serialize for hkbCharacter<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(814261701u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacter", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "nearbyCharacters",
                    &self.m_nearbyCharacters,
                )?;
            serializer.serialize_field("currentLod", &self.m_currentLod)?;
            serializer.skip_field("numTracksInLod", &self.m_numTracksInLod)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.skip_field("ragdollDriver", &self.m_ragdollDriver)?;
            serializer
                .skip_field(
                    "characterControllerDriver",
                    &self.m_characterControllerDriver,
                )?;
            serializer.skip_field("footIkDriver", &self.m_footIkDriver)?;
            serializer.skip_field("handIkDriver", &self.m_handIkDriver)?;
            serializer.serialize_field("setup", &self.m_setup)?;
            serializer.serialize_field("behaviorGraph", &self.m_behaviorGraph)?;
            serializer.serialize_field("projectData", &self.m_projectData)?;
            serializer.skip_field("animationBindingSet", &self.m_animationBindingSet)?;
            serializer.skip_field("raycastInterface", &self.m_raycastInterface)?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("eventQueue", &self.m_eventQueue)?;
            serializer.skip_field("worldFromModel", &self.m_worldFromModel)?;
            serializer.skip_field("poseLocal", &self.m_poseLocal)?;
            serializer.skip_field("deleteWorldFromModel", &self.m_deleteWorldFromModel)?;
            serializer.skip_field("deletePoseLocal", &self.m_deletePoseLocal)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field("nearbyCharacters", &self.m_nearbyCharacters)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
