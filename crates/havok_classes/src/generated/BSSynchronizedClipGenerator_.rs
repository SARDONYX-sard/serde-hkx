use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSSynchronizedClipGenerator`
/// -         version: `1`
/// -       signature: `0xd83bea64`
/// -          size: 256(x86)/304(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSSynchronizedClipGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `pClipGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_pClipGenerator: Pointer,
    /// # C++ Info
    /// -          name: `SyncAnimPrefix`(ctype: `char*`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_SyncAnimPrefix: CString<'a>,
    /// # C++ Info
    /// -          name: `bSyncClipIgnoreMarkPlacement`(ctype: `hkBool`)
    /// -        offset:  56(x86)/ 96(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bSyncClipIgnoreMarkPlacement: bool,
    /// # C++ Info
    /// -          name: `fGetToMarkTime`(ctype: `hkReal`)
    /// -        offset:  60(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fGetToMarkTime: f32,
    /// # C++ Info
    /// -          name: `fMarkErrorThreshold`(ctype: `hkReal`)
    /// -        offset:  64(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fMarkErrorThreshold: f32,
    /// # C++ Info
    /// -          name: `bLeadCharacter`(ctype: `hkBool`)
    /// -        offset:  68(x86)/108(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bLeadCharacter: bool,
    /// # C++ Info
    /// -          name: `bReorientSupportChar`(ctype: `hkBool`)
    /// -        offset:  69(x86)/109(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bReorientSupportChar: bool,
    /// # C++ Info
    /// -          name: `bApplyMotionFromRoot`(ctype: `hkBool`)
    /// -        offset:  70(x86)/110(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_bApplyMotionFromRoot: bool,
    /// # C++ Info
    /// -          name: `pSyncScene`(ctype: `void*`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pSyncScene: Pointer,
    /// # C++ Info
    /// -          name: `StartMarkWS`(ctype: `hkQsTransform`)
    /// -        offset:  80(x86)/128(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_StartMarkWS: QsTransform,
    /// # C++ Info
    /// -          name: `EndMarkWS`(ctype: `hkQsTransform`)
    /// -        offset: 128(x86)/176(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_EndMarkWS: QsTransform,
    /// # C++ Info
    /// -          name: `StartMarkMS`(ctype: `hkQsTransform`)
    /// -        offset: 176(x86)/224(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_StartMarkMS: QsTransform,
    /// # C++ Info
    /// -          name: `fCurrentLerp`(ctype: `hkReal`)
    /// -        offset: 224(x86)/272(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fCurrentLerp: f32,
    /// # C++ Info
    /// -          name: `pLocalSyncBinding`(ctype: `void*`)
    /// -        offset: 228(x86)/280(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pLocalSyncBinding: Pointer,
    /// # C++ Info
    /// -          name: `pEventMap`(ctype: `void*`)
    /// -        offset: 232(x86)/288(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_pEventMap: Pointer,
    /// # C++ Info
    /// -          name: `sAnimationBindingIndex`(ctype: `hkInt16`)
    /// -        offset: 236(x86)/296(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_sAnimationBindingIndex: i16,
    /// # C++ Info
    /// -          name: `bAtMark`(ctype: `hkBool`)
    /// -        offset: 238(x86)/298(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bAtMark: bool,
    /// # C++ Info
    /// -          name: `bAllCharactersInScene`(ctype: `hkBool`)
    /// -        offset: 239(x86)/299(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bAllCharactersInScene: bool,
    /// # C++ Info
    /// -          name: `bAllCharactersAtMarks`(ctype: `hkBool`)
    /// -        offset: 240(x86)/300(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bAllCharactersAtMarks: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSSynchronizedClipGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSSynchronizedClipGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3627805284u32)
        }
    }
    impl<'a> _serde::Serialize for BSSynchronizedClipGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3627805284u32)));
            let mut serializer = __serializer
                .serialize_struct("BSSynchronizedClipGenerator", class_meta)?;
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
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("pClipGenerator", &self.m_pClipGenerator)?;
            serializer
                .serialize_cstring_meta_field("SyncAnimPrefix", &self.m_SyncAnimPrefix)?;
            serializer
                .serialize_field(
                    "bSyncClipIgnoreMarkPlacement",
                    &self.m_bSyncClipIgnoreMarkPlacement,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("fGetToMarkTime", &self.m_fGetToMarkTime)?;
            serializer
                .serialize_field("fMarkErrorThreshold", &self.m_fMarkErrorThreshold)?;
            serializer.serialize_field("bLeadCharacter", &self.m_bLeadCharacter)?;
            serializer
                .serialize_field("bReorientSupportChar", &self.m_bReorientSupportChar)?;
            serializer
                .serialize_field("bApplyMotionFromRoot", &self.m_bApplyMotionFromRoot)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.skip_field("pSyncScene", &self.m_pSyncScene)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("StartMarkWS", &self.m_StartMarkWS)?;
            serializer.skip_field("EndMarkWS", &self.m_EndMarkWS)?;
            serializer.skip_field("StartMarkMS", &self.m_StartMarkMS)?;
            serializer.skip_field("fCurrentLerp", &self.m_fCurrentLerp)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("pLocalSyncBinding", &self.m_pLocalSyncBinding)?;
            serializer.skip_field("pEventMap", &self.m_pEventMap)?;
            serializer
                .serialize_field(
                    "sAnimationBindingIndex",
                    &self.m_sAnimationBindingIndex,
                )?;
            serializer.skip_field("bAtMark", &self.m_bAtMark)?;
            serializer
                .skip_field("bAllCharactersInScene", &self.m_bAllCharactersInScene)?;
            serializer
                .skip_field("bAllCharactersAtMarks", &self.m_bAllCharactersAtMarks)?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_cstring_field("SyncAnimPrefix", &self.m_SyncAnimPrefix)?;
            serializer.end()
        }
    }
};
