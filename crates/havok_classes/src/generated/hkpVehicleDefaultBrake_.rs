use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleDefaultBrake`
/// - version: `0`
/// - signature: `0x4b4f8816`
/// - size: ` 24`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultBrake {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleBrake,
    /// # C++ Info
    /// - name: `wheelBrakingProperties`(ctype: `hkArray<struct hkpVehicleDefaultBrakeWheelBrakingProperties>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wheelBrakingProperties: Vec<hkpVehicleDefaultBrakeWheelBrakingProperties>,
    /// # C++ Info
    /// - name: `wheelsMinTimeToBlock`(ctype: `hkReal`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_wheelsMinTimeToBlock: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultBrake {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultBrake"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4b4f8816)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultBrake {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x4b4f8816)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultBrake", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "wheelBrakingProperties",
                    &self.m_wheelBrakingProperties,
                )?;
            serializer
                .serialize_field("wheelsMinTimeToBlock", &self.m_wheelsMinTimeToBlock)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "wheelBrakingProperties",
                    &self.m_wheelBrakingProperties,
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
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultBrake {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_wheelsMinTimeToBlock,
                m_wheelBrakingProperties,
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
                        "wheelsMinTimeToBlock" => Ok(__Field::m_wheelsMinTimeToBlock),
                        "wheelBrakingProperties" => Ok(__Field::m_wheelBrakingProperties),
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
            struct __hkpVehicleDefaultBrakeVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleDefaultBrake>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpVehicleDefaultBrakeVisitor<'de> {
                type Value = hkpVehicleDefaultBrake;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleDefaultBrake",
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
                    let mut m_wheelBrakingProperties: _serde::__private::Option<
                        Vec<hkpVehicleDefaultBrakeWheelBrakingProperties>,
                    > = _serde::__private::None;
                    let mut m_wheelsMinTimeToBlock: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wheelBrakingProperties,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelBrakingProperties",
                                        ),
                                    );
                                }
                                m_wheelBrakingProperties = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpVehicleDefaultBrakeWheelBrakingProperties>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_wheelsMinTimeToBlock,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsMinTimeToBlock",
                                        ),
                                    );
                                }
                                m_wheelsMinTimeToBlock = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_wheelBrakingProperties = match m_wheelBrakingProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelBrakingProperties",
                                ),
                            );
                        }
                    };
                    let m_wheelsMinTimeToBlock = match m_wheelsMinTimeToBlock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsMinTimeToBlock",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleDefaultBrake {
                        __ptr,
                        parent,
                        m_wheelBrakingProperties,
                        m_wheelsMinTimeToBlock,
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
                    let mut m_wheelsMinTimeToBlock: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_wheelBrakingProperties: _serde::__private::Option<
                        Vec<hkpVehicleDefaultBrakeWheelBrakingProperties>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_wheelsMinTimeToBlock => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wheelsMinTimeToBlock,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelsMinTimeToBlock",
                                        ),
                                    );
                                }
                                m_wheelsMinTimeToBlock = _serde::__private::Some(
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
                            __Field::m_wheelBrakingProperties => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_wheelBrakingProperties,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelBrakingProperties",
                                        ),
                                    );
                                }
                                m_wheelBrakingProperties = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkpVehicleDefaultBrakeWheelBrakingProperties>,
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
                            _ => {}
                        }
                    }
                    let m_wheelsMinTimeToBlock = match m_wheelsMinTimeToBlock {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelsMinTimeToBlock",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wheelBrakingProperties = match m_wheelBrakingProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelBrakingProperties",
                                ),
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
                    let parent = hkpVehicleBrake { __ptr, parent };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleDefaultBrake {
                        __ptr,
                        parent,
                        m_wheelBrakingProperties,
                        m_wheelsMinTimeToBlock,
                    })
                }
            }
            const FIELDS: &[&str] = &["wheelBrakingProperties", "wheelsMinTimeToBlock"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultBrake",
                FIELDS,
                __hkpVehicleDefaultBrakeVisitor {
                    marker: _serde::__private::PhantomData::<hkpVehicleDefaultBrake>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
