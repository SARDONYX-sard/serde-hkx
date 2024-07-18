use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpGravityGun`
/// - version: `0`
/// - signature: `0x5e2754cd`
/// - size: ` 96`(x86)/`128`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGravityGun<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpFirstPersonGun<'a>,
    /// # C++ Info
    /// - name: `grabbedBodies`(ctype: `hkArray<void*>`)
    /// - offset: ` 32`(x86)/` 56`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_grabbedBodies: Vec<Pointer>,
    /// # C++ Info
    /// - name: `maxNumObjectsPicked`(ctype: `hkInt32`)
    /// - offset: ` 44`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxNumObjectsPicked: i32,
    /// # C++ Info
    /// - name: `maxMassOfObjectPicked`(ctype: `hkReal`)
    /// - offset: ` 48`(x86)/` 76`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxMassOfObjectPicked: f32,
    /// # C++ Info
    /// - name: `maxDistOfObjectPicked`(ctype: `hkReal`)
    /// - offset: ` 52`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxDistOfObjectPicked: f32,
    /// # C++ Info
    /// - name: `impulseAppliedWhenObjectNotPicked`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_impulseAppliedWhenObjectNotPicked: f32,
    /// # C++ Info
    /// - name: `throwVelocity`(ctype: `hkReal`)
    /// - offset: ` 60`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_throwVelocity: f32,
    /// # C++ Info
    /// - name: `capturedObjectPosition`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_capturedObjectPosition: Vector4,
    /// # C++ Info
    /// - name: `capturedObjectsOffset`(ctype: `hkVector4`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_capturedObjectsOffset: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpGravityGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGravityGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5e2754cd)
        }
    }
    impl<'a> _serde::Serialize for hkpGravityGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5e2754cd)));
            let mut serializer = __serializer
                .serialize_struct("hkpGravityGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("keyboardKey", &self.parent.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("listeners", &self.parent.m_listeners)?;
            serializer.skip_array_meta_field("grabbedBodies", &self.m_grabbedBodies)?;
            serializer
                .serialize_field("maxNumObjectsPicked", &self.m_maxNumObjectsPicked)?;
            serializer
                .serialize_field(
                    "maxMassOfObjectPicked",
                    &self.m_maxMassOfObjectPicked,
                )?;
            serializer
                .serialize_field(
                    "maxDistOfObjectPicked",
                    &self.m_maxDistOfObjectPicked,
                )?;
            serializer
                .serialize_field(
                    "impulseAppliedWhenObjectNotPicked",
                    &self.m_impulseAppliedWhenObjectNotPicked,
                )?;
            serializer.serialize_field("throwVelocity", &self.m_throwVelocity)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "capturedObjectPosition",
                    &self.m_capturedObjectPosition,
                )?;
            serializer
                .serialize_field(
                    "capturedObjectsOffset",
                    &self.m_capturedObjectsOffset,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_array_field("grabbedBodies", &self.m_grabbedBodies)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_grabbedBodies,
    m_maxNumObjectsPicked,
    m_maxMassOfObjectPicked,
    m_maxDistOfObjectPicked,
    m_impulseAppliedWhenObjectNotPicked,
    m_throwVelocity,
    m_capturedObjectPosition,
    m_capturedObjectsOffset,
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
            "maxNumObjectsPicked" => Ok(__Field::m_maxNumObjectsPicked),
            "maxMassOfObjectPicked" => Ok(__Field::m_maxMassOfObjectPicked),
            "maxDistOfObjectPicked" => Ok(__Field::m_maxDistOfObjectPicked),
            "impulseAppliedWhenObjectNotPicked" => {
                Ok(__Field::m_impulseAppliedWhenObjectNotPicked)
            }
            "throwVelocity" => Ok(__Field::m_throwVelocity),
            "capturedObjectPosition" => Ok(__Field::m_capturedObjectPosition),
            "capturedObjectsOffset" => Ok(__Field::m_capturedObjectsOffset),
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
pub(super) struct __hkpGravityGunVisitor<'de> {
    marker: core::marker::PhantomData<hkpGravityGun<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpGravityGunVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpGravityGun<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpGravityGun<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpGravityGunVisitor<'de> {
    type Value = hkpGravityGun<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpGravityGun")
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
        let mut m_grabbedBodies: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_maxNumObjectsPicked: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_maxMassOfObjectPicked: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxDistOfObjectPicked: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_impulseAppliedWhenObjectNotPicked: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_throwVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_capturedObjectPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_capturedObjectsOffset: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_grabbedBodies) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "grabbedBodies",
                            ),
                        );
                    }
                    m_grabbedBodies = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_maxNumObjectsPicked) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxNumObjectsPicked",
                            ),
                        );
                    }
                    m_maxNumObjectsPicked = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_maxMassOfObjectPicked) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxMassOfObjectPicked",
                            ),
                        );
                    }
                    m_maxMassOfObjectPicked = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_maxDistOfObjectPicked) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxDistOfObjectPicked",
                            ),
                        );
                    }
                    m_maxDistOfObjectPicked = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(
                        &m_impulseAppliedWhenObjectNotPicked,
                    ) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "impulseAppliedWhenObjectNotPicked",
                            ),
                        );
                    }
                    m_impulseAppliedWhenObjectNotPicked = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_throwVelocity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "throwVelocity",
                            ),
                        );
                    }
                    m_throwVelocity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_capturedObjectPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "capturedObjectPosition",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_capturedObjectPosition = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_capturedObjectsOffset) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "capturedObjectsOffset",
                            ),
                        );
                    }
                    m_capturedObjectsOffset = _serde::__private::Some(
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
        let m_grabbedBodies = match m_grabbedBodies {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("grabbedBodies"),
                );
            }
        };
        let m_maxNumObjectsPicked = match m_maxNumObjectsPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxNumObjectsPicked",
                    ),
                );
            }
        };
        let m_maxMassOfObjectPicked = match m_maxMassOfObjectPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxMassOfObjectPicked",
                    ),
                );
            }
        };
        let m_maxDistOfObjectPicked = match m_maxDistOfObjectPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxDistOfObjectPicked",
                    ),
                );
            }
        };
        let m_impulseAppliedWhenObjectNotPicked = match m_impulseAppliedWhenObjectNotPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "impulseAppliedWhenObjectNotPicked",
                    ),
                );
            }
        };
        let m_throwVelocity = match m_throwVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("throwVelocity"),
                );
            }
        };
        let m_capturedObjectPosition = match m_capturedObjectPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "capturedObjectPosition",
                    ),
                );
            }
        };
        let m_capturedObjectsOffset = match m_capturedObjectsOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "capturedObjectsOffset",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpGravityGun {
            __ptr,
            parent,
            m_grabbedBodies,
            m_maxNumObjectsPicked,
            m_maxMassOfObjectPicked,
            m_maxDistOfObjectPicked,
            m_impulseAppliedWhenObjectNotPicked,
            m_throwVelocity,
            m_capturedObjectPosition,
            m_capturedObjectsOffset,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpFirstPersonGunVisitor::visit_as_parent(&mut __map)?;
        let mut m_maxNumObjectsPicked: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_maxMassOfObjectPicked: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxDistOfObjectPicked: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_impulseAppliedWhenObjectNotPicked: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_throwVelocity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_capturedObjectPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_capturedObjectsOffset: _serde::__private::Option<Vector4> = _serde::__private::None;
        for _ in 0..7usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_maxNumObjectsPicked => {
                        if _serde::__private::Option::is_some(&m_maxNumObjectsPicked) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxNumObjectsPicked",
                                ),
                            );
                        }
                        m_maxNumObjectsPicked = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_maxMassOfObjectPicked => {
                        if _serde::__private::Option::is_some(&m_maxMassOfObjectPicked) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxMassOfObjectPicked",
                                ),
                            );
                        }
                        m_maxMassOfObjectPicked = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_maxDistOfObjectPicked => {
                        if _serde::__private::Option::is_some(&m_maxDistOfObjectPicked) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxDistOfObjectPicked",
                                ),
                            );
                        }
                        m_maxDistOfObjectPicked = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_impulseAppliedWhenObjectNotPicked => {
                        if _serde::__private::Option::is_some(
                            &m_impulseAppliedWhenObjectNotPicked,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "impulseAppliedWhenObjectNotPicked",
                                ),
                            );
                        }
                        m_impulseAppliedWhenObjectNotPicked = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_throwVelocity => {
                        if _serde::__private::Option::is_some(&m_throwVelocity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "throwVelocity",
                                ),
                            );
                        }
                        m_throwVelocity = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_capturedObjectPosition => {
                        if _serde::__private::Option::is_some(
                            &m_capturedObjectPosition,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "capturedObjectPosition",
                                ),
                            );
                        }
                        m_capturedObjectPosition = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_capturedObjectsOffset => {
                        if _serde::__private::Option::is_some(&m_capturedObjectsOffset) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "capturedObjectsOffset",
                                ),
                            );
                        }
                        m_capturedObjectsOffset = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_maxNumObjectsPicked = match m_maxNumObjectsPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxNumObjectsPicked",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxMassOfObjectPicked = match m_maxMassOfObjectPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxMassOfObjectPicked",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_maxDistOfObjectPicked = match m_maxDistOfObjectPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "maxDistOfObjectPicked",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_impulseAppliedWhenObjectNotPicked = match m_impulseAppliedWhenObjectNotPicked {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "impulseAppliedWhenObjectNotPicked",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_throwVelocity = match m_throwVelocity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("throwVelocity"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_capturedObjectPosition = match m_capturedObjectPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "capturedObjectPosition",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_capturedObjectsOffset = match m_capturedObjectsOffset {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "capturedObjectsOffset",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpGravityGun {
            __ptr,
            parent,
            m_maxNumObjectsPicked,
            m_maxMassOfObjectPicked,
            m_maxDistOfObjectPicked,
            m_impulseAppliedWhenObjectNotPicked,
            m_throwVelocity,
            m_capturedObjectPosition,
            m_capturedObjectsOffset,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpGravityGun<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "grabbedBodies",
                "maxNumObjectsPicked",
                "maxMassOfObjectPicked",
                "maxDistOfObjectPicked",
                "impulseAppliedWhenObjectNotPicked",
                "throwVelocity",
                "capturedObjectPosition",
                "capturedObjectsOffset",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpGravityGun",
                FIELDS,
                __hkpGravityGunVisitor {
                    marker: _serde::__private::PhantomData::<hkpGravityGun>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
