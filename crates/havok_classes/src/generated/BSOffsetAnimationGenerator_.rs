use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSOffsetAnimationGenerator`
/// - version: `1`
/// - signature: `0xb8571122`
/// - size: `128`(x86)/`176`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSOffsetAnimationGenerator<'a> {
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
    /// - name: `pDefaultGenerator`(ctype: `struct hkbGenerator*`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16`
    pub m_pDefaultGenerator: Pointer,
    /// # C++ Info
    /// - name: `pOffsetClipGenerator`(ctype: `struct hkbGenerator*`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16`
    pub m_pOffsetClipGenerator: Pointer,
    /// # C++ Info
    /// - name: `fOffsetVariable`(ctype: `hkReal`)
    /// - offset: ` 68`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fOffsetVariable: f32,
    /// # C++ Info
    /// - name: `fOffsetRangeStart`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fOffsetRangeStart: f32,
    /// # C++ Info
    /// - name: `fOffsetRangeEnd`(ctype: `hkReal`)
    /// - offset: ` 76`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fOffsetRangeEnd: f32,
    /// # C++ Info
    /// - name: `BoneOffsetA`(ctype: `hkArray<void>`)
    /// - offset: ` 80`(x86)/`120`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_BoneOffsetA: Vec<()>,
    /// # C++ Info
    /// - name: `BoneIndexA`(ctype: `hkArray<void>`)
    /// - offset: ` 92`(x86)/`136`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_BoneIndexA: Vec<()>,
    /// # C++ Info
    /// - name: `fCurrentPercentage`(ctype: `hkReal`)
    /// - offset: `104`(x86)/`152`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_fCurrentPercentage: f32,
    /// # C++ Info
    /// - name: `iCurrentFrame`(ctype: `hkUint32`)
    /// - offset: `108`(x86)/`156`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_iCurrentFrame: u32,
    /// # C++ Info
    /// - name: `bZeroOffset`(ctype: `hkBool`)
    /// - offset: `112`(x86)/`160`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_bZeroOffset: bool,
    /// # C++ Info
    /// - name: `bOffsetValid`(ctype: `hkBool`)
    /// - offset: `113`(x86)/`161`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_bOffsetValid: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSOffsetAnimationGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSOffsetAnimationGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb8571122)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.push(self.m_pDefaultGenerator.get());
            v.push(self.m_pOffsetClipGenerator.get());
            v
        }
    }
    impl<'a> _serde::Serialize for BSOffsetAnimationGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb8571122)));
            let mut serializer = __serializer
                .serialize_struct(
                    "BSOffsetAnimationGenerator",
                    class_meta,
                    (128u64, 176u64),
                )?;
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
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer
                .skip_fixed_array_field(
                    "padNode",
                    self.parent.parent.m_padNode.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("pDefaultGenerator", &self.m_pDefaultGenerator)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field("pOffsetClipGenerator", &self.m_pOffsetClipGenerator)?;
            serializer.serialize_field("fOffsetVariable", &self.m_fOffsetVariable)?;
            serializer.serialize_field("fOffsetRangeStart", &self.m_fOffsetRangeStart)?;
            serializer.serialize_field("fOffsetRangeEnd", &self.m_fOffsetRangeEnd)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_array_field("BoneOffsetA", &self.m_BoneOffsetA, TypeSize::NonPtr)?;
            serializer
                .skip_array_field("BoneIndexA", &self.m_BoneIndexA, TypeSize::NonPtr)?;
            serializer.skip_field("fCurrentPercentage", &self.m_fCurrentPercentage)?;
            serializer.skip_field("iCurrentFrame", &self.m_iCurrentFrame)?;
            serializer.skip_field("bZeroOffset", &self.m_bZeroOffset)?;
            serializer.skip_field("bOffsetValid", &self.m_bOffsetValid)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSOffsetAnimationGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_pDefaultGenerator,
                m_pOffsetClipGenerator,
                m_fOffsetVariable,
                m_fOffsetRangeStart,
                m_fOffsetRangeEnd,
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
                        "pDefaultGenerator" => Ok(__Field::m_pDefaultGenerator),
                        "pOffsetClipGenerator" => Ok(__Field::m_pOffsetClipGenerator),
                        "fOffsetVariable" => Ok(__Field::m_fOffsetVariable),
                        "fOffsetRangeStart" => Ok(__Field::m_fOffsetRangeStart),
                        "fOffsetRangeEnd" => Ok(__Field::m_fOffsetRangeEnd),
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
            struct __BSOffsetAnimationGeneratorVisitor<'de> {
                marker: _serde::__private::PhantomData<BSOffsetAnimationGenerator<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __BSOffsetAnimationGeneratorVisitor<'de> {
                type Value = BSOffsetAnimationGenerator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSOffsetAnimationGenerator",
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
                    let mut m_pDefaultGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_pOffsetClipGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_fOffsetVariable: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fOffsetRangeStart: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fOffsetRangeEnd: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_BoneOffsetA: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_BoneIndexA: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_fCurrentPercentage: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_iCurrentFrame: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_bZeroOffset: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_bOffsetValid: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pDefaultGenerator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pDefaultGenerator",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 8usize)?;
                                m_pDefaultGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pOffsetClipGenerator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pOffsetClipGenerator",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 12usize, 8usize)?;
                                m_pOffsetClipGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_fOffsetVariable) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fOffsetVariable",
                                        ),
                                    );
                                }
                                m_fOffsetVariable = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fOffsetRangeStart,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fOffsetRangeStart",
                                        ),
                                    );
                                }
                                m_fOffsetRangeStart = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_fOffsetRangeEnd) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fOffsetRangeEnd",
                                        ),
                                    );
                                }
                                m_fOffsetRangeEnd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_BoneOffsetA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "BoneOffsetA",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_BoneOffsetA = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_BoneIndexA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "BoneIndexA",
                                        ),
                                    );
                                }
                                m_BoneIndexA = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fCurrentPercentage,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fCurrentPercentage",
                                        ),
                                    );
                                }
                                m_fCurrentPercentage = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_iCurrentFrame) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "iCurrentFrame",
                                        ),
                                    );
                                }
                                m_iCurrentFrame = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_bZeroOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bZeroOffset",
                                        ),
                                    );
                                }
                                m_bZeroOffset = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_bOffsetValid) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bOffsetValid",
                                        ),
                                    );
                                }
                                m_bOffsetValid = _serde::__private::Some(
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
                    __A::pad(&mut __map, 14usize, 14usize)?;
                    let m_pDefaultGenerator = match m_pDefaultGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pDefaultGenerator",
                                ),
                            );
                        }
                    };
                    let m_pOffsetClipGenerator = match m_pOffsetClipGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pOffsetClipGenerator",
                                ),
                            );
                        }
                    };
                    let m_fOffsetVariable = match m_fOffsetVariable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fOffsetVariable",
                                ),
                            );
                        }
                    };
                    let m_fOffsetRangeStart = match m_fOffsetRangeStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fOffsetRangeStart",
                                ),
                            );
                        }
                    };
                    let m_fOffsetRangeEnd = match m_fOffsetRangeEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fOffsetRangeEnd",
                                ),
                            );
                        }
                    };
                    let m_BoneOffsetA = match m_BoneOffsetA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "BoneOffsetA",
                                ),
                            );
                        }
                    };
                    let m_BoneIndexA = match m_BoneIndexA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "BoneIndexA",
                                ),
                            );
                        }
                    };
                    let m_fCurrentPercentage = match m_fCurrentPercentage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fCurrentPercentage",
                                ),
                            );
                        }
                    };
                    let m_iCurrentFrame = match m_iCurrentFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "iCurrentFrame",
                                ),
                            );
                        }
                    };
                    let m_bZeroOffset = match m_bZeroOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bZeroOffset",
                                ),
                            );
                        }
                    };
                    let m_bOffsetValid = match m_bOffsetValid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bOffsetValid",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSOffsetAnimationGenerator {
                        __ptr,
                        parent,
                        m_pDefaultGenerator,
                        m_pOffsetClipGenerator,
                        m_fOffsetVariable,
                        m_fOffsetRangeStart,
                        m_fOffsetRangeEnd,
                        m_BoneOffsetA,
                        m_BoneIndexA,
                        m_fCurrentPercentage,
                        m_iCurrentFrame,
                        m_bZeroOffset,
                        m_bOffsetValid,
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
                    let mut m_pDefaultGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_pOffsetClipGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_fOffsetVariable: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fOffsetRangeStart: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fOffsetRangeEnd: _serde::__private::Option<f32> = _serde::__private::None;
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
                            __Field::m_pDefaultGenerator => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pDefaultGenerator,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pDefaultGenerator",
                                        ),
                                    );
                                }
                                m_pDefaultGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_pOffsetClipGenerator => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_pOffsetClipGenerator,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pOffsetClipGenerator",
                                        ),
                                    );
                                }
                                m_pOffsetClipGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fOffsetVariable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fOffsetVariable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fOffsetVariable",
                                        ),
                                    );
                                }
                                m_fOffsetVariable = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fOffsetRangeStart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fOffsetRangeStart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fOffsetRangeStart",
                                        ),
                                    );
                                }
                                m_fOffsetRangeStart = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fOffsetRangeEnd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fOffsetRangeEnd) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fOffsetRangeEnd",
                                        ),
                                    );
                                }
                                m_fOffsetRangeEnd = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_pDefaultGenerator = match m_pDefaultGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pDefaultGenerator",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pOffsetClipGenerator = match m_pOffsetClipGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pOffsetClipGenerator",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fOffsetVariable = match m_fOffsetVariable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fOffsetVariable",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fOffsetRangeStart = match m_fOffsetRangeStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fOffsetRangeStart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fOffsetRangeEnd = match m_fOffsetRangeEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fOffsetRangeEnd",
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
                    _serde::__private::Ok(BSOffsetAnimationGenerator {
                        __ptr,
                        parent,
                        m_pDefaultGenerator,
                        m_pOffsetClipGenerator,
                        m_fOffsetVariable,
                        m_fOffsetRangeStart,
                        m_fOffsetRangeEnd,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "pDefaultGenerator",
                "pOffsetClipGenerator",
                "fOffsetVariable",
                "fOffsetRangeStart",
                "fOffsetRangeEnd",
                "BoneOffsetA",
                "BoneIndexA",
                "fCurrentPercentage",
                "iCurrentFrame",
                "bZeroOffset",
                "bOffsetValid",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSOffsetAnimationGenerator",
                FIELDS,
                __BSOffsetAnimationGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<BSOffsetAnimationGenerator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
