use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbProjectStringData`
/// -         version: `1`
/// -       signature: `0x76ad60a`
/// -          size:  76(x86)/120(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbProjectStringData<'a> {
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
    /// -          name: `animationFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_animationFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `behaviorFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_behaviorFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `characterFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_characterFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `eventNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_eventNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// -          name: `animationPath`(ctype: `hkStringPtr`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_animationPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `behaviorPath`(ctype: `hkStringPtr`)
    /// -        offset:  60(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `characterPath`(ctype: `hkStringPtr`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_characterPath: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `fullPathToSource`(ctype: `hkStringPtr`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_fullPathToSource: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `rootPath`(ctype: `hkStringPtr`)
    /// -        offset:  72(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_rootPath: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbProjectStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbProjectStringData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x76ad60a)
        }
    }
    impl<'a> _serde::Serialize for hkbProjectStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x76ad60a)));
            let mut serializer =
                __serializer.serialize_struct("hkbProjectStringData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("animationFilenames", &self.m_animationFilenames)?;
            serializer
                .serialize_array_meta_field("behaviorFilenames", &self.m_behaviorFilenames)?;
            serializer
                .serialize_array_meta_field("characterFilenames", &self.m_characterFilenames)?;
            serializer.serialize_array_meta_field("eventNames", &self.m_eventNames)?;
            serializer.serialize_stringptr_meta_field("animationPath", &self.m_animationPath)?;
            serializer.serialize_stringptr_meta_field("behaviorPath", &self.m_behaviorPath)?;
            serializer.serialize_stringptr_meta_field("characterPath", &self.m_characterPath)?;
            serializer
                .serialize_stringptr_meta_field("fullPathToSource", &self.m_fullPathToSource)?;
            serializer.skip_stringptr_meta_field("rootPath", &self.m_rootPath)?;
            serializer.serialize_array_field("animationFilenames", &self.m_animationFilenames)?;
            serializer.serialize_array_field("behaviorFilenames", &self.m_behaviorFilenames)?;
            serializer.serialize_array_field("characterFilenames", &self.m_characterFilenames)?;
            serializer.serialize_array_field("eventNames", &self.m_eventNames)?;
            serializer.serialize_stringptr_field("animationPath", &self.m_animationPath)?;
            serializer.serialize_stringptr_field("behaviorPath", &self.m_behaviorPath)?;
            serializer.serialize_stringptr_field("characterPath", &self.m_characterPath)?;
            serializer.serialize_stringptr_field("fullPathToSource", &self.m_fullPathToSource)?;
            serializer.serialize_stringptr_field("rootPath", &self.m_rootPath)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum __Field {
    m_animationFilenames,
    m_behaviorFilenames,
    m_characterFilenames,
    m_eventNames,
    m_animationPath,
    m_behaviorPath,
    m_characterPath,
    m_fullPathToSource,
    m_rootPath,
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
            "animationFilenames" => Ok(__Field::m_animationFilenames),
            "behaviorFilenames" => Ok(__Field::m_behaviorFilenames),
            "characterFilenames" => Ok(__Field::m_characterFilenames),
            "eventNames" => Ok(__Field::m_eventNames),
            "animationPath" => Ok(__Field::m_animationPath),
            "behaviorPath" => Ok(__Field::m_behaviorPath),
            "characterPath" => Ok(__Field::m_characterPath),
            "fullPathToSource" => Ok(__Field::m_fullPathToSource),
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
pub(super) struct __hkbProjectStringDataVisitor<'de> {
    marker: core::marker::PhantomData<hkbProjectStringData<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbProjectStringDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbProjectStringData<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbProjectStringData<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbProjectStringDataVisitor<'de> {
    type Value = hkbProjectStringData<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbProjectStringData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_animationFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_behaviorFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_characterFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_eventNames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_animationPath: _serde::__private::Option<StringPtr<'de>> =
            _serde::__private::None;
        let mut m_behaviorPath: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_characterPath: _serde::__private::Option<StringPtr<'de>> =
            _serde::__private::None;
        let mut m_fullPathToSource: _serde::__private::Option<StringPtr<'de>> =
            _serde::__private::None;
        let mut m_rootPath: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..9usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_animationFilenames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "animationFilenames",
                            ),
                        );
                    }
                    m_animationFilenames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_behaviorFilenames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("behaviorFilenames"),
                        );
                    }
                    m_behaviorFilenames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_characterFilenames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "characterFilenames",
                            ),
                        );
                    }
                    m_characterFilenames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_eventNames) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("eventNames"),
                        );
                    }
                    m_eventNames = _serde::__private::Some(
                        match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_animationPath) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("animationPath"),
                        );
                    }
                    m_animationPath = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_behaviorPath) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("behaviorPath"),
                        );
                    }
                    m_behaviorPath = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_characterPath) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("characterPath"),
                        );
                    }
                    m_characterPath = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_fullPathToSource) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("fullPathToSource"),
                        );
                    }
                    m_fullPathToSource = _serde::__private::Some(
                        match __A::next_value::<StringPtr<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_rootPath) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("rootPath"),
                        );
                    }
                    m_rootPath = _serde::__private::Some(
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
        let m_animationFilenames = match m_animationFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "animationFilenames",
                ));
            }
        };
        let m_behaviorFilenames = match m_behaviorFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "behaviorFilenames",
                ));
            }
        };
        let m_characterFilenames = match m_characterFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "characterFilenames",
                ));
            }
        };
        let m_eventNames = match m_eventNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "eventNames",
                ));
            }
        };
        let m_animationPath = match m_animationPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "animationPath",
                ));
            }
        };
        let m_behaviorPath = match m_behaviorPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "behaviorPath",
                ));
            }
        };
        let m_characterPath = match m_characterPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "characterPath",
                ));
            }
        };
        let m_fullPathToSource = match m_fullPathToSource {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "fullPathToSource",
                ));
            }
        };
        let m_rootPath = match m_rootPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "rootPath",
                ));
            }
        };
        _serde::__private::Ok(hkbProjectStringData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_animationFilenames,
            m_behaviorFilenames,
            m_characterFilenames,
            m_eventNames,
            m_animationPath,
            m_behaviorPath,
            m_characterPath,
            m_fullPathToSource,
            m_rootPath,
        })
    }
    fn visit_struct<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_animationFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_behaviorFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_characterFilenames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_eventNames: _serde::__private::Option<Vec<StringPtr<'de>>> =
            _serde::__private::None;
        let mut m_animationPath: _serde::__private::Option<StringPtr<'de>> =
            _serde::__private::None;
        let mut m_behaviorPath: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_characterPath: _serde::__private::Option<StringPtr<'de>> =
            _serde::__private::None;
        let mut m_fullPathToSource: _serde::__private::Option<StringPtr<'de>> =
            _serde::__private::None;

        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<__Field>(&mut __map)? {
                tracing::debug!(?__key);
                match __key {
                    __Field::m_animationFilenames => {
                        if _serde::__private::Option::is_some(&m_animationFilenames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "animationFilenames",
                                ),
                            );
                        }
                        m_animationFilenames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_behaviorFilenames => {
                        if _serde::__private::Option::is_some(&m_behaviorFilenames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "behaviorFilenames",
                                ),
                            );
                        }
                        m_behaviorFilenames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_characterFilenames => {
                        if _serde::__private::Option::is_some(&m_characterFilenames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "characterFilenames",
                                ),
                            );
                        }

                        m_characterFilenames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_eventNames => {
                        if _serde::__private::Option::is_some(&m_eventNames) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("eventNames"),
                            );
                        }
                        m_eventNames = _serde::__private::Some(
                            match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_animationPath => {
                        if _serde::__private::Option::is_some(&m_animationPath) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("animationPath"),
                            );
                        }
                        m_animationPath = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_behaviorPath => {
                        if _serde::__private::Option::is_some(&m_behaviorPath) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("behaviorPath"),
                            );
                        }
                        m_behaviorPath = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_characterPath => {
                        if _serde::__private::Option::is_some(&m_characterPath) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("characterPath"),
                            );
                        }
                        m_characterPath = _serde::__private::Some(
                            match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fullPathToSource => {
                        if _serde::__private::Option::is_some(&m_fullPathToSource) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fullPathToSource",
                                ),
                            );
                        }
                        m_fullPathToSource = _serde::__private::Some(
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
            };
        }

        let m_animationFilenames = match m_animationFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "animationFilenames",
                ));
            }
        };
        let m_behaviorFilenames = match m_behaviorFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "behaviorFilenames",
                ));
            }
        };
        let m_characterFilenames = match m_characterFilenames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "characterFilenames",
                ));
            }
        };
        let m_eventNames = match m_eventNames {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "eventNames",
                ));
            }
        };
        let m_animationPath = match m_animationPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "animationPath",
                ));
            }
        };
        let m_behaviorPath = match m_behaviorPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "behaviorPath",
                ));
            }
        };
        let m_characterPath = match m_characterPath {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "characterPath",
                ));
            }
        };
        let m_fullPathToSource = match m_fullPathToSource {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(<__A::Error as _serde::de::Error>::missing_field(
                    "fullPathToSource",
                ));
            }
        };
        _serde::__private::Ok(hkbProjectStringData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_animationFilenames,
            m_behaviorFilenames,
            m_characterFilenames,
            m_eventNames,
            m_animationPath,
            m_behaviorPath,
            m_characterPath,
            m_fullPathToSource,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbProjectStringData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "animationFilenames",
                "behaviorFilenames",
                "characterFilenames",
                "eventNames",
                "animationPath",
                "behaviorPath",
                "characterPath",
                "fullPathToSource",
                "rootPath",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbProjectStringData",
                FIELDS,
                __hkbProjectStringDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbProjectStringData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
