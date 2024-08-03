use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaKeyFrameHierarchyUtilityControlData`
/// - version: `0`
/// - signature: `0xa3d0ac71`
/// - size: ` 48`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaKeyFrameHierarchyUtilityControlData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `hierarchyGain`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_hierarchyGain: f32,
    /// # C++ Info
    /// - name: `velocityDamping`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_velocityDamping: f32,
    /// # C++ Info
    /// - name: `accelerationGain`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_accelerationGain: f32,
    /// # C++ Info
    /// - name: `velocityGain`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_velocityGain: f32,
    /// # C++ Info
    /// - name: `positionGain`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_positionGain: f32,
    /// # C++ Info
    /// - name: `positionMaxLinearVelocity`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_positionMaxLinearVelocity: f32,
    /// # C++ Info
    /// - name: `positionMaxAngularVelocity`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_positionMaxAngularVelocity: f32,
    /// # C++ Info
    /// - name: `snapGain`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapGain: f32,
    /// # C++ Info
    /// - name: `snapMaxLinearVelocity`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapMaxLinearVelocity: f32,
    /// # C++ Info
    /// - name: `snapMaxAngularVelocity`(ctype: `hkReal`)
    /// - offset: ` 36`(x86)/` 36`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapMaxAngularVelocity: f32,
    /// # C++ Info
    /// - name: `snapMaxLinearDistance`(ctype: `hkReal`)
    /// - offset: ` 40`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapMaxLinearDistance: f32,
    /// # C++ Info
    /// - name: `snapMaxAngularDistance`(ctype: `hkReal`)
    /// - offset: ` 44`(x86)/` 44`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_snapMaxAngularDistance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaKeyFrameHierarchyUtilityControlData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaKeyFrameHierarchyUtilityControlData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa3d0ac71)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkaKeyFrameHierarchyUtilityControlData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa3d0ac71)));
            let mut serializer = __serializer
                .serialize_struct("hkaKeyFrameHierarchyUtilityControlData", class_meta)?;
            serializer.serialize_field("hierarchyGain", &self.m_hierarchyGain)?;
            serializer.serialize_field("velocityDamping", &self.m_velocityDamping)?;
            serializer.serialize_field("accelerationGain", &self.m_accelerationGain)?;
            serializer.serialize_field("velocityGain", &self.m_velocityGain)?;
            serializer.serialize_field("positionGain", &self.m_positionGain)?;
            serializer
                .serialize_field(
                    "positionMaxLinearVelocity",
                    &self.m_positionMaxLinearVelocity,
                )?;
            serializer
                .serialize_field(
                    "positionMaxAngularVelocity",
                    &self.m_positionMaxAngularVelocity,
                )?;
            serializer.serialize_field("snapGain", &self.m_snapGain)?;
            serializer
                .serialize_field(
                    "snapMaxLinearVelocity",
                    &self.m_snapMaxLinearVelocity,
                )?;
            serializer
                .serialize_field(
                    "snapMaxAngularVelocity",
                    &self.m_snapMaxAngularVelocity,
                )?;
            serializer
                .serialize_field(
                    "snapMaxLinearDistance",
                    &self.m_snapMaxLinearDistance,
                )?;
            serializer
                .serialize_field(
                    "snapMaxAngularDistance",
                    &self.m_snapMaxAngularDistance,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkaKeyFrameHierarchyUtilityControlData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_hierarchyGain,
                m_velocityDamping,
                m_accelerationGain,
                m_velocityGain,
                m_positionGain,
                m_positionMaxLinearVelocity,
                m_positionMaxAngularVelocity,
                m_snapGain,
                m_snapMaxLinearVelocity,
                m_snapMaxAngularVelocity,
                m_snapMaxLinearDistance,
                m_snapMaxAngularDistance,
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
                        "hierarchyGain" => Ok(__Field::m_hierarchyGain),
                        "velocityDamping" => Ok(__Field::m_velocityDamping),
                        "accelerationGain" => Ok(__Field::m_accelerationGain),
                        "velocityGain" => Ok(__Field::m_velocityGain),
                        "positionGain" => Ok(__Field::m_positionGain),
                        "positionMaxLinearVelocity" => {
                            Ok(__Field::m_positionMaxLinearVelocity)
                        }
                        "positionMaxAngularVelocity" => {
                            Ok(__Field::m_positionMaxAngularVelocity)
                        }
                        "snapGain" => Ok(__Field::m_snapGain),
                        "snapMaxLinearVelocity" => Ok(__Field::m_snapMaxLinearVelocity),
                        "snapMaxAngularVelocity" => Ok(__Field::m_snapMaxAngularVelocity),
                        "snapMaxLinearDistance" => Ok(__Field::m_snapMaxLinearDistance),
                        "snapMaxAngularDistance" => Ok(__Field::m_snapMaxAngularDistance),
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
            struct __hkaKeyFrameHierarchyUtilityControlDataVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkaKeyFrameHierarchyUtilityControlData,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkaKeyFrameHierarchyUtilityControlDataVisitor<'de> {
                type Value = hkaKeyFrameHierarchyUtilityControlData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaKeyFrameHierarchyUtilityControlData",
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
                    let mut m_hierarchyGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_velocityDamping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_accelerationGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_velocityGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_positionGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_positionMaxLinearVelocity: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_positionMaxAngularVelocity: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_snapGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxLinearVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxAngularVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxLinearDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxAngularDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_hierarchyGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hierarchyGain",
                                        ),
                                    );
                                }
                                m_hierarchyGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_velocityDamping) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocityDamping",
                                        ),
                                    );
                                }
                                m_velocityDamping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_accelerationGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "accelerationGain",
                                        ),
                                    );
                                }
                                m_accelerationGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_velocityGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocityGain",
                                        ),
                                    );
                                }
                                m_velocityGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_positionGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionGain",
                                        ),
                                    );
                                }
                                m_positionGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_positionMaxLinearVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionMaxLinearVelocity",
                                        ),
                                    );
                                }
                                m_positionMaxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_positionMaxAngularVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionMaxAngularVelocity",
                                        ),
                                    );
                                }
                                m_positionMaxAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_snapGain) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapGain",
                                        ),
                                    );
                                }
                                m_snapGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxLinearVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxLinearVelocity",
                                        ),
                                    );
                                }
                                m_snapMaxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxAngularVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxAngularVelocity",
                                        ),
                                    );
                                }
                                m_snapMaxAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxLinearDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxLinearDistance",
                                        ),
                                    );
                                }
                                m_snapMaxLinearDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxAngularDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxAngularDistance",
                                        ),
                                    );
                                }
                                m_snapMaxAngularDistance = _serde::__private::Some(
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
                    let m_hierarchyGain = match m_hierarchyGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hierarchyGain",
                                ),
                            );
                        }
                    };
                    let m_velocityDamping = match m_velocityDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "velocityDamping",
                                ),
                            );
                        }
                    };
                    let m_accelerationGain = match m_accelerationGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "accelerationGain",
                                ),
                            );
                        }
                    };
                    let m_velocityGain = match m_velocityGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "velocityGain",
                                ),
                            );
                        }
                    };
                    let m_positionGain = match m_positionGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionGain",
                                ),
                            );
                        }
                    };
                    let m_positionMaxLinearVelocity = match m_positionMaxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionMaxLinearVelocity",
                                ),
                            );
                        }
                    };
                    let m_positionMaxAngularVelocity = match m_positionMaxAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionMaxAngularVelocity",
                                ),
                            );
                        }
                    };
                    let m_snapGain = match m_snapGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("snapGain"),
                            );
                        }
                    };
                    let m_snapMaxLinearVelocity = match m_snapMaxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxLinearVelocity",
                                ),
                            );
                        }
                    };
                    let m_snapMaxAngularVelocity = match m_snapMaxAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxAngularVelocity",
                                ),
                            );
                        }
                    };
                    let m_snapMaxLinearDistance = match m_snapMaxLinearDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxLinearDistance",
                                ),
                            );
                        }
                    };
                    let m_snapMaxAngularDistance = match m_snapMaxAngularDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxAngularDistance",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaKeyFrameHierarchyUtilityControlData {
                        __ptr,
                        m_hierarchyGain,
                        m_velocityDamping,
                        m_accelerationGain,
                        m_velocityGain,
                        m_positionGain,
                        m_positionMaxLinearVelocity,
                        m_positionMaxAngularVelocity,
                        m_snapGain,
                        m_snapMaxLinearVelocity,
                        m_snapMaxAngularVelocity,
                        m_snapMaxLinearDistance,
                        m_snapMaxAngularDistance,
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
                    let mut m_hierarchyGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_velocityDamping: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_accelerationGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_velocityGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_positionGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_positionMaxLinearVelocity: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_positionMaxAngularVelocity: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_snapGain: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxLinearVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxAngularVelocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxLinearDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_snapMaxAngularDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_hierarchyGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_hierarchyGain) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "hierarchyGain",
                                        ),
                                    );
                                }
                                m_hierarchyGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_velocityDamping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_velocityDamping) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocityDamping",
                                        ),
                                    );
                                }
                                m_velocityDamping = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_accelerationGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_accelerationGain) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "accelerationGain",
                                        ),
                                    );
                                }
                                m_accelerationGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_velocityGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_velocityGain) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocityGain",
                                        ),
                                    );
                                }
                                m_velocityGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_positionGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_positionGain) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionGain",
                                        ),
                                    );
                                }
                                m_positionGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_positionMaxLinearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_positionMaxLinearVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionMaxLinearVelocity",
                                        ),
                                    );
                                }
                                m_positionMaxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_positionMaxAngularVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_positionMaxAngularVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionMaxAngularVelocity",
                                        ),
                                    );
                                }
                                m_positionMaxAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_snapGain => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_snapGain) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapGain",
                                        ),
                                    );
                                }
                                m_snapGain = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_snapMaxLinearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxLinearVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxLinearVelocity",
                                        ),
                                    );
                                }
                                m_snapMaxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_snapMaxAngularVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxAngularVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxAngularVelocity",
                                        ),
                                    );
                                }
                                m_snapMaxAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_snapMaxLinearDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxLinearDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxLinearDistance",
                                        ),
                                    );
                                }
                                m_snapMaxLinearDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_snapMaxAngularDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_snapMaxAngularDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "snapMaxAngularDistance",
                                        ),
                                    );
                                }
                                m_snapMaxAngularDistance = _serde::__private::Some(
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
                    let m_hierarchyGain = match m_hierarchyGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "hierarchyGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_velocityDamping = match m_velocityDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "velocityDamping",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_accelerationGain = match m_accelerationGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "accelerationGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_velocityGain = match m_velocityGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "velocityGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_positionGain = match m_positionGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionGain",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_positionMaxLinearVelocity = match m_positionMaxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionMaxLinearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_positionMaxAngularVelocity = match m_positionMaxAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionMaxAngularVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapGain = match m_snapGain {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("snapGain"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapMaxLinearVelocity = match m_snapMaxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxLinearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapMaxAngularVelocity = match m_snapMaxAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxAngularVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapMaxLinearDistance = match m_snapMaxLinearDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxLinearDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_snapMaxAngularDistance = match m_snapMaxAngularDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "snapMaxAngularDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkaKeyFrameHierarchyUtilityControlData {
                        __ptr,
                        m_hierarchyGain,
                        m_velocityDamping,
                        m_accelerationGain,
                        m_velocityGain,
                        m_positionGain,
                        m_positionMaxLinearVelocity,
                        m_positionMaxAngularVelocity,
                        m_snapGain,
                        m_snapMaxLinearVelocity,
                        m_snapMaxAngularVelocity,
                        m_snapMaxLinearDistance,
                        m_snapMaxAngularDistance,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "hierarchyGain",
                "velocityDamping",
                "accelerationGain",
                "velocityGain",
                "positionGain",
                "positionMaxLinearVelocity",
                "positionMaxAngularVelocity",
                "snapGain",
                "snapMaxLinearVelocity",
                "snapMaxAngularVelocity",
                "snapMaxLinearDistance",
                "snapMaxAngularDistance",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaKeyFrameHierarchyUtilityControlData",
                FIELDS,
                __hkaKeyFrameHierarchyUtilityControlDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaKeyFrameHierarchyUtilityControlData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
