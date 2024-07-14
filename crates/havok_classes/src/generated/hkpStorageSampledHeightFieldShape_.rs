use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStorageSampledHeightFieldShape`
/// -         version: `0`
/// -       signature: `0x15ff414b`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStorageSampledHeightFieldShape {
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
    /// -          name: `storage`(ctype: `hkArray<hkReal>`)
    /// -        offset:  96(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_storage: Vec<f32>,
    /// # C++ Info
    /// -          name: `triangleFlip`(ctype: `hkBool`)
    /// -        offset: 108(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_triangleFlip: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStorageSampledHeightFieldShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStorageSampledHeightFieldShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x15ff414b)
        }
    }
    impl _serde::Serialize for hkpStorageSampledHeightFieldShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x15ff414b)));
            let mut serializer = __serializer
                .serialize_struct("hkpStorageSampledHeightFieldShape", class_meta)?;
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
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
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
pub(super) struct __hkpStorageSampledHeightFieldShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpStorageSampledHeightFieldShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpStorageSampledHeightFieldShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpStorageSampledHeightFieldShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpStorageSampledHeightFieldShape,
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
impl<'de> _serde::de::Visitor<'de> for __hkpStorageSampledHeightFieldShapeVisitor<'de> {
    type Value = hkpStorageSampledHeightFieldShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpStorageSampledHeightFieldShape",
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
        let parent = __A::next_value(&mut __map)?;
        let mut m_storage: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_triangleFlip: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_storage) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("storage"),
                        );
                    }
                    m_storage = _serde::__private::Some(
                        match __A::next_value::<Vec<f32>>(&mut __map) {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 3usize, 15usize)?;
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
        _serde::__private::Ok(hkpStorageSampledHeightFieldShape {
            __ptr,
            parent,
            m_storage,
            m_triangleFlip,
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
        let parent = __hkpSampledHeightFieldShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_storage: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
        let mut m_triangleFlip: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
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
                            match __A::next_value::<Vec<f32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
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
                                    return _serde::__private::Err(__err);
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
        _serde::__private::Ok(hkpStorageSampledHeightFieldShape {
            __ptr,
            parent,
            m_storage,
            m_triangleFlip,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpStorageSampledHeightFieldShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["storage", "triangleFlip"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpStorageSampledHeightFieldShape",
                FIELDS,
                __hkpStorageSampledHeightFieldShapeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpStorageSampledHeightFieldShape,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
