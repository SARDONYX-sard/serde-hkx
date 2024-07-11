use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCharacterControllerModifier`
/// -         version: `0`
/// -       signature: `0xf675d6fb`
/// -          size: 144(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterControllerModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `controlData`(ctype: `struct hkbCharacterControllerControlData`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  32(x86)/ 32(x86_64)
    ///
    pub m_controlData: hkbCharacterControllerControlData,
    /// # C++ Info
    /// -          name: `initialVelocity`(ctype: `hkVector4`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_initialVelocity: Vector4,
    /// # C++ Info
    /// -          name: `initialVelocityCoordinates`(ctype: `enum InitialVelocityCoordinates`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_initialVelocityCoordinates: InitialVelocityCoordinates,
    /// # C++ Info
    /// -          name: `motionMode`(ctype: `enum MotionMode`)
    /// -        offset:  97(x86)/129(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_motionMode: MotionMode,
    /// # C++ Info
    /// -          name: `forceDownwardMomentum`(ctype: `hkBool`)
    /// -        offset:  98(x86)/130(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_forceDownwardMomentum: bool,
    /// # C++ Info
    /// -          name: `applyGravity`(ctype: `hkBool`)
    /// -        offset:  99(x86)/131(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_applyGravity: bool,
    /// # C++ Info
    /// -          name: `setInitialVelocity`(ctype: `hkBool`)
    /// -        offset: 100(x86)/132(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_setInitialVelocity: bool,
    /// # C++ Info
    /// -          name: `isTouchingGround`(ctype: `hkBool`)
    /// -        offset: 101(x86)/133(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isTouchingGround: bool,
    /// # C++ Info
    /// -          name: `gravity`(ctype: `hkVector4`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_gravity: Vector4,
    /// # C++ Info
    /// -          name: `timestep`(ctype: `hkReal`)
    /// -        offset: 128(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_timestep: f32,
    /// # C++ Info
    /// -          name: `isInitialVelocityAdded`(ctype: `hkBool`)
    /// -        offset: 132(x86)/164(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_isInitialVelocityAdded: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterControllerModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterControllerModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf675d6fb)
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterControllerModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf675d6fb)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterControllerModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_field("initialVelocity", &self.m_initialVelocity)?;
            serializer
                .serialize_field(
                    "initialVelocityCoordinates",
                    &self.m_initialVelocityCoordinates,
                )?;
            serializer.serialize_field("motionMode", &self.m_motionMode)?;
            serializer
                .serialize_field(
                    "forceDownwardMomentum",
                    &self.m_forceDownwardMomentum,
                )?;
            serializer.serialize_field("applyGravity", &self.m_applyGravity)?;
            serializer
                .serialize_field("setInitialVelocity", &self.m_setInitialVelocity)?;
            serializer.serialize_field("isTouchingGround", &self.m_isTouchingGround)?;
            serializer.pad_field([0u8; 10usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.skip_field("gravity", &self.m_gravity)?;
            serializer.skip_field("timestep", &self.m_timestep)?;
            serializer
                .skip_field("isInitialVelocityAdded", &self.m_isInitialVelocityAdded)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_controlData,
    m_initialVelocity,
    m_initialVelocityCoordinates,
    m_motionMode,
    m_forceDownwardMomentum,
    m_applyGravity,
    m_setInitialVelocity,
    m_isTouchingGround,
    m_gravity,
    m_timestep,
    m_isInitialVelocityAdded,
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
            "controlData" => Ok(__Field::m_controlData),
            "initialVelocity" => Ok(__Field::m_initialVelocity),
            "initialVelocityCoordinates" => Ok(__Field::m_initialVelocityCoordinates),
            "motionMode" => Ok(__Field::m_motionMode),
            "forceDownwardMomentum" => Ok(__Field::m_forceDownwardMomentum),
            "applyGravity" => Ok(__Field::m_applyGravity),
            "setInitialVelocity" => Ok(__Field::m_setInitialVelocity),
            "isTouchingGround" => Ok(__Field::m_isTouchingGround),
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
pub(super) struct __hkbCharacterControllerModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbCharacterControllerModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbCharacterControllerModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbCharacterControllerModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbCharacterControllerModifier<'de>,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbCharacterControllerModifierVisitor<'de> {
    type Value = hkbCharacterControllerModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbCharacterControllerModifier",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_controlData: _serde::__private::Option<
            hkbCharacterControllerControlData,
        > = _serde::__private::None;
        let mut m_initialVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_initialVelocityCoordinates: _serde::__private::Option<
            InitialVelocityCoordinates,
        > = _serde::__private::None;
        let mut m_motionMode: _serde::__private::Option<MotionMode> = _serde::__private::None;
        let mut m_forceDownwardMomentum: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_applyGravity: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_setInitialVelocity: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isTouchingGround: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_gravity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_timestep: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_isInitialVelocityAdded: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..11usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_controlData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "controlData",
                            ),
                        );
                    }
                    m_controlData = _serde::__private::Some(
                        match __A::next_value::<
                            hkbCharacterControllerControlData,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_initialVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initialVelocity",
                            ),
                        );
                    }
                    m_initialVelocity = _serde::__private::Some(
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
                        &m_initialVelocityCoordinates,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "initialVelocityCoordinates",
                            ),
                        );
                    }
                    m_initialVelocityCoordinates = _serde::__private::Some(
                        match __A::next_value::<InitialVelocityCoordinates>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_motionMode) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "motionMode",
                            ),
                        );
                    }
                    m_motionMode = _serde::__private::Some(
                        match __A::next_value::<MotionMode>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_forceDownwardMomentum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "forceDownwardMomentum",
                            ),
                        );
                    }
                    m_forceDownwardMomentum = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_applyGravity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "applyGravity",
                            ),
                        );
                    }
                    m_applyGravity = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_setInitialVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "setInitialVelocity",
                            ),
                        );
                    }
                    m_setInitialVelocity = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_isTouchingGround) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isTouchingGround",
                            ),
                        );
                    }
                    m_isTouchingGround = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_gravity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("gravity"),
                        );
                    }
                    __A::pad(&mut __map, 10usize, 10usize)?;
                    m_gravity = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_timestep) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "timestep",
                            ),
                        );
                    }
                    m_timestep = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_isInitialVelocityAdded) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "isInitialVelocityAdded",
                            ),
                        );
                    }
                    m_isInitialVelocityAdded = _serde::__private::Some(
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
        __A::pad(&mut __map, 11usize, 11usize)?;
        let m_controlData = match m_controlData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("controlData"),
                );
            }
        };
        let m_initialVelocity = match m_initialVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initialVelocity"),
                );
            }
        };
        let m_initialVelocityCoordinates = match m_initialVelocityCoordinates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "initialVelocityCoordinates",
                    ),
                );
            }
        };
        let m_motionMode = match m_motionMode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motionMode"),
                );
            }
        };
        let m_forceDownwardMomentum = match m_forceDownwardMomentum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "forceDownwardMomentum",
                    ),
                );
            }
        };
        let m_applyGravity = match m_applyGravity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("applyGravity"),
                );
            }
        };
        let m_setInitialVelocity = match m_setInitialVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "setInitialVelocity",
                    ),
                );
            }
        };
        let m_isTouchingGround = match m_isTouchingGround {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isTouchingGround"),
                );
            }
        };
        let m_gravity = match m_gravity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("gravity"),
                );
            }
        };
        let m_timestep = match m_timestep {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("timestep"),
                );
            }
        };
        let m_isInitialVelocityAdded = match m_isInitialVelocityAdded {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "isInitialVelocityAdded",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterControllerModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_controlData,
            m_initialVelocity,
            m_initialVelocityCoordinates,
            m_motionMode,
            m_forceDownwardMomentum,
            m_applyGravity,
            m_setInitialVelocity,
            m_isTouchingGround,
            m_gravity,
            m_timestep,
            m_isInitialVelocityAdded,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_controlData: _serde::__private::Option<
            hkbCharacterControllerControlData,
        > = _serde::__private::None;
        let mut m_initialVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_initialVelocityCoordinates: _serde::__private::Option<
            InitialVelocityCoordinates,
        > = _serde::__private::None;
        let mut m_motionMode: _serde::__private::Option<MotionMode> = _serde::__private::None;
        let mut m_forceDownwardMomentum: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_applyGravity: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_setInitialVelocity: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_isTouchingGround: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_controlData => {
                        if _serde::__private::Option::is_some(&m_controlData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "controlData",
                                ),
                            );
                        }
                        m_controlData = _serde::__private::Some(
                            match __A::next_value::<
                                hkbCharacterControllerControlData,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_initialVelocity => {
                        if _serde::__private::Option::is_some(&m_initialVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "initialVelocity",
                                ),
                            );
                        }
                        m_initialVelocity = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_initialVelocityCoordinates => {
                        if _serde::__private::Option::is_some(
                            &m_initialVelocityCoordinates,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "initialVelocityCoordinates",
                                ),
                            );
                        }
                        m_initialVelocityCoordinates = _serde::__private::Some(
                            match __A::next_value::<
                                InitialVelocityCoordinates,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_motionMode => {
                        if _serde::__private::Option::is_some(&m_motionMode) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "motionMode",
                                ),
                            );
                        }
                        m_motionMode = _serde::__private::Some(
                            match __A::next_value::<MotionMode>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_forceDownwardMomentum => {
                        if _serde::__private::Option::is_some(&m_forceDownwardMomentum) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "forceDownwardMomentum",
                                ),
                            );
                        }
                        m_forceDownwardMomentum = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_applyGravity => {
                        if _serde::__private::Option::is_some(&m_applyGravity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "applyGravity",
                                ),
                            );
                        }
                        m_applyGravity = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_setInitialVelocity => {
                        if _serde::__private::Option::is_some(&m_setInitialVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "setInitialVelocity",
                                ),
                            );
                        }
                        m_setInitialVelocity = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_isTouchingGround => {
                        if _serde::__private::Option::is_some(&m_isTouchingGround) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "isTouchingGround",
                                ),
                            );
                        }
                        m_isTouchingGround = _serde::__private::Some(
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
        }
        let m_controlData = match m_controlData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("controlData"),
                );
            }
        };
        let m_initialVelocity = match m_initialVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("initialVelocity"),
                );
            }
        };
        let m_initialVelocityCoordinates = match m_initialVelocityCoordinates {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "initialVelocityCoordinates",
                    ),
                );
            }
        };
        let m_motionMode = match m_motionMode {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motionMode"),
                );
            }
        };
        let m_forceDownwardMomentum = match m_forceDownwardMomentum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "forceDownwardMomentum",
                    ),
                );
            }
        };
        let m_applyGravity = match m_applyGravity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("applyGravity"),
                );
            }
        };
        let m_setInitialVelocity = match m_setInitialVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "setInitialVelocity",
                    ),
                );
            }
        };
        let m_isTouchingGround = match m_isTouchingGround {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("isTouchingGround"),
                );
            }
        };
        _serde::__private::Ok(hkbCharacterControllerModifier {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_controlData,
            m_initialVelocity,
            m_initialVelocityCoordinates,
            m_motionMode,
            m_forceDownwardMomentum,
            m_applyGravity,
            m_setInitialVelocity,
            m_isTouchingGround,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterControllerModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "controlData",
                "initialVelocity",
                "initialVelocityCoordinates",
                "motionMode",
                "forceDownwardMomentum",
                "applyGravity",
                "setInitialVelocity",
                "isTouchingGround",
                "gravity",
                "timestep",
                "isInitialVelocityAdded",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterControllerModifier",
                FIELDS,
                __hkbCharacterControllerModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCharacterControllerModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum InitialVelocityCoordinates {
    #[default]
    INITIAL_VELOCITY_IN_WORLD_COORDINATES = 0isize,
    INITIAL_VELOCITY_IN_MODEL_COORDINATES = 1isize,
}
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum MotionMode {
    #[default]
    MOTION_MODE_FOLLOW_ANIMATION = 0isize,
    MOTION_MODE_DYNAMIC = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for InitialVelocityCoordinates {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INITIAL_VELOCITY_IN_WORLD_COORDINATES => {
                    __serializer
                        .serialize_field("INITIAL_VELOCITY_IN_WORLD_COORDINATES", &0u64)
                }
                Self::INITIAL_VELOCITY_IN_MODEL_COORDINATES => {
                    __serializer
                        .serialize_field("INITIAL_VELOCITY_IN_MODEL_COORDINATES", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(
                    S::Error::custom("Failed enum InitialVelocityCoordinates to_i8"),
                )?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MotionMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MOTION_MODE_FOLLOW_ANIMATION => {
                    __serializer.serialize_field("MOTION_MODE_FOLLOW_ANIMATION", &0u64)
                }
                Self::MOTION_MODE_DYNAMIC => {
                    __serializer.serialize_field("MOTION_MODE_DYNAMIC", &1u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum MotionMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for InitialVelocityCoordinates {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v
                                    .eq_ignore_ascii_case(
                                        "INITIAL_VELOCITY_IN_WORLD_COORDINATES",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "INITIAL_VELOCITY_IN_MODEL_COORDINATES",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<InitialVelocityCoordinates>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = InitialVelocityCoordinates;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum InitialVelocityCoordinates",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                InitialVelocityCoordinates::INITIAL_VELOCITY_IN_WORLD_COORDINATES,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                InitialVelocityCoordinates::INITIAL_VELOCITY_IN_MODEL_COORDINATES,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "INITIAL_VELOCITY_IN_WORLD_COORDINATES",
                "INITIAL_VELOCITY_IN_MODEL_COORDINATES",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "InitialVelocityCoordinates",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<InitialVelocityCoordinates>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for MotionMode {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0"
                                || v
                                    .eq_ignore_ascii_case("MOTION_MODE_FOLLOW_ANIMATION") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("MOTION_MODE_DYNAMIC") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MotionMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MotionMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MotionMode",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MotionMode::MOTION_MODE_FOLLOW_ANIMATION,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionMode::MOTION_MODE_DYNAMIC)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MOTION_MODE_FOLLOW_ANIMATION",
                "MOTION_MODE_DYNAMIC",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MotionMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MotionMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
