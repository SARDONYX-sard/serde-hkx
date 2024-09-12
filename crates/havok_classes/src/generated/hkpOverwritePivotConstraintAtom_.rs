use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpOverwritePivotConstraintAtom`
/// - version: `0`
/// - signature: `0x1f11b467`
/// - size: `  4`(x86)/`  4`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpOverwritePivotConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// - name: `copyToPivotBFromPivotA`(ctype: `hkUint8`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_copyToPivotBFromPivotA: u8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpOverwritePivotConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpOverwritePivotConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x1f11b467)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpOverwritePivotConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x1f11b467)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpOverwritePivotConstraintAtom",
                    class_meta,
                    (4u64, 4u64),
                )?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer
                .serialize_field(
                    "copyToPivotBFromPivotA",
                    &self.m_copyToPivotBFromPivotA,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpOverwritePivotConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_copyToPivotBFromPivotA,
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
                        "type" => Ok(__Field::m_type),
                        "copyToPivotBFromPivotA" => Ok(__Field::m_copyToPivotBFromPivotA),
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
            struct __hkpOverwritePivotConstraintAtomVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpOverwritePivotConstraintAtom>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpOverwritePivotConstraintAtomVisitor<'de> {
                type Value = hkpOverwritePivotConstraintAtom;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpOverwritePivotConstraintAtom",
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
                    let mut m_copyToPivotBFromPivotA: _serde::__private::Option<u8> = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_copyToPivotBFromPivotA,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "copyToPivotBFromPivotA",
                                        ),
                                    );
                                }
                                m_copyToPivotBFromPivotA = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
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
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    let m_copyToPivotBFromPivotA = match m_copyToPivotBFromPivotA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "copyToPivotBFromPivotA",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpOverwritePivotConstraintAtom {
                        __ptr,
                        parent,
                        m_copyToPivotBFromPivotA,
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
                    let mut m_type: _serde::__private::Option<AtomType> = _serde::__private::None;
                    let mut m_copyToPivotBFromPivotA: _serde::__private::Option<u8> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<AtomType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_copyToPivotBFromPivotA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_copyToPivotBFromPivotA,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "copyToPivotBFromPivotA",
                                        ),
                                    );
                                }
                                m_copyToPivotBFromPivotA = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_copyToPivotBFromPivotA = match m_copyToPivotBFromPivotA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "copyToPivotBFromPivotA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkpConstraintAtom { __ptr, m_type };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpOverwritePivotConstraintAtom {
                        __ptr,
                        parent,
                        m_copyToPivotBFromPivotA,
                    })
                }
            }
            const FIELDS: &[&str] = &["copyToPivotBFromPivotA"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpOverwritePivotConstraintAtom",
                FIELDS,
                __hkpOverwritePivotConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpOverwritePivotConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
