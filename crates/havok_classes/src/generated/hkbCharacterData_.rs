use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacterData`
/// - version: `7`
/// - signature: `0x300d6808`
/// - size: `144`(x86)/`176`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `characterControllerInfo`(ctype: `struct hkbCharacterDataCharacterControllerInfo`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterControllerInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterControllerInfo"))]
    pub m_characterControllerInfo: hkbCharacterDataCharacterControllerInfo,
    /// # C++ Info
    /// - name: `modelUpMS`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "modelUpMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "modelUpMS"))]
    pub m_modelUpMS: Vector4,
    /// # C++ Info
    /// - name: `modelForwardMS`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "modelForwardMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "modelForwardMS"))]
    pub m_modelForwardMS: Vector4,
    /// # C++ Info
    /// - name: `modelRightMS`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "modelRightMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "modelRightMS"))]
    pub m_modelRightMS: Vector4,
    /// # C++ Info
    /// - name: `characterPropertyInfos`(ctype: `hkArray<struct hkbVariableInfo>`)
    /// - offset: ` 80`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterPropertyInfos"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterPropertyInfos"))]
    pub m_characterPropertyInfos: Vec<hkbVariableInfo>,
    /// # C++ Info
    /// - name: `numBonesPerLod`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 92`(x86)/`112`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "numBonesPerLod"))]
    #[cfg_attr(feature = "serde", serde(rename = "numBonesPerLod"))]
    pub m_numBonesPerLod: Vec<i32>,
    /// # C++ Info
    /// - name: `characterPropertyValues`(ctype: `struct hkbVariableValueSet*`)
    /// - offset: `104`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterPropertyValues"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterPropertyValues"))]
    pub m_characterPropertyValues: Pointer,
    /// # C++ Info
    /// - name: `footIkDriverInfo`(ctype: `struct hkbFootIkDriverInfo*`)
    /// - offset: `108`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "footIkDriverInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "footIkDriverInfo"))]
    pub m_footIkDriverInfo: Pointer,
    /// # C++ Info
    /// - name: `handIkDriverInfo`(ctype: `struct hkbHandIkDriverInfo*`)
    /// - offset: `112`(x86)/`144`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "handIkDriverInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "handIkDriverInfo"))]
    pub m_handIkDriverInfo: Pointer,
    /// # C++ Info
    /// - name: `stringData`(ctype: `struct hkbCharacterStringData*`)
    /// - offset: `116`(x86)/`152`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "stringData"))]
    #[cfg_attr(feature = "serde", serde(rename = "stringData"))]
    pub m_stringData: Pointer,
    /// # C++ Info
    /// - name: `mirroredSkeletonInfo`(ctype: `struct hkbMirroredSkeletonInfo*`)
    /// - offset: `120`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "mirroredSkeletonInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "mirroredSkeletonInfo"))]
    pub m_mirroredSkeletonInfo: Pointer,
    /// # C++ Info
    /// - name: `scale`(ctype: `hkReal`)
    /// - offset: `124`(x86)/`168`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "scale"))]
    #[cfg_attr(feature = "serde", serde(rename = "scale"))]
    pub m_scale: f32,
    /// # C++ Info
    /// - name: `numHands`(ctype: `hkInt16`)
    /// - offset: `128`(x86)/`172`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "numHands"))]
    #[cfg_attr(feature = "serde", serde(rename = "numHands"))]
    pub m_numHands: i16,
    /// # C++ Info
    /// - name: `numFloatSlots`(ctype: `hkInt16`)
    /// - offset: `130`(x86)/`174`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "numFloatSlots"))]
    #[cfg_attr(feature = "serde", serde(rename = "numFloatSlots"))]
    pub m_numFloatSlots: i16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCharacterData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x300d6808)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_characterControllerInfo.deps_indexes());
            v.extend(
                self
                    .m_characterPropertyInfos
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.push(self.m_characterPropertyValues.get());
            v.push(self.m_footIkDriverInfo.get());
            v.push(self.m_handIkDriverInfo.get());
            v.push(self.m_stringData.get());
            v.push(self.m_mirroredSkeletonInfo.get());
            v
        }
    }
    impl _serde::Serialize for hkbCharacterData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x300d6808)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterData", class_meta, (144u64, 176u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "characterControllerInfo",
                    &self.m_characterControllerInfo,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("modelUpMS", &self.m_modelUpMS)?;
            serializer.serialize_field("modelForwardMS", &self.m_modelForwardMS)?;
            serializer.serialize_field("modelRightMS", &self.m_modelRightMS)?;
            serializer
                .serialize_array_field(
                    "characterPropertyInfos",
                    &self.m_characterPropertyInfos,
                    TypeSize::Struct {
                        size_x86: 6u64,
                        size_x86_64: 6u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "numBonesPerLod",
                    &self.m_numBonesPerLod,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field(
                    "characterPropertyValues",
                    &self.m_characterPropertyValues,
                )?;
            serializer.serialize_field("footIkDriverInfo", &self.m_footIkDriverInfo)?;
            serializer.serialize_field("handIkDriverInfo", &self.m_handIkDriverInfo)?;
            serializer.serialize_field("stringData", &self.m_stringData)?;
            serializer
                .serialize_field("mirroredSkeletonInfo", &self.m_mirroredSkeletonInfo)?;
            serializer.serialize_field("scale", &self.m_scale)?;
            serializer.skip_field("numHands", &self.m_numHands)?;
            serializer.skip_field("numFloatSlots", &self.m_numFloatSlots)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_characterControllerInfo,
                m_modelUpMS,
                m_modelForwardMS,
                m_modelRightMS,
                m_characterPropertyInfos,
                m_numBonesPerLod,
                m_characterPropertyValues,
                m_footIkDriverInfo,
                m_handIkDriverInfo,
                m_stringData,
                m_mirroredSkeletonInfo,
                m_scale,
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
                        "characterControllerInfo" => {
                            Ok(__Field::m_characterControllerInfo)
                        }
                        "modelUpMS" => Ok(__Field::m_modelUpMS),
                        "modelForwardMS" => Ok(__Field::m_modelForwardMS),
                        "modelRightMS" => Ok(__Field::m_modelRightMS),
                        "characterPropertyInfos" => Ok(__Field::m_characterPropertyInfos),
                        "numBonesPerLod" => Ok(__Field::m_numBonesPerLod),
                        "characterPropertyValues" => {
                            Ok(__Field::m_characterPropertyValues)
                        }
                        "footIkDriverInfo" => Ok(__Field::m_footIkDriverInfo),
                        "handIkDriverInfo" => Ok(__Field::m_handIkDriverInfo),
                        "stringData" => Ok(__Field::m_stringData),
                        "mirroredSkeletonInfo" => Ok(__Field::m_mirroredSkeletonInfo),
                        "scale" => Ok(__Field::m_scale),
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
            struct __hkbCharacterDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCharacterData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbCharacterDataVisitor<'de> {
                type Value = hkbCharacterData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCharacterData",
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
                    let mut m_characterControllerInfo: _serde::__private::Option<
                        hkbCharacterDataCharacterControllerInfo,
                    > = _serde::__private::None;
                    let mut m_modelUpMS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_modelForwardMS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_modelRightMS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_characterPropertyInfos: _serde::__private::Option<
                        Vec<hkbVariableInfo>,
                    > = _serde::__private::None;
                    let mut m_numBonesPerLod: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_characterPropertyValues: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_footIkDriverInfo: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_handIkDriverInfo: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_mirroredSkeletonInfo: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_scale: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_numHands: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_numFloatSlots: _serde::__private::Option<i16> = _serde::__private::None;
                    for i in 0..14usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterControllerInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterControllerInfo",
                                        ),
                                    );
                                }
                                m_characterControllerInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbCharacterDataCharacterControllerInfo,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_modelUpMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modelUpMS",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 8usize)?;
                                m_modelUpMS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_modelForwardMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modelForwardMS",
                                        ),
                                    );
                                }
                                m_modelForwardMS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_modelRightMS) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modelRightMS",
                                        ),
                                    );
                                }
                                m_modelRightMS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyInfos,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyInfos",
                                        ),
                                    );
                                }
                                m_characterPropertyInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_numBonesPerLod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBonesPerLod",
                                        ),
                                    );
                                }
                                m_numBonesPerLod = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyValues,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyValues",
                                        ),
                                    );
                                }
                                m_characterPropertyValues = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_footIkDriverInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footIkDriverInfo",
                                        ),
                                    );
                                }
                                m_footIkDriverInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_handIkDriverInfo) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handIkDriverInfo",
                                        ),
                                    );
                                }
                                m_handIkDriverInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_stringData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stringData",
                                        ),
                                    );
                                }
                                m_stringData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mirroredSkeletonInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSkeletonInfo",
                                        ),
                                    );
                                }
                                m_mirroredSkeletonInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_scale) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("scale"),
                                    );
                                }
                                m_scale = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_numHands) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numHands",
                                        ),
                                    );
                                }
                                m_numHands = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_numFloatSlots) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numFloatSlots",
                                        ),
                                    );
                                }
                                m_numFloatSlots = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
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
                    __A::pad(&mut __map, 12usize, 0usize)?;
                    let m_characterControllerInfo = match m_characterControllerInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterControllerInfo",
                                ),
                            );
                        }
                    };
                    let m_modelUpMS = match m_modelUpMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modelUpMS",
                                ),
                            );
                        }
                    };
                    let m_modelForwardMS = match m_modelForwardMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modelForwardMS",
                                ),
                            );
                        }
                    };
                    let m_modelRightMS = match m_modelRightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modelRightMS",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyInfos = match m_characterPropertyInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyInfos",
                                ),
                            );
                        }
                    };
                    let m_numBonesPerLod = match m_numBonesPerLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numBonesPerLod",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyValues = match m_characterPropertyValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyValues",
                                ),
                            );
                        }
                    };
                    let m_footIkDriverInfo = match m_footIkDriverInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footIkDriverInfo",
                                ),
                            );
                        }
                    };
                    let m_handIkDriverInfo = match m_handIkDriverInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handIkDriverInfo",
                                ),
                            );
                        }
                    };
                    let m_stringData = match m_stringData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stringData",
                                ),
                            );
                        }
                    };
                    let m_mirroredSkeletonInfo = match m_mirroredSkeletonInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSkeletonInfo",
                                ),
                            );
                        }
                    };
                    let m_scale = match m_scale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("scale"),
                            );
                        }
                    };
                    let m_numHands = match m_numHands {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numHands"),
                            );
                        }
                    };
                    let m_numFloatSlots = match m_numFloatSlots {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numFloatSlots",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacterData {
                        __ptr,
                        parent,
                        m_characterControllerInfo,
                        m_modelUpMS,
                        m_modelForwardMS,
                        m_modelRightMS,
                        m_characterPropertyInfos,
                        m_numBonesPerLod,
                        m_characterPropertyValues,
                        m_footIkDriverInfo,
                        m_handIkDriverInfo,
                        m_stringData,
                        m_mirroredSkeletonInfo,
                        m_scale,
                        m_numHands,
                        m_numFloatSlots,
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
                    let mut m_characterControllerInfo: _serde::__private::Option<
                        hkbCharacterDataCharacterControllerInfo,
                    > = _serde::__private::None;
                    let mut m_modelUpMS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_modelForwardMS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_modelRightMS: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_characterPropertyInfos: _serde::__private::Option<
                        Vec<hkbVariableInfo>,
                    > = _serde::__private::None;
                    let mut m_numBonesPerLod: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_characterPropertyValues: _serde::__private::Option<
                        Pointer,
                    > = _serde::__private::None;
                    let mut m_footIkDriverInfo: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_handIkDriverInfo: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_stringData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_mirroredSkeletonInfo: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_scale: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_characterControllerInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterControllerInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterControllerInfo",
                                        ),
                                    );
                                }
                                m_characterControllerInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbCharacterDataCharacterControllerInfo,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_modelUpMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_modelUpMS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modelUpMS",
                                        ),
                                    );
                                }
                                m_modelUpMS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_modelForwardMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_modelForwardMS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modelForwardMS",
                                        ),
                                    );
                                }
                                m_modelForwardMS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_modelRightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_modelRightMS) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "modelRightMS",
                                        ),
                                    );
                                }
                                m_modelRightMS = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterPropertyInfos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyInfos,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyInfos",
                                        ),
                                    );
                                }
                                m_characterPropertyInfos = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkbVariableInfo>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numBonesPerLod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numBonesPerLod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBonesPerLod",
                                        ),
                                    );
                                }
                                m_numBonesPerLod = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterPropertyValues => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyValues,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyValues",
                                        ),
                                    );
                                }
                                m_characterPropertyValues = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_footIkDriverInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_footIkDriverInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footIkDriverInfo",
                                        ),
                                    );
                                }
                                m_footIkDriverInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handIkDriverInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_handIkDriverInfo) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handIkDriverInfo",
                                        ),
                                    );
                                }
                                m_handIkDriverInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stringData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stringData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stringData",
                                        ),
                                    );
                                }
                                m_stringData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mirroredSkeletonInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_mirroredSkeletonInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSkeletonInfo",
                                        ),
                                    );
                                }
                                m_mirroredSkeletonInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_scale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_scale) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("scale"),
                                    );
                                }
                                m_scale = _serde::__private::Some(
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
                    let m_characterControllerInfo = match m_characterControllerInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterControllerInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_modelUpMS = match m_modelUpMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modelUpMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_modelForwardMS = match m_modelForwardMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modelForwardMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_modelRightMS = match m_modelRightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "modelRightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterPropertyInfos = match m_characterPropertyInfos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyInfos",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numBonesPerLod = match m_numBonesPerLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numBonesPerLod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterPropertyValues = match m_characterPropertyValues {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyValues",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_footIkDriverInfo = match m_footIkDriverInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footIkDriverInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handIkDriverInfo = match m_handIkDriverInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handIkDriverInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stringData = match m_stringData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stringData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mirroredSkeletonInfo = match m_mirroredSkeletonInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSkeletonInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_scale = match m_scale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("scale"),
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
                    _serde::__private::Ok(hkbCharacterData {
                        __ptr,
                        parent,
                        m_characterControllerInfo,
                        m_modelUpMS,
                        m_modelForwardMS,
                        m_modelRightMS,
                        m_characterPropertyInfos,
                        m_numBonesPerLod,
                        m_characterPropertyValues,
                        m_footIkDriverInfo,
                        m_handIkDriverInfo,
                        m_stringData,
                        m_mirroredSkeletonInfo,
                        m_scale,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "characterControllerInfo",
                "modelUpMS",
                "modelForwardMS",
                "modelRightMS",
                "characterPropertyInfos",
                "numBonesPerLod",
                "characterPropertyValues",
                "footIkDriverInfo",
                "handIkDriverInfo",
                "stringData",
                "mirroredSkeletonInfo",
                "scale",
                "numHands",
                "numFloatSlots",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterData",
                FIELDS,
                __hkbCharacterDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
