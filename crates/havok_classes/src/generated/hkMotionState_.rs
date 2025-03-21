use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkMotionState`
/// - version: `1`
/// - signature: `0x5797386e`
/// - size: `176`(x86)/`176`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMotionState<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// # C++ Info
    /// - name: `transform`(ctype: `hkTransform`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 64`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "transform"))]
    #[cfg_attr(feature = "serde", serde(rename = "transform"))]
    pub m_transform: Transform,
    /// # C++ Info
    /// - name: `sweptTransform`(ctype: `struct hkSweptTransform`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: ` 80`(x86)/` 80`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "sweptTransform"))]
    #[cfg_attr(feature = "serde", serde(rename = "sweptTransform"))]
    pub m_sweptTransform: hkSweptTransform<'a>,
    /// # C++ Info
    /// - name: `deltaAngle`(ctype: `hkVector4`)
    /// - offset: `144`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "deltaAngle"))]
    #[cfg_attr(feature = "serde", serde(rename = "deltaAngle"))]
    pub m_deltaAngle: Vector4,
    /// # C++ Info
    /// - name: `objectRadius`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`160`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "objectRadius"))]
    #[cfg_attr(feature = "serde", serde(rename = "objectRadius"))]
    pub m_objectRadius: f32,
    /// # C++ Info
    /// - name: `linearDamping`(ctype: `hkHalf`)
    /// - offset: `164`(x86)/`164`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "linearDamping"))]
    #[cfg_attr(feature = "serde", serde(rename = "linearDamping"))]
    pub m_linearDamping: f16,
    /// # C++ Info
    /// - name: `angularDamping`(ctype: `hkHalf`)
    /// - offset: `166`(x86)/`166`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "angularDamping"))]
    #[cfg_attr(feature = "serde", serde(rename = "angularDamping"))]
    pub m_angularDamping: f16,
    /// # C++ Info
    /// - name: `timeFactor`(ctype: `hkHalf`)
    /// - offset: `168`(x86)/`168`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeFactor"))]
    pub m_timeFactor: f16,
    /// # C++ Info
    /// - name: `maxLinearVelocity`(ctype: `hkUint8`)
    /// - offset: `170`(x86)/`170`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxLinearVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxLinearVelocity"))]
    pub m_maxLinearVelocity: U8<'a>,
    /// # C++ Info
    /// - name: `maxAngularVelocity`(ctype: `hkUint8`)
    /// - offset: `171`(x86)/`171`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxAngularVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxAngularVelocity"))]
    pub m_maxAngularVelocity: U8<'a>,
    /// # C++ Info
    /// - name: `deactivationClass`(ctype: `hkUint8`)
    /// - offset: `172`(x86)/`172`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "deactivationClass"))]
    #[cfg_attr(feature = "serde", serde(rename = "deactivationClass"))]
    pub m_deactivationClass: U8<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMotionState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMotionState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5797386e)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_sweptTransform.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkMotionState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x5797386e)));
            let mut serializer = __serializer
                .serialize_struct("hkMotionState", class_meta, (176u64, 176u64))?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.serialize_field("sweptTransform", &self.m_sweptTransform)?;
            serializer.serialize_field("deltaAngle", &self.m_deltaAngle)?;
            serializer.serialize_field("objectRadius", &self.m_objectRadius)?;
            serializer.serialize_field("linearDamping", &self.m_linearDamping)?;
            serializer.serialize_field("angularDamping", &self.m_angularDamping)?;
            serializer.serialize_field("timeFactor", &self.m_timeFactor)?;
            serializer.serialize_field("maxLinearVelocity", &self.m_maxLinearVelocity)?;
            serializer
                .serialize_field("maxAngularVelocity", &self.m_maxAngularVelocity)?;
            serializer.serialize_field("deactivationClass", &self.m_deactivationClass)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMotionState<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_transform,
                m_sweptTransform,
                m_deltaAngle,
                m_objectRadius,
                m_linearDamping,
                m_angularDamping,
                m_timeFactor,
                m_maxLinearVelocity,
                m_maxAngularVelocity,
                m_deactivationClass,
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
                        "transform" => Ok(__Field::m_transform),
                        "sweptTransform" => Ok(__Field::m_sweptTransform),
                        "deltaAngle" => Ok(__Field::m_deltaAngle),
                        "objectRadius" => Ok(__Field::m_objectRadius),
                        "linearDamping" => Ok(__Field::m_linearDamping),
                        "angularDamping" => Ok(__Field::m_angularDamping),
                        "timeFactor" => Ok(__Field::m_timeFactor),
                        "maxLinearVelocity" => Ok(__Field::m_maxLinearVelocity),
                        "maxAngularVelocity" => Ok(__Field::m_maxAngularVelocity),
                        "deactivationClass" => Ok(__Field::m_deactivationClass),
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
            struct __hkMotionStateVisitor<'de> {
                marker: _serde::__private::PhantomData<hkMotionState<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkMotionStateVisitor<'de> {
                type Value = hkMotionState<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkMotionState")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_transform: _serde::__private::Option<Transform> = _serde::__private::None;
                    let mut m_sweptTransform: _serde::__private::Option<
                        hkSweptTransform,
                    > = _serde::__private::None;
                    let mut m_deltaAngle: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_objectRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_linearDamping: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_angularDamping: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_timeFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_maxLinearVelocity: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_maxAngularVelocity: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_deactivationClass: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_transform) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transform",
                                        ),
                                    );
                                }
                                m_transform = _serde::__private::Some(
                                    match __A::next_value::<Transform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_sweptTransform) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sweptTransform",
                                        ),
                                    );
                                }
                                m_sweptTransform = _serde::__private::Some(
                                    match __A::next_value::<hkSweptTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_deltaAngle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deltaAngle",
                                        ),
                                    );
                                }
                                m_deltaAngle = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_objectRadius) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "objectRadius",
                                        ),
                                    );
                                }
                                m_objectRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_linearDamping) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearDamping",
                                        ),
                                    );
                                }
                                m_linearDamping = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_angularDamping) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularDamping",
                                        ),
                                    );
                                }
                                m_angularDamping = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_timeFactor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeFactor",
                                        ),
                                    );
                                }
                                m_timeFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxLinearVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearVelocity",
                                        ),
                                    );
                                }
                                m_maxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxAngularVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngularVelocity",
                                        ),
                                    );
                                }
                                m_maxAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationClass,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationClass",
                                        ),
                                    );
                                }
                                m_deactivationClass = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
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
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    let m_transform = match m_transform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transform",
                                ),
                            );
                        }
                    };
                    let m_sweptTransform = match m_sweptTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sweptTransform",
                                ),
                            );
                        }
                    };
                    let m_deltaAngle = match m_deltaAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deltaAngle",
                                ),
                            );
                        }
                    };
                    let m_objectRadius = match m_objectRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "objectRadius",
                                ),
                            );
                        }
                    };
                    let m_linearDamping = match m_linearDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearDamping",
                                ),
                            );
                        }
                    };
                    let m_angularDamping = match m_angularDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularDamping",
                                ),
                            );
                        }
                    };
                    let m_timeFactor = match m_timeFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeFactor",
                                ),
                            );
                        }
                    };
                    let m_maxLinearVelocity = match m_maxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearVelocity",
                                ),
                            );
                        }
                    };
                    let m_maxAngularVelocity = match m_maxAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAngularVelocity",
                                ),
                            );
                        }
                    };
                    let m_deactivationClass = match m_deactivationClass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationClass",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkMotionState {
                        __ptr,
                        m_transform,
                        m_sweptTransform,
                        m_deltaAngle,
                        m_objectRadius,
                        m_linearDamping,
                        m_angularDamping,
                        m_timeFactor,
                        m_maxLinearVelocity,
                        m_maxAngularVelocity,
                        m_deactivationClass,
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
                    let mut m_transform: _serde::__private::Option<Transform> = _serde::__private::None;
                    let mut m_sweptTransform: _serde::__private::Option<
                        hkSweptTransform,
                    > = _serde::__private::None;
                    let mut m_deltaAngle: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_objectRadius: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_linearDamping: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_angularDamping: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_timeFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_maxLinearVelocity: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_maxAngularVelocity: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_deactivationClass: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_transform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_transform) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transform",
                                        ),
                                    );
                                }
                                m_transform = _serde::__private::Some(
                                    match __A::next_value::<Transform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sweptTransform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sweptTransform) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sweptTransform",
                                        ),
                                    );
                                }
                                m_sweptTransform = _serde::__private::Some(
                                    match __A::next_value::<hkSweptTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_deltaAngle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_deltaAngle) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deltaAngle",
                                        ),
                                    );
                                }
                                m_deltaAngle = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_objectRadius => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_objectRadius) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "objectRadius",
                                        ),
                                    );
                                }
                                m_objectRadius = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linearDamping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linearDamping) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearDamping",
                                        ),
                                    );
                                }
                                m_linearDamping = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angularDamping => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angularDamping) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularDamping",
                                        ),
                                    );
                                }
                                m_angularDamping = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeFactor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeFactor",
                                        ),
                                    );
                                }
                                m_timeFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxLinearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxLinearVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearVelocity",
                                        ),
                                    );
                                }
                                m_maxLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxAngularVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxAngularVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxAngularVelocity",
                                        ),
                                    );
                                }
                                m_maxAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationClass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationClass,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationClass",
                                        ),
                                    );
                                }
                                m_deactivationClass = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_transform = match m_transform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transform",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sweptTransform = match m_sweptTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sweptTransform",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deltaAngle = match m_deltaAngle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deltaAngle",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_objectRadius = match m_objectRadius {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "objectRadius",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linearDamping = match m_linearDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearDamping",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angularDamping = match m_angularDamping {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularDamping",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeFactor = match m_timeFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxLinearVelocity = match m_maxLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxAngularVelocity = match m_maxAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxAngularVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationClass = match m_deactivationClass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationClass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkMotionState {
                        __ptr: __ptr.clone(),
                        m_transform,
                        m_sweptTransform,
                        m_deltaAngle,
                        m_objectRadius,
                        m_linearDamping,
                        m_angularDamping,
                        m_timeFactor,
                        m_maxLinearVelocity,
                        m_maxAngularVelocity,
                        m_deactivationClass,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "transform",
                "sweptTransform",
                "deltaAngle",
                "objectRadius",
                "linearDamping",
                "angularDamping",
                "timeFactor",
                "maxLinearVelocity",
                "maxAngularVelocity",
                "deactivationClass",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMotionState",
                FIELDS,
                __hkMotionStateVisitor {
                    marker: _serde::__private::PhantomData::<hkMotionState>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
