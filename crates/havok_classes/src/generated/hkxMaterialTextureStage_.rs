use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxMaterialTextureStage`
/// -         version: `0`
/// -       signature: `0xfa6facb2`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxMaterialTextureStage {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `texture`(ctype: `struct hkReferencedObject*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_texture: Pointer,
    /// # C++ Info
    /// -          name: `usageHint`(ctype: `enum TextureType`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_usageHint: TextureType,
    /// # C++ Info
    /// -          name: `tcoordChannel`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tcoordChannel: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxMaterialTextureStage {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxMaterialTextureStage"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfa6facb2)
        }
    }
    impl _serde::Serialize for hkxMaterialTextureStage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xfa6facb2)));
            let mut serializer = __serializer
                .serialize_struct("hkxMaterialTextureStage", class_meta)?;
            serializer.serialize_field("texture", &self.m_texture)?;
            serializer.serialize_field("usageHint", &self.m_usageHint)?;
            serializer.serialize_field("tcoordChannel", &self.m_tcoordChannel)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_texture,
    m_usageHint,
    m_tcoordChannel,
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
            "texture" => Ok(__Field::m_texture),
            "usageHint" => Ok(__Field::m_usageHint),
            "tcoordChannel" => Ok(__Field::m_tcoordChannel),
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
pub(super) struct __hkxMaterialTextureStageVisitor<'de> {
    marker: core::marker::PhantomData<hkxMaterialTextureStage>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxMaterialTextureStageVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxMaterialTextureStage, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxMaterialTextureStage>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxMaterialTextureStageVisitor<'de> {
    type Value = hkxMaterialTextureStage;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxMaterialTextureStage")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_texture: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_usageHint: _serde::__private::Option<TextureType> = _serde::__private::None;
        let mut m_tcoordChannel: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_texture) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("texture"),
                        );
                    }
                    m_texture = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_usageHint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "usageHint",
                            ),
                        );
                    }
                    m_usageHint = _serde::__private::Some(
                        match __A::next_value::<TextureType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_tcoordChannel) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "tcoordChannel",
                            ),
                        );
                    }
                    m_tcoordChannel = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        let m_texture = match m_texture {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("texture"),
                );
            }
        };
        let m_usageHint = match m_usageHint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("usageHint"),
                );
            }
        };
        let m_tcoordChannel = match m_tcoordChannel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tcoordChannel"),
                );
            }
        };
        _serde::__private::Ok(hkxMaterialTextureStage {
            __ptr: __A::class_ptr(&mut __map),
            m_texture,
            m_usageHint,
            m_tcoordChannel,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_texture: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_usageHint: _serde::__private::Option<TextureType> = _serde::__private::None;
        let mut m_tcoordChannel: _serde::__private::Option<i32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_texture => {
                    if _serde::__private::Option::is_some(&m_texture) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("texture"),
                        );
                    }
                    m_texture = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_usageHint => {
                    if _serde::__private::Option::is_some(&m_usageHint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "usageHint",
                            ),
                        );
                    }
                    m_usageHint = _serde::__private::Some(
                        match __A::next_value::<TextureType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_tcoordChannel => {
                    if _serde::__private::Option::is_some(&m_tcoordChannel) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "tcoordChannel",
                            ),
                        );
                    }
                    m_tcoordChannel = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
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
        let m_texture = match m_texture {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("texture"),
                );
            }
        };
        let m_usageHint = match m_usageHint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("usageHint"),
                );
            }
        };
        let m_tcoordChannel = match m_tcoordChannel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tcoordChannel"),
                );
            }
        };
        _serde::__private::Ok(hkxMaterialTextureStage {
            __ptr: __A::class_ptr(&mut __map),
            m_texture,
            m_usageHint,
            m_tcoordChannel,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxMaterialTextureStage {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["texture", "usageHint", "tcoordChannel"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxMaterialTextureStage",
                FIELDS,
                __hkxMaterialTextureStageVisitor {
                    marker: _serde::__private::PhantomData::<hkxMaterialTextureStage>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
