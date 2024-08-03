use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpSampledHeightFieldShape`
/// - version: `0`
/// - signature: `0x11213421`
/// - size: ` 96`(x86)/`112`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSampledHeightFieldShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpHeightFieldShape,
    /// # C++ Info
    /// - name: `xRes`(ctype: `hkInt32`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_xRes: i32,
    /// # C++ Info
    /// - name: `zRes`(ctype: `hkInt32`)
    /// - offset: ` 20`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_zRes: i32,
    /// # C++ Info
    /// - name: `heightCenter`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_heightCenter: f32,
    /// # C++ Info
    /// - name: `useProjectionBasedHeight`(ctype: `hkBool`)
    /// - offset: ` 28`(x86)/` 44`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useProjectionBasedHeight: bool,
    /// # C++ Info
    /// - name: `heightfieldType`(ctype: `enum HeightFieldType`)
    /// - offset: ` 29`(x86)/` 45`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_heightfieldType: HeightFieldType,
    /// # C++ Info
    /// - name: `intToFloatScale`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_intToFloatScale: Vector4,
    /// # C++ Info
    /// - name: `floatToIntScale`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_floatToIntScale: Vector4,
    /// # C++ Info
    /// - name: `floatToIntOffsetFloorCorrected`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_floatToIntOffsetFloorCorrected: Vector4,
    /// # C++ Info
    /// - name: `extents`(ctype: `hkVector4`)
    /// - offset: ` 80`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_extents: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSampledHeightFieldShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSampledHeightFieldShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x11213421)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpSampledHeightFieldShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x11213421)));
            let mut serializer = __serializer
                .serialize_struct("hkpSampledHeightFieldShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("xRes", &self.m_xRes)?;
            serializer.serialize_field("zRes", &self.m_zRes)?;
            serializer.serialize_field("heightCenter", &self.m_heightCenter)?;
            serializer
                .serialize_field(
                    "useProjectionBasedHeight",
                    &self.m_useProjectionBasedHeight,
                )?;
            serializer.serialize_field("heightfieldType", &self.m_heightfieldType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("intToFloatScale", &self.m_intToFloatScale)?;
            serializer.serialize_field("floatToIntScale", &self.m_floatToIntScale)?;
            serializer
                .serialize_field(
                    "floatToIntOffsetFloorCorrected",
                    &self.m_floatToIntOffsetFloorCorrected,
                )?;
            serializer.serialize_field("extents", &self.m_extents)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSampledHeightFieldShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_xRes,
                m_zRes,
                m_heightCenter,
                m_useProjectionBasedHeight,
                m_heightfieldType,
                m_intToFloatScale,
                m_floatToIntScale,
                m_floatToIntOffsetFloorCorrected,
                m_extents,
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
                        "xRes" => Ok(__Field::m_xRes),
                        "zRes" => Ok(__Field::m_zRes),
                        "heightCenter" => Ok(__Field::m_heightCenter),
                        "useProjectionBasedHeight" => {
                            Ok(__Field::m_useProjectionBasedHeight)
                        }
                        "heightfieldType" => Ok(__Field::m_heightfieldType),
                        "intToFloatScale" => Ok(__Field::m_intToFloatScale),
                        "floatToIntScale" => Ok(__Field::m_floatToIntScale),
                        "floatToIntOffsetFloorCorrected" => {
                            Ok(__Field::m_floatToIntOffsetFloorCorrected)
                        }
                        "extents" => Ok(__Field::m_extents),
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
            struct __hkpSampledHeightFieldShapeVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpSampledHeightFieldShape>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpSampledHeightFieldShapeVisitor<'de> {
                type Value = hkpSampledHeightFieldShape;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpSampledHeightFieldShape",
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
                    let mut m_xRes: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_zRes: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_heightCenter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useProjectionBasedHeight: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_heightfieldType: _serde::__private::Option<
                        HeightFieldType,
                    > = _serde::__private::None;
                    let mut m_intToFloatScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_floatToIntScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_floatToIntOffsetFloorCorrected: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_extents: _serde::__private::Option<Vector4> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_xRes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("xRes"),
                                    );
                                }
                                m_xRes = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_zRes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("zRes"),
                                    );
                                }
                                m_zRes = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_heightCenter) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_useProjectionBasedHeight,
                                ) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_heightfieldType) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_intToFloatScale) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intToFloatScale",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 2usize)?;
                                m_intToFloatScale = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_floatToIntScale) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_floatToIntOffsetFloorCorrected,
                                ) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_extents) {
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
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_xRes = match m_xRes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("xRes"),
                            );
                        }
                    };
                    let m_zRes = match m_zRes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("zRes"),
                            );
                        }
                    };
                    let m_heightCenter = match m_heightCenter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heightCenter",
                                ),
                            );
                        }
                    };
                    let m_useProjectionBasedHeight = match m_useProjectionBasedHeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "useProjectionBasedHeight",
                                ),
                            );
                        }
                    };
                    let m_heightfieldType = match m_heightfieldType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heightfieldType",
                                ),
                            );
                        }
                    };
                    let m_intToFloatScale = match m_intToFloatScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intToFloatScale",
                                ),
                            );
                        }
                    };
                    let m_floatToIntScale = match m_floatToIntScale {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatToIntScale",
                                ),
                            );
                        }
                    };
                    let m_floatToIntOffsetFloorCorrected = match m_floatToIntOffsetFloorCorrected {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatToIntOffsetFloorCorrected",
                                ),
                            );
                        }
                    };
                    let m_extents = match m_extents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("extents"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpSampledHeightFieldShape {
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
                    let mut m_xRes: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_zRes: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_heightCenter: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_useProjectionBasedHeight: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_heightfieldType: _serde::__private::Option<
                        HeightFieldType,
                    > = _serde::__private::None;
                    let mut m_intToFloatScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_floatToIntScale: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_floatToIntOffsetFloorCorrected: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_extents: _serde::__private::Option<Vector4> = _serde::__private::None;
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                                            return _serde::__private::Err(__err);
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpSampledHeightFieldShape {
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
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "xRes",
                "zRes",
                "heightCenter",
                "useProjectionBasedHeight",
                "heightfieldType",
                "intToFloatScale",
                "floatToIntScale",
                "floatToIntOffsetFloorCorrected",
                "extents",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSampledHeightFieldShape",
                FIELDS,
                __hkpSampledHeightFieldShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpSampledHeightFieldShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum HeightFieldType {
    #[default]
    HEIGHTFIELD_STORAGE = 0isize,
    HEIGHTFIELD_COMPRESSED = 1isize,
    HEIGHTFIELD_USER = 2isize,
    HEIGHTFIELD_MAX_ID = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HeightFieldType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::HEIGHTFIELD_STORAGE => {
                    __serializer.serialize_field("HEIGHTFIELD_STORAGE", &0u64)
                }
                Self::HEIGHTFIELD_COMPRESSED => {
                    __serializer.serialize_field("HEIGHTFIELD_COMPRESSED", &1u64)
                }
                Self::HEIGHTFIELD_USER => {
                    __serializer.serialize_field("HEIGHTFIELD_USER", &2u64)
                }
                Self::HEIGHTFIELD_MAX_ID => {
                    __serializer.serialize_field("HEIGHTFIELD_MAX_ID", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum HeightFieldType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for HeightFieldType {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        2u8 => _serde::__private::Ok(__Field::__field2),
                        3u8 => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v.eq_ignore_ascii_case("HEIGHTFIELD_STORAGE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("HEIGHTFIELD_COMPRESSED") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("HEIGHTFIELD_USER") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("HEIGHTFIELD_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<HeightFieldType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HeightFieldType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum HeightFieldType",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(HeightFieldType::HEIGHTFIELD_STORAGE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                HeightFieldType::HEIGHTFIELD_COMPRESSED,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(HeightFieldType::HEIGHTFIELD_USER)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(HeightFieldType::HEIGHTFIELD_MAX_ID)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "HEIGHTFIELD_STORAGE",
                "HEIGHTFIELD_COMPRESSED",
                "HEIGHTFIELD_USER",
                "HEIGHTFIELD_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "HeightFieldType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HeightFieldType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
