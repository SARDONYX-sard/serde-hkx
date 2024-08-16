use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSSynchronizedClipGenerator`
/// - version: `1`
/// - signature: `0xd83bea64`
/// - size: `256`(x86)/`304`(x86_64)
/// -  vtable: `true`
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
    /// - name: `pClipGenerator`(ctype: `struct hkbGenerator*`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16`
    pub m_pClipGenerator: Pointer,
    /// # C++ Info
    /// - name: `SyncAnimPrefix`(ctype: `char*`)
    /// - offset: ` 52`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_SyncAnimPrefix: CString<'a>,
    /// # C++ Info
    /// - name: `bSyncClipIgnoreMarkPlacement`(ctype: `hkBool`)
    /// - offset: ` 56`(x86)/` 96`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_bSyncClipIgnoreMarkPlacement: bool,
    /// # C++ Info
    /// - name: `fGetToMarkTime`(ctype: `hkReal`)
    /// - offset: ` 60`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fGetToMarkTime: f32,
    /// # C++ Info
    /// - name: `fMarkErrorThreshold`(ctype: `hkReal`)
    /// - offset: ` 64`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fMarkErrorThreshold: f32,
    /// # C++ Info
    /// - name: `bLeadCharacter`(ctype: `hkBool`)
    /// - offset: ` 68`(x86)/`108`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_bLeadCharacter: bool,
    /// # C++ Info
    /// - name: `bReorientSupportChar`(ctype: `hkBool`)
    /// - offset: ` 69`(x86)/`109`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_bReorientSupportChar: bool,
    /// # C++ Info
    /// - name: `bApplyMotionFromRoot`(ctype: `hkBool`)
    /// - offset: ` 70`(x86)/`110`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_bApplyMotionFromRoot: bool,
    /// # C++ Info
    /// - name: `pSyncScene`(ctype: `void*`)
    /// - offset: ` 72`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_pSyncScene: Pointer,
    /// # C++ Info
    /// - name: `StartMarkWS`(ctype: `hkQsTransform`)
    /// - offset: ` 80`(x86)/`128`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_StartMarkWS: QsTransform,
    /// # C++ Info
    /// - name: `EndMarkWS`(ctype: `hkQsTransform`)
    /// - offset: `128`(x86)/`176`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_EndMarkWS: QsTransform,
    /// # C++ Info
    /// - name: `StartMarkMS`(ctype: `hkQsTransform`)
    /// - offset: `176`(x86)/`224`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_StartMarkMS: QsTransform,
    /// # C++ Info
    /// - name: `fCurrentLerp`(ctype: `hkReal`)
    /// - offset: `224`(x86)/`272`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_fCurrentLerp: f32,
    /// # C++ Info
    /// - name: `pLocalSyncBinding`(ctype: `void*`)
    /// - offset: `228`(x86)/`280`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_pLocalSyncBinding: Pointer,
    /// # C++ Info
    /// - name: `pEventMap`(ctype: `void*`)
    /// - offset: `232`(x86)/`288`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_pEventMap: Pointer,
    /// # C++ Info
    /// - name: `sAnimationBindingIndex`(ctype: `hkInt16`)
    /// - offset: `236`(x86)/`296`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_sAnimationBindingIndex: i16,
    /// # C++ Info
    /// - name: `bAtMark`(ctype: `hkBool`)
    /// - offset: `238`(x86)/`298`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_bAtMark: bool,
    /// # C++ Info
    /// - name: `bAllCharactersInScene`(ctype: `hkBool`)
    /// - offset: `239`(x86)/`299`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_bAllCharactersInScene: bool,
    /// # C++ Info
    /// - name: `bAllCharactersAtMarks`(ctype: `hkBool`)
    /// - offset: `240`(x86)/`300`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
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
            _serde::__private::Signature::new(0xd83bea64)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.push(self.m_pClipGenerator.get());
            v.push(self.m_pSyncScene.get());
            v.push(self.m_pLocalSyncBinding.get());
            v.push(self.m_pEventMap.get());
            v
        }
    }
    impl<'a> _serde::Serialize for BSSynchronizedClipGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd83bea64)));
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
            serializer
                .skip_fixed_array_field(
                    "padNode",
                    self.parent.parent.m_padNode.as_slice(),
                )?;
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSSynchronizedClipGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_pClipGenerator,
                m_SyncAnimPrefix,
                m_bSyncClipIgnoreMarkPlacement,
                m_fGetToMarkTime,
                m_fMarkErrorThreshold,
                m_bLeadCharacter,
                m_bReorientSupportChar,
                m_bApplyMotionFromRoot,
                m_sAnimationBindingIndex,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "pClipGenerator" => Ok(__Field::m_pClipGenerator),
                        "SyncAnimPrefix" => Ok(__Field::m_SyncAnimPrefix),
                        "bSyncClipIgnoreMarkPlacement" => {
                            Ok(__Field::m_bSyncClipIgnoreMarkPlacement)
                        }
                        "fGetToMarkTime" => Ok(__Field::m_fGetToMarkTime),
                        "fMarkErrorThreshold" => Ok(__Field::m_fMarkErrorThreshold),
                        "bLeadCharacter" => Ok(__Field::m_bLeadCharacter),
                        "bReorientSupportChar" => Ok(__Field::m_bReorientSupportChar),
                        "bApplyMotionFromRoot" => Ok(__Field::m_bApplyMotionFromRoot),
                        "sAnimationBindingIndex" => Ok(__Field::m_sAnimationBindingIndex),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __BSSynchronizedClipGeneratorVisitor<'de> {
                marker: _serde::__private::PhantomData<BSSynchronizedClipGenerator<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __BSSynchronizedClipGeneratorVisitor<'de> {
                type Value = BSSynchronizedClipGenerator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSSynchronizedClipGenerator",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_pClipGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_SyncAnimPrefix: _serde::__private::Option<CString<'de>> = _serde::__private::None;
                    let mut m_bSyncClipIgnoreMarkPlacement: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_fGetToMarkTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fMarkErrorThreshold: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bLeadCharacter: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bReorientSupportChar: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bApplyMotionFromRoot: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_pSyncScene: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_StartMarkWS: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_EndMarkWS: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_StartMarkMS: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_fCurrentLerp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_pLocalSyncBinding: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_pEventMap: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_sAnimationBindingIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_bAtMark: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bAllCharactersInScene: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bAllCharactersAtMarks: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_pClipGenerator) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pClipGenerator",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 8usize)?;
                                m_pClipGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_SyncAnimPrefix) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SyncAnimPrefix",
                                        ),
                                    );
                                }
                                m_SyncAnimPrefix = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bSyncClipIgnoreMarkPlacement,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bSyncClipIgnoreMarkPlacement",
                                        ),
                                    );
                                }
                                m_bSyncClipIgnoreMarkPlacement = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_fGetToMarkTime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fGetToMarkTime",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 3usize, 3usize)?;
                                m_fGetToMarkTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fMarkErrorThreshold,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fMarkErrorThreshold",
                                        ),
                                    );
                                }
                                m_fMarkErrorThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_bLeadCharacter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bLeadCharacter",
                                        ),
                                    );
                                }
                                m_bLeadCharacter = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bReorientSupportChar,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bReorientSupportChar",
                                        ),
                                    );
                                }
                                m_bReorientSupportChar = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bApplyMotionFromRoot,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bApplyMotionFromRoot",
                                        ),
                                    );
                                }
                                m_bApplyMotionFromRoot = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_pSyncScene) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pSyncScene",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 1usize)?;
                                m_pSyncScene = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_StartMarkWS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "StartMarkWS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 8usize)?;
                                m_StartMarkWS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_EndMarkWS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "EndMarkWS",
                                        ),
                                    );
                                }
                                m_EndMarkWS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_StartMarkMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "StartMarkMS",
                                        ),
                                    );
                                }
                                m_StartMarkMS = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_fCurrentLerp) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fCurrentLerp",
                                        ),
                                    );
                                }
                                m_fCurrentLerp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pLocalSyncBinding,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pLocalSyncBinding",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_pLocalSyncBinding = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_pEventMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pEventMap",
                                        ),
                                    );
                                }
                                m_pEventMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sAnimationBindingIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sAnimationBindingIndex",
                                        ),
                                    );
                                }
                                m_sAnimationBindingIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_bAtMark) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bAtMark",
                                        ),
                                    );
                                }
                                m_bAtMark = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bAllCharactersInScene,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bAllCharactersInScene",
                                        ),
                                    );
                                }
                                m_bAllCharactersInScene = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_bAllCharactersAtMarks,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bAllCharactersAtMarks",
                                        ),
                                    );
                                }
                                m_bAllCharactersAtMarks = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 15usize, 3usize)?;
                    let m_pClipGenerator = match m_pClipGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pClipGenerator",
                                ),
                            );
                        }
                    };
                    let m_SyncAnimPrefix = match m_SyncAnimPrefix {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "SyncAnimPrefix",
                                ),
                            );
                        }
                    };
                    let m_bSyncClipIgnoreMarkPlacement = match m_bSyncClipIgnoreMarkPlacement {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bSyncClipIgnoreMarkPlacement",
                                ),
                            );
                        }
                    };
                    let m_fGetToMarkTime = match m_fGetToMarkTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fGetToMarkTime",
                                ),
                            );
                        }
                    };
                    let m_fMarkErrorThreshold = match m_fMarkErrorThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fMarkErrorThreshold",
                                ),
                            );
                        }
                    };
                    let m_bLeadCharacter = match m_bLeadCharacter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bLeadCharacter",
                                ),
                            );
                        }
                    };
                    let m_bReorientSupportChar = match m_bReorientSupportChar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bReorientSupportChar",
                                ),
                            );
                        }
                    };
                    let m_bApplyMotionFromRoot = match m_bApplyMotionFromRoot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bApplyMotionFromRoot",
                                ),
                            );
                        }
                    };
                    let m_pSyncScene = match m_pSyncScene {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pSyncScene",
                                ),
                            );
                        }
                    };
                    let m_StartMarkWS = match m_StartMarkWS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "StartMarkWS",
                                ),
                            );
                        }
                    };
                    let m_EndMarkWS = match m_EndMarkWS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "EndMarkWS",
                                ),
                            );
                        }
                    };
                    let m_StartMarkMS = match m_StartMarkMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "StartMarkMS",
                                ),
                            );
                        }
                    };
                    let m_fCurrentLerp = match m_fCurrentLerp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fCurrentLerp",
                                ),
                            );
                        }
                    };
                    let m_pLocalSyncBinding = match m_pLocalSyncBinding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pLocalSyncBinding",
                                ),
                            );
                        }
                    };
                    let m_pEventMap = match m_pEventMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pEventMap",
                                ),
                            );
                        }
                    };
                    let m_sAnimationBindingIndex = match m_sAnimationBindingIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sAnimationBindingIndex",
                                ),
                            );
                        }
                    };
                    let m_bAtMark = match m_bAtMark {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bAtMark"),
                            );
                        }
                    };
                    let m_bAllCharactersInScene = match m_bAllCharactersInScene {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bAllCharactersInScene",
                                ),
                            );
                        }
                    };
                    let m_bAllCharactersAtMarks = match m_bAllCharactersAtMarks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bAllCharactersAtMarks",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSSynchronizedClipGenerator {
                        __ptr,
                        parent,
                        m_pClipGenerator,
                        m_SyncAnimPrefix,
                        m_bSyncClipIgnoreMarkPlacement,
                        m_fGetToMarkTime,
                        m_fMarkErrorThreshold,
                        m_bLeadCharacter,
                        m_bReorientSupportChar,
                        m_bApplyMotionFromRoot,
                        m_pSyncScene,
                        m_StartMarkWS,
                        m_EndMarkWS,
                        m_StartMarkMS,
                        m_fCurrentLerp,
                        m_pLocalSyncBinding,
                        m_pEventMap,
                        m_sAnimationBindingIndex,
                        m_bAtMark,
                        m_bAllCharactersInScene,
                        m_bAllCharactersAtMarks,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_pClipGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_SyncAnimPrefix: _serde::__private::Option<CString<'de>> = _serde::__private::None;
                    let mut m_bSyncClipIgnoreMarkPlacement: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_fGetToMarkTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fMarkErrorThreshold: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bLeadCharacter: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bReorientSupportChar: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bApplyMotionFromRoot: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_sAnimationBindingIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pClipGenerator => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pClipGenerator) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pClipGenerator",
                                        ),
                                    );
                                }
                                m_pClipGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_SyncAnimPrefix => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_SyncAnimPrefix) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SyncAnimPrefix",
                                        ),
                                    );
                                }
                                m_SyncAnimPrefix = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bSyncClipIgnoreMarkPlacement => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_bSyncClipIgnoreMarkPlacement,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bSyncClipIgnoreMarkPlacement",
                                        ),
                                    );
                                }
                                m_bSyncClipIgnoreMarkPlacement = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fGetToMarkTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fGetToMarkTime) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fGetToMarkTime",
                                        ),
                                    );
                                }
                                m_fGetToMarkTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fMarkErrorThreshold => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fMarkErrorThreshold,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fMarkErrorThreshold",
                                        ),
                                    );
                                }
                                m_fMarkErrorThreshold = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bLeadCharacter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bLeadCharacter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bLeadCharacter",
                                        ),
                                    );
                                }
                                m_bLeadCharacter = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bReorientSupportChar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_bReorientSupportChar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bReorientSupportChar",
                                        ),
                                    );
                                }
                                m_bReorientSupportChar = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bApplyMotionFromRoot => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_bApplyMotionFromRoot,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bApplyMotionFromRoot",
                                        ),
                                    );
                                }
                                m_bApplyMotionFromRoot = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sAnimationBindingIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sAnimationBindingIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sAnimationBindingIndex",
                                        ),
                                    );
                                }
                                m_sAnimationBindingIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pClipGenerator = match m_pClipGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pClipGenerator",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_SyncAnimPrefix = match m_SyncAnimPrefix {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "SyncAnimPrefix",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bSyncClipIgnoreMarkPlacement = match m_bSyncClipIgnoreMarkPlacement {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bSyncClipIgnoreMarkPlacement",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fGetToMarkTime = match m_fGetToMarkTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fGetToMarkTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fMarkErrorThreshold = match m_fMarkErrorThreshold {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fMarkErrorThreshold",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bLeadCharacter = match m_bLeadCharacter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bLeadCharacter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bReorientSupportChar = match m_bReorientSupportChar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bReorientSupportChar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bApplyMotionFromRoot = match m_bApplyMotionFromRoot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bApplyMotionFromRoot",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sAnimationBindingIndex = match m_sAnimationBindingIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sAnimationBindingIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator { __ptr, parent };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(BSSynchronizedClipGenerator {
                        __ptr,
                        parent,
                        m_pClipGenerator,
                        m_SyncAnimPrefix,
                        m_bSyncClipIgnoreMarkPlacement,
                        m_fGetToMarkTime,
                        m_fMarkErrorThreshold,
                        m_bLeadCharacter,
                        m_bReorientSupportChar,
                        m_bApplyMotionFromRoot,
                        m_sAnimationBindingIndex,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "pClipGenerator",
                "SyncAnimPrefix",
                "bSyncClipIgnoreMarkPlacement",
                "fGetToMarkTime",
                "fMarkErrorThreshold",
                "bLeadCharacter",
                "bReorientSupportChar",
                "bApplyMotionFromRoot",
                "pSyncScene",
                "StartMarkWS",
                "EndMarkWS",
                "StartMarkMS",
                "fCurrentLerp",
                "pLocalSyncBinding",
                "pEventMap",
                "sAnimationBindingIndex",
                "bAtMark",
                "bAllCharactersInScene",
                "bAllCharactersAtMarks",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSSynchronizedClipGenerator",
                FIELDS,
                __BSSynchronizedClipGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSSynchronizedClipGenerator,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
