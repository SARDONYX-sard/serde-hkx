use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxTextureInplace`
/// -         version: `1`
/// -       signature: `0xd45841d6`
/// -          size:  32(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxTextureInplace<'a> {
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
    /// -          name: `fileType`(ctype: `hkChar[4]`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fileType: [char; 4usize],
    /// # C++ Info
    /// -          name: `data`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_data: Vec<u8>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `originalFilename`(ctype: `hkStringPtr`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_originalFilename: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkxTextureInplace<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxTextureInplace"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd45841d6)
        }
    }
    impl<'a> _serde::Serialize for hkxTextureInplace<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd45841d6)));
            let mut serializer = __serializer
                .serialize_struct("hkxTextureInplace", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("fileType", &self.m_fileType.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalFilename",
                    &self.m_originalFilename,
                )?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer
                .serialize_stringptr_field(
                    "originalFilename",
                    &self.m_originalFilename,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_fileType,
    m_data,
    m_name,
    m_originalFilename,
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
            "fileType" => Ok(__Field::m_fileType),
            "data" => Ok(__Field::m_data),
            "name" => Ok(__Field::m_name),
            "originalFilename" => Ok(__Field::m_originalFilename),
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
pub(super) struct __hkxTextureInplaceVisitor<'de> {
    marker: core::marker::PhantomData<hkxTextureInplace<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxTextureInplaceVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxTextureInplace<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxTextureInplace<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxTextureInplaceVisitor<'de> {
    type Value = hkxTextureInplace<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxTextureInplace")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_fileType: _serde::__private::Option<[char; 4usize]> = _serde::__private::None;
        let mut m_data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_originalFilename: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_fileType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fileType",
                            ),
                        );
                    }
                    m_fileType = _serde::__private::Some(
                        match __A::next_value::<[char; 4usize]>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_data = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_name) {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_originalFilename) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "originalFilename",
                            ),
                        );
                    }
                    m_originalFilename = _serde::__private::Some(
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
        let m_fileType = match m_fileType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fileType"),
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
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_originalFilename = match m_originalFilename {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("originalFilename"),
                );
            }
        };
        _serde::__private::Ok(hkxTextureInplace {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_fileType,
            m_data,
            m_name,
            m_originalFilename,
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
        let mut m_fileType: _serde::__private::Option<[char; 4usize]> = _serde::__private::None;
        let mut m_data: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        let mut m_originalFilename: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_fileType => {
                        if _serde::__private::Option::is_some(&m_fileType) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fileType",
                                ),
                            );
                        }
                        m_fileType = _serde::__private::Some(
                            match __A::next_value::<[char; 4usize]>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_data => {
                        if _serde::__private::Option::is_some(&m_data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                            );
                        }
                        m_data = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_name => {
                        if _serde::__private::Option::is_some(&m_name) {
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
                    __Field::m_originalFilename => {
                        if _serde::__private::Option::is_some(&m_originalFilename) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "originalFilename",
                                ),
                            );
                        }
                        m_originalFilename = _serde::__private::Some(
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
        }
        let m_fileType = match m_fileType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fileType"),
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
        let m_name = match m_name {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("name"),
                );
            }
        };
        let m_originalFilename = match m_originalFilename {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("originalFilename"),
                );
            }
        };
        _serde::__private::Ok(hkxTextureInplace {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_fileType,
            m_data,
            m_name,
            m_originalFilename,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxTextureInplace<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["fileType", "data", "name", "originalFilename"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxTextureInplace",
                FIELDS,
                __hkxTextureInplaceVisitor {
                    marker: _serde::__private::PhantomData::<hkxTextureInplace>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
