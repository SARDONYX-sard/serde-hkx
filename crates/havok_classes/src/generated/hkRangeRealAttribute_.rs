use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkRangeRealAttribute`
/// - version: `0`
/// - signature: `0x949db24f`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkRangeRealAttribute {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `absmin`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_absmin: f32,
    /// # C++ Info
    /// - name: `absmax`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_absmax: f32,
    /// # C++ Info
    /// - name: `softmin`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_softmin: f32,
    /// # C++ Info
    /// - name: `softmax`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_softmax: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkRangeRealAttribute {
        #[inline]
        fn name(&self) -> &'static str {
            "hkRangeRealAttribute"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x949db24f)
        }
    }
    impl _serde::Serialize for hkRangeRealAttribute {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x949db24f)));
            let mut serializer = __serializer
                .serialize_struct("hkRangeRealAttribute", class_meta)?;
            serializer.serialize_field("absmin", &self.m_absmin)?;
            serializer.serialize_field("absmax", &self.m_absmax)?;
            serializer.serialize_field("softmin", &self.m_softmin)?;
            serializer.serialize_field("softmax", &self.m_softmax)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_absmin,
    m_absmax,
    m_softmin,
    m_softmax,
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
            "absmin" => Ok(__Field::m_absmin),
            "absmax" => Ok(__Field::m_absmax),
            "softmin" => Ok(__Field::m_softmin),
            "softmax" => Ok(__Field::m_softmax),
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
pub(super) struct __hkRangeRealAttributeVisitor<'de> {
    marker: core::marker::PhantomData<hkRangeRealAttribute>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkRangeRealAttributeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkRangeRealAttribute, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkRangeRealAttribute>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkRangeRealAttributeVisitor<'de> {
    type Value = hkRangeRealAttribute;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkRangeRealAttribute")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_absmin: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_absmax: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_softmin: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_softmax: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_absmin) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("absmin"),
                        );
                    }
                    m_absmin = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_absmax) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("absmax"),
                        );
                    }
                    m_absmax = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_softmin) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("softmin"),
                        );
                    }
                    m_softmin = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_softmax) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("softmax"),
                        );
                    }
                    m_softmax = _serde::__private::Some(
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
        let m_absmin = match m_absmin {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("absmin"),
                );
            }
        };
        let m_absmax = match m_absmax {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("absmax"),
                );
            }
        };
        let m_softmin = match m_softmin {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("softmin"),
                );
            }
        };
        let m_softmax = match m_softmax {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("softmax"),
                );
            }
        };
        _serde::__private::Ok(hkRangeRealAttribute {
            __ptr,
            m_absmin,
            m_absmax,
            m_softmin,
            m_softmax,
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
        let mut m_absmin: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_absmax: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_softmin: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_softmax: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_absmin => {
                        if _serde::__private::Option::is_some(&m_absmin) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("absmin"),
                            );
                        }
                        m_absmin = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_absmax => {
                        if _serde::__private::Option::is_some(&m_absmax) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("absmax"),
                            );
                        }
                        m_absmax = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_softmin => {
                        if _serde::__private::Option::is_some(&m_softmin) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "softmin",
                                ),
                            );
                        }
                        m_softmin = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_softmax => {
                        if _serde::__private::Option::is_some(&m_softmax) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "softmax",
                                ),
                            );
                        }
                        m_softmax = _serde::__private::Some(
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
        let m_absmin = match m_absmin {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("absmin"),
                );
            }
        };
        let m_absmax = match m_absmax {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("absmax"),
                );
            }
        };
        let m_softmin = match m_softmin {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("softmin"),
                );
            }
        };
        let m_softmax = match m_softmax {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("softmax"),
                );
            }
        };
        _serde::__private::Ok(hkRangeRealAttribute {
            __ptr,
            m_absmin,
            m_absmax,
            m_softmin,
            m_softmax,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkRangeRealAttribute {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["absmin", "absmax", "softmin", "softmax"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkRangeRealAttribute",
                FIELDS,
                __hkRangeRealAttributeVisitor {
                    marker: _serde::__private::PhantomData::<hkRangeRealAttribute>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
