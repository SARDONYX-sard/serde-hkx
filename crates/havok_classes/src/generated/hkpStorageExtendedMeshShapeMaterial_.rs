use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageExtendedMeshShapeMaterial`
/// -         version: `1`
/// -       signature: `0x2ca3e906`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageExtendedMeshShapeMaterial {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpMeshMaterial,
    /// # C++ Info
    /// -          name: `restitution`(ctype: `hkHalf`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_restitution: f16,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkHalf`)
    /// -        offset:   6(x86)/  6(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_friction: f16,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStorageExtendedMeshShapeMaterial {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageExtendedMeshShapeMaterial"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2ca3e906)
        }
    }
    impl _serde::Serialize for hkpStorageExtendedMeshShapeMaterial {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2ca3e906)));
            let mut serializer = __serializer
                .serialize_struct("hkpStorageExtendedMeshShapeMaterial", class_meta)?;
            serializer.serialize_field("filterInfo", &self.parent.m_filterInfo)?;
            serializer.serialize_field("restitution", &self.m_restitution)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_restitution,
    m_friction,
    m_userData,
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
            "restitution" => Ok(__Field::m_restitution),
            "friction" => Ok(__Field::m_friction),
            "userData" => Ok(__Field::m_userData),
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
pub(super) struct __hkpStorageExtendedMeshShapeMaterialVisitor<'de> {
    marker: core::marker::PhantomData<hkpStorageExtendedMeshShapeMaterial>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpStorageExtendedMeshShapeMaterialVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpStorageExtendedMeshShapeMaterial, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpStorageExtendedMeshShapeMaterial,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de>
for __hkpStorageExtendedMeshShapeMaterialVisitor<'de> {
    type Value = hkpStorageExtendedMeshShapeMaterial;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpStorageExtendedMeshShapeMaterial",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_restitution: _serde::__private::Option<f16> = _serde::__private::None;
        let mut m_friction: _serde::__private::Option<f16> = _serde::__private::None;
        let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_restitution) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "restitution",
                            ),
                        );
                    }
                    m_restitution = _serde::__private::Some(
                        match __A::next_value::<f16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_friction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "friction",
                            ),
                        );
                    }
                    m_friction = _serde::__private::Some(
                        match __A::next_value::<f16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_userData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userData",
                            ),
                        );
                    }
                    m_userData = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
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
        let m_restitution = match m_restitution {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("restitution"),
                );
            }
        };
        let m_friction = match m_friction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("friction"),
                );
            }
        };
        let m_userData = match m_userData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData"),
                );
            }
        };
        _serde::__private::Ok(hkpStorageExtendedMeshShapeMaterial {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_restitution,
            m_friction,
            m_userData,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpMeshMaterialVisitor::visit_as_parent(&mut __map)?;
        let mut m_restitution: _serde::__private::Option<f16> = _serde::__private::None;
        let mut m_friction: _serde::__private::Option<f16> = _serde::__private::None;
        let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_restitution => {
                    if _serde::__private::Option::is_some(&m_restitution) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "restitution",
                            ),
                        );
                    }
                    m_restitution = _serde::__private::Some(
                        match __A::next_value::<f16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_friction => {
                    if _serde::__private::Option::is_some(&m_friction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "friction",
                            ),
                        );
                    }
                    m_friction = _serde::__private::Some(
                        match __A::next_value::<f16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_userData => {
                    if _serde::__private::Option::is_some(&m_userData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userData",
                            ),
                        );
                    }
                    m_userData = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
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
        let m_restitution = match m_restitution {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("restitution"),
                );
            }
        };
        let m_friction = match m_friction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("friction"),
                );
            }
        };
        let m_userData = match m_userData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData"),
                );
            }
        };
        _serde::__private::Ok(hkpStorageExtendedMeshShapeMaterial {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_restitution,
            m_friction,
            m_userData,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpStorageExtendedMeshShapeMaterial {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["restitution", "friction", "userData"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpStorageExtendedMeshShapeMaterial",
                FIELDS,
                __hkpStorageExtendedMeshShapeMaterialVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpStorageExtendedMeshShapeMaterial,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
