use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkAlignSceneToNodeOptions`
/// -         version: `2`
/// -       signature: `0x207cb01`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkAlignSceneToNodeOptions<'a> {
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
    /// -          name: `invert`(ctype: `hkBool`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_invert: bool,
    /// # C++ Info
    /// -          name: `transformPositionX`(ctype: `hkBool`)
    /// -        offset:   9(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformPositionX: bool,
    /// # C++ Info
    /// -          name: `transformPositionY`(ctype: `hkBool`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformPositionY: bool,
    /// # C++ Info
    /// -          name: `transformPositionZ`(ctype: `hkBool`)
    /// -        offset:  11(x86)/ 19(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformPositionZ: bool,
    /// # C++ Info
    /// -          name: `transformRotation`(ctype: `hkBool`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformRotation: bool,
    /// # C++ Info
    /// -          name: `transformScale`(ctype: `hkBool`)
    /// -        offset:  13(x86)/ 21(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformScale: bool,
    /// # C++ Info
    /// -          name: `transformSkew`(ctype: `hkBool`)
    /// -        offset:  14(x86)/ 22(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_transformSkew: bool,
    /// # C++ Info
    /// -          name: `keyframe`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_keyframe: i32,
    /// # C++ Info
    /// -          name: `nodeName`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_nodeName: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkAlignSceneToNodeOptions<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkAlignSceneToNodeOptions"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x207cb01)
        }
    }
    impl<'a> _serde::Serialize for hkAlignSceneToNodeOptions<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x207cb01)));
            let mut serializer = __serializer
                .serialize_struct("hkAlignSceneToNodeOptions", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("invert", &self.m_invert)?;
            serializer
                .serialize_field("transformPositionX", &self.m_transformPositionX)?;
            serializer
                .serialize_field("transformPositionY", &self.m_transformPositionY)?;
            serializer
                .serialize_field("transformPositionZ", &self.m_transformPositionZ)?;
            serializer.serialize_field("transformRotation", &self.m_transformRotation)?;
            serializer.serialize_field("transformScale", &self.m_transformScale)?;
            serializer.serialize_field("transformSkew", &self.m_transformSkew)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("keyframe", &self.m_keyframe)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("nodeName", &self.m_nodeName)?;
            serializer.serialize_stringptr_field("nodeName", &self.m_nodeName)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_invert,
    m_transformPositionX,
    m_transformPositionY,
    m_transformPositionZ,
    m_transformRotation,
    m_transformScale,
    m_transformSkew,
    m_keyframe,
    m_nodeName,
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
            "invert" => Ok(__Field::m_invert),
            "transformPositionX" => Ok(__Field::m_transformPositionX),
            "transformPositionY" => Ok(__Field::m_transformPositionY),
            "transformPositionZ" => Ok(__Field::m_transformPositionZ),
            "transformRotation" => Ok(__Field::m_transformRotation),
            "transformScale" => Ok(__Field::m_transformScale),
            "transformSkew" => Ok(__Field::m_transformSkew),
            "keyframe" => Ok(__Field::m_keyframe),
            "nodeName" => Ok(__Field::m_nodeName),
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
pub(super) struct __hkAlignSceneToNodeOptionsVisitor<'de> {
    marker: core::marker::PhantomData<hkAlignSceneToNodeOptions<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkAlignSceneToNodeOptionsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkAlignSceneToNodeOptions<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkAlignSceneToNodeOptions<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkAlignSceneToNodeOptionsVisitor<'de> {
    type Value = hkAlignSceneToNodeOptions<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkAlignSceneToNodeOptions")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_invert: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformPositionX: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformPositionY: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformPositionZ: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformScale: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformSkew: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_keyframe: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_nodeName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..9usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_invert) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("invert"),
                        );
                    }
                    m_invert = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_transformPositionX) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformPositionX",
                            ),
                        );
                    }
                    m_transformPositionX = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_transformPositionY) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformPositionY",
                            ),
                        );
                    }
                    m_transformPositionY = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_transformPositionZ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformPositionZ",
                            ),
                        );
                    }
                    m_transformPositionZ = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_transformRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformRotation",
                            ),
                        );
                    }
                    m_transformRotation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_transformScale) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformScale",
                            ),
                        );
                    }
                    m_transformScale = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_transformSkew) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformSkew",
                            ),
                        );
                    }
                    m_transformSkew = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_keyframe) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keyframe",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_keyframe = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_nodeName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nodeName",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_nodeName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_invert = match m_invert {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("invert"),
                );
            }
        };
        let m_transformPositionX = match m_transformPositionX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "transformPositionX",
                    ),
                );
            }
        };
        let m_transformPositionY = match m_transformPositionY {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "transformPositionY",
                    ),
                );
            }
        };
        let m_transformPositionZ = match m_transformPositionZ {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "transformPositionZ",
                    ),
                );
            }
        };
        let m_transformRotation = match m_transformRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformRotation"),
                );
            }
        };
        let m_transformScale = match m_transformScale {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformScale"),
                );
            }
        };
        let m_transformSkew = match m_transformSkew {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformSkew"),
                );
            }
        };
        let m_keyframe = match m_keyframe {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("keyframe"),
                );
            }
        };
        let m_nodeName = match m_nodeName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeName"),
                );
            }
        };
        _serde::__private::Ok(hkAlignSceneToNodeOptions {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_invert,
            m_transformPositionX,
            m_transformPositionY,
            m_transformPositionZ,
            m_transformRotation,
            m_transformScale,
            m_transformSkew,
            m_keyframe,
            m_nodeName,
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
        let mut m_invert: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformPositionX: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformPositionY: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformPositionZ: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformScale: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_transformSkew: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_keyframe: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_nodeName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_invert => {
                    if _serde::__private::Option::is_some(&m_invert) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("invert"),
                        );
                    }
                    m_invert = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_transformPositionX => {
                    if _serde::__private::Option::is_some(&m_transformPositionX) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformPositionX",
                            ),
                        );
                    }
                    m_transformPositionX = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_transformPositionY => {
                    if _serde::__private::Option::is_some(&m_transformPositionY) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformPositionY",
                            ),
                        );
                    }
                    m_transformPositionY = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_transformPositionZ => {
                    if _serde::__private::Option::is_some(&m_transformPositionZ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformPositionZ",
                            ),
                        );
                    }
                    m_transformPositionZ = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_transformRotation => {
                    if _serde::__private::Option::is_some(&m_transformRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformRotation",
                            ),
                        );
                    }
                    m_transformRotation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_transformScale => {
                    if _serde::__private::Option::is_some(&m_transformScale) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformScale",
                            ),
                        );
                    }
                    m_transformScale = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_transformSkew => {
                    if _serde::__private::Option::is_some(&m_transformSkew) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transformSkew",
                            ),
                        );
                    }
                    m_transformSkew = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_keyframe => {
                    if _serde::__private::Option::is_some(&m_keyframe) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "keyframe",
                            ),
                        );
                    }
                    m_keyframe = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_nodeName => {
                    if _serde::__private::Option::is_some(&m_nodeName) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "nodeName",
                            ),
                        );
                    }
                    m_nodeName = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
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
        let m_invert = match m_invert {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("invert"),
                );
            }
        };
        let m_transformPositionX = match m_transformPositionX {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "transformPositionX",
                    ),
                );
            }
        };
        let m_transformPositionY = match m_transformPositionY {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "transformPositionY",
                    ),
                );
            }
        };
        let m_transformPositionZ = match m_transformPositionZ {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "transformPositionZ",
                    ),
                );
            }
        };
        let m_transformRotation = match m_transformRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformRotation"),
                );
            }
        };
        let m_transformScale = match m_transformScale {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformScale"),
                );
            }
        };
        let m_transformSkew = match m_transformSkew {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transformSkew"),
                );
            }
        };
        let m_keyframe = match m_keyframe {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("keyframe"),
                );
            }
        };
        let m_nodeName = match m_nodeName {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("nodeName"),
                );
            }
        };
        _serde::__private::Ok(hkAlignSceneToNodeOptions {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_invert,
            m_transformPositionX,
            m_transformPositionY,
            m_transformPositionZ,
            m_transformRotation,
            m_transformScale,
            m_transformSkew,
            m_keyframe,
            m_nodeName,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkAlignSceneToNodeOptions<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "invert",
                "transformPositionX",
                "transformPositionY",
                "transformPositionZ",
                "transformRotation",
                "transformScale",
                "transformSkew",
                "keyframe",
                "nodeName",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkAlignSceneToNodeOptions",
                FIELDS,
                __hkAlignSceneToNodeOptionsVisitor {
                    marker: _serde::__private::PhantomData::<hkAlignSceneToNodeOptions>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
