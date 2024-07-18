use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCompressedSampledHeightFieldShape`
/// - version: `0`
/// - signature: `0x97b6e143`
/// - size: `128`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCompressedSampledHeightFieldShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpSampledHeightFieldShape,
    /// # C++ Info
    /// - name: `storage`(ctype: `hkArray<hkUint16>`)
    /// - offset: ` 96`(x86)/`112`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_storage: Vec<u16>,
    /// # C++ Info
    /// - name: `triangleFlip`(ctype: `hkBool`)
    /// - offset: `108`(x86)/`128`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_triangleFlip: bool,
    /// # C++ Info
    /// - name: `offset`(ctype: `hkReal`)
    /// - offset: `112`(x86)/`132`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_offset: f32,
    /// # C++ Info
    /// - name: `scale`(ctype: `hkReal`)
    /// - offset: `116`(x86)/`136`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_scale: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCompressedSampledHeightFieldShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCompressedSampledHeightFieldShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x97b6e143)
        }
    }
    impl _serde::Serialize for hkpCompressedSampledHeightFieldShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x97b6e143)));
            let mut serializer = __serializer
                .serialize_struct("hkpCompressedSampledHeightFieldShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("xRes", &self.parent.m_xRes)?;
            serializer.serialize_field("zRes", &self.parent.m_zRes)?;
            serializer.serialize_field("heightCenter", &self.parent.m_heightCenter)?;
            serializer
                .serialize_field(
                    "useProjectionBasedHeight",
                    &self.parent.m_useProjectionBasedHeight,
                )?;
            serializer
                .serialize_field("heightfieldType", &self.parent.m_heightfieldType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field("intToFloatScale", &self.parent.m_intToFloatScale)?;
            serializer
                .serialize_field("floatToIntScale", &self.parent.m_floatToIntScale)?;
            serializer
                .serialize_field(
                    "floatToIntOffsetFloorCorrected",
                    &self.parent.m_floatToIntOffsetFloorCorrected,
                )?;
            serializer.serialize_field("extents", &self.parent.m_extents)?;
            serializer.serialize_array_meta_field("storage", &self.m_storage)?;
            serializer.serialize_field("triangleFlip", &self.m_triangleFlip)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.serialize_field("scale", &self.m_scale)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("storage", &self.m_storage)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_storage,
    m_triangleFlip,
    m_offset,
    m_scale,
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
            "storage" => Ok(__Field::m_storage),
            "triangleFlip" => Ok(__Field::m_triangleFlip),
            "offset" => Ok(__Field::m_offset),
            "scale" => Ok(__Field::m_scale),
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
pub(super) struct __hkpCompressedSampledHeightFieldShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpCompressedSampledHeightFieldShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCompressedSampledHeightFieldShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCompressedSampledHeightFieldShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpCompressedSampledHeightFieldShape,
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
for __hkpCompressedSampledHeightFieldShapeVisitor<'de> {
    type Value = hkpCompressedSampledHeightFieldShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpCompressedSampledHeightFieldShape",
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
        let mut m_storage: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_triangleFlip: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_offset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_scale: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_storage) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("storage"),
                        );
                    }
                    m_storage = _serde::__private::Some(
                        match __A::next_value::<Vec<u16>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_triangleFlip) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "triangleFlip",
                            ),
                        );
                    }
                    m_triangleFlip = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_offset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_offset = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_scale) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("scale"),
                        );
                    }
                    m_scale = _serde::__private::Some(
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
        __A::pad(&mut __map, 8usize, 4usize)?;
        let m_storage = match m_storage {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("storage"),
                );
            }
        };
        let m_triangleFlip = match m_triangleFlip {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleFlip"),
                );
            }
        };
        let m_offset = match m_offset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offset"),
                );
            }
        };
        let m_scale = match m_scale {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("scale"),
                );
            }
        };
        _serde::__private::Ok(hkpCompressedSampledHeightFieldShape {
            __ptr,
            parent,
            m_storage,
            m_triangleFlip,
            m_offset,
            m_scale,
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
        let parent = __hkpSampledHeightFieldShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_storage: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
        let mut m_triangleFlip: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_offset: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_scale: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..4usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_storage => {
                        if _serde::__private::Option::is_some(&m_storage) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "storage",
                                ),
                            );
                        }
                        m_storage = _serde::__private::Some(
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
                    __Field::m_triangleFlip => {
                        if _serde::__private::Option::is_some(&m_triangleFlip) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "triangleFlip",
                                ),
                            );
                        }
                        m_triangleFlip = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_offset => {
                        if _serde::__private::Option::is_some(&m_offset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                            );
                        }
                        m_offset = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_scale => {
                        if _serde::__private::Option::is_some(&m_scale) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("scale"),
                            );
                        }
                        m_scale = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        let m_storage = match m_storage {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("storage"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_triangleFlip = match m_triangleFlip {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("triangleFlip"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_offset = match m_offset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("offset"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_scale = match m_scale {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("scale"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpCompressedSampledHeightFieldShape {
            __ptr,
            parent,
            m_storage,
            m_triangleFlip,
            m_offset,
            m_scale,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCompressedSampledHeightFieldShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["storage", "triangleFlip", "offset", "scale"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCompressedSampledHeightFieldShape",
                FIELDS,
                __hkpCompressedSampledHeightFieldShapeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpCompressedSampledHeightFieldShape,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
