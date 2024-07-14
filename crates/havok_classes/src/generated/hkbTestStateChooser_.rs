use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbTestStateChooser`
/// -         version: `0`
/// -       signature: `0xc0fcc436`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTestStateChooser<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbStateChooser,
    /// # C++ Info
    /// -          name: `int`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_int: i32,
    /// # C++ Info
    /// -          name: `real`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_real: f32,
    /// # C++ Info
    /// -          name: `string`(ctype: `hkStringPtr`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_string: StringPtr<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbTestStateChooser<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbTestStateChooser"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc0fcc436)
        }
    }
    impl<'a> _serde::Serialize for hkbTestStateChooser<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc0fcc436)));
            let mut serializer = __serializer
                .serialize_struct("hkbTestStateChooser", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("int", &self.m_int)?;
            serializer.serialize_field("real", &self.m_real)?;
            serializer.serialize_stringptr_meta_field("string", &self.m_string)?;
            serializer.serialize_stringptr_field("string", &self.m_string)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_int,
    m_real,
    m_string,
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
            "int" => Ok(__Field::m_int),
            "real" => Ok(__Field::m_real),
            "string" => Ok(__Field::m_string),
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
pub(super) struct __hkbTestStateChooserVisitor<'de> {
    marker: core::marker::PhantomData<hkbTestStateChooser<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbTestStateChooserVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbTestStateChooser<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbTestStateChooser<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbTestStateChooserVisitor<'de> {
    type Value = hkbTestStateChooser<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbTestStateChooser")
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
        let mut m_int: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_real: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_string: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_int) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("int"),
                        );
                    }
                    m_int = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_real) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("real"),
                        );
                    }
                    m_real = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_string) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("string"),
                        );
                    }
                    m_string = _serde::__private::Some(
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
        let m_int = match m_int {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("int"),
                );
            }
        };
        let m_real = match m_real {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("real"),
                );
            }
        };
        let m_string = match m_string {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("string"),
                );
            }
        };
        _serde::__private::Ok(hkbTestStateChooser {
            __ptr,
            parent,
            m_int,
            m_real,
            m_string,
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
        let parent = __hkbStateChooserVisitor::visit_as_parent(&mut __map)?;
        let mut m_int: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_real: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_string: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_int => {
                        if _serde::__private::Option::is_some(&m_int) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("int"),
                            );
                        }
                        m_int = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_real => {
                        if _serde::__private::Option::is_some(&m_real) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("real"),
                            );
                        }
                        m_real = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_string => {
                        if _serde::__private::Option::is_some(&m_string) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("string"),
                            );
                        }
                        m_string = _serde::__private::Some(
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
        let m_int = match m_int {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("int"),
                );
            }
        };
        let m_real = match m_real {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("real"),
                );
            }
        };
        let m_string = match m_string {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("string"),
                );
            }
        };
        _serde::__private::Ok(hkbTestStateChooser {
            __ptr,
            parent,
            m_int,
            m_real,
            m_string,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbTestStateChooser<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["int", "real", "string"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbTestStateChooser",
                FIELDS,
                __hkbTestStateChooserVisitor {
                    marker: _serde::__private::PhantomData::<hkbTestStateChooser>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
