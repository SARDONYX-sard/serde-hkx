use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpVehicleFrictionStatus`
/// - version: `0`
/// - signature: `0x1c076a84`
/// - size: ` 72`(x86)/` 72`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleFrictionStatus {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `axis`(ctype: `struct hkpVehicleFrictionStatusAxisStatus[2]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 36`(x86)/` 72`(x86_64)
    pub m_axis: [hkpVehicleFrictionStatusAxisStatus; 2usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleFrictionStatus {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleFrictionStatus"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1c076a84)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_axis
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl _serde::Serialize for hkpVehicleFrictionStatus {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x1c076a84)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleFrictionStatus", class_meta)?;
            serializer.serialize_fixed_array_field("axis", self.m_axis.as_slice())?;
            serializer.pad_field([0u8; 36usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleFrictionStatus {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_axis,
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
                        "axis" => Ok(__Field::m_axis),
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
            struct __hkpVehicleFrictionStatusVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpVehicleFrictionStatus>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpVehicleFrictionStatusVisitor<'de> {
                type Value = hkpVehicleFrictionStatus;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpVehicleFrictionStatus",
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
                    let mut m_axis: _serde::__private::Option<
                        [hkpVehicleFrictionStatusAxisStatus; 2usize],
                    > = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_axis) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("axis"),
                                    );
                                }
                                m_axis = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hkpVehicleFrictionStatusAxisStatus; 2usize],
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
                    __A::pad(&mut __map, 36usize, 0usize)?;
                    let m_axis = match m_axis {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("axis"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpVehicleFrictionStatus {
                        __ptr,
                        m_axis,
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
                    let mut m_axis: _serde::__private::Option<
                        [hkpVehicleFrictionStatusAxisStatus; 2usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_axis => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_axis) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("axis"),
                                    );
                                }
                                m_axis = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hkpVehicleFrictionStatusAxisStatus; 2usize],
                                    >(&mut __map) {
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
                    let m_axis = match m_axis {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("axis"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpVehicleFrictionStatus {
                        __ptr,
                        m_axis,
                    })
                }
            }
            const FIELDS: &[&str] = &["axis"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleFrictionStatus",
                FIELDS,
                __hkpVehicleFrictionStatusVisitor {
                    marker: _serde::__private::PhantomData::<hkpVehicleFrictionStatus>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
