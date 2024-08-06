use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaRagdollInstance`
/// - version: `0`
/// - signature: `0x154948e8`
/// - size: ` 48`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaRagdollInstance {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// - name: `rigidBodies`(ctype: `hkArray<hkpRigidBody*>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_rigidBodies: Vec<Pointer>,
    /// # C++ Info
    /// - name: `constraints`(ctype: `hkArray<hkpConstraintInstance*>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_constraints: Vec<Pointer>,
    /// # C++ Info
    /// - name: `boneToRigidBodyMap`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_boneToRigidBodyMap: Vec<i32>,
    /// # C++ Info
    /// - name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_skeleton: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaRagdollInstance {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaRagdollInstance"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x154948e8)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(self.m_rigidBodies.iter().map(|ptr| ptr.get()));
            v.extend(self.m_constraints.iter().map(|ptr| ptr.get()));
            v.push(self.m_skeleton.get());
            v
        }
    }
    impl _serde::Serialize for hkaRagdollInstance {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x154948e8)));
            let mut serializer = __serializer
                .serialize_struct("hkaRagdollInstance", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("rigidBodies", &self.m_rigidBodies)?;
            serializer.serialize_array_meta_field("constraints", &self.m_constraints)?;
            serializer
                .serialize_array_meta_field(
                    "boneToRigidBodyMap",
                    &self.m_boneToRigidBodyMap,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.serialize_array_field("rigidBodies", &self.m_rigidBodies)?;
            serializer.serialize_array_field("constraints", &self.m_constraints)?;
            serializer
                .serialize_array_field(
                    "boneToRigidBodyMap",
                    &self.m_boneToRigidBodyMap,
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
    impl<'de> _serde::Deserialize<'de> for hkaRagdollInstance {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_rigidBodies,
                m_constraints,
                m_boneToRigidBodyMap,
                m_skeleton,
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
                        "rigidBodies" => Ok(__Field::m_rigidBodies),
                        "constraints" => Ok(__Field::m_constraints),
                        "boneToRigidBodyMap" => Ok(__Field::m_boneToRigidBodyMap),
                        "skeleton" => Ok(__Field::m_skeleton),
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
            struct __hkaRagdollInstanceVisitor<'de> {
                marker: _serde::__private::PhantomData<hkaRagdollInstance>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkaRagdollInstanceVisitor<'de> {
                type Value = hkaRagdollInstance;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaRagdollInstance",
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
                    let mut m_rigidBodies: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_constraints: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_boneToRigidBodyMap: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_skeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_rigidBodies) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBodies",
                                        ),
                                    );
                                }
                                m_rigidBodies = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_constraints) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraints",
                                        ),
                                    );
                                }
                                m_constraints = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_boneToRigidBodyMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneToRigidBodyMap",
                                        ),
                                    );
                                }
                                m_boneToRigidBodyMap = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_skeleton) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeleton",
                                        ),
                                    );
                                }
                                m_skeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_rigidBodies = match m_rigidBodies {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBodies",
                                ),
                            );
                        }
                    };
                    let m_constraints = match m_constraints {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraints",
                                ),
                            );
                        }
                    };
                    let m_boneToRigidBodyMap = match m_boneToRigidBodyMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneToRigidBodyMap",
                                ),
                            );
                        }
                    };
                    let m_skeleton = match m_skeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("skeleton"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaRagdollInstance {
                        __ptr,
                        parent,
                        m_rigidBodies,
                        m_constraints,
                        m_boneToRigidBodyMap,
                        m_skeleton,
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
                    let mut m_rigidBodies: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_constraints: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
                    let mut m_boneToRigidBodyMap: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_skeleton: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_rigidBodies => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rigidBodies) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBodies",
                                        ),
                                    );
                                }
                                m_rigidBodies = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_constraints => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_constraints) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraints",
                                        ),
                                    );
                                }
                                m_constraints = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneToRigidBodyMap => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneToRigidBodyMap,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneToRigidBodyMap",
                                        ),
                                    );
                                }
                                m_boneToRigidBodyMap = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skeleton => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skeleton) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeleton",
                                        ),
                                    );
                                }
                                m_skeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_rigidBodies = match m_rigidBodies {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBodies",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constraints = match m_constraints {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraints",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneToRigidBodyMap = match m_boneToRigidBodyMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneToRigidBodyMap",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skeleton = match m_skeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("skeleton"),
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkaRagdollInstance {
                        __ptr,
                        parent,
                        m_rigidBodies,
                        m_constraints,
                        m_boneToRigidBodyMap,
                        m_skeleton,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "rigidBodies",
                "constraints",
                "boneToRigidBodyMap",
                "skeleton",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaRagdollInstance",
                FIELDS,
                __hkaRagdollInstanceVisitor {
                    marker: _serde::__private::PhantomData::<hkaRagdollInstance>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
