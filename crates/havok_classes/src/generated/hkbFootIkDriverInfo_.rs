use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbFootIkDriverInfo`
/// - version: `0`
/// - signature: `0xc6a09dbf`
/// - size: ` 56`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkDriverInfo<'a> {
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
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `legs`(ctype: `hkArray<struct hkbFootIkDriverInfoLeg>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "legs"))]
    #[cfg_attr(feature = "serde", serde(rename = "legs"))]
    pub m_legs: Vec<hkbFootIkDriverInfoLeg<'a>>,
    /// # C++ Info
    /// - name: `raycastDistanceUp`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "raycastDistanceUp"))]
    #[cfg_attr(feature = "serde", serde(rename = "raycastDistanceUp"))]
    pub m_raycastDistanceUp: f32,
    /// # C++ Info
    /// - name: `raycastDistanceDown`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "raycastDistanceDown"))]
    #[cfg_attr(feature = "serde", serde(rename = "raycastDistanceDown"))]
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// - name: `originalGroundHeightMS`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "originalGroundHeightMS"))]
    #[cfg_attr(feature = "serde", serde(rename = "originalGroundHeightMS"))]
    pub m_originalGroundHeightMS: f32,
    /// # C++ Info
    /// - name: `verticalOffset`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 44`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "verticalOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "verticalOffset"))]
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: ` 36`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "collisionFilterInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "collisionFilterInfo"))]
    pub m_collisionFilterInfo: U32<'a>,
    /// # C++ Info
    /// - name: `forwardAlignFraction`(ctype: `hkReal`)
    /// - offset: ` 40`(x86)/` 52`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "forwardAlignFraction"))]
    #[cfg_attr(feature = "serde", serde(rename = "forwardAlignFraction"))]
    pub m_forwardAlignFraction: f32,
    /// # C++ Info
    /// - name: `sidewaysAlignFraction`(ctype: `hkReal`)
    /// - offset: ` 44`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "sidewaysAlignFraction"))]
    #[cfg_attr(feature = "serde", serde(rename = "sidewaysAlignFraction"))]
    pub m_sidewaysAlignFraction: f32,
    /// # C++ Info
    /// - name: `sidewaysSampleWidth`(ctype: `hkReal`)
    /// - offset: ` 48`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "sidewaysSampleWidth"))]
    #[cfg_attr(feature = "serde", serde(rename = "sidewaysSampleWidth"))]
    pub m_sidewaysSampleWidth: f32,
    /// # C++ Info
    /// - name: `lockFeetWhenPlanted`(ctype: `hkBool`)
    /// - offset: ` 52`(x86)/` 64`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "lockFeetWhenPlanted"))]
    #[cfg_attr(feature = "serde", serde(rename = "lockFeetWhenPlanted"))]
    pub m_lockFeetWhenPlanted: bool,
    /// # C++ Info
    /// - name: `useCharacterUpVector`(ctype: `hkBool`)
    /// - offset: ` 53`(x86)/` 65`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "useCharacterUpVector"))]
    #[cfg_attr(feature = "serde", serde(rename = "useCharacterUpVector"))]
    pub m_useCharacterUpVector: bool,
    /// # C++ Info
    /// - name: `isQuadrupedNarrow`(ctype: `hkBool`)
    /// - offset: ` 54`(x86)/` 66`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "isQuadrupedNarrow"))]
    #[cfg_attr(feature = "serde", serde(rename = "isQuadrupedNarrow"))]
    pub m_isQuadrupedNarrow: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbFootIkDriverInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkDriverInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc6a09dbf)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_legs
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkbFootIkDriverInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xc6a09dbf)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkDriverInfo", class_meta, (56u64, 72u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "legs",
                    &self.m_legs,
                    TypeSize::Struct {
                        size_x86: 96u64,
                        size_x86_64: 96u64,
                    },
                )?;
            serializer.serialize_field("raycastDistanceUp", &self.m_raycastDistanceUp)?;
            serializer
                .serialize_field("raycastDistanceDown", &self.m_raycastDistanceDown)?;
            serializer
                .serialize_field(
                    "originalGroundHeightMS",
                    &self.m_originalGroundHeightMS,
                )?;
            serializer.serialize_field("verticalOffset", &self.m_verticalOffset)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer
                .serialize_field("forwardAlignFraction", &self.m_forwardAlignFraction)?;
            serializer
                .serialize_field(
                    "sidewaysAlignFraction",
                    &self.m_sidewaysAlignFraction,
                )?;
            serializer
                .serialize_field("sidewaysSampleWidth", &self.m_sidewaysSampleWidth)?;
            serializer
                .serialize_field("lockFeetWhenPlanted", &self.m_lockFeetWhenPlanted)?;
            serializer
                .serialize_field("useCharacterUpVector", &self.m_useCharacterUpVector)?;
            serializer.serialize_field("isQuadrupedNarrow", &self.m_isQuadrupedNarrow)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkDriverInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_legs,
                m_raycastDistanceUp,
                m_raycastDistanceDown,
                m_originalGroundHeightMS,
                m_verticalOffset,
                m_collisionFilterInfo,
                m_forwardAlignFraction,
                m_sidewaysAlignFraction,
                m_sidewaysSampleWidth,
                m_lockFeetWhenPlanted,
                m_useCharacterUpVector,
                m_isQuadrupedNarrow,
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
                        "legs" => Ok(__Field::m_legs),
                        "raycastDistanceUp" => Ok(__Field::m_raycastDistanceUp),
                        "raycastDistanceDown" => Ok(__Field::m_raycastDistanceDown),
                        "originalGroundHeightMS" => Ok(__Field::m_originalGroundHeightMS),
                        "verticalOffset" => Ok(__Field::m_verticalOffset),
                        "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
                        "forwardAlignFraction" => Ok(__Field::m_forwardAlignFraction),
                        "sidewaysAlignFraction" => Ok(__Field::m_sidewaysAlignFraction),
                        "sidewaysSampleWidth" => Ok(__Field::m_sidewaysSampleWidth),
                        "lockFeetWhenPlanted" => Ok(__Field::m_lockFeetWhenPlanted),
                        "useCharacterUpVector" => Ok(__Field::m_useCharacterUpVector),
                        "isQuadrupedNarrow" => Ok(__Field::m_isQuadrupedNarrow),
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
            struct __hkbFootIkDriverInfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbFootIkDriverInfo<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbFootIkDriverInfoVisitor<'de> {
                type Value = hkbFootIkDriverInfo<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbFootIkDriverInfo",
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
                    let mut m_legs: _serde::__private::Option<
                        Vec<hkbFootIkDriverInfoLeg>,
                    > = _serde::__private::None;
                    let mut m_raycastDistanceUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_originalGroundHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_forwardAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysSampleWidth: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lockFeetWhenPlanted: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useCharacterUpVector: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isQuadrupedNarrow: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_legs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("legs"),
                                    );
                                }
                                m_legs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbFootIkDriverInfoLeg>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceUp,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceUp",
                                        ),
                                    );
                                }
                                m_raycastDistanceUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceDown,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceDown",
                                        ),
                                    );
                                }
                                m_raycastDistanceDown = _serde::__private::Some(
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
                                    &m_originalGroundHeightMS,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalGroundHeightMS",
                                        ),
                                    );
                                }
                                m_originalGroundHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_verticalOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalOffset",
                                        ),
                                    );
                                }
                                m_verticalOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_forwardAlignFraction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardAlignFraction",
                                        ),
                                    );
                                }
                                m_forwardAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysAlignFraction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysAlignFraction",
                                        ),
                                    );
                                }
                                m_sidewaysAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysSampleWidth,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysSampleWidth",
                                        ),
                                    );
                                }
                                m_sidewaysSampleWidth = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_lockFeetWhenPlanted,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockFeetWhenPlanted",
                                        ),
                                    );
                                }
                                m_lockFeetWhenPlanted = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useCharacterUpVector,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useCharacterUpVector",
                                        ),
                                    );
                                }
                                m_useCharacterUpVector = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_isQuadrupedNarrow,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isQuadrupedNarrow",
                                        ),
                                    );
                                }
                                m_isQuadrupedNarrow = _serde::__private::Some(
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
                    __A::pad(&mut __map, 1usize, 5usize)?;
                    let m_legs = match m_legs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("legs"),
                            );
                        }
                    };
                    let m_raycastDistanceUp = match m_raycastDistanceUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceUp",
                                ),
                            );
                        }
                    };
                    let m_raycastDistanceDown = match m_raycastDistanceDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceDown",
                                ),
                            );
                        }
                    };
                    let m_originalGroundHeightMS = match m_originalGroundHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalGroundHeightMS",
                                ),
                            );
                        }
                    };
                    let m_verticalOffset = match m_verticalOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalOffset",
                                ),
                            );
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                        }
                    };
                    let m_forwardAlignFraction = match m_forwardAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardAlignFraction",
                                ),
                            );
                        }
                    };
                    let m_sidewaysAlignFraction = match m_sidewaysAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysAlignFraction",
                                ),
                            );
                        }
                    };
                    let m_sidewaysSampleWidth = match m_sidewaysSampleWidth {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysSampleWidth",
                                ),
                            );
                        }
                    };
                    let m_lockFeetWhenPlanted = match m_lockFeetWhenPlanted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockFeetWhenPlanted",
                                ),
                            );
                        }
                    };
                    let m_useCharacterUpVector = match m_useCharacterUpVector {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useCharacterUpVector",
                                ),
                            );
                        }
                    };
                    let m_isQuadrupedNarrow = match m_isQuadrupedNarrow {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isQuadrupedNarrow",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbFootIkDriverInfo {
                        __ptr,
                        parent,
                        m_legs,
                        m_raycastDistanceUp,
                        m_raycastDistanceDown,
                        m_originalGroundHeightMS,
                        m_verticalOffset,
                        m_collisionFilterInfo,
                        m_forwardAlignFraction,
                        m_sidewaysAlignFraction,
                        m_sidewaysSampleWidth,
                        m_lockFeetWhenPlanted,
                        m_useCharacterUpVector,
                        m_isQuadrupedNarrow,
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
                    let mut m_legs: _serde::__private::Option<
                        Vec<hkbFootIkDriverInfoLeg>,
                    > = _serde::__private::None;
                    let mut m_raycastDistanceUp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_originalGroundHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_forwardAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sidewaysSampleWidth: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_lockFeetWhenPlanted: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_useCharacterUpVector: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isQuadrupedNarrow: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_legs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_legs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("legs"),
                                    );
                                }
                                m_legs = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbFootIkDriverInfoLeg>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_raycastDistanceUp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceUp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceUp",
                                        ),
                                    );
                                }
                                m_raycastDistanceUp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_raycastDistanceDown => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceDown,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceDown",
                                        ),
                                    );
                                }
                                m_raycastDistanceDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_originalGroundHeightMS => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_originalGroundHeightMS,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalGroundHeightMS",
                                        ),
                                    );
                                }
                                m_originalGroundHeightMS = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_verticalOffset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_verticalOffset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verticalOffset",
                                        ),
                                    );
                                }
                                m_verticalOffset = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionFilterInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_forwardAlignFraction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_forwardAlignFraction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardAlignFraction",
                                        ),
                                    );
                                }
                                m_forwardAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sidewaysAlignFraction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysAlignFraction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysAlignFraction",
                                        ),
                                    );
                                }
                                m_sidewaysAlignFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sidewaysSampleWidth => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_sidewaysSampleWidth,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sidewaysSampleWidth",
                                        ),
                                    );
                                }
                                m_sidewaysSampleWidth = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lockFeetWhenPlanted => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_lockFeetWhenPlanted,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lockFeetWhenPlanted",
                                        ),
                                    );
                                }
                                m_lockFeetWhenPlanted = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_useCharacterUpVector => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useCharacterUpVector,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useCharacterUpVector",
                                        ),
                                    );
                                }
                                m_useCharacterUpVector = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isQuadrupedNarrow => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_isQuadrupedNarrow,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isQuadrupedNarrow",
                                        ),
                                    );
                                }
                                m_isQuadrupedNarrow = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_legs = match m_legs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("legs"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_raycastDistanceUp = match m_raycastDistanceUp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceUp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_raycastDistanceDown = match m_raycastDistanceDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceDown",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_originalGroundHeightMS = match m_originalGroundHeightMS {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalGroundHeightMS",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_verticalOffset = match m_verticalOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "verticalOffset",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_forwardAlignFraction = match m_forwardAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardAlignFraction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sidewaysAlignFraction = match m_sidewaysAlignFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysAlignFraction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sidewaysSampleWidth = match m_sidewaysSampleWidth {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sidewaysSampleWidth",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lockFeetWhenPlanted = match m_lockFeetWhenPlanted {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "lockFeetWhenPlanted",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useCharacterUpVector = match m_useCharacterUpVector {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useCharacterUpVector",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isQuadrupedNarrow = match m_isQuadrupedNarrow {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isQuadrupedNarrow",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbFootIkDriverInfo {
                        __ptr: __ptr.clone(),
                        parent,
                        m_legs,
                        m_raycastDistanceUp,
                        m_raycastDistanceDown,
                        m_originalGroundHeightMS,
                        m_verticalOffset,
                        m_collisionFilterInfo,
                        m_forwardAlignFraction,
                        m_sidewaysAlignFraction,
                        m_sidewaysSampleWidth,
                        m_lockFeetWhenPlanted,
                        m_useCharacterUpVector,
                        m_isQuadrupedNarrow,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "legs",
                "raycastDistanceUp",
                "raycastDistanceDown",
                "originalGroundHeightMS",
                "verticalOffset",
                "collisionFilterInfo",
                "forwardAlignFraction",
                "sidewaysAlignFraction",
                "sidewaysSampleWidth",
                "lockFeetWhenPlanted",
                "useCharacterUpVector",
                "isQuadrupedNarrow",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbFootIkDriverInfo",
                FIELDS,
                __hkbFootIkDriverInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbFootIkDriverInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
