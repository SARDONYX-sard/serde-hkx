use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleFrictionDescription`
/// - version: `0`
/// - signature: `0x1034549a`
/// - size: `208`(x86)/`208`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionDescription {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `wheelDistance`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_wheelDistance: f32,
    /// # C++ Info
    /// - name: `chassisMassInv`(ctype: `hkReal`)
    /// - offset: `  4`(x86)/`  4`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_chassisMassInv: f32,
    /// # C++ Info
    /// - name: `axleDescr`(ctype: `struct hkpVehicleFrictionDescriptionAxisDescription[2]`)
    /// - offset: `  8`(x86)/`  8`(x86_64)
    /// - type_size: `100`(x86)/`200`(x86_64)
    pub m_axleDescr: [hkpVehicleFrictionDescriptionAxisDescription; 2usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleFrictionDescription {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionDescription"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1034549a)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_axleDescr
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl _serde::Serialize for hkpVehicleFrictionDescription {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x1034549a)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleFrictionDescription", class_meta)?;
            serializer.serialize_field("wheelDistance", &self.m_wheelDistance)?;
            serializer.serialize_field("chassisMassInv", &self.m_chassisMassInv)?;
            serializer
                .serialize_fixed_array_field("axleDescr", self.m_axleDescr.as_slice())?;
            serializer.pad_field([0u8; 100usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleFrictionDescription {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_wheelDistance,
                m_chassisMassInv,
                m_axleDescr,
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
                        "wheelDistance" => Ok(__Field::m_wheelDistance),
                        "chassisMassInv" => Ok(__Field::m_chassisMassInv),
                        "axleDescr" => Ok(__Field::m_axleDescr),
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
            struct __hkpVehicleFrictionDescriptionVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleFrictionDescription>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleFrictionDescriptionVisitor<'de> {
                type Value = hkpVehicleFrictionDescription;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleFrictionDescription",
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
                    let mut m_wheelDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_chassisMassInv: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_axleDescr: _serde::__private::Option<
                        [hkpVehicleFrictionDescriptionAxisDescription; 2usize],
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_wheelDistance) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelDistance",
                                        ),
                                    );
                                }
                                m_wheelDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_chassisMassInv) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chassisMassInv",
                                        ),
                                    );
                                }
                                m_chassisMassInv = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_axleDescr) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "axleDescr",
                                        ),
                                    );
                                }
                                m_axleDescr = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hkpVehicleFrictionDescriptionAxisDescription; 2usize],
                                    >(&mut __map) {
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
                    __A::pad(&mut __map, 100usize, 0usize)?;
                    let m_wheelDistance = match m_wheelDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelDistance",
                                ),
                            );
                        }
                    };
                    let m_chassisMassInv = match m_chassisMassInv {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "chassisMassInv",
                                ),
                            );
                        }
                    };
                    let m_axleDescr = match m_axleDescr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "axleDescr",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleFrictionDescription {
                        __ptr,
                        m_wheelDistance,
                        m_chassisMassInv,
                        m_axleDescr,
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
                    let mut m_wheelDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_chassisMassInv: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_axleDescr: _serde::__private::Option<
                        [hkpVehicleFrictionDescriptionAxisDescription; 2usize],
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
                            __Field::m_wheelDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wheelDistance) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wheelDistance",
                                        ),
                                    );
                                }
                                m_wheelDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_chassisMassInv => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_chassisMassInv) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "chassisMassInv",
                                        ),
                                    );
                                }
                                m_chassisMassInv = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_axleDescr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_axleDescr) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "axleDescr",
                                        ),
                                    );
                                }
                                m_axleDescr = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hkpVehicleFrictionDescriptionAxisDescription; 2usize],
                                    >(&mut __map) {
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
                    let m_wheelDistance = match m_wheelDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wheelDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_chassisMassInv = match m_chassisMassInv {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "chassisMassInv",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_axleDescr = match m_axleDescr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "axleDescr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleFrictionDescription {
                        __ptr,
                        m_wheelDistance,
                        m_chassisMassInv,
                        m_axleDescr,
                    })
                }
            }
            const FIELDS: &[&str] = &["wheelDistance", "chassisMassInv", "axleDescr"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleFrictionDescription",
                FIELDS,
                __hkpVehicleFrictionDescriptionVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleFrictionDescription,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
