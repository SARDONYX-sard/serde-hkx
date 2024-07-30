use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpMotion`
/// - version: `3`
/// - signature: `0x98aadb4f`
/// - size: `288`(x86)/`320`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMotion {
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
    /// - name: `type`(ctype: `enum MotionType`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_type: MotionType,
    /// # C++ Info
    /// - name: `deactivationIntegrateCounter`(ctype: `hkUint8`)
    /// - offset: `  9`(x86)/` 17`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_deactivationIntegrateCounter: u8,
    /// # C++ Info
    /// - name: `deactivationNumInactiveFrames`(ctype: `hkUint16[2]`)
    /// - offset: ` 10`(x86)/` 18`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_deactivationNumInactiveFrames: [u16; 2usize],
    /// # C++ Info
    /// - name: `motionState`(ctype: `struct hkMotionState`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `176`(x86)/`176`(x86_64)
    pub m_motionState: hkMotionState,
    /// # C++ Info
    /// - name: `inertiaAndMassInv`(ctype: `hkVector4`)
    /// - offset: `192`(x86)/`208`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_inertiaAndMassInv: Vector4,
    /// # C++ Info
    /// - name: `linearVelocity`(ctype: `hkVector4`)
    /// - offset: `208`(x86)/`224`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// - name: `angularVelocity`(ctype: `hkVector4`)
    /// - offset: `224`(x86)/`240`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_angularVelocity: Vector4,
    /// # C++ Info
    /// - name: `deactivationRefPosition`(ctype: `hkVector4[2]`)
    /// - offset: `240`(x86)/`256`(x86_64)
    /// - type_size: ` 32`(x86)/` 32`(x86_64)
    pub m_deactivationRefPosition: [Vector4; 2usize],
    /// # C++ Info
    /// - name: `deactivationRefOrientation`(ctype: `hkUint32[2]`)
    /// - offset: `272`(x86)/`288`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_deactivationRefOrientation: [u32; 2usize],
    /// # C++ Info
    /// - name: `savedMotion`(ctype: `struct hkpMaxSizeMotion*`)
    /// - offset: `280`(x86)/`296`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_savedMotion: Pointer,
    /// # C++ Info
    /// - name: `savedQualityTypeIndex`(ctype: `hkUint16`)
    /// - offset: `284`(x86)/`304`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_savedQualityTypeIndex: u16,
    /// # C++ Info
    /// - name: `gravityFactor`(ctype: `hkHalf`)
    /// - offset: `286`(x86)/`306`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_gravityFactor: f16,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMotion {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMotion"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x98aadb4f)
        }
    }
    impl _serde::Serialize for hkpMotion {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x98aadb4f)));
            let mut serializer = __serializer.serialize_struct("hkpMotion", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer
                .serialize_field(
                    "deactivationIntegrateCounter",
                    &self.m_deactivationIntegrateCounter,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "deactivationNumInactiveFrames",
                    self.m_deactivationNumInactiveFrames.as_slice(),
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.serialize_field("motionState", &self.m_motionState)?;
            serializer.serialize_field("inertiaAndMassInv", &self.m_inertiaAndMassInv)?;
            serializer.serialize_field("linearVelocity", &self.m_linearVelocity)?;
            serializer.serialize_field("angularVelocity", &self.m_angularVelocity)?;
            serializer
                .serialize_fixed_array_field(
                    "deactivationRefPosition",
                    self.m_deactivationRefPosition.as_slice(),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "deactivationRefOrientation",
                    self.m_deactivationRefOrientation.as_slice(),
                )?;
            serializer.serialize_field("savedMotion", &self.m_savedMotion)?;
            serializer
                .serialize_field(
                    "savedQualityTypeIndex",
                    &self.m_savedQualityTypeIndex,
                )?;
            serializer.serialize_field("gravityFactor", &self.m_gravityFactor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMotion {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_deactivationIntegrateCounter,
                m_deactivationNumInactiveFrames,
                m_motionState,
                m_inertiaAndMassInv,
                m_linearVelocity,
                m_angularVelocity,
                m_deactivationRefPosition,
                m_deactivationRefOrientation,
                m_savedMotion,
                m_savedQualityTypeIndex,
                m_gravityFactor,
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
                        "deactivationIntegrateCounter" => {
                            Ok(__Field::m_deactivationIntegrateCounter)
                        }
                        "deactivationNumInactiveFrames" => {
                            Ok(__Field::m_deactivationNumInactiveFrames)
                        }
                        "motionState" => Ok(__Field::m_motionState),
                        "inertiaAndMassInv" => Ok(__Field::m_inertiaAndMassInv),
                        "linearVelocity" => Ok(__Field::m_linearVelocity),
                        "angularVelocity" => Ok(__Field::m_angularVelocity),
                        "deactivationRefPosition" => {
                            Ok(__Field::m_deactivationRefPosition)
                        }
                        "deactivationRefOrientation" => {
                            Ok(__Field::m_deactivationRefOrientation)
                        }
                        "savedMotion" => Ok(__Field::m_savedMotion),
                        "savedQualityTypeIndex" => Ok(__Field::m_savedQualityTypeIndex),
                        "gravityFactor" => Ok(__Field::m_gravityFactor),
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
            struct __hkpMotionVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpMotion>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpMotionVisitor<'de> {
                type Value = hkpMotion;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkpMotion")
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
                    let mut m_type: _serde::__private::Option<MotionType> = _serde::__private::None;
                    let mut m_deactivationIntegrateCounter: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_deactivationNumInactiveFrames: _serde::__private::Option<
                        [u16; 2usize],
                    > = _serde::__private::None;
                    let mut m_motionState: _serde::__private::Option<hkMotionState> = _serde::__private::None;
                    let mut m_inertiaAndMassInv: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_angularVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_deactivationRefPosition: _serde::__private::Option<
                        [Vector4; 2usize],
                    > = _serde::__private::None;
                    let mut m_deactivationRefOrientation: _serde::__private::Option<
                        [u32; 2usize],
                    > = _serde::__private::None;
                    let mut m_savedMotion: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_savedQualityTypeIndex: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_gravityFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_type) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<MotionType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationIntegrateCounter,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationIntegrateCounter",
                                        ),
                                    );
                                }
                                m_deactivationIntegrateCounter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationNumInactiveFrames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationNumInactiveFrames",
                                        ),
                                    );
                                }
                                m_deactivationNumInactiveFrames = _serde::__private::Some(
                                    match __A::next_value::<[u16; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_motionState) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionState",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 2usize, 10usize)?;
                                m_motionState = _serde::__private::Some(
                                    match __A::next_value::<hkMotionState>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_inertiaAndMassInv,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inertiaAndMassInv",
                                        ),
                                    );
                                }
                                m_inertiaAndMassInv = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_linearVelocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearVelocity",
                                        ),
                                    );
                                }
                                m_linearVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_angularVelocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularVelocity",
                                        ),
                                    );
                                }
                                m_angularVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationRefPosition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationRefPosition",
                                        ),
                                    );
                                }
                                m_deactivationRefPosition = _serde::__private::Some(
                                    match __A::next_value::<[Vector4; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_deactivationRefOrientation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationRefOrientation",
                                        ),
                                    );
                                }
                                m_deactivationRefOrientation = _serde::__private::Some(
                                    match __A::next_value::<[u32; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_savedMotion) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "savedMotion",
                                        ),
                                    );
                                }
                                m_savedMotion = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_savedQualityTypeIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "savedQualityTypeIndex",
                                        ),
                                    );
                                }
                                m_savedQualityTypeIndex = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_gravityFactor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravityFactor",
                                        ),
                                    );
                                }
                                m_gravityFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 12usize)?;
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                        }
                    };
                    let m_deactivationIntegrateCounter = match m_deactivationIntegrateCounter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationIntegrateCounter",
                                ),
                            );
                        }
                    };
                    let m_deactivationNumInactiveFrames = match m_deactivationNumInactiveFrames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationNumInactiveFrames",
                                ),
                            );
                        }
                    };
                    let m_motionState = match m_motionState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionState",
                                ),
                            );
                        }
                    };
                    let m_inertiaAndMassInv = match m_inertiaAndMassInv {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inertiaAndMassInv",
                                ),
                            );
                        }
                    };
                    let m_linearVelocity = match m_linearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearVelocity",
                                ),
                            );
                        }
                    };
                    let m_angularVelocity = match m_angularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularVelocity",
                                ),
                            );
                        }
                    };
                    let m_deactivationRefPosition = match m_deactivationRefPosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationRefPosition",
                                ),
                            );
                        }
                    };
                    let m_deactivationRefOrientation = match m_deactivationRefOrientation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationRefOrientation",
                                ),
                            );
                        }
                    };
                    let m_savedMotion = match m_savedMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "savedMotion",
                                ),
                            );
                        }
                    };
                    let m_savedQualityTypeIndex = match m_savedQualityTypeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "savedQualityTypeIndex",
                                ),
                            );
                        }
                    };
                    let m_gravityFactor = match m_gravityFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gravityFactor",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpMotion {
                        __ptr,
                        parent,
                        m_type,
                        m_deactivationIntegrateCounter,
                        m_deactivationNumInactiveFrames,
                        m_motionState,
                        m_inertiaAndMassInv,
                        m_linearVelocity,
                        m_angularVelocity,
                        m_deactivationRefPosition,
                        m_deactivationRefOrientation,
                        m_savedMotion,
                        m_savedQualityTypeIndex,
                        m_gravityFactor,
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
                    let mut m_type: _serde::__private::Option<MotionType> = _serde::__private::None;
                    let mut m_deactivationIntegrateCounter: _serde::__private::Option<
                        u8,
                    > = _serde::__private::None;
                    let mut m_deactivationNumInactiveFrames: _serde::__private::Option<
                        [u16; 2usize],
                    > = _serde::__private::None;
                    let mut m_motionState: _serde::__private::Option<hkMotionState> = _serde::__private::None;
                    let mut m_inertiaAndMassInv: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_angularVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_deactivationRefPosition: _serde::__private::Option<
                        [Vector4; 2usize],
                    > = _serde::__private::None;
                    let mut m_deactivationRefOrientation: _serde::__private::Option<
                        [u32; 2usize],
                    > = _serde::__private::None;
                    let mut m_savedMotion: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_savedQualityTypeIndex: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_gravityFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<MotionType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationIntegrateCounter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationIntegrateCounter,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationIntegrateCounter",
                                        ),
                                    );
                                }
                                m_deactivationIntegrateCounter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationNumInactiveFrames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationNumInactiveFrames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationNumInactiveFrames",
                                        ),
                                    );
                                }
                                m_deactivationNumInactiveFrames = _serde::__private::Some(
                                    match __A::next_value::<[u16; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_motionState => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motionState) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionState",
                                        ),
                                    );
                                }
                                m_motionState = _serde::__private::Some(
                                    match __A::next_value::<hkMotionState>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_inertiaAndMassInv => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_inertiaAndMassInv,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inertiaAndMassInv",
                                        ),
                                    );
                                }
                                m_inertiaAndMassInv = _serde::__private::Some(
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
                            __Field::m_linearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linearVelocity) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearVelocity",
                                        ),
                                    );
                                }
                                m_linearVelocity = _serde::__private::Some(
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
                            __Field::m_angularVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angularVelocity) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularVelocity",
                                        ),
                                    );
                                }
                                m_angularVelocity = _serde::__private::Some(
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
                            __Field::m_deactivationRefPosition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationRefPosition,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationRefPosition",
                                        ),
                                    );
                                }
                                m_deactivationRefPosition = _serde::__private::Some(
                                    match __A::next_value::<[Vector4; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_deactivationRefOrientation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_deactivationRefOrientation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "deactivationRefOrientation",
                                        ),
                                    );
                                }
                                m_deactivationRefOrientation = _serde::__private::Some(
                                    match __A::next_value::<[u32; 2usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_savedMotion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_savedMotion) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "savedMotion",
                                        ),
                                    );
                                }
                                m_savedMotion = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_savedQualityTypeIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_savedQualityTypeIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "savedQualityTypeIndex",
                                        ),
                                    );
                                }
                                m_savedQualityTypeIndex = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_gravityFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gravityFactor) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gravityFactor",
                                        ),
                                    );
                                }
                                m_gravityFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
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
                    let m_deactivationIntegrateCounter = match m_deactivationIntegrateCounter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationIntegrateCounter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationNumInactiveFrames = match m_deactivationNumInactiveFrames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationNumInactiveFrames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionState = match m_motionState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionState",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inertiaAndMassInv = match m_inertiaAndMassInv {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inertiaAndMassInv",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linearVelocity = match m_linearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angularVelocity = match m_angularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationRefPosition = match m_deactivationRefPosition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationRefPosition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_deactivationRefOrientation = match m_deactivationRefOrientation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "deactivationRefOrientation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_savedMotion = match m_savedMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "savedMotion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_savedQualityTypeIndex = match m_savedQualityTypeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "savedQualityTypeIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_gravityFactor = match m_gravityFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gravityFactor",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpMotion {
                        __ptr,
                        parent,
                        m_type,
                        m_deactivationIntegrateCounter,
                        m_deactivationNumInactiveFrames,
                        m_motionState,
                        m_inertiaAndMassInv,
                        m_linearVelocity,
                        m_angularVelocity,
                        m_deactivationRefPosition,
                        m_deactivationRefOrientation,
                        m_savedMotion,
                        m_savedQualityTypeIndex,
                        m_gravityFactor,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "type",
                "deactivationIntegrateCounter",
                "deactivationNumInactiveFrames",
                "motionState",
                "inertiaAndMassInv",
                "linearVelocity",
                "angularVelocity",
                "deactivationRefPosition",
                "deactivationRefOrientation",
                "savedMotion",
                "savedQualityTypeIndex",
                "gravityFactor",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMotion",
                FIELDS,
                __hkpMotionVisitor {
                    marker: _serde::__private::PhantomData::<hkpMotion>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum MotionType {
    #[default]
    MOTION_INVALID = 0isize,
    MOTION_DYNAMIC = 1isize,
    MOTION_SPHERE_INERTIA = 2isize,
    MOTION_BOX_INERTIA = 3isize,
    MOTION_KEYFRAMED = 4isize,
    MOTION_FIXED = 5isize,
    MOTION_THIN_BOX_INERTIA = 6isize,
    MOTION_CHARACTER = 7isize,
    MOTION_MAX_ID = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MotionType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MOTION_INVALID => {
                    __serializer.serialize_field("MOTION_INVALID", &0u64)
                }
                Self::MOTION_DYNAMIC => {
                    __serializer.serialize_field("MOTION_DYNAMIC", &1u64)
                }
                Self::MOTION_SPHERE_INERTIA => {
                    __serializer.serialize_field("MOTION_SPHERE_INERTIA", &2u64)
                }
                Self::MOTION_BOX_INERTIA => {
                    __serializer.serialize_field("MOTION_BOX_INERTIA", &3u64)
                }
                Self::MOTION_KEYFRAMED => {
                    __serializer.serialize_field("MOTION_KEYFRAMED", &4u64)
                }
                Self::MOTION_FIXED => __serializer.serialize_field("MOTION_FIXED", &5u64),
                Self::MOTION_THIN_BOX_INERTIA => {
                    __serializer.serialize_field("MOTION_THIN_BOX_INERTIA", &6u64)
                }
                Self::MOTION_CHARACTER => {
                    __serializer.serialize_field("MOTION_CHARACTER", &7u64)
                }
                Self::MOTION_MAX_ID => {
                    __serializer.serialize_field("MOTION_MAX_ID", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MotionType to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for MotionType {
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
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
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
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        2u8 => _serde::__private::Ok(__Field::__field2),
                        3u8 => _serde::__private::Ok(__Field::__field3),
                        4u8 => _serde::__private::Ok(__Field::__field4),
                        5u8 => _serde::__private::Ok(__Field::__field5),
                        6u8 => _serde::__private::Ok(__Field::__field6),
                        7u8 => _serde::__private::Ok(__Field::__field7),
                        8u8 => _serde::__private::Ok(__Field::__field8),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8",
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
                            v if v == "0" || v.eq_ignore_ascii_case("MOTION_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("MOTION_DYNAMIC") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("MOTION_SPHERE_INERTIA") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("MOTION_BOX_INERTIA") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("MOTION_KEYFRAMED") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("MOTION_FIXED") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("MOTION_THIN_BOX_INERTIA") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("MOTION_CHARACTER") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("MOTION_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field8)
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MotionType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MotionType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MotionType",
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
                            _serde::__private::Ok(MotionType::MOTION_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_DYNAMIC)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_SPHERE_INERTIA)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_BOX_INERTIA)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_KEYFRAMED)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_FIXED)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_THIN_BOX_INERTIA)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_CHARACTER)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(MotionType::MOTION_MAX_ID)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MOTION_INVALID",
                "MOTION_DYNAMIC",
                "MOTION_SPHERE_INERTIA",
                "MOTION_BOX_INERTIA",
                "MOTION_KEYFRAMED",
                "MOTION_FIXED",
                "MOTION_THIN_BOX_INERTIA",
                "MOTION_CHARACTER",
                "MOTION_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MotionType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MotionType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
