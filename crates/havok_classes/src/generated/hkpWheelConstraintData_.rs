use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpWheelConstraintData`
/// - version: `0`
/// - signature: `0xb4c46671`
/// - size: `352`(x86)/`368`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpWheelConstraintData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintData,
    /// # C++ Info
    /// - name: `atoms`(ctype: `struct hkpWheelConstraintDataAtoms`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `304`(x86)/`304`(x86_64)
    /// - flags: `ALIGN_16`
    pub m_atoms: hkpWheelConstraintDataAtoms,
    /// # C++ Info
    /// - name: `initialAxleInB`(ctype: `hkVector4`)
    /// - offset: `320`(x86)/`336`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_initialAxleInB: Vector4,
    /// # C++ Info
    /// - name: `initialSteeringAxisInB`(ctype: `hkVector4`)
    /// - offset: `336`(x86)/`352`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_initialSteeringAxisInB: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpWheelConstraintData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpWheelConstraintData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb4c46671)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_atoms.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkpWheelConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb4c46671)));
            let mut serializer = __serializer
                .serialize_struct("hkpWheelConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("initialAxleInB", &self.m_initialAxleInB)?;
            serializer
                .serialize_field(
                    "initialSteeringAxisInB",
                    &self.m_initialSteeringAxisInB,
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
    impl<'de> _serde::Deserialize<'de> for hkpWheelConstraintData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_atoms,
                m_initialAxleInB,
                m_initialSteeringAxisInB,
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
                        "atoms" => Ok(__Field::m_atoms),
                        "initialAxleInB" => Ok(__Field::m_initialAxleInB),
                        "initialSteeringAxisInB" => Ok(__Field::m_initialSteeringAxisInB),
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
            struct __hkpWheelConstraintDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpWheelConstraintData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpWheelConstraintDataVisitor<'de> {
                type Value = hkpWheelConstraintData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpWheelConstraintData",
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
                    let mut m_atoms: _serde::__private::Option<
                        hkpWheelConstraintDataAtoms,
                    > = _serde::__private::None;
                    let mut m_initialAxleInB: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_initialSteeringAxisInB: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_atoms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 8usize)?;
                                m_atoms = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpWheelConstraintDataAtoms,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_initialAxleInB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialAxleInB",
                                        ),
                                    );
                                }
                                m_initialAxleInB = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_initialSteeringAxisInB,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialSteeringAxisInB",
                                        ),
                                    );
                                }
                                m_initialSteeringAxisInB = _serde::__private::Some(
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
                    let m_atoms = match m_atoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("atoms"),
                            );
                        }
                    };
                    let m_initialAxleInB = match m_initialAxleInB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialAxleInB",
                                ),
                            );
                        }
                    };
                    let m_initialSteeringAxisInB = match m_initialSteeringAxisInB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialSteeringAxisInB",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpWheelConstraintData {
                        __ptr,
                        parent,
                        m_atoms,
                        m_initialAxleInB,
                        m_initialSteeringAxisInB,
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
                    let mut m_atoms: _serde::__private::Option<
                        hkpWheelConstraintDataAtoms,
                    > = _serde::__private::None;
                    let mut m_initialAxleInB: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_initialSteeringAxisInB: _serde::__private::Option<
                        Vector4,
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
                            __Field::m_atoms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_atoms) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                                    );
                                }
                                m_atoms = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpWheelConstraintDataAtoms,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_initialAxleInB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_initialAxleInB) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialAxleInB",
                                        ),
                                    );
                                }
                                m_initialAxleInB = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_initialSteeringAxisInB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_initialSteeringAxisInB,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initialSteeringAxisInB",
                                        ),
                                    );
                                }
                                m_initialSteeringAxisInB = _serde::__private::Some(
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
                    let m_atoms = match m_atoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("atoms"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_initialAxleInB = match m_initialAxleInB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialAxleInB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_initialSteeringAxisInB = match m_initialSteeringAxisInB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initialSteeringAxisInB",
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
                    let parent = hkpConstraintData {
                        __ptr,
                        parent,
                        m_userData,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpWheelConstraintData {
                        __ptr,
                        parent,
                        m_atoms,
                        m_initialAxleInB,
                        m_initialSteeringAxisInB,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "atoms",
                "initialAxleInB",
                "initialSteeringAxisInB",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpWheelConstraintData",
                FIELDS,
                __hkpWheelConstraintDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpWheelConstraintData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
