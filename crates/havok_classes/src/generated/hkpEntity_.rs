use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpEntity`
/// -         version: `3`
/// -       signature: `0xa03c774b`
/// -          size: 544(x86)/720(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntity<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpWorldObject<'a>,
    /// # C++ Info
    /// -          name: `material`(ctype: `struct hkpMaterial`)
    /// -        offset: 140(x86)/208(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_material: hkpMaterial,
    /// # C++ Info
    /// -          name: `limitContactImpulseUtilAndFlag`(ctype: `void*`)
    /// -        offset: 152(x86)/224(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_limitContactImpulseUtilAndFlag: Pointer,
    /// # C++ Info
    /// -          name: `damageMultiplier`(ctype: `hkReal`)
    /// -        offset: 156(x86)/232(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_damageMultiplier: f32,
    /// # C++ Info
    /// -          name: `breakableBody`(ctype: `void*`)
    /// -        offset: 160(x86)/240(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_breakableBody: Pointer,
    /// # C++ Info
    /// -          name: `solverData`(ctype: `hkUint32`)
    /// -        offset: 164(x86)/248(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_solverData: u32,
    /// # C++ Info
    /// -          name: `storageIndex`(ctype: `hkUint16`)
    /// -        offset: 168(x86)/252(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_storageIndex: u16,
    /// # C++ Info
    /// -          name: `contactPointCallbackDelay`(ctype: `hkUint16`)
    /// -        offset: 170(x86)/254(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_contactPointCallbackDelay: u16,
    /// # C++ Info
    /// -          name: `constraintsMaster`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset: 172(x86)/256(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_constraintsMaster: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `constraintsSlave`(ctype: `hkArray<hkpConstraintInstance*>`)
    /// -        offset: 180(x86)/272(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `NOT_OWNED|SERIALIZE_IGNORED`
    ///
    pub m_constraintsSlave: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `constraintRuntime`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 192(x86)/288(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_constraintRuntime: Vec<u8>,
    /// # C++ Info
    /// -          name: `simulationIsland`(ctype: `void*`)
    /// -        offset: 204(x86)/304(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_simulationIsland: Pointer,
    /// # C++ Info
    /// -          name: `autoRemoveLevel`(ctype: `hkInt8`)
    /// -        offset: 208(x86)/312(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_autoRemoveLevel: i8,
    /// # C++ Info
    /// -          name: `numShapeKeysInContactPointProperties`(ctype: `hkUint8`)
    /// -        offset: 209(x86)/313(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_numShapeKeysInContactPointProperties: u8,
    /// # C++ Info
    /// -          name: `responseModifierFlags`(ctype: `hkUint8`)
    /// -        offset: 210(x86)/314(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_responseModifierFlags: u8,
    /// # C++ Info
    /// -          name: `uid`(ctype: `hkUint32`)
    /// -        offset: 212(x86)/316(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_uid: u32,
    /// # C++ Info
    /// -          name: `spuCollisionCallback`(ctype: `struct hkpEntitySpuCollisionCallback`)
    /// -        offset: 216(x86)/320(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_spuCollisionCallback: hkpEntitySpuCollisionCallback,
    /// # C++ Info
    /// -          name: `motion`(ctype: `struct hkpMaxSizeMotion`)
    /// -        offset: 224(x86)/336(x86_64)
    /// -     type_size: 288(x86)/320(x86_64)
    ///
    pub m_motion: hkpMaxSizeMotion,
    /// # C++ Info
    /// -          name: `contactListeners`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset: 512(x86)/656(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_contactListeners: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `actions`(ctype: `struct hkpEntitySmallArraySerializeOverrideType`)
    /// -        offset: 520(x86)/672(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_actions: hkpEntitySmallArraySerializeOverrideType,
    /// # C++ Info
    /// -          name: `localFrame`(ctype: `struct hkLocalFrame*`)
    /// -        offset: 528(x86)/688(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_localFrame: Pointer,
    /// # C++ Info
    /// -          name: `extendedListeners`(ctype: `struct hkpEntityExtendedListeners*`)
    /// -        offset: 532(x86)/696(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_extendedListeners: Pointer,
    /// # C++ Info
    /// -          name: `npData`(ctype: `hkUint32`)
    /// -        offset: 536(x86)/704(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_npData: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpEntity<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpEntity"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa03c774b)
        }
    }
    impl<'a> _serde::Serialize for hkpEntity<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa03c774b)));
            let mut serializer = __serializer.serialize_struct("hkpEntity", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.m_world)?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("collidable", &self.parent.m_collidable)?;
            serializer
                .serialize_field("multiThreadCheck", &self.parent.m_multiThreadCheck)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer
                .serialize_array_meta_field("properties", &self.parent.m_properties)?;
            serializer.skip_field("treeData", &self.parent.m_treeData)?;
            serializer.serialize_field("material", &self.m_material)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "limitContactImpulseUtilAndFlag",
                    &self.m_limitContactImpulseUtilAndFlag,
                )?;
            serializer.serialize_field("damageMultiplier", &self.m_damageMultiplier)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("breakableBody", &self.m_breakableBody)?;
            serializer.skip_field("solverData", &self.m_solverData)?;
            serializer.serialize_field("storageIndex", &self.m_storageIndex)?;
            serializer
                .serialize_field(
                    "contactPointCallbackDelay",
                    &self.m_contactPointCallbackDelay,
                )?;
            serializer.skip_field("constraintsMaster", &self.m_constraintsMaster)?;
            serializer
                .skip_array_meta_field("constraintsSlave", &self.m_constraintsSlave)?;
            serializer
                .skip_array_meta_field("constraintRuntime", &self.m_constraintRuntime)?;
            serializer.skip_field("simulationIsland", &self.m_simulationIsland)?;
            serializer.serialize_field("autoRemoveLevel", &self.m_autoRemoveLevel)?;
            serializer
                .serialize_field(
                    "numShapeKeysInContactPointProperties",
                    &self.m_numShapeKeysInContactPointProperties,
                )?;
            serializer
                .serialize_field(
                    "responseModifierFlags",
                    &self.m_responseModifierFlags,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("uid", &self.m_uid)?;
            serializer
                .serialize_field("spuCollisionCallback", &self.m_spuCollisionCallback)?;
            serializer.serialize_field("motion", &self.m_motion)?;
            serializer.skip_field("contactListeners", &self.m_contactListeners)?;
            serializer.skip_field("actions", &self.m_actions)?;
            serializer.serialize_field("localFrame", &self.m_localFrame)?;
            serializer.skip_field("extendedListeners", &self.m_extendedListeners)?;
            serializer.serialize_field("npData", &self.m_npData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("properties", &self.parent.m_properties)?;
            serializer
                .serialize_array_field("constraintsSlave", &self.m_constraintsSlave)?;
            serializer
                .serialize_array_field("constraintRuntime", &self.m_constraintRuntime)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_material,
    m_limitContactImpulseUtilAndFlag,
    m_damageMultiplier,
    m_breakableBody,
    m_solverData,
    m_storageIndex,
    m_contactPointCallbackDelay,
    m_constraintsMaster,
    m_constraintsSlave,
    m_constraintRuntime,
    m_simulationIsland,
    m_autoRemoveLevel,
    m_numShapeKeysInContactPointProperties,
    m_responseModifierFlags,
    m_uid,
    m_spuCollisionCallback,
    m_motion,
    m_contactListeners,
    m_actions,
    m_localFrame,
    m_extendedListeners,
    m_npData,
    __ignore,
}
struct __FieldVisitor;
impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
    type Value = __Field;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "field identifier")
    }
    /// Intended for use in XML.
    #[allow(clippy::match_single_binding)]
    #[allow(clippy::reversed_empty_ranges)]
    #[allow(clippy::single_match)]
    fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
    where
        __E: _serde::de::Error,
    {
        match __value {
            "material" => Ok(__Field::m_material),
            "damageMultiplier" => Ok(__Field::m_damageMultiplier),
            "storageIndex" => Ok(__Field::m_storageIndex),
            "contactPointCallbackDelay" => Ok(__Field::m_contactPointCallbackDelay),
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
    fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
    }
}
pub(super) struct __hkpEntityVisitor<'de> {
    marker: core::marker::PhantomData<hkpEntity<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpEntityVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpEntity<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpEntity<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpEntityVisitor<'de> {
    type Value = hkpEntity<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpEntity")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_material: _serde::__private::Option<hkpMaterial> = _serde::__private::None;
        let mut m_limitContactImpulseUtilAndFlag: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_damageMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_breakableBody: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_solverData: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_storageIndex: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_contactPointCallbackDelay: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_constraintsMaster: _serde::__private::Option<
            hkpEntitySmallArraySerializeOverrideType,
        > = _serde::__private::None;
        let mut m_constraintsSlave: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_constraintRuntime: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
        let mut m_simulationIsland: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_autoRemoveLevel: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_numShapeKeysInContactPointProperties: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_responseModifierFlags: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_uid: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_spuCollisionCallback: _serde::__private::Option<
            hkpEntitySpuCollisionCallback,
        > = _serde::__private::None;
        let mut m_motion: _serde::__private::Option<hkpMaxSizeMotion> = _serde::__private::None;
        let mut m_contactListeners: _serde::__private::Option<
            hkpEntitySmallArraySerializeOverrideType,
        > = _serde::__private::None;
        let mut m_actions: _serde::__private::Option<
            hkpEntitySmallArraySerializeOverrideType,
        > = _serde::__private::None;
        let mut m_localFrame: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_extendedListeners: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_npData: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..22usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_material) {
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
                1usize => {
                    if _serde::__private::Option::is_some(
                        &m_limitContactImpulseUtilAndFlag,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "limitContactImpulseUtilAndFlag",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_limitContactImpulseUtilAndFlag = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_damageMultiplier) {
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
                3usize => {
                    if _serde::__private::Option::is_some(&m_breakableBody) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "breakableBody",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_breakableBody = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_solverData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "solverData",
                            ),
                        );
                    }
                    m_solverData = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_storageIndex) {
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
                6usize => {
                    if _serde::__private::Option::is_some(&m_contactPointCallbackDelay) {
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
                7usize => {
                    if _serde::__private::Option::is_some(&m_constraintsMaster) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "constraintsMaster",
                            ),
                        );
                    }
                    m_constraintsMaster = _serde::__private::Some(
                        match __A::next_value::<
                            hkpEntitySmallArraySerializeOverrideType,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_constraintsSlave) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "constraintsSlave",
                            ),
                        );
                    }
                    m_constraintsSlave = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_constraintRuntime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "constraintRuntime",
                            ),
                        );
                    }
                    m_constraintRuntime = _serde::__private::Some(
                        match __A::next_value::<Vec<u8>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_simulationIsland) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "simulationIsland",
                            ),
                        );
                    }
                    m_simulationIsland = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_autoRemoveLevel) {
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
                12usize => {
                    if _serde::__private::Option::is_some(
                        &m_numShapeKeysInContactPointProperties,
                    ) {
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
                13usize => {
                    if _serde::__private::Option::is_some(&m_responseModifierFlags) {
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
                14usize => {
                    if _serde::__private::Option::is_some(&m_uid) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("uid"),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_uid = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                15usize => {
                    if _serde::__private::Option::is_some(&m_spuCollisionCallback) {
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
                16usize => {
                    if _serde::__private::Option::is_some(&m_motion) {
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
                17usize => {
                    if _serde::__private::Option::is_some(&m_contactListeners) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "contactListeners",
                            ),
                        );
                    }
                    m_contactListeners = _serde::__private::Some(
                        match __A::next_value::<
                            hkpEntitySmallArraySerializeOverrideType,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                18usize => {
                    if _serde::__private::Option::is_some(&m_actions) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("actions"),
                        );
                    }
                    m_actions = _serde::__private::Some(
                        match __A::next_value::<
                            hkpEntitySmallArraySerializeOverrideType,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                19usize => {
                    if _serde::__private::Option::is_some(&m_localFrame) {
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
                20usize => {
                    if _serde::__private::Option::is_some(&m_extendedListeners) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "extendedListeners",
                            ),
                        );
                    }
                    m_extendedListeners = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                21usize => {
                    if _serde::__private::Option::is_some(&m_npData) {
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
                _ => {}
            }
        }
        __A::pad(&mut __map, 4usize, 12usize)?;
        let m_material = match m_material {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("material"),
                );
            }
        };
        let m_limitContactImpulseUtilAndFlag = match m_limitContactImpulseUtilAndFlag {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "limitContactImpulseUtilAndFlag",
                    ),
                );
            }
        };
        let m_damageMultiplier = match m_damageMultiplier {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("damageMultiplier"),
                );
            }
        };
        let m_breakableBody = match m_breakableBody {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("breakableBody"),
                );
            }
        };
        let m_solverData = match m_solverData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("solverData"),
                );
            }
        };
        let m_storageIndex = match m_storageIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("storageIndex"),
                );
            }
        };
        let m_contactPointCallbackDelay = match m_contactPointCallbackDelay {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contactPointCallbackDelay",
                    ),
                );
            }
        };
        let m_constraintsMaster = match m_constraintsMaster {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("constraintsMaster"),
                );
            }
        };
        let m_constraintsSlave = match m_constraintsSlave {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("constraintsSlave"),
                );
            }
        };
        let m_constraintRuntime = match m_constraintRuntime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("constraintRuntime"),
                );
            }
        };
        let m_simulationIsland = match m_simulationIsland {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("simulationIsland"),
                );
            }
        };
        let m_autoRemoveLevel = match m_autoRemoveLevel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("autoRemoveLevel"),
                );
            }
        };
        let m_numShapeKeysInContactPointProperties = match m_numShapeKeysInContactPointProperties {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numShapeKeysInContactPointProperties",
                    ),
                );
            }
        };
        let m_responseModifierFlags = match m_responseModifierFlags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "responseModifierFlags",
                    ),
                );
            }
        };
        let m_uid = match m_uid {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uid"),
                );
            }
        };
        let m_spuCollisionCallback = match m_spuCollisionCallback {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "spuCollisionCallback",
                    ),
                );
            }
        };
        let m_motion = match m_motion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motion"),
                );
            }
        };
        let m_contactListeners = match m_contactListeners {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("contactListeners"),
                );
            }
        };
        let m_actions = match m_actions {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("actions"),
                );
            }
        };
        let m_localFrame = match m_localFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localFrame"),
                );
            }
        };
        let m_extendedListeners = match m_extendedListeners {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("extendedListeners"),
                );
            }
        };
        let m_npData = match m_npData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("npData"),
                );
            }
        };
        _serde::__private::Ok(hkpEntity {
            __ptr,
            parent,
            m_material,
            m_limitContactImpulseUtilAndFlag,
            m_damageMultiplier,
            m_breakableBody,
            m_solverData,
            m_storageIndex,
            m_contactPointCallbackDelay,
            m_constraintsMaster,
            m_constraintsSlave,
            m_constraintRuntime,
            m_simulationIsland,
            m_autoRemoveLevel,
            m_numShapeKeysInContactPointProperties,
            m_responseModifierFlags,
            m_uid,
            m_spuCollisionCallback,
            m_motion,
            m_contactListeners,
            m_actions,
            m_localFrame,
            m_extendedListeners,
            m_npData,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpWorldObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_material: _serde::__private::Option<hkpMaterial> = _serde::__private::None;
        let mut m_damageMultiplier: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_storageIndex: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_contactPointCallbackDelay: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_autoRemoveLevel: _serde::__private::Option<i8> = _serde::__private::None;
        let mut m_numShapeKeysInContactPointProperties: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_responseModifierFlags: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_uid: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_spuCollisionCallback: _serde::__private::Option<
            hkpEntitySpuCollisionCallback,
        > = _serde::__private::None;
        let mut m_motion: _serde::__private::Option<hkpMaxSizeMotion> = _serde::__private::None;
        let mut m_localFrame: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_npData: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..12usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_material => {
                        if _serde::__private::Option::is_some(&m_material) {
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
                        if _serde::__private::Option::is_some(&m_damageMultiplier) {
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
                        if _serde::__private::Option::is_some(&m_storageIndex) {
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
                        if _serde::__private::Option::is_some(
                            &m_contactPointCallbackDelay,
                        ) {
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
                        if _serde::__private::Option::is_some(&m_autoRemoveLevel) {
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
                        if _serde::__private::Option::is_some(
                            &m_numShapeKeysInContactPointProperties,
                        ) {
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
                        if _serde::__private::Option::is_some(&m_responseModifierFlags) {
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
                        if _serde::__private::Option::is_some(&m_uid) {
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
                        if _serde::__private::Option::is_some(&m_spuCollisionCallback) {
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
                        if _serde::__private::Option::is_some(&m_motion) {
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
                        if _serde::__private::Option::is_some(&m_localFrame) {
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
                        if _serde::__private::Option::is_some(&m_npData) {
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
                    _ => {}
                }
            }
        }
        let m_material = match m_material {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("material"),
                );
            }
        };
        let m_damageMultiplier = match m_damageMultiplier {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("damageMultiplier"),
                );
            }
        };
        let m_storageIndex = match m_storageIndex {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("storageIndex"),
                );
            }
        };
        let m_contactPointCallbackDelay = match m_contactPointCallbackDelay {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "contactPointCallbackDelay",
                    ),
                );
            }
        };
        let m_autoRemoveLevel = match m_autoRemoveLevel {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("autoRemoveLevel"),
                );
            }
        };
        let m_numShapeKeysInContactPointProperties = match m_numShapeKeysInContactPointProperties {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "numShapeKeysInContactPointProperties",
                    ),
                );
            }
        };
        let m_responseModifierFlags = match m_responseModifierFlags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "responseModifierFlags",
                    ),
                );
            }
        };
        let m_uid = match m_uid {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("uid"),
                );
            }
        };
        let m_spuCollisionCallback = match m_spuCollisionCallback {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "spuCollisionCallback",
                    ),
                );
            }
        };
        let m_motion = match m_motion {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motion"),
                );
            }
        };
        let m_localFrame = match m_localFrame {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("localFrame"),
                );
            }
        };
        let m_npData = match m_npData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("npData"),
                );
            }
        };
        _serde::__private::Ok(hkpEntity {
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
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpEntity<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "material",
                "limitContactImpulseUtilAndFlag",
                "damageMultiplier",
                "breakableBody",
                "solverData",
                "storageIndex",
                "contactPointCallbackDelay",
                "constraintsMaster",
                "constraintsSlave",
                "constraintRuntime",
                "simulationIsland",
                "autoRemoveLevel",
                "numShapeKeysInContactPointProperties",
                "responseModifierFlags",
                "uid",
                "spuCollisionCallback",
                "motion",
                "contactListeners",
                "actions",
                "localFrame",
                "extendedListeners",
                "npData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpEntity",
                FIELDS,
                __hkpEntityVisitor {
                    marker: _serde::__private::PhantomData::<hkpEntity>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
