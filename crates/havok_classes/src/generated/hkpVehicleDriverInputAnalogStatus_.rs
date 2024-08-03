use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleDriverInputAnalogStatus`
/// - version: `0`
/// - signature: `0x2b4a5803`
/// - size: ` 20`(x86)/` 32`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDriverInputAnalogStatus {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleDriverInputStatus,
    /// # C++ Info
    /// - name: `positionX`(ctype: `hkReal`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_positionX: f32,
    /// # C++ Info
    /// - name: `positionY`(ctype: `hkReal`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_positionY: f32,
    /// # C++ Info
    /// - name: `handbrakeButtonPressed`(ctype: `hkBool`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_handbrakeButtonPressed: bool,
    /// # C++ Info
    /// - name: `reverseButtonPressed`(ctype: `hkBool`)
    /// - offset: ` 17`(x86)/` 25`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_reverseButtonPressed: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDriverInputAnalogStatus {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDriverInputAnalogStatus"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x2b4a5803)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpVehicleDriverInputAnalogStatus {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x2b4a5803)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDriverInputAnalogStatus", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("positionX", &self.m_positionX)?;
            serializer.serialize_field("positionY", &self.m_positionY)?;
            serializer
                .serialize_field(
                    "handbrakeButtonPressed",
                    &self.m_handbrakeButtonPressed,
                )?;
            serializer
                .serialize_field("reverseButtonPressed", &self.m_reverseButtonPressed)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDriverInputAnalogStatus {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_positionX,
                m_positionY,
                m_handbrakeButtonPressed,
                m_reverseButtonPressed,
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
                        "positionX" => Ok(__Field::m_positionX),
                        "positionY" => Ok(__Field::m_positionY),
                        "handbrakeButtonPressed" => Ok(__Field::m_handbrakeButtonPressed),
                        "reverseButtonPressed" => Ok(__Field::m_reverseButtonPressed),
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
            struct __hkpVehicleDriverInputAnalogStatusVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpVehicleDriverInputAnalogStatus,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleDriverInputAnalogStatusVisitor<'de> {
                type Value = hkpVehicleDriverInputAnalogStatus;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleDriverInputAnalogStatus",
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
                    let mut m_positionX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_positionY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_handbrakeButtonPressed: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_reverseButtonPressed: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_positionX) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionX",
                                        ),
                                    );
                                }
                                m_positionX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_positionY) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionY",
                                        ),
                                    );
                                }
                                m_positionY = _serde::__private::Some(
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
                                    &m_handbrakeButtonPressed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handbrakeButtonPressed",
                                        ),
                                    );
                                }
                                m_handbrakeButtonPressed = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_reverseButtonPressed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reverseButtonPressed",
                                        ),
                                    );
                                }
                                m_reverseButtonPressed = _serde::__private::Some(
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
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    let m_positionX = match m_positionX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionX",
                                ),
                            );
                        }
                    };
                    let m_positionY = match m_positionY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionY",
                                ),
                            );
                        }
                    };
                    let m_handbrakeButtonPressed = match m_handbrakeButtonPressed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handbrakeButtonPressed",
                                ),
                            );
                        }
                    };
                    let m_reverseButtonPressed = match m_reverseButtonPressed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reverseButtonPressed",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleDriverInputAnalogStatus {
                        __ptr,
                        parent,
                        m_positionX,
                        m_positionY,
                        m_handbrakeButtonPressed,
                        m_reverseButtonPressed,
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
                    let mut m_positionX: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_positionY: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_handbrakeButtonPressed: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_reverseButtonPressed: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_positionX => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_positionX) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionX",
                                        ),
                                    );
                                }
                                m_positionX = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_positionY => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_positionY) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "positionY",
                                        ),
                                    );
                                }
                                m_positionY = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_handbrakeButtonPressed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_handbrakeButtonPressed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handbrakeButtonPressed",
                                        ),
                                    );
                                }
                                m_handbrakeButtonPressed = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_reverseButtonPressed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_reverseButtonPressed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reverseButtonPressed",
                                        ),
                                    );
                                }
                                m_reverseButtonPressed = _serde::__private::Some(
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
                    let m_positionX = match m_positionX {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionX",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_positionY = match m_positionY {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "positionY",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_handbrakeButtonPressed = match m_handbrakeButtonPressed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handbrakeButtonPressed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_reverseButtonPressed = match m_reverseButtonPressed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "reverseButtonPressed",
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
                    let parent = hkpVehicleDriverInputStatus {
                        __ptr,
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleDriverInputAnalogStatus {
                        __ptr,
                        parent,
                        m_positionX,
                        m_positionY,
                        m_handbrakeButtonPressed,
                        m_reverseButtonPressed,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "positionX",
                "positionY",
                "handbrakeButtonPressed",
                "reverseButtonPressed",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDriverInputAnalogStatus",
                FIELDS,
                __hkpVehicleDriverInputAnalogStatusVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDriverInputAnalogStatus,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
