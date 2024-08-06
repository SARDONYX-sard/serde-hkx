use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpRigidBody`
/// - version: `0`
/// - signature: `0x75f8d805`
/// - size: `544`(x86)/`720`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpRigidBody<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpEntity<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpRigidBody<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpRigidBody"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x75f8d805)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.m_world.get());
            v.extend(self.parent.parent.m_collidable.deps_indexes());
            v.extend(self.parent.parent.m_multiThreadCheck.deps_indexes());
            v.extend(
                self
                    .parent
                    .parent
                    .m_properties
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.push(self.parent.parent.m_treeData.get());
            v.extend(self.parent.m_material.deps_indexes());
            v.push(self.parent.m_limitContactImpulseUtilAndFlag.get());
            v.push(self.parent.m_breakableBody.get());
            v.extend(self.parent.m_constraintsMaster.deps_indexes());
            v.extend(self.parent.m_constraintsSlave.iter().map(|ptr| ptr.get()));
            v.push(self.parent.m_simulationIsland.get());
            v.extend(self.parent.m_spuCollisionCallback.deps_indexes());
            v.extend(self.parent.m_motion.deps_indexes());
            v.extend(self.parent.m_contactListeners.deps_indexes());
            v.extend(self.parent.m_actions.deps_indexes());
            v.push(self.parent.m_localFrame.get());
            v.push(self.parent.m_extendedListeners.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkpRigidBody<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x75f8d805)));
            let mut serializer = __serializer
                .serialize_struct("hkpRigidBody", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.parent.m_collidable)?;
            serializer
                .serialize_field(
                    "multiThreadCheck",
                    &self.parent.parent.m_multiThreadCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_meta_field(
                    "properties",
                    &self.parent.parent.m_properties,
                )?;
            serializer.skip_field("treeData", &self.parent.parent.m_treeData)?;
            serializer.serialize_field("material", &self.parent.m_material)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "limitContactImpulseUtilAndFlag",
                    &self.parent.m_limitContactImpulseUtilAndFlag,
                )?;
            serializer
                .serialize_field("damageMultiplier", &self.parent.m_damageMultiplier)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("breakableBody", &self.parent.m_breakableBody)?;
            serializer.skip_field("solverData", &self.parent.m_solverData)?;
            serializer.serialize_field("storageIndex", &self.parent.m_storageIndex)?;
            serializer
                .serialize_field(
                    "contactPointCallbackDelay",
                    &self.parent.m_contactPointCallbackDelay,
                )?;
            serializer
                .skip_field("constraintsMaster", &self.parent.m_constraintsMaster)?;
            serializer
                .skip_array_meta_field(
                    "constraintsSlave",
                    &self.parent.m_constraintsSlave,
                )?;
            serializer
                .skip_array_meta_field(
                    "constraintRuntime",
                    &self.parent.m_constraintRuntime,
                )?;
            serializer.skip_field("simulationIsland", &self.parent.m_simulationIsland)?;
            serializer
                .serialize_field("autoRemoveLevel", &self.parent.m_autoRemoveLevel)?;
            serializer
                .serialize_field(
                    "numShapeKeysInContactPointProperties",
                    &self.parent.m_numShapeKeysInContactPointProperties,
                )?;
            serializer
                .serialize_field(
                    "responseModifierFlags",
                    &self.parent.m_responseModifierFlags,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("uid", &self.parent.m_uid)?;
            serializer
                .serialize_field(
                    "spuCollisionCallback",
                    &self.parent.m_spuCollisionCallback,
                )?;
            serializer.serialize_field("motion", &self.parent.m_motion)?;
            serializer.skip_field("contactListeners", &self.parent.m_contactListeners)?;
            serializer.skip_field("actions", &self.parent.m_actions)?;
            serializer.serialize_field("localFrame", &self.parent.m_localFrame)?;
            serializer
                .skip_field("extendedListeners", &self.parent.m_extendedListeners)?;
            serializer.serialize_field("npData", &self.parent.m_npData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer
                .serialize_array_field("properties", &self.parent.parent.m_properties)?;
            serializer
                .serialize_array_field(
                    "constraintsSlave",
                    &self.parent.m_constraintsSlave,
                )?;
            serializer
                .serialize_array_field(
                    "constraintRuntime",
                    &self.parent.m_constraintRuntime,
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
    impl<'de> _serde::Deserialize<'de> for hkpRigidBody<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_collidable,
                m_multiThreadCheck,
                m_name,
                m_properties,
                m_material,
                m_damageMultiplier,
                m_storageIndex,
                m_contactPointCallbackDelay,
                m_autoRemoveLevel,
                m_numShapeKeysInContactPointProperties,
                m_responseModifierFlags,
                m_uid,
                m_spuCollisionCallback,
                m_motion,
                m_localFrame,
                m_npData,
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
                        "collidable" => Ok(__Field::m_collidable),
                        "multiThreadCheck" => Ok(__Field::m_multiThreadCheck),
                        "name" => Ok(__Field::m_name),
                        "properties" => Ok(__Field::m_properties),
                        "material" => Ok(__Field::m_material),
                        "damageMultiplier" => Ok(__Field::m_damageMultiplier),
                        "storageIndex" => Ok(__Field::m_storageIndex),
                        "contactPointCallbackDelay" => {
                            Ok(__Field::m_contactPointCallbackDelay)
                        }
                        "autoRemoveLevel" => Ok(__Field::m_autoRemoveLevel),
                        "numShapeKeysInContactPointProperties" => {
                            Ok(__Field::m_numShapeKeysInContactPointProperties)
                        }
                        "responseModifierFlags" => Ok(__Field::m_responseModifierFlags),
                        "uid" => Ok(__Field::m_uid),
                        "spuCollisionCallback" => Ok(__Field::m_spuCollisionCallback),
                        "motion" => Ok(__Field::m_motion),
                        "localFrame" => Ok(__Field::m_localFrame),
                        "npData" => Ok(__Field::m_npData),
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
            struct __hkpRigidBodyVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpRigidBody<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpRigidBodyVisitor<'de> {
                type Value = hkpRigidBody<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkpRigidBody")
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
                    for i in 0..0usize {
                        match i {
                            _ => {}
                        }
                    }
                    _serde::__private::Ok(hkpRigidBody { __ptr, parent })
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
                    let mut m_collidable: _serde::__private::Option<
                        hkpLinkedCollidable,
                    > = _serde::__private::None;
                    let mut m_multiThreadCheck: _serde::__private::Option<
                        hkMultiThreadCheck,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_properties: _serde::__private::Option<Vec<hkpProperty>> = _serde::__private::None;
                    let mut m_material: _serde::__private::Option<hkpMaterial> = _serde::__private::None;
                    let mut m_damageMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_storageIndex: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_contactPointCallbackDelay: _serde::__private::Option<
                        u16,
                    > = _serde::__private::None;
                    let mut m_autoRemoveLevel: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_numShapeKeysInContactPointProperties: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_responseModifierFlags: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_uid: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_spuCollisionCallback: _serde::__private::Option<
                        hkpEntitySpuCollisionCallback,
                    > = _serde::__private::None;
                    let mut m_motion: _serde::__private::Option<hkpMaxSizeMotion> = _serde::__private::None;
                    let mut m_localFrame: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_npData: _serde::__private::Option<u32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
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
                            __Field::m_collidable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_collidable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collidable",
                                        ),
                                    );
                                }
                                m_collidable = _serde::__private::Some(
                                    match __A::next_value::<hkpLinkedCollidable>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_multiThreadCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_multiThreadCheck) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "multiThreadCheck",
                                        ),
                                    );
                                }
                                m_multiThreadCheck = _serde::__private::Some(
                                    match __A::next_value::<hkMultiThreadCheck>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_properties => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_properties) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "properties",
                                        ),
                                    );
                                }
                                m_properties = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkpProperty>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_material => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_material) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "material",
                                        ),
                                    );
                                }
                                m_material = _serde::__private::Some(
                                    match __A::next_value::<hkpMaterial>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_damageMultiplier => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_damageMultiplier) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "damageMultiplier",
                                        ),
                                    );
                                }
                                m_damageMultiplier = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_storageIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_storageIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "storageIndex",
                                        ),
                                    );
                                }
                                m_storageIndex = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_contactPointCallbackDelay => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_contactPointCallbackDelay,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactPointCallbackDelay",
                                        ),
                                    );
                                }
                                m_contactPointCallbackDelay = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_autoRemoveLevel => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_autoRemoveLevel) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "autoRemoveLevel",
                                        ),
                                    );
                                }
                                m_autoRemoveLevel = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numShapeKeysInContactPointProperties => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numShapeKeysInContactPointProperties,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numShapeKeysInContactPointProperties",
                                        ),
                                    );
                                }
                                m_numShapeKeysInContactPointProperties = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_responseModifierFlags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_responseModifierFlags,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "responseModifierFlags",
                                        ),
                                    );
                                }
                                m_responseModifierFlags = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_uid => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_uid) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("uid"),
                                    );
                                }
                                m_uid = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_spuCollisionCallback => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_spuCollisionCallback,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spuCollisionCallback",
                                        ),
                                    );
                                }
                                m_spuCollisionCallback = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpEntitySpuCollisionCallback,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motion) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("motion"),
                                    );
                                }
                                m_motion = _serde::__private::Some(
                                    match __A::next_value::<hkpMaxSizeMotion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_localFrame => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_localFrame) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "localFrame",
                                        ),
                                    );
                                }
                                m_localFrame = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_npData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_npData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("npData"),
                                    );
                                }
                                m_npData = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
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
                    let m_collidable = match m_collidable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collidable",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_multiThreadCheck = match m_multiThreadCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "multiThreadCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_properties = match m_properties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "properties",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_material = match m_material {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("material"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_damageMultiplier = match m_damageMultiplier {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "damageMultiplier",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_storageIndex = match m_storageIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "storageIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_contactPointCallbackDelay = match m_contactPointCallbackDelay {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactPointCallbackDelay",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_autoRemoveLevel = match m_autoRemoveLevel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "autoRemoveLevel",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numShapeKeysInContactPointProperties = match m_numShapeKeysInContactPointProperties {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numShapeKeysInContactPointProperties",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_responseModifierFlags = match m_responseModifierFlags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "responseModifierFlags",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_uid = match m_uid {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("uid"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_spuCollisionCallback = match m_spuCollisionCallback {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "spuCollisionCallback",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motion = match m_motion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motion"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_localFrame = match m_localFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "localFrame",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_npData = match m_npData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("npData"),
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
                    let parent = hkpWorldObject {
                        __ptr,
                        parent,
                        m_userData,
                        m_collidable,
                        m_multiThreadCheck,
                        m_name,
                        m_properties,
                        ..Default::default()
                    };
                    let parent = hkpEntity {
                        __ptr,
                        parent,
                        m_material,
                        m_damageMultiplier,
                        m_storageIndex,
                        m_contactPointCallbackDelay,
                        m_autoRemoveLevel,
                        m_numShapeKeysInContactPointProperties,
                        m_responseModifierFlags,
                        m_uid,
                        m_spuCollisionCallback,
                        m_motion,
                        m_localFrame,
                        m_npData,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpRigidBody { __ptr, parent })
                }
            }
            const FIELDS: &[&str] = &[];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpRigidBody",
                FIELDS,
                __hkpRigidBodyVisitor {
                    marker: _serde::__private::PhantomData::<hkpRigidBody>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
