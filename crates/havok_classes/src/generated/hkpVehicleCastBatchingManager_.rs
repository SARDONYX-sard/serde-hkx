use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleCastBatchingManager`
/// - version: `0`
/// - signature: `0x53340a9`
/// - size: ` 24`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleCastBatchingManager {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleManager,
    /// # C++ Info
    /// - name: `totalNumWheels`(ctype: `hkUint16`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_totalNumWheels: u16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleCastBatchingManager {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleCastBatchingManager"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x53340a9)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.parent.m_registeredVehicles.iter().map(|ptr| ptr.get()));
            v
        }
    }
    impl _serde::Serialize for hkpVehicleCastBatchingManager {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x53340a9)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleCastBatchingManager", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "registeredVehicles",
                    &self.parent.m_registeredVehicles,
                )?;
            serializer.serialize_field("totalNumWheels", &self.m_totalNumWheels)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "registeredVehicles",
                    &self.parent.m_registeredVehicles,
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
    impl<'de> _serde::Deserialize<'de> for hkpVehicleCastBatchingManager {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_registeredVehicles,
                m_totalNumWheels,
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
                        "registeredVehicles" => Ok(__Field::m_registeredVehicles),
                        "totalNumWheels" => Ok(__Field::m_totalNumWheels),
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
            struct __hkpVehicleCastBatchingManagerVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleCastBatchingManager>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleCastBatchingManagerVisitor<'de> {
                type Value = hkpVehicleCastBatchingManager;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleCastBatchingManager",
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
                    let mut m_totalNumWheels: _serde::__private::Option<u16> = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_totalNumWheels) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "totalNumWheels",
                                        ),
                                    );
                                }
                                m_totalNumWheels = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
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
                    let m_totalNumWheels = match m_totalNumWheels {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "totalNumWheels",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleCastBatchingManager {
                        __ptr,
                        parent,
                        m_totalNumWheels,
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
                    let mut m_registeredVehicles: _serde::__private::Option<
                        Vec<Pointer>,
                    > = _serde::__private::None;
                    let mut m_totalNumWheels: _serde::__private::Option<u16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_registeredVehicles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_registeredVehicles,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "registeredVehicles",
                                        ),
                                    );
                                }
                                m_registeredVehicles = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_totalNumWheels => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_totalNumWheels) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "totalNumWheels",
                                        ),
                                    );
                                }
                                m_totalNumWheels = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
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
                    let m_registeredVehicles = match m_registeredVehicles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "registeredVehicles",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_totalNumWheels = match m_totalNumWheels {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "totalNumWheels",
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
                    let parent = hkpVehicleManager {
                        __ptr,
                        parent,
                        m_registeredVehicles,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleCastBatchingManager {
                        __ptr,
                        parent,
                        m_totalNumWheels,
                    })
                }
            }
            const FIELDS: &[&str] = &["totalNumWheels"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleCastBatchingManager",
                FIELDS,
                __hkpVehicleCastBatchingManagerVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleCastBatchingManager,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
