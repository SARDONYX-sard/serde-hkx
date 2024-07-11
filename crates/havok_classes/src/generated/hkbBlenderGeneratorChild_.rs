use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBlenderGeneratorChild`
/// -         version: `2`
/// -       signature: `0xe2b384b0`
/// -          size:  48(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlenderGeneratorChild {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbBindable,
    /// # C++ Info
    /// -          name: `generator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_generator: Pointer,
    /// # C++ Info
    /// -          name: `boneWeights`(ctype: `struct hkbBoneWeightArray*`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_boneWeights: Pointer,
    /// # C++ Info
    /// -          name: `weight`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_weight: f32,
    /// # C++ Info
    /// -          name: `worldFromModelWeight`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_worldFromModelWeight: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbBlenderGeneratorChild {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlenderGeneratorChild"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe2b384b0)
        }
    }
    impl _serde::Serialize for hkbBlenderGeneratorChild {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe2b384b0)));
            let mut serializer = __serializer
                .serialize_struct("hkbBlenderGeneratorChild", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field("areBindablesCached", &self.parent.m_areBindablesCached)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("generator", &self.m_generator)?;
            serializer.serialize_field("boneWeights", &self.m_boneWeights)?;
            serializer.serialize_field("weight", &self.m_weight)?;
            serializer
                .serialize_field("worldFromModelWeight", &self.m_worldFromModelWeight)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.m_cachedBindables,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_generator,
    m_boneWeights,
    m_weight,
    m_worldFromModelWeight,
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
            "generator" => Ok(__Field::m_generator),
            "boneWeights" => Ok(__Field::m_boneWeights),
            "weight" => Ok(__Field::m_weight),
            "worldFromModelWeight" => Ok(__Field::m_worldFromModelWeight),
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
pub(super) struct __hkbBlenderGeneratorChildVisitor<'de> {
    marker: core::marker::PhantomData<hkbBlenderGeneratorChild>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbBlenderGeneratorChildVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbBlenderGeneratorChild, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbBlenderGeneratorChild>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbBlenderGeneratorChildVisitor<'de> {
    type Value = hkbBlenderGeneratorChild;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbBlenderGeneratorChild")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_generator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_boneWeights: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_weight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_worldFromModelWeight: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_generator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "generator",
                            ),
                        );
                    }
                    m_generator = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_boneWeights) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "boneWeights",
                            ),
                        );
                    }
                    m_boneWeights = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_weight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("weight"),
                        );
                    }
                    m_weight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_worldFromModelWeight) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "worldFromModelWeight",
                            ),
                        );
                    }
                    m_worldFromModelWeight = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 8usize)?;
        let m_generator = match m_generator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("generator"),
                );
            }
        };
        let m_boneWeights = match m_boneWeights {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneWeights"),
                );
            }
        };
        let m_weight = match m_weight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weight"),
                );
            }
        };
        let m_worldFromModelWeight = match m_worldFromModelWeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelWeight",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbBlenderGeneratorChild {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_generator,
            m_boneWeights,
            m_weight,
            m_worldFromModelWeight,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkbBindableVisitor::visit_as_parent(&mut __map)?;
        let mut m_generator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_boneWeights: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_weight: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_worldFromModelWeight: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_generator => {
                        if _serde::__private::Option::is_some(&m_generator) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "generator",
                                ),
                            );
                        }
                        m_generator = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_boneWeights => {
                        if _serde::__private::Option::is_some(&m_boneWeights) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "boneWeights",
                                ),
                            );
                        }
                        m_boneWeights = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_weight => {
                        if _serde::__private::Option::is_some(&m_weight) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("weight"),
                            );
                        }
                        m_weight = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_worldFromModelWeight => {
                        if _serde::__private::Option::is_some(&m_worldFromModelWeight) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "worldFromModelWeight",
                                ),
                            );
                        }
                        m_worldFromModelWeight = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        }
        let m_generator = match m_generator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("generator"),
                );
            }
        };
        let m_boneWeights = match m_boneWeights {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneWeights"),
                );
            }
        };
        let m_weight = match m_weight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("weight"),
                );
            }
        };
        let m_worldFromModelWeight = match m_worldFromModelWeight {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelWeight",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbBlenderGeneratorChild {
            __ptr,
            parent,
            m_generator,
            m_boneWeights,
            m_weight,
            m_worldFromModelWeight,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbBlenderGeneratorChild {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "generator",
                "boneWeights",
                "weight",
                "worldFromModelWeight",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBlenderGeneratorChild",
                FIELDS,
                __hkbBlenderGeneratorChildVisitor {
                    marker: _serde::__private::PhantomData::<hkbBlenderGeneratorChild>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
