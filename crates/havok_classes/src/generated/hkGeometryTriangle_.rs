use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkGeometryTriangle`
/// - version: `0`
/// - signature: `0x9687513b`
/// - size: ` 16`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkGeometryTriangle {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `a`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_a: i32,
    /// # C++ Info
    /// - name: `b`(ctype: `hkInt32`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_b: i32,
    /// # C++ Info
    /// - name: `c`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_c: i32,
    /// # C++ Info
    /// - name: `material`(ctype: `hkInt32`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_material: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkGeometryTriangle {
        #[inline]
        fn name(&self) -> &'static str {
            "hkGeometryTriangle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9687513b)
        }
    }
    impl _serde::Serialize for hkGeometryTriangle {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9687513b)));
            let mut serializer = __serializer
                .serialize_struct("hkGeometryTriangle", class_meta)?;
            serializer.serialize_field("a", &self.m_a)?;
            serializer.serialize_field("b", &self.m_b)?;
            serializer.serialize_field("c", &self.m_c)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_a,
    m_b,
    m_c,
    m_material,
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
            "a" => Ok(__Field::m_a),
            "b" => Ok(__Field::m_b),
            "c" => Ok(__Field::m_c),
            "material" => Ok(__Field::m_material),
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
pub(super) struct __hkGeometryTriangleVisitor<'de> {
    marker: core::marker::PhantomData<hkGeometryTriangle>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkGeometryTriangleVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkGeometryTriangle, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkGeometryTriangle>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkGeometryTriangleVisitor<'de> {
    type Value = hkGeometryTriangle;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkGeometryTriangle")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_a: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_b: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_c: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_material: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_a) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("a"),
                        );
                    }
                    m_a = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_b) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("b"),
                        );
                    }
                    m_b = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_c) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("c"),
                        );
                    }
                    m_c = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_material) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "material",
                            ),
                        );
                    }
                    m_material = _serde::__private::Some(
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
        let m_a = match m_a {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("a"),
                );
            }
        };
        let m_b = match m_b {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("b"),
                );
            }
        };
        let m_c = match m_c {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("c"),
                );
            }
        };
        let m_material = match m_material {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("material"),
                );
            }
        };
        _serde::__private::Ok(hkGeometryTriangle {
            __ptr,
            m_a,
            m_b,
            m_c,
            m_material,
        })
    }
    #[allow(clippy::manual_unwrap_or_default)]
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_a: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_b: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_c: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_material: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..4usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_a => {
                        if _serde::__private::Option::is_some(&m_a) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("a"),
                            );
                        }
                        m_a = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_b => {
                        if _serde::__private::Option::is_some(&m_b) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("b"),
                            );
                        }
                        m_b = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_c => {
                        if _serde::__private::Option::is_some(&m_c) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("c"),
                            );
                        }
                        m_c = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_material => {
                        if _serde::__private::Option::is_some(&m_material) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "material",
                                ),
                            );
                        }
                        m_material = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_a = match m_a {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("a"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_b = match m_b {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("b"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_c = match m_c {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("c"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_material = match m_material {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("material"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkGeometryTriangle {
            __ptr,
            m_a,
            m_b,
            m_c,
            m_material,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkGeometryTriangle {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["a", "b", "c", "material"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkGeometryTriangle",
                FIELDS,
                __hkGeometryTriangleVisitor {
                    marker: _serde::__private::PhantomData::<hkGeometryTriangle>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
