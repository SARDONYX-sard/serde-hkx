use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkDriverInfo`
/// -         version: `0`
/// -       signature: `0xc6a09dbf`
/// -          size:  56(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkDriverInfo {
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
    /// -          name: `legs`(ctype: `hkArray<struct hkbFootIkDriverInfoLeg>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_legs: Vec<hkbFootIkDriverInfoLeg>,
    /// # C++ Info
    /// -          name: `raycastDistanceUp`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceUp: f32,
    /// # C++ Info
    /// -          name: `raycastDistanceDown`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// -          name: `originalGroundHeightMS`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_originalGroundHeightMS: f32,
    /// # C++ Info
    /// -          name: `verticalOffset`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `forwardAlignFraction`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forwardAlignFraction: f32,
    /// # C++ Info
    /// -          name: `sidewaysAlignFraction`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sidewaysAlignFraction: f32,
    /// # C++ Info
    /// -          name: `sidewaysSampleWidth`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sidewaysSampleWidth: f32,
    /// # C++ Info
    /// -          name: `lockFeetWhenPlanted`(ctype: `hkBool`)
    /// -        offset:  52(x86)/ 64(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lockFeetWhenPlanted: bool,
    /// # C++ Info
    /// -          name: `useCharacterUpVector`(ctype: `hkBool`)
    /// -        offset:  53(x86)/ 65(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useCharacterUpVector: bool,
    /// # C++ Info
    /// -          name: `isQuadrupedNarrow`(ctype: `hkBool`)
    /// -        offset:  54(x86)/ 66(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isQuadrupedNarrow: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbFootIkDriverInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkDriverInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc6a09dbf)
        }
    }
    impl _serde::Serialize for hkbFootIkDriverInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc6a09dbf)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkDriverInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("legs", &self.m_legs)?;
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
            serializer.serialize_array_field("legs", &self.m_legs)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
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
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
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
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkbFootIkDriverInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbFootIkDriverInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbFootIkDriverInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbFootIkDriverInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbFootIkDriverInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbFootIkDriverInfoVisitor<'de> {
    type Value = hkbFootIkDriverInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbFootIkDriverInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_legs: _serde::__private::Option<Vec<hkbFootIkDriverInfoLeg>> = _serde::__private::None;
        let mut m_raycastDistanceUp: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_originalGroundHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
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
                    if _serde::__private::Option::is_some(&m_raycastDistanceUp) {
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
                    if _serde::__private::Option::is_some(&m_raycastDistanceDown) {
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
                    if _serde::__private::Option::is_some(&m_originalGroundHeightMS) {
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
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_forwardAlignFraction) {
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
                    if _serde::__private::Option::is_some(&m_sidewaysAlignFraction) {
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
                    if _serde::__private::Option::is_some(&m_sidewaysSampleWidth) {
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
                    if _serde::__private::Option::is_some(&m_lockFeetWhenPlanted) {
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
                    if _serde::__private::Option::is_some(&m_useCharacterUpVector) {
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
                    if _serde::__private::Option::is_some(&m_isQuadrupedNarrow) {
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
                    <__A::Error as _serde::de::Error>::missing_field("raycastDistanceUp"),
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
                    <__A::Error as _serde::de::Error>::missing_field("verticalOffset"),
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
                    <__A::Error as _serde::de::Error>::missing_field("isQuadrupedNarrow"),
                );
            }
        };
        _serde::__private::Ok(hkbFootIkDriverInfo {
            __ptr: __A::class_ptr(&mut __map),
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
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_legs: _serde::__private::Option<Vec<hkbFootIkDriverInfoLeg>> = _serde::__private::None;
        let mut m_raycastDistanceUp: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_originalGroundHeightMS: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_verticalOffset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_forwardAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_sidewaysAlignFraction: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_sidewaysSampleWidth: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_lockFeetWhenPlanted: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_useCharacterUpVector: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isQuadrupedNarrow: _serde::__private::Option<bool> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_legs => {
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
                __Field::m_raycastDistanceUp => {
                    if _serde::__private::Option::is_some(&m_raycastDistanceUp) {
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
                    if _serde::__private::Option::is_some(&m_raycastDistanceDown) {
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
                    if _serde::__private::Option::is_some(&m_originalGroundHeightMS) {
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
                __Field::m_collisionFilterInfo => {
                    if _serde::__private::Option::is_some(&m_collisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionFilterInfo",
                            ),
                        );
                    }
                    m_collisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_forwardAlignFraction => {
                    if _serde::__private::Option::is_some(&m_forwardAlignFraction) {
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
                    if _serde::__private::Option::is_some(&m_sidewaysAlignFraction) {
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
                    if _serde::__private::Option::is_some(&m_sidewaysSampleWidth) {
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
                    if _serde::__private::Option::is_some(&m_lockFeetWhenPlanted) {
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
                    if _serde::__private::Option::is_some(&m_useCharacterUpVector) {
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
                    if _serde::__private::Option::is_some(&m_isQuadrupedNarrow) {
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
                    <__A::Error as _serde::de::Error>::missing_field("raycastDistanceUp"),
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
                    <__A::Error as _serde::de::Error>::missing_field("verticalOffset"),
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
                    <__A::Error as _serde::de::Error>::missing_field("isQuadrupedNarrow"),
                );
            }
        };
        _serde::__private::Ok(hkbFootIkDriverInfo {
            __ptr: __A::class_ptr(&mut __map),
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbFootIkDriverInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
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
