use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacterSetup`
/// - version: `2`
/// - signature: `0xe5a2a413`
/// - size: ` 48`(x86)/` 88`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterSetup {
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
    /// - name: `retargetingSkeletonMappers`(ctype: `hkArray<hkaSkeletonMapper*>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_retargetingSkeletonMappers: Vec<Pointer>,
    /// # C++ Info
    /// - name: `animationSkeleton`(ctype: `struct hkaSkeleton*`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_animationSkeleton: Pointer,
    /// # C++ Info
    /// - name: `ragdollToAnimationSkeletonMapper`(ctype: `struct hkaSkeletonMapper*`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_ragdollToAnimationSkeletonMapper: Pointer,
    /// # C++ Info
    /// - name: `animationToRagdollSkeletonMapper`(ctype: `struct hkaSkeletonMapper*`)
    /// - offset: ` 28`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_animationToRagdollSkeletonMapper: Pointer,
    /// # C++ Info
    /// - name: `animationBindingSet`(ctype: `void*`)
    /// - offset: ` 32`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_animationBindingSet: Pointer,
    /// # C++ Info
    /// - name: `data`(ctype: `struct hkbCharacterData*`)
    /// - offset: ` 36`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_data: Pointer,
    /// # C++ Info
    /// - name: `mirroredSkeleton`(ctype: `void*`)
    /// - offset: ` 40`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_mirroredSkeleton: Pointer,
    /// # C++ Info
    /// - name: `characterPropertyIdMap`(ctype: `void*`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_characterPropertyIdMap: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterSetup {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterSetup"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe5a2a413)
        }
    }
    impl _serde::Serialize for hkbCharacterSetup {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe5a2a413)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterSetup", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "retargetingSkeletonMappers",
                    &self.m_retargetingSkeletonMappers,
                )?;
            serializer.serialize_field("animationSkeleton", &self.m_animationSkeleton)?;
            serializer
                .serialize_field(
                    "ragdollToAnimationSkeletonMapper",
                    &self.m_ragdollToAnimationSkeletonMapper,
                )?;
            serializer
                .serialize_field(
                    "animationToRagdollSkeletonMapper",
                    &self.m_animationToRagdollSkeletonMapper,
                )?;
            serializer.skip_field("animationBindingSet", &self.m_animationBindingSet)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.skip_field("mirroredSkeleton", &self.m_mirroredSkeleton)?;
            serializer
                .skip_field("characterPropertyIdMap", &self.m_characterPropertyIdMap)?;
            serializer
                .serialize_array_field(
                    "retargetingSkeletonMappers",
                    &self.m_retargetingSkeletonMappers,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterSetup {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_retargetingSkeletonMappers,
                m_animationSkeleton,
                m_ragdollToAnimationSkeletonMapper,
                m_animationToRagdollSkeletonMapper,
                m_data,
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
                        "retargetingSkeletonMappers" => {
                            Ok(__Field::m_retargetingSkeletonMappers)
                        }
                        "animationSkeleton" => Ok(__Field::m_animationSkeleton),
                        "ragdollToAnimationSkeletonMapper" => {
                            Ok(__Field::m_ragdollToAnimationSkeletonMapper)
                        }
                        "animationToRagdollSkeletonMapper" => {
                            Ok(__Field::m_animationToRagdollSkeletonMapper)
                        }
                        "data" => Ok(__Field::m_data),
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
            struct __hkbCharacterSetupVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCharacterSetup>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbCharacterSetupVisitor<'de> {
                type Value = hkbCharacterSetup;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCharacterSetup",
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
                    let mut m_retargetingSkeletonMappers: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_animationSkeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_ragdollToAnimationSkeletonMapper: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_animationToRagdollSkeletonMapper: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_animationBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_mirroredSkeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_characterPropertyIdMap: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_retargetingSkeletonMappers,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "retargetingSkeletonMappers",
                                        ),
                                    );
                                }
                                m_retargetingSkeletonMappers = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_animationSkeleton,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationSkeleton",
                                        ),
                                    );
                                }
                                m_animationSkeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_ragdollToAnimationSkeletonMapper,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollToAnimationSkeletonMapper",
                                        ),
                                    );
                                }
                                m_ragdollToAnimationSkeletonMapper = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_animationToRagdollSkeletonMapper,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationToRagdollSkeletonMapper",
                                        ),
                                    );
                                }
                                m_animationToRagdollSkeletonMapper = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_animationBindingSet,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBindingSet",
                                        ),
                                    );
                                }
                                m_animationBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_data) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_mirroredSkeleton) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSkeleton",
                                        ),
                                    );
                                }
                                m_mirroredSkeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyIdMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyIdMap",
                                        ),
                                    );
                                }
                                m_characterPropertyIdMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_retargetingSkeletonMappers = match m_retargetingSkeletonMappers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "retargetingSkeletonMappers",
                                ),
                            );
                        }
                    };
                    let m_animationSkeleton = match m_animationSkeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationSkeleton",
                                ),
                            );
                        }
                    };
                    let m_ragdollToAnimationSkeletonMapper = match m_ragdollToAnimationSkeletonMapper {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollToAnimationSkeletonMapper",
                                ),
                            );
                        }
                    };
                    let m_animationToRagdollSkeletonMapper = match m_animationToRagdollSkeletonMapper {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationToRagdollSkeletonMapper",
                                ),
                            );
                        }
                    };
                    let m_animationBindingSet = match m_animationBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBindingSet",
                                ),
                            );
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                        }
                    };
                    let m_mirroredSkeleton = match m_mirroredSkeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSkeleton",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyIdMap = match m_characterPropertyIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyIdMap",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacterSetup {
                        __ptr,
                        parent,
                        m_retargetingSkeletonMappers,
                        m_animationSkeleton,
                        m_ragdollToAnimationSkeletonMapper,
                        m_animationToRagdollSkeletonMapper,
                        m_animationBindingSet,
                        m_data,
                        m_mirroredSkeleton,
                        m_characterPropertyIdMap,
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
                    let mut m_retargetingSkeletonMappers: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_animationSkeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_ragdollToAnimationSkeletonMapper: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_animationToRagdollSkeletonMapper: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_retargetingSkeletonMappers => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_retargetingSkeletonMappers,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "retargetingSkeletonMappers",
                                        ),
                                    );
                                }
                                m_retargetingSkeletonMappers = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_animationSkeleton => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_animationSkeleton,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationSkeleton",
                                        ),
                                    );
                                }
                                m_animationSkeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_ragdollToAnimationSkeletonMapper => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_ragdollToAnimationSkeletonMapper,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollToAnimationSkeletonMapper",
                                        ),
                                    );
                                }
                                m_ragdollToAnimationSkeletonMapper = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_animationToRagdollSkeletonMapper => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_animationToRagdollSkeletonMapper,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationToRagdollSkeletonMapper",
                                        ),
                                    );
                                }
                                m_animationToRagdollSkeletonMapper = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_data => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_data) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_retargetingSkeletonMappers = match m_retargetingSkeletonMappers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "retargetingSkeletonMappers",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_animationSkeleton = match m_animationSkeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationSkeleton",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ragdollToAnimationSkeletonMapper = match m_ragdollToAnimationSkeletonMapper {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollToAnimationSkeletonMapper",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_animationToRagdollSkeletonMapper = match m_animationToRagdollSkeletonMapper {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationToRagdollSkeletonMapper",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCharacterSetup {
                        __ptr,
                        parent,
                        m_retargetingSkeletonMappers,
                        m_animationSkeleton,
                        m_ragdollToAnimationSkeletonMapper,
                        m_animationToRagdollSkeletonMapper,
                        m_data,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "retargetingSkeletonMappers",
                "animationSkeleton",
                "ragdollToAnimationSkeletonMapper",
                "animationToRagdollSkeletonMapper",
                "animationBindingSet",
                "data",
                "mirroredSkeleton",
                "characterPropertyIdMap",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterSetup",
                FIELDS,
                __hkbCharacterSetupVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterSetup>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
