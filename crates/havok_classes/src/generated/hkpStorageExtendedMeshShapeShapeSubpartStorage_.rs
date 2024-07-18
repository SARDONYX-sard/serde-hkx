use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpStorageExtendedMeshShapeShapeSubpartStorage`
/// - version: `2`
/// - signature: `0x3f7d804c`
/// - size: ` 44`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageExtendedMeshShapeShapeSubpartStorage {
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
    /// - name: `materialIndices`(ctype: `hkArray<hkUint8>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_materialIndices: Vec<u8>,
    /// # C++ Info
    /// - name: `materials`(ctype: `hkArray<struct hkpStorageExtendedMeshShapeMaterial>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_materials: Vec<hkpStorageExtendedMeshShapeMaterial>,
    /// # C++ Info
    /// - name: `materialIndices16`(ctype: `hkArray<hkUint16>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_materialIndices16: Vec<u16>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStorageExtendedMeshShapeShapeSubpartStorage {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageExtendedMeshShapeShapeSubpartStorage"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x3f7d804c)
        }
    }
    impl _serde::Serialize for hkpStorageExtendedMeshShapeShapeSubpartStorage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x3f7d804c)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpStorageExtendedMeshShapeShapeSubpartStorage",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_array_meta_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_meta_field(
                    "materialIndices16",
                    &self.m_materialIndices16,
                )?;
            serializer
                .serialize_array_field("materialIndices", &self.m_materialIndices)?;
            serializer.serialize_array_field("materials", &self.m_materials)?;
            serializer
                .serialize_array_field("materialIndices16", &self.m_materialIndices16)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_materialIndices,
    m_materials,
    m_materialIndices16,
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
            "materialIndices" => Ok(__Field::m_materialIndices),
            "materials" => Ok(__Field::m_materials),
            "materialIndices16" => Ok(__Field::m_materialIndices16),
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
pub(super) struct __hkpStorageExtendedMeshShapeShapeSubpartStorageVisitor<'de> {
    marker: core::marker::PhantomData<hkpStorageExtendedMeshShapeShapeSubpartStorage>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpStorageExtendedMeshShapeShapeSubpartStorageVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkpStorageExtendedMeshShapeShapeSubpartStorage,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpStorageExtendedMeshShapeShapeSubpartStorage,
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
for __hkpStorageExtendedMeshShapeShapeSubpartStorageVisitor<'de> {
    type Value = hkpStorageExtendedMeshShapeShapeSubpartStorage;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpStorageExtendedMeshShapeShapeSubpartStorage",
        )
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
        let mut m_materialIndices: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_materials: _serde::__private::Option<
            Vec<hkpStorageExtendedMeshShapeMaterial>,
        > = _serde::__private::None;
        let mut m_materialIndices16: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_materialIndices) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndices",
                            ),
                        );
                    }
                    m_materialIndices = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_materials) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materials",
                            ),
                        );
                    }
                    m_materials = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpStorageExtendedMeshShapeMaterial>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_materialIndices16) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "materialIndices16",
                            ),
                        );
                    }
                    m_materialIndices16 = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
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
        let m_materialIndices = match m_materialIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndices"),
                );
            }
        };
        let m_materials = match m_materials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materials"),
                );
            }
        };
        let m_materialIndices16 = match m_materialIndices16 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndices16"),
                );
            }
        };
        _serde::__private::Ok(hkpStorageExtendedMeshShapeShapeSubpartStorage {
            __ptr,
            parent,
            m_materialIndices,
            m_materials,
            m_materialIndices16,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_materialIndices: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_materials: _serde::__private::Option<
            Vec<hkpStorageExtendedMeshShapeMaterial>,
        > = _serde::__private::None;
        let mut m_materialIndices16: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        for _ in 0..3usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_materialIndices => {
                        if _serde::__private::Option::is_some(&m_materialIndices) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndices",
                                ),
                            );
                        }
                        m_materialIndices = _serde::__private::Some(
                            match __A::next_value::<Vec<u8>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_materials => {
                        if _serde::__private::Option::is_some(&m_materials) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materials",
                                ),
                            );
                        }
                        m_materials = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkpStorageExtendedMeshShapeMaterial>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_materialIndices16 => {
                        if _serde::__private::Option::is_some(&m_materialIndices16) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "materialIndices16",
                                ),
                            );
                        }
                        m_materialIndices16 = _serde::__private::Some(
                            match __A::next_value::<Vec<u16>>(&mut __map) {
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
        let m_materialIndices = match m_materialIndices {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndices"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_materials = match m_materials {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materials"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_materialIndices16 = match m_materialIndices16 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("materialIndices16"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpStorageExtendedMeshShapeShapeSubpartStorage {
            __ptr,
            parent,
            m_materialIndices,
            m_materials,
            m_materialIndices16,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkpStorageExtendedMeshShapeShapeSubpartStorage {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "materialIndices",
                "materials",
                "materialIndices16",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpStorageExtendedMeshShapeShapeSubpartStorage",
                FIELDS,
                __hkpStorageExtendedMeshShapeShapeSubpartStorageVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpStorageExtendedMeshShapeShapeSubpartStorage,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
