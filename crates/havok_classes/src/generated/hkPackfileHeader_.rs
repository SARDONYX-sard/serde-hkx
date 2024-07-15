use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkPackfileHeader`
/// - version: `1`
/// - signature: `0x79f9ffda`
/// - size: ` 64`(x86)/` 64`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkPackfileHeader {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `magic`(ctype: `hkInt32[2]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_magic: [i32; 2usize],
    /// # C++ Info
    /// - name: `userTag`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_userTag: i32,
    /// # C++ Info
    /// - name: `fileVersion`(ctype: `hkInt32`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_fileVersion: i32,
    /// # C++ Info
    /// - name: `layoutRules`(ctype: `hkUint8[4]`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_layoutRules: [u8; 4usize],
    /// # C++ Info
    /// - name: `numSections`(ctype: `hkInt32`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numSections: i32,
    /// # C++ Info
    /// - name: `contentsSectionIndex`(ctype: `hkInt32`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_contentsSectionIndex: i32,
    /// # C++ Info
    /// - name: `contentsSectionOffset`(ctype: `hkInt32`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_contentsSectionOffset: i32,
    /// # C++ Info
    /// - name: `contentsClassNameSectionIndex`(ctype: `hkInt32`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_contentsClassNameSectionIndex: i32,
    /// # C++ Info
    /// - name: `contentsClassNameSectionOffset`(ctype: `hkInt32`)
    /// - offset: ` 36`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_contentsClassNameSectionOffset: i32,
    /// # C++ Info
    /// - name: `contentsVersion`(ctype: `hkChar[16]`)
    /// - offset: ` 40`(x86)/` 40`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_contentsVersion: [char; 16usize],
    /// # C++ Info
    /// - name: `flags`(ctype: `hkInt32`)
    /// - offset: ` 56`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_flags: i32,
    /// # C++ Info
    /// - name: `pad`(ctype: `hkInt32[1]`)
    /// - offset: ` 60`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_pad: [i32; 1usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkPackfileHeader {
        #[inline]
        fn name(&self) -> &'static str {
            "hkPackfileHeader"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x79f9ffda)
        }
    }
    impl _serde::Serialize for hkPackfileHeader {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x79f9ffda)));
            let mut serializer = __serializer
                .serialize_struct("hkPackfileHeader", class_meta)?;
            serializer.serialize_field("magic", &self.m_magic.as_slice())?;
            serializer.serialize_field("userTag", &self.m_userTag)?;
            serializer.serialize_field("fileVersion", &self.m_fileVersion)?;
            serializer.serialize_field("layoutRules", &self.m_layoutRules.as_slice())?;
            serializer.serialize_field("numSections", &self.m_numSections)?;
            serializer
                .serialize_field("contentsSectionIndex", &self.m_contentsSectionIndex)?;
            serializer
                .serialize_field(
                    "contentsSectionOffset",
                    &self.m_contentsSectionOffset,
                )?;
            serializer
                .serialize_field(
                    "contentsClassNameSectionIndex",
                    &self.m_contentsClassNameSectionIndex,
                )?;
            serializer
                .serialize_field(
                    "contentsClassNameSectionOffset",
                    &self.m_contentsClassNameSectionOffset,
                )?;
            serializer
                .serialize_field("contentsVersion", &self.m_contentsVersion.as_slice())?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("pad", &self.m_pad.as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_magic,
    m_userTag,
    m_fileVersion,
    m_layoutRules,
    m_numSections,
    m_contentsSectionIndex,
    m_contentsSectionOffset,
    m_contentsClassNameSectionIndex,
    m_contentsClassNameSectionOffset,
    m_contentsVersion,
    m_flags,
    m_pad,
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
            "magic" => Ok(__Field::m_magic),
            "userTag" => Ok(__Field::m_userTag),
            "fileVersion" => Ok(__Field::m_fileVersion),
            "layoutRules" => Ok(__Field::m_layoutRules),
            "numSections" => Ok(__Field::m_numSections),
            "contentsSectionIndex" => Ok(__Field::m_contentsSectionIndex),
            "contentsSectionOffset" => Ok(__Field::m_contentsSectionOffset),
            "contentsClassNameSectionIndex" => {
                Ok(__Field::m_contentsClassNameSectionIndex)
            }
            "contentsClassNameSectionOffset" => {
                Ok(__Field::m_contentsClassNameSectionOffset)
            }
            "contentsVersion" => Ok(__Field::m_contentsVersion),
            "flags" => Ok(__Field::m_flags),
            "pad" => Ok(__Field::m_pad),
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
pub(super) struct __hkPackfileHeaderVisitor<'de> {
    marker: core::marker::PhantomData<hkPackfileHeader>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkPackfileHeaderVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkPackfileHeader, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkPackfileHeader>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkPackfileHeaderVisitor<'de> {
    type Value = hkPackfileHeader;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkPackfileHeader")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_magic: _serde::__private::Option<[i32; 2usize]> = _serde::__private::None;
        let mut m_userTag: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_fileVersion: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_layoutRules: _serde::__private::Option<[u8; 4usize]> = _serde::__private::None;
        let mut m_numSections: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsSectionIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsSectionOffset: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsClassNameSectionIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsClassNameSectionOffset: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsVersion: _serde::__private::Option<[char; 16usize]> = _serde::__private::None;
        let mut m_flags: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_pad: _serde::__private::Option<[i32; 1usize]> = _serde::__private::None;
        for i in 0..12usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_magic) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("magic"),
                        );
                    }
                    m_magic = _serde::__private::Some(
                        match __A::next_value::<[i32; 2usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_userTag) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("userTag"),
                        );
                    }
                    m_userTag = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_fileVersion) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fileVersion",
                            ),
                        );
                    }
                    m_fileVersion = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_layoutRules) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "layoutRules",
                            ),
                        );
                    }
                    m_layoutRules = _serde::__private::Some(
                        match __A::next_value::<[u8; 4usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_numSections) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numSections",
                            ),
                        );
                    }
                    m_numSections = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_contentsSectionIndex) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contentsSectionIndex",
                            ),
                        );
                    }
                    m_contentsSectionIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_contentsSectionOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contentsSectionOffset",
                            ),
                        );
                    }
                    m_contentsSectionOffset = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(
                        &m_contentsClassNameSectionIndex,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contentsClassNameSectionIndex",
                            ),
                        );
                    }
                    m_contentsClassNameSectionIndex = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(
                        &m_contentsClassNameSectionOffset,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contentsClassNameSectionOffset",
                            ),
                        );
                    }
                    m_contentsClassNameSectionOffset = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_contentsVersion) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contentsVersion",
                            ),
                        );
                    }
                    m_contentsVersion = _serde::__private::Some(
                        match __A::next_value::<[char; 16usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_flags) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                        );
                    }
                    m_flags = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_pad) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pad"),
                        );
                    }
                    m_pad = _serde::__private::Some(
                        match __A::next_value::<[i32; 1usize]>(&mut __map) {
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
        let m_magic = match m_magic {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("magic"),
                );
            }
        };
        let m_userTag = match m_userTag {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userTag"),
                );
            }
        };
        let m_fileVersion = match m_fileVersion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fileVersion"),
                );
            }
        };
        let m_layoutRules = match m_layoutRules {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("layoutRules"),
                );
            }
        };
        let m_numSections = match m_numSections {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numSections"),
                );
            }
        };
        let m_contentsSectionIndex = match m_contentsSectionIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsSectionIndex",
                    ),
                );
            }
        };
        let m_contentsSectionOffset = match m_contentsSectionOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsSectionOffset",
                    ),
                );
            }
        };
        let m_contentsClassNameSectionIndex = match m_contentsClassNameSectionIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsClassNameSectionIndex",
                    ),
                );
            }
        };
        let m_contentsClassNameSectionOffset = match m_contentsClassNameSectionOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsClassNameSectionOffset",
                    ),
                );
            }
        };
        let m_contentsVersion = match m_contentsVersion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contentsVersion"),
                );
            }
        };
        let m_flags = match m_flags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("flags"),
                );
            }
        };
        let m_pad = match m_pad {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad"),
                );
            }
        };
        _serde::__private::Ok(hkPackfileHeader {
            __ptr,
            m_magic,
            m_userTag,
            m_fileVersion,
            m_layoutRules,
            m_numSections,
            m_contentsSectionIndex,
            m_contentsSectionOffset,
            m_contentsClassNameSectionIndex,
            m_contentsClassNameSectionOffset,
            m_contentsVersion,
            m_flags,
            m_pad,
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
        let mut m_magic: _serde::__private::Option<[i32; 2usize]> = _serde::__private::None;
        let mut m_userTag: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_fileVersion: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_layoutRules: _serde::__private::Option<[u8; 4usize]> = _serde::__private::None;
        let mut m_numSections: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsSectionIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsSectionOffset: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsClassNameSectionIndex: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsClassNameSectionOffset: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_contentsVersion: _serde::__private::Option<[char; 16usize]> = _serde::__private::None;
        let mut m_flags: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_pad: _serde::__private::Option<[i32; 1usize]> = _serde::__private::None;
        for _ in 0..12usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_magic => {
                        if _serde::__private::Option::is_some(&m_magic) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("magic"),
                            );
                        }
                        m_magic = _serde::__private::Some(
                            match __A::next_value::<[i32; 2usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userTag => {
                        if _serde::__private::Option::is_some(&m_userTag) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userTag",
                                ),
                            );
                        }
                        m_userTag = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fileVersion => {
                        if _serde::__private::Option::is_some(&m_fileVersion) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fileVersion",
                                ),
                            );
                        }
                        m_fileVersion = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_layoutRules => {
                        if _serde::__private::Option::is_some(&m_layoutRules) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "layoutRules",
                                ),
                            );
                        }
                        m_layoutRules = _serde::__private::Some(
                            match __A::next_value::<[u8; 4usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_numSections => {
                        if _serde::__private::Option::is_some(&m_numSections) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numSections",
                                ),
                            );
                        }
                        m_numSections = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contentsSectionIndex => {
                        if _serde::__private::Option::is_some(&m_contentsSectionIndex) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contentsSectionIndex",
                                ),
                            );
                        }
                        m_contentsSectionIndex = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contentsSectionOffset => {
                        if _serde::__private::Option::is_some(&m_contentsSectionOffset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contentsSectionOffset",
                                ),
                            );
                        }
                        m_contentsSectionOffset = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contentsClassNameSectionIndex => {
                        if _serde::__private::Option::is_some(
                            &m_contentsClassNameSectionIndex,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contentsClassNameSectionIndex",
                                ),
                            );
                        }
                        m_contentsClassNameSectionIndex = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contentsClassNameSectionOffset => {
                        if _serde::__private::Option::is_some(
                            &m_contentsClassNameSectionOffset,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contentsClassNameSectionOffset",
                                ),
                            );
                        }
                        m_contentsClassNameSectionOffset = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_contentsVersion => {
                        if _serde::__private::Option::is_some(&m_contentsVersion) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "contentsVersion",
                                ),
                            );
                        }
                        m_contentsVersion = _serde::__private::Some(
                            match __A::next_value::<[char; 16usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_flags => {
                        if _serde::__private::Option::is_some(&m_flags) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                            );
                        }
                        m_flags = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_pad => {
                        if _serde::__private::Option::is_some(&m_pad) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("pad"),
                            );
                        }
                        m_pad = _serde::__private::Some(
                            match __A::next_value::<[i32; 1usize]>(&mut __map) {
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
        let m_magic = match m_magic {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("magic"),
                );
            }
        };
        let m_userTag = match m_userTag {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userTag"),
                );
            }
        };
        let m_fileVersion = match m_fileVersion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fileVersion"),
                );
            }
        };
        let m_layoutRules = match m_layoutRules {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("layoutRules"),
                );
            }
        };
        let m_numSections = match m_numSections {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numSections"),
                );
            }
        };
        let m_contentsSectionIndex = match m_contentsSectionIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsSectionIndex",
                    ),
                );
            }
        };
        let m_contentsSectionOffset = match m_contentsSectionOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsSectionOffset",
                    ),
                );
            }
        };
        let m_contentsClassNameSectionIndex = match m_contentsClassNameSectionIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsClassNameSectionIndex",
                    ),
                );
            }
        };
        let m_contentsClassNameSectionOffset = match m_contentsClassNameSectionOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contentsClassNameSectionOffset",
                    ),
                );
            }
        };
        let m_contentsVersion = match m_contentsVersion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contentsVersion"),
                );
            }
        };
        let m_flags = match m_flags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("flags"),
                );
            }
        };
        let m_pad = match m_pad {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pad"),
                );
            }
        };
        _serde::__private::Ok(hkPackfileHeader {
            __ptr,
            m_magic,
            m_userTag,
            m_fileVersion,
            m_layoutRules,
            m_numSections,
            m_contentsSectionIndex,
            m_contentsSectionOffset,
            m_contentsClassNameSectionIndex,
            m_contentsClassNameSectionOffset,
            m_contentsVersion,
            m_flags,
            m_pad,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkPackfileHeader {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "magic",
                "userTag",
                "fileVersion",
                "layoutRules",
                "numSections",
                "contentsSectionIndex",
                "contentsSectionOffset",
                "contentsClassNameSectionIndex",
                "contentsClassNameSectionOffset",
                "contentsVersion",
                "flags",
                "pad",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkPackfileHeader",
                FIELDS,
                __hkPackfileHeaderVisitor {
                    marker: _serde::__private::PhantomData::<hkPackfileHeader>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
