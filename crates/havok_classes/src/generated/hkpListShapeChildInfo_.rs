use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpListShapeChildInfo`
/// -         version: `0`
/// -       signature: `0x80df0f90`
/// -          size:  16(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpListShapeChildInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `shape`(ctype: `struct hkpShape*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_shape: Pointer,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `shapeSize`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 12(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_shapeSize: i32,
    /// # C++ Info
    /// -          name: `numChildShapes`(ctype: `hkInt32`)
    /// -        offset:  12(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_numChildShapes: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpListShapeChildInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpListShapeChildInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x80df0f90)
        }
    }
    impl _serde::Serialize for hkpListShapeChildInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x80df0f90)));
            let mut serializer = __serializer
                .serialize_struct("hkpListShapeChildInfo", class_meta)?;
            serializer.serialize_field("shape", &self.m_shape)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.skip_field("shapeSize", &self.m_shapeSize)?;
            serializer.skip_field("numChildShapes", &self.m_numChildShapes)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_shape,
    m_collisionFilterInfo,
    m_shapeSize,
    m_numChildShapes,
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
            "shape" => Ok(__Field::m_shape),
            "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
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
pub(super) struct __hkpListShapeChildInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpListShapeChildInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpListShapeChildInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpListShapeChildInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpListShapeChildInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpListShapeChildInfoVisitor<'de> {
    type Value = hkpListShapeChildInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpListShapeChildInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_shapeSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_numChildShapes: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_shape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                        );
                    }
                    m_shape = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
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
                2usize => {
                    if _serde::__private::Option::is_some(&m_shapeSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "shapeSize",
                            ),
                        );
                    }
                    m_shapeSize = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_numChildShapes) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numChildShapes",
                            ),
                        );
                    }
                    m_numChildShapes = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 12usize)?;
        let m_shape = match m_shape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shape"),
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
        let m_shapeSize = match m_shapeSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapeSize"),
                );
            }
        };
        let m_numChildShapes = match m_numChildShapes {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numChildShapes"),
                );
            }
        };
        _serde::__private::Ok(hkpListShapeChildInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_shape,
            m_collisionFilterInfo,
            m_shapeSize,
            m_numChildShapes,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_shape => {
                        if _serde::__private::Option::is_some(&m_shape) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                            );
                        }
                        m_shape = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
                    _ => {}
                }
            }
        }
        let m_shape = match m_shape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shape"),
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
        _serde::__private::Ok(hkpListShapeChildInfo {
            __ptr: __A::class_ptr(&mut __map),
            m_shape,
            m_collisionFilterInfo,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpListShapeChildInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "shape",
                "collisionFilterInfo",
                "shapeSize",
                "numChildShapes",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpListShapeChildInfo",
                FIELDS,
                __hkpListShapeChildInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkpListShapeChildInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
