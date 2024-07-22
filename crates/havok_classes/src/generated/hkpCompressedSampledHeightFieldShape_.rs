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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCompressedSampledHeightFieldShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_extents,
                m_floatToIntOffsetFloorCorrected,
                m_floatToIntScale,
                m_intToFloatScale,
                m_heightfieldType,
                m_useProjectionBasedHeight,
                m_heightCenter,
                m_zRes,
                m_xRes,
                m_scale,
                m_offset,
                m_triangleFlip,
                m_storage,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "userData" => Ok(__Field::m_userData),
                        "extents" => Ok(__Field::m_extents),
                        "floatToIntOffsetFloorCorrected" => {
                            Ok(__Field::m_floatToIntOffsetFloorCorrected)
                        }
                        "floatToIntScale" => Ok(__Field::m_floatToIntScale),
                        "intToFloatScale" => Ok(__Field::m_intToFloatScale),
                        "heightfieldType" => Ok(__Field::m_heightfieldType),
                        "useProjectionBasedHeight" => {
                            Ok(__Field::m_useProjectionBasedHeight)
                        }
                        "heightCenter" => Ok(__Field::m_heightCenter),
                        "zRes" => Ok(__Field::m_zRes),
                        "xRes" => Ok(__Field::m_xRes),
                        "scale" => Ok(__Field::m_scale),
                        "offset" => Ok(__Field::m_offset),
                        "triangleFlip" => Ok(__Field::m_triangleFlip),
                        "storage" => Ok(__Field::m_storage),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpCompressedSampledHeightFieldShapeVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpCompressedSampledHeightFieldShape,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpCompressedSampledHeightFieldShapeVisitor<'de> {
                type Value = hkpCompressedSampledHeightFieldShape;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "storage",
                                        ),
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
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleFlip",
                                ),
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
                    let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_extents: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_floatToIntOffsetFloorCorrected: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_floatToIntScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_intToFloatScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_heightfieldType: _serde::__private::Option<
                        HeightFieldType,
                    > = _serde::__private::None;
                    let mut m_useProjectionBasedHeight: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_heightCenter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_zRes: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_xRes: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_scale: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_offset: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_triangleFlip: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_storage: _serde::__private::Option<Vec<u16>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_extents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_extents) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extents",
                                        ),
                                    );
                                }
                                m_extents = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_floatToIntOffsetFloorCorrected => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_floatToIntOffsetFloorCorrected,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatToIntOffsetFloorCorrected",
                                        ),
                                    );
                                }
                                m_floatToIntOffsetFloorCorrected = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_floatToIntScale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatToIntScale) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatToIntScale",
                                        ),
                                    );
                                }
                                m_floatToIntScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_intToFloatScale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intToFloatScale) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intToFloatScale",
                                        ),
                                    );
                                }
                                m_intToFloatScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_heightfieldType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_heightfieldType) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "heightfieldType",
                                        ),
                                    );
                                }
                                m_heightfieldType = _serde::__private::Some(
                                    match __A::next_value::<HeightFieldType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_useProjectionBasedHeight => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_useProjectionBasedHeight,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "useProjectionBasedHeight",
                                        ),
                                    );
                                }
                                m_useProjectionBasedHeight = _serde::__private::Some(
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
                            __Field::m_heightCenter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_heightCenter) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "heightCenter",
                                        ),
                                    );
                                }
                                m_heightCenter = _serde::__private::Some(
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
                            __Field::m_zRes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_zRes) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("zRes"),
                                    );
                                }
                                m_zRes = _serde::__private::Some(
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
                            __Field::m_xRes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_xRes) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("xRes"),
                                    );
                                }
                                m_xRes = _serde::__private::Some(
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
                            __Field::m_scale => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_scale) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_offset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offset) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_triangleFlip => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_triangleFlip) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            __Field::m_storage => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_storage) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
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
                            _ => {}
                        }
                    }
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extents = match m_extents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("extents"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatToIntOffsetFloorCorrected = match m_floatToIntOffsetFloorCorrected {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatToIntOffsetFloorCorrected",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatToIntScale = match m_floatToIntScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatToIntScale",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intToFloatScale = match m_intToFloatScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intToFloatScale",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_heightfieldType = match m_heightfieldType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heightfieldType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_useProjectionBasedHeight = match m_useProjectionBasedHeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useProjectionBasedHeight",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_heightCenter = match m_heightCenter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heightCenter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_zRes = match m_zRes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("zRes"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_xRes = match m_xRes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("xRes"),
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
                    let m_triangleFlip = match m_triangleFlip {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triangleFlip",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let parent = hkpShape {
                        __ptr,
                        parent,
                        m_userData,
                        ..Default::default()
                    };
                    let parent = hkpHeightFieldShape {
                        __ptr,
                        parent,
                    };
                    let parent = hkpSampledHeightFieldShape {
                        __ptr,
                        parent,
                        m_xRes,
                        m_zRes,
                        m_heightCenter,
                        m_useProjectionBasedHeight,
                        m_heightfieldType,
                        m_intToFloatScale,
                        m_floatToIntScale,
                        m_floatToIntOffsetFloorCorrected,
                        m_extents,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
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
