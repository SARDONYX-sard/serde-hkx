use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacterSteppedInfo`
/// - version: `2`
/// - signature: `0x2eda84f8`
/// - size: `112`(x86)/`112`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterSteppedInfo {
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
    /// - name: `characterId`(ctype: `hkUint64`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_characterId: u64,
    /// # C++ Info
    /// - name: `deltaTime`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_deltaTime: f32,
    /// # C++ Info
    /// - name: `worldFromModel`(ctype: `hkQsTransform`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// - name: `poseModelSpace`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: ` 80`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_poseModelSpace: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `rigidAttachmentTransforms`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: ` 92`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_rigidAttachmentTransforms: Vec<QsTransform>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterSteppedInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterSteppedInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2eda84f8)
        }
    }
    impl _serde::Serialize for hkbCharacterSteppedInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2eda84f8)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterSteppedInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("deltaTime", &self.m_deltaTime)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer
                .serialize_array_meta_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_meta_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_array_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
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
    impl<'de> _serde::Deserialize<'de> for hkbCharacterSteppedInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_rigidAttachmentTransforms,
                m_poseModelSpace,
                m_worldFromModel,
                m_deltaTime,
                m_characterId,
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
                        "rigidAttachmentTransforms" => {
                            Ok(__Field::m_rigidAttachmentTransforms)
                        }
                        "poseModelSpace" => Ok(__Field::m_poseModelSpace),
                        "worldFromModel" => Ok(__Field::m_worldFromModel),
                        "deltaTime" => Ok(__Field::m_deltaTime),
                        "characterId" => Ok(__Field::m_characterId),
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
            struct __hkbCharacterSteppedInfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCharacterSteppedInfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbCharacterSteppedInfoVisitor<'de> {
                type Value = hkbCharacterSteppedInfo;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCharacterSteppedInfo",
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
                    let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_deltaTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_poseModelSpace: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_rigidAttachmentTransforms: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_characterId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterId",
                                        ),
                                    );
                                }
                                m_characterId = _serde::__private::Some(
                                    match __A::next_value::<u64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_deltaTime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deltaTime",
                                        ),
                                    );
                                }
                                m_deltaTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromModel",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 12usize, 4usize)?;
                                m_worldFromModel = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_poseModelSpace) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseModelSpace",
                                        ),
                                    );
                                }
                                m_poseModelSpace = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_rigidAttachmentTransforms,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidAttachmentTransforms",
                                        ),
                                    );
                                }
                                m_rigidAttachmentTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
                    __A::pad(&mut __map, 8usize, 0usize)?;
                    let m_characterId = match m_characterId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterId",
                                ),
                            );
                        }
                    };
                    let m_deltaTime = match m_deltaTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deltaTime",
                                ),
                            );
                        }
                    };
                    let m_worldFromModel = match m_worldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromModel",
                                ),
                            );
                        }
                    };
                    let m_poseModelSpace = match m_poseModelSpace {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseModelSpace",
                                ),
                            );
                        }
                    };
                    let m_rigidAttachmentTransforms = match m_rigidAttachmentTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidAttachmentTransforms",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacterSteppedInfo {
                        __ptr,
                        parent,
                        m_characterId,
                        m_deltaTime,
                        m_worldFromModel,
                        m_poseModelSpace,
                        m_rigidAttachmentTransforms,
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
                    let mut m_rigidAttachmentTransforms: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_poseModelSpace: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_deltaTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_characterId: _serde::__private::Option<u64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_rigidAttachmentTransforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_rigidAttachmentTransforms,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidAttachmentTransforms",
                                        ),
                                    );
                                }
                                m_rigidAttachmentTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_poseModelSpace => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_poseModelSpace) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "poseModelSpace",
                                        ),
                                    );
                                }
                                m_poseModelSpace = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_worldFromModel => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromModel",
                                        ),
                                    );
                                }
                                m_worldFromModel = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deltaTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_deltaTime) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deltaTime",
                                        ),
                                    );
                                }
                                m_deltaTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_characterId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_characterId) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterId",
                                        ),
                                    );
                                }
                                m_characterId = _serde::__private::Some(
                                    match __A::next_value::<u64>(&mut __map) {
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
                    let m_rigidAttachmentTransforms = match m_rigidAttachmentTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidAttachmentTransforms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_poseModelSpace = match m_poseModelSpace {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "poseModelSpace",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_worldFromModel = match m_worldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromModel",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deltaTime = match m_deltaTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deltaTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterId = match m_characterId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterId",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCharacterSteppedInfo {
                        __ptr,
                        parent,
                        m_characterId,
                        m_deltaTime,
                        m_worldFromModel,
                        m_poseModelSpace,
                        m_rigidAttachmentTransforms,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "characterId",
                "deltaTime",
                "worldFromModel",
                "poseModelSpace",
                "rigidAttachmentTransforms",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterSteppedInfo",
                FIELDS,
                __hkbCharacterSteppedInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterSteppedInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
