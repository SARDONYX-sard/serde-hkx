use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleFrictionStatusAxisStatus`
/// - version: `0`
/// - signature: `0xe70e2bb4`
/// - size: ` 36`(x86)/` 36`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionStatusAxisStatus<'a> {
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
    /// - name: `forward_slip_velocity`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "forward_slip_velocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "forward_slip_velocity"))]
    pub m_forward_slip_velocity: f32,
    /// # C++ Info
    /// - name: `side_slip_velocity`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "side_slip_velocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "side_slip_velocity"))]
    pub m_side_slip_velocity: f32,
    /// # C++ Info
    /// - name: `skid_energy_density`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "skid_energy_density"))]
    #[cfg_attr(feature = "serde", serde(rename = "skid_energy_density"))]
    pub m_skid_energy_density: f32,
    /// # C++ Info
    /// - name: `side_force`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "side_force"))]
    #[cfg_attr(feature = "serde", serde(rename = "side_force"))]
    pub m_side_force: f32,
    /// # C++ Info
    /// - name: `delayed_forward_impulse`(ctype: `hkReal`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "delayed_forward_impulse"))]
    #[cfg_attr(feature = "serde", serde(rename = "delayed_forward_impulse"))]
    pub m_delayed_forward_impulse: f32,
    /// # C++ Info
    /// - name: `sideRhs`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "sideRhs"))]
    #[cfg_attr(feature = "serde", serde(rename = "sideRhs"))]
    pub m_sideRhs: f32,
    /// # C++ Info
    /// - name: `forwardRhs`(ctype: `hkReal`)
    /// - offset: ` 24`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "forwardRhs"))]
    #[cfg_attr(feature = "serde", serde(rename = "forwardRhs"))]
    pub m_forwardRhs: f32,
    /// # C++ Info
    /// - name: `relativeSideForce`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "relativeSideForce"))]
    #[cfg_attr(feature = "serde", serde(rename = "relativeSideForce"))]
    pub m_relativeSideForce: f32,
    /// # C++ Info
    /// - name: `relativeForwardForce`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "relativeForwardForce"))]
    #[cfg_attr(feature = "serde", serde(rename = "relativeForwardForce"))]
    pub m_relativeForwardForce: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpVehicleFrictionStatusAxisStatus<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionStatusAxisStatus"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe70e2bb4)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpVehicleFrictionStatusAxisStatus<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xe70e2bb4)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpVehicleFrictionStatusAxisStatus",
                    class_meta,
                    (36u64, 36u64),
                )?;
            serializer
                .serialize_field(
                    "forward_slip_velocity",
                    &self.m_forward_slip_velocity,
                )?;
            serializer
                .serialize_field("side_slip_velocity", &self.m_side_slip_velocity)?;
            serializer
                .serialize_field("skid_energy_density", &self.m_skid_energy_density)?;
            serializer.serialize_field("side_force", &self.m_side_force)?;
            serializer
                .serialize_field(
                    "delayed_forward_impulse",
                    &self.m_delayed_forward_impulse,
                )?;
            serializer.serialize_field("sideRhs", &self.m_sideRhs)?;
            serializer.serialize_field("forwardRhs", &self.m_forwardRhs)?;
            serializer.serialize_field("relativeSideForce", &self.m_relativeSideForce)?;
            serializer
                .serialize_field("relativeForwardForce", &self.m_relativeForwardForce)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleFrictionStatusAxisStatus<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_forward_slip_velocity,
                m_side_slip_velocity,
                m_skid_energy_density,
                m_side_force,
                m_delayed_forward_impulse,
                m_sideRhs,
                m_forwardRhs,
                m_relativeSideForce,
                m_relativeForwardForce,
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
                        "forward_slip_velocity" => Ok(__Field::m_forward_slip_velocity),
                        "side_slip_velocity" => Ok(__Field::m_side_slip_velocity),
                        "skid_energy_density" => Ok(__Field::m_skid_energy_density),
                        "side_force" => Ok(__Field::m_side_force),
                        "delayed_forward_impulse" => {
                            Ok(__Field::m_delayed_forward_impulse)
                        }
                        "sideRhs" => Ok(__Field::m_sideRhs),
                        "forwardRhs" => Ok(__Field::m_forwardRhs),
                        "relativeSideForce" => Ok(__Field::m_relativeSideForce),
                        "relativeForwardForce" => Ok(__Field::m_relativeForwardForce),
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
            struct __hkpVehicleFrictionStatusAxisStatusVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpVehicleFrictionStatusAxisStatus<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleFrictionStatusAxisStatusVisitor<'de> {
                type Value = hkpVehicleFrictionStatusAxisStatus<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleFrictionStatusAxisStatus",
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
                    let mut m_forward_slip_velocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_side_slip_velocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_skid_energy_density: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_side_force: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_delayed_forward_impulse: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sideRhs: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_forwardRhs: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_relativeSideForce: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_relativeForwardForce: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_forward_slip_velocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forward_slip_velocity",
                                        ),
                                    );
                                }
                                m_forward_slip_velocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_side_slip_velocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "side_slip_velocity",
                                        ),
                                    );
                                }
                                m_side_slip_velocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_skid_energy_density,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skid_energy_density",
                                        ),
                                    );
                                }
                                m_skid_energy_density = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_side_force) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "side_force",
                                        ),
                                    );
                                }
                                m_side_force = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_delayed_forward_impulse,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayed_forward_impulse",
                                        ),
                                    );
                                }
                                m_delayed_forward_impulse = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_sideRhs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sideRhs",
                                        ),
                                    );
                                }
                                m_sideRhs = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_forwardRhs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardRhs",
                                        ),
                                    );
                                }
                                m_forwardRhs = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_relativeSideForce,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativeSideForce",
                                        ),
                                    );
                                }
                                m_relativeSideForce = _serde::__private::Some(
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
                                    &m_relativeForwardForce,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativeForwardForce",
                                        ),
                                    );
                                }
                                m_relativeForwardForce = _serde::__private::Some(
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
                    let m_forward_slip_velocity = match m_forward_slip_velocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forward_slip_velocity",
                                ),
                            );
                        }
                    };
                    let m_side_slip_velocity = match m_side_slip_velocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "side_slip_velocity",
                                ),
                            );
                        }
                    };
                    let m_skid_energy_density = match m_skid_energy_density {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skid_energy_density",
                                ),
                            );
                        }
                    };
                    let m_side_force = match m_side_force {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "side_force",
                                ),
                            );
                        }
                    };
                    let m_delayed_forward_impulse = match m_delayed_forward_impulse {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "delayed_forward_impulse",
                                ),
                            );
                        }
                    };
                    let m_sideRhs = match m_sideRhs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("sideRhs"),
                            );
                        }
                    };
                    let m_forwardRhs = match m_forwardRhs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardRhs",
                                ),
                            );
                        }
                    };
                    let m_relativeSideForce = match m_relativeSideForce {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativeSideForce",
                                ),
                            );
                        }
                    };
                    let m_relativeForwardForce = match m_relativeForwardForce {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativeForwardForce",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleFrictionStatusAxisStatus {
                        __ptr,
                        m_forward_slip_velocity,
                        m_side_slip_velocity,
                        m_skid_energy_density,
                        m_side_force,
                        m_delayed_forward_impulse,
                        m_sideRhs,
                        m_forwardRhs,
                        m_relativeSideForce,
                        m_relativeForwardForce,
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
                    let mut m_forward_slip_velocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_side_slip_velocity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_skid_energy_density: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_side_force: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_delayed_forward_impulse: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_sideRhs: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_forwardRhs: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_relativeSideForce: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_relativeForwardForce: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_forward_slip_velocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_forward_slip_velocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forward_slip_velocity",
                                        ),
                                    );
                                }
                                m_forward_slip_velocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_side_slip_velocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_side_slip_velocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "side_slip_velocity",
                                        ),
                                    );
                                }
                                m_side_slip_velocity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skid_energy_density => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_skid_energy_density,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skid_energy_density",
                                        ),
                                    );
                                }
                                m_skid_energy_density = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_side_force => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_side_force) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "side_force",
                                        ),
                                    );
                                }
                                m_side_force = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_delayed_forward_impulse => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_delayed_forward_impulse,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "delayed_forward_impulse",
                                        ),
                                    );
                                }
                                m_delayed_forward_impulse = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sideRhs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sideRhs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sideRhs",
                                        ),
                                    );
                                }
                                m_sideRhs = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_forwardRhs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_forwardRhs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forwardRhs",
                                        ),
                                    );
                                }
                                m_forwardRhs = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_relativeSideForce => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_relativeSideForce,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativeSideForce",
                                        ),
                                    );
                                }
                                m_relativeSideForce = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_relativeForwardForce => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_relativeForwardForce,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "relativeForwardForce",
                                        ),
                                    );
                                }
                                m_relativeForwardForce = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_forward_slip_velocity = match m_forward_slip_velocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forward_slip_velocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_side_slip_velocity = match m_side_slip_velocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "side_slip_velocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skid_energy_density = match m_skid_energy_density {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skid_energy_density",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_side_force = match m_side_force {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "side_force",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_delayed_forward_impulse = match m_delayed_forward_impulse {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "delayed_forward_impulse",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sideRhs = match m_sideRhs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("sideRhs"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_forwardRhs = match m_forwardRhs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forwardRhs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_relativeSideForce = match m_relativeSideForce {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativeSideForce",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_relativeForwardForce = match m_relativeForwardForce {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "relativeForwardForce",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleFrictionStatusAxisStatus {
                        __ptr: __ptr.clone(),
                        m_forward_slip_velocity,
                        m_side_slip_velocity,
                        m_skid_energy_density,
                        m_side_force,
                        m_delayed_forward_impulse,
                        m_sideRhs,
                        m_forwardRhs,
                        m_relativeSideForce,
                        m_relativeForwardForce,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "forward_slip_velocity",
                "side_slip_velocity",
                "skid_energy_density",
                "side_force",
                "delayed_forward_impulse",
                "sideRhs",
                "forwardRhs",
                "relativeSideForce",
                "relativeForwardForce",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleFrictionStatusAxisStatus",
                FIELDS,
                __hkpVehicleFrictionStatusAxisStatusVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleFrictionStatusAxisStatus,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
