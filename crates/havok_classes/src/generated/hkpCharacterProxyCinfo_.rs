use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCharacterProxyCinfo`
/// - version: `1`
/// - signature: `0x586d97b2`
/// - size: `144`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCharacterProxyCinfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCharacterControllerCinfo,
    /// # C++ Info
    /// - name: `position`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_position: Vector4,
    /// # C++ Info
    /// - name: `velocity`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_velocity: Vector4,
    /// # C++ Info
    /// - name: `dynamicFriction`(ctype: `hkReal`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_dynamicFriction: f32,
    /// # C++ Info
    /// - name: `staticFriction`(ctype: `hkReal`)
    /// - offset: ` 52`(x86)/` 52`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_staticFriction: f32,
    /// # C++ Info
    /// - name: `keepContactTolerance`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_keepContactTolerance: f32,
    /// # C++ Info
    /// - name: `up`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_up: Vector4,
    /// # C++ Info
    /// - name: `extraUpStaticFriction`(ctype: `hkReal`)
    /// - offset: ` 80`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_extraUpStaticFriction: f32,
    /// # C++ Info
    /// - name: `extraDownStaticFriction`(ctype: `hkReal`)
    /// - offset: ` 84`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_extraDownStaticFriction: f32,
    /// # C++ Info
    /// - name: `shapePhantom`(ctype: `struct hkpShapePhantom*`)
    /// - offset: ` 88`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_shapePhantom: Pointer,
    /// # C++ Info
    /// - name: `keepDistance`(ctype: `hkReal`)
    /// - offset: ` 92`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_keepDistance: f32,
    /// # C++ Info
    /// - name: `contactAngleSensitivity`(ctype: `hkReal`)
    /// - offset: ` 96`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_contactAngleSensitivity: f32,
    /// # C++ Info
    /// - name: `userPlanes`(ctype: `hkUint32`)
    /// - offset: `100`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_userPlanes: u32,
    /// # C++ Info
    /// - name: `maxCharacterSpeedForSolver`(ctype: `hkReal`)
    /// - offset: `104`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxCharacterSpeedForSolver: f32,
    /// # C++ Info
    /// - name: `characterStrength`(ctype: `hkReal`)
    /// - offset: `108`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_characterStrength: f32,
    /// # C++ Info
    /// - name: `characterMass`(ctype: `hkReal`)
    /// - offset: `112`(x86)/`116`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_characterMass: f32,
    /// # C++ Info
    /// - name: `maxSlope`(ctype: `hkReal`)
    /// - offset: `116`(x86)/`120`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxSlope: f32,
    /// # C++ Info
    /// - name: `penetrationRecoverySpeed`(ctype: `hkReal`)
    /// - offset: `120`(x86)/`124`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_penetrationRecoverySpeed: f32,
    /// # C++ Info
    /// - name: `maxCastIterations`(ctype: `hkInt32`)
    /// - offset: `124`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxCastIterations: i32,
    /// # C++ Info
    /// - name: `refreshManifoldInCheckSupport`(ctype: `hkBool`)
    /// - offset: `128`(x86)/`132`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_refreshManifoldInCheckSupport: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCharacterProxyCinfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCharacterProxyCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x586d97b2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_shapePhantom.get());
            v
        }
    }
    impl _serde::Serialize for hkpCharacterProxyCinfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x586d97b2)));
            let mut serializer = __serializer
                .serialize_struct("hkpCharacterProxyCinfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("velocity", &self.m_velocity)?;
            serializer.serialize_field("dynamicFriction", &self.m_dynamicFriction)?;
            serializer.serialize_field("staticFriction", &self.m_staticFriction)?;
            serializer
                .serialize_field("keepContactTolerance", &self.m_keepContactTolerance)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer
                .serialize_field(
                    "extraUpStaticFriction",
                    &self.m_extraUpStaticFriction,
                )?;
            serializer
                .serialize_field(
                    "extraDownStaticFriction",
                    &self.m_extraDownStaticFriction,
                )?;
            serializer.serialize_field("shapePhantom", &self.m_shapePhantom)?;
            serializer.serialize_field("keepDistance", &self.m_keepDistance)?;
            serializer
                .serialize_field(
                    "contactAngleSensitivity",
                    &self.m_contactAngleSensitivity,
                )?;
            serializer.serialize_field("userPlanes", &self.m_userPlanes)?;
            serializer
                .serialize_field(
                    "maxCharacterSpeedForSolver",
                    &self.m_maxCharacterSpeedForSolver,
                )?;
            serializer.serialize_field("characterStrength", &self.m_characterStrength)?;
            serializer.serialize_field("characterMass", &self.m_characterMass)?;
            serializer.serialize_field("maxSlope", &self.m_maxSlope)?;
            serializer
                .serialize_field(
                    "penetrationRecoverySpeed",
                    &self.m_penetrationRecoverySpeed,
                )?;
            serializer.serialize_field("maxCastIterations", &self.m_maxCastIterations)?;
            serializer
                .serialize_field(
                    "refreshManifoldInCheckSupport",
                    &self.m_refreshManifoldInCheckSupport,
                )?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCharacterProxyCinfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_position,
                m_velocity,
                m_dynamicFriction,
                m_staticFriction,
                m_keepContactTolerance,
                m_up,
                m_extraUpStaticFriction,
                m_extraDownStaticFriction,
                m_shapePhantom,
                m_keepDistance,
                m_contactAngleSensitivity,
                m_userPlanes,
                m_maxCharacterSpeedForSolver,
                m_characterStrength,
                m_characterMass,
                m_maxSlope,
                m_penetrationRecoverySpeed,
                m_maxCastIterations,
                m_refreshManifoldInCheckSupport,
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
                        "position" => Ok(__Field::m_position),
                        "velocity" => Ok(__Field::m_velocity),
                        "dynamicFriction" => Ok(__Field::m_dynamicFriction),
                        "staticFriction" => Ok(__Field::m_staticFriction),
                        "keepContactTolerance" => Ok(__Field::m_keepContactTolerance),
                        "up" => Ok(__Field::m_up),
                        "extraUpStaticFriction" => Ok(__Field::m_extraUpStaticFriction),
                        "extraDownStaticFriction" => {
                            Ok(__Field::m_extraDownStaticFriction)
                        }
                        "shapePhantom" => Ok(__Field::m_shapePhantom),
                        "keepDistance" => Ok(__Field::m_keepDistance),
                        "contactAngleSensitivity" => {
                            Ok(__Field::m_contactAngleSensitivity)
                        }
                        "userPlanes" => Ok(__Field::m_userPlanes),
                        "maxCharacterSpeedForSolver" => {
                            Ok(__Field::m_maxCharacterSpeedForSolver)
                        }
                        "characterStrength" => Ok(__Field::m_characterStrength),
                        "characterMass" => Ok(__Field::m_characterMass),
                        "maxSlope" => Ok(__Field::m_maxSlope),
                        "penetrationRecoverySpeed" => {
                            Ok(__Field::m_penetrationRecoverySpeed)
                        }
                        "maxCastIterations" => Ok(__Field::m_maxCastIterations),
                        "refreshManifoldInCheckSupport" => {
                            Ok(__Field::m_refreshManifoldInCheckSupport)
                        }
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
            struct __hkpCharacterProxyCinfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpCharacterProxyCinfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpCharacterProxyCinfoVisitor<'de> {
                type Value = hkpCharacterProxyCinfo;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpCharacterProxyCinfo",
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
                    let mut m_position: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_velocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_dynamicFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_staticFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_keepContactTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_extraUpStaticFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_extraDownStaticFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_shapePhantom: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_keepDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_contactAngleSensitivity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_userPlanes: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_maxCharacterSpeedForSolver: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_characterStrength: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_characterMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxSlope: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_penetrationRecoverySpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxCastIterations: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_refreshManifoldInCheckSupport: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_position) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "position",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 0usize)?;
                                m_position = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_velocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocity",
                                        ),
                                    );
                                }
                                m_velocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_dynamicFriction) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFriction",
                                        ),
                                    );
                                }
                                m_dynamicFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_staticFriction) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticFriction",
                                        ),
                                    );
                                }
                                m_staticFriction = _serde::__private::Some(
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
                                    &m_keepContactTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keepContactTolerance",
                                        ),
                                    );
                                }
                                m_keepContactTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_up) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("up"),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 4usize)?;
                                m_up = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_extraUpStaticFriction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extraUpStaticFriction",
                                        ),
                                    );
                                }
                                m_extraUpStaticFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_extraDownStaticFriction,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extraDownStaticFriction",
                                        ),
                                    );
                                }
                                m_extraDownStaticFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_shapePhantom) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapePhantom",
                                        ),
                                    );
                                }
                                m_shapePhantom = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_keepDistance) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keepDistance",
                                        ),
                                    );
                                }
                                m_keepDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_contactAngleSensitivity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactAngleSensitivity",
                                        ),
                                    );
                                }
                                m_contactAngleSensitivity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_userPlanes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userPlanes",
                                        ),
                                    );
                                }
                                m_userPlanes = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxCharacterSpeedForSolver,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCharacterSpeedForSolver",
                                        ),
                                    );
                                }
                                m_maxCharacterSpeedForSolver = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterStrength,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterStrength",
                                        ),
                                    );
                                }
                                m_characterStrength = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_characterMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterMass",
                                        ),
                                    );
                                }
                                m_characterMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_maxSlope) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSlope",
                                        ),
                                    );
                                }
                                m_maxSlope = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_penetrationRecoverySpeed,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "penetrationRecoverySpeed",
                                        ),
                                    );
                                }
                                m_penetrationRecoverySpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxCastIterations,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCastIterations",
                                        ),
                                    );
                                }
                                m_maxCastIterations = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_refreshManifoldInCheckSupport,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refreshManifoldInCheckSupport",
                                        ),
                                    );
                                }
                                m_refreshManifoldInCheckSupport = _serde::__private::Some(
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
                    __A::pad(&mut __map, 15usize, 11usize)?;
                    let m_position = match m_position {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("position"),
                            );
                        }
                    };
                    let m_velocity = match m_velocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("velocity"),
                            );
                        }
                    };
                    let m_dynamicFriction = match m_dynamicFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicFriction",
                                ),
                            );
                        }
                    };
                    let m_staticFriction = match m_staticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticFriction",
                                ),
                            );
                        }
                    };
                    let m_keepContactTolerance = match m_keepContactTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keepContactTolerance",
                                ),
                            );
                        }
                    };
                    let m_up = match m_up {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("up"),
                            );
                        }
                    };
                    let m_extraUpStaticFriction = match m_extraUpStaticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extraUpStaticFriction",
                                ),
                            );
                        }
                    };
                    let m_extraDownStaticFriction = match m_extraDownStaticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extraDownStaticFriction",
                                ),
                            );
                        }
                    };
                    let m_shapePhantom = match m_shapePhantom {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapePhantom",
                                ),
                            );
                        }
                    };
                    let m_keepDistance = match m_keepDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keepDistance",
                                ),
                            );
                        }
                    };
                    let m_contactAngleSensitivity = match m_contactAngleSensitivity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactAngleSensitivity",
                                ),
                            );
                        }
                    };
                    let m_userPlanes = match m_userPlanes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userPlanes",
                                ),
                            );
                        }
                    };
                    let m_maxCharacterSpeedForSolver = match m_maxCharacterSpeedForSolver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCharacterSpeedForSolver",
                                ),
                            );
                        }
                    };
                    let m_characterStrength = match m_characterStrength {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterStrength",
                                ),
                            );
                        }
                    };
                    let m_characterMass = match m_characterMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterMass",
                                ),
                            );
                        }
                    };
                    let m_maxSlope = match m_maxSlope {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("maxSlope"),
                            );
                        }
                    };
                    let m_penetrationRecoverySpeed = match m_penetrationRecoverySpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "penetrationRecoverySpeed",
                                ),
                            );
                        }
                    };
                    let m_maxCastIterations = match m_maxCastIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCastIterations",
                                ),
                            );
                        }
                    };
                    let m_refreshManifoldInCheckSupport = match m_refreshManifoldInCheckSupport {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refreshManifoldInCheckSupport",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpCharacterProxyCinfo {
                        __ptr,
                        parent,
                        m_position,
                        m_velocity,
                        m_dynamicFriction,
                        m_staticFriction,
                        m_keepContactTolerance,
                        m_up,
                        m_extraUpStaticFriction,
                        m_extraDownStaticFriction,
                        m_shapePhantom,
                        m_keepDistance,
                        m_contactAngleSensitivity,
                        m_userPlanes,
                        m_maxCharacterSpeedForSolver,
                        m_characterStrength,
                        m_characterMass,
                        m_maxSlope,
                        m_penetrationRecoverySpeed,
                        m_maxCastIterations,
                        m_refreshManifoldInCheckSupport,
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
                    let mut m_position: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_velocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_dynamicFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_staticFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_keepContactTolerance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_extraUpStaticFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_extraDownStaticFriction: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_shapePhantom: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_keepDistance: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_contactAngleSensitivity: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_userPlanes: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_maxCharacterSpeedForSolver: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_characterStrength: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_characterMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxSlope: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_penetrationRecoverySpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxCastIterations: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_refreshManifoldInCheckSupport: _serde::__private::Option<
                        bool,
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
                            __Field::m_position => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_position) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "position",
                                        ),
                                    );
                                }
                                m_position = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_velocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_velocity) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "velocity",
                                        ),
                                    );
                                }
                                m_velocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dynamicFriction) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFriction",
                                        ),
                                    );
                                }
                                m_dynamicFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticFriction) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticFriction",
                                        ),
                                    );
                                }
                                m_staticFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_keepContactTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_keepContactTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keepContactTolerance",
                                        ),
                                    );
                                }
                                m_keepContactTolerance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_up => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_up) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("up"),
                                    );
                                }
                                m_up = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extraUpStaticFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_extraUpStaticFriction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extraUpStaticFriction",
                                        ),
                                    );
                                }
                                m_extraUpStaticFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extraDownStaticFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_extraDownStaticFriction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extraDownStaticFriction",
                                        ),
                                    );
                                }
                                m_extraDownStaticFriction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapePhantom => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapePhantom) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapePhantom",
                                        ),
                                    );
                                }
                                m_shapePhantom = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_keepDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_keepDistance) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "keepDistance",
                                        ),
                                    );
                                }
                                m_keepDistance = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_contactAngleSensitivity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_contactAngleSensitivity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "contactAngleSensitivity",
                                        ),
                                    );
                                }
                                m_contactAngleSensitivity = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userPlanes => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userPlanes) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userPlanes",
                                        ),
                                    );
                                }
                                m_userPlanes = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxCharacterSpeedForSolver => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxCharacterSpeedForSolver,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCharacterSpeedForSolver",
                                        ),
                                    );
                                }
                                m_maxCharacterSpeedForSolver = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterStrength => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterStrength,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterStrength",
                                        ),
                                    );
                                }
                                m_characterStrength = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_characterMass) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterMass",
                                        ),
                                    );
                                }
                                m_characterMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxSlope => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_maxSlope) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxSlope",
                                        ),
                                    );
                                }
                                m_maxSlope = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_penetrationRecoverySpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_penetrationRecoverySpeed,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "penetrationRecoverySpeed",
                                        ),
                                    );
                                }
                                m_penetrationRecoverySpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxCastIterations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxCastIterations,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCastIterations",
                                        ),
                                    );
                                }
                                m_maxCastIterations = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_refreshManifoldInCheckSupport => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_refreshManifoldInCheckSupport,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "refreshManifoldInCheckSupport",
                                        ),
                                    );
                                }
                                m_refreshManifoldInCheckSupport = _serde::__private::Some(
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
                    let m_position = match m_position {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("position"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_velocity = match m_velocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("velocity"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicFriction = match m_dynamicFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticFriction = match m_staticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_keepContactTolerance = match m_keepContactTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keepContactTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_up = match m_up {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("up"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extraUpStaticFriction = match m_extraUpStaticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extraUpStaticFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extraDownStaticFriction = match m_extraDownStaticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extraDownStaticFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapePhantom = match m_shapePhantom {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapePhantom",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_keepDistance = match m_keepDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "keepDistance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_contactAngleSensitivity = match m_contactAngleSensitivity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "contactAngleSensitivity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userPlanes = match m_userPlanes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userPlanes",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxCharacterSpeedForSolver = match m_maxCharacterSpeedForSolver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCharacterSpeedForSolver",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterStrength = match m_characterStrength {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterStrength",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterMass = match m_characterMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterMass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxSlope = match m_maxSlope {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("maxSlope"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_penetrationRecoverySpeed = match m_penetrationRecoverySpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "penetrationRecoverySpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxCastIterations = match m_maxCastIterations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCastIterations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_refreshManifoldInCheckSupport = match m_refreshManifoldInCheckSupport {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "refreshManifoldInCheckSupport",
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
                    let parent = hkpCharacterControllerCinfo {
                        __ptr,
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpCharacterProxyCinfo {
                        __ptr,
                        parent,
                        m_position,
                        m_velocity,
                        m_dynamicFriction,
                        m_staticFriction,
                        m_keepContactTolerance,
                        m_up,
                        m_extraUpStaticFriction,
                        m_extraDownStaticFriction,
                        m_shapePhantom,
                        m_keepDistance,
                        m_contactAngleSensitivity,
                        m_userPlanes,
                        m_maxCharacterSpeedForSolver,
                        m_characterStrength,
                        m_characterMass,
                        m_maxSlope,
                        m_penetrationRecoverySpeed,
                        m_maxCastIterations,
                        m_refreshManifoldInCheckSupport,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "position",
                "velocity",
                "dynamicFriction",
                "staticFriction",
                "keepContactTolerance",
                "up",
                "extraUpStaticFriction",
                "extraDownStaticFriction",
                "shapePhantom",
                "keepDistance",
                "contactAngleSensitivity",
                "userPlanes",
                "maxCharacterSpeedForSolver",
                "characterStrength",
                "characterMass",
                "maxSlope",
                "penetrationRecoverySpeed",
                "maxCastIterations",
                "refreshManifoldInCheckSupport",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCharacterProxyCinfo",
                FIELDS,
                __hkpCharacterProxyCinfoVisitor {
                    marker: _serde::__private::PhantomData::<hkpCharacterProxyCinfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
