use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpPointToPathConstraintData`
/// - version: `0`
/// - signature: `0x8e7cb5da`
/// - size: `176`(x86)/`192`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPointToPathConstraintData {
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
    /// - name: `atoms`(ctype: `struct hkpBridgeAtoms`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: ` 12`(x86)/` 24`(x86_64)
    pub m_atoms: hkpBridgeAtoms,
    /// # C++ Info
    /// - name: `path`(ctype: `struct hkpParametricCurve*`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_path: Pointer,
    /// # C++ Info
    /// - name: `maxFrictionForce`(ctype: `hkReal`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxFrictionForce: f32,
    /// # C++ Info
    /// - name: `angularConstrainedDOF`(ctype: `enum OrientationConstraintType`)
    /// - offset: ` 32`(x86)/` 60`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_angularConstrainedDOF: OrientationConstraintType,
    /// # C++ Info
    /// - name: `transform_OS_KS`(ctype: `hkTransform[2]`)
    /// - offset: ` 48`(x86)/` 64`(x86_64)
    /// - type_size: `128`(x86)/`128`(x86_64)
    pub m_transform_OS_KS: [Transform; 2usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPointToPathConstraintData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPointToPathConstraintData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x8e7cb5da)
        }
    }
    impl _serde::Serialize for hkpPointToPathConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x8e7cb5da)));
            let mut serializer = __serializer
                .serialize_struct("hkpPointToPathConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("path", &self.m_path)?;
            serializer.serialize_field("maxFrictionForce", &self.m_maxFrictionForce)?;
            serializer
                .serialize_field(
                    "angularConstrainedDOF",
                    &self.m_angularConstrainedDOF,
                )?;
            serializer.pad_field([0u8; 15usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_fixed_array_field(
                    "transform_OS_KS",
                    self.m_transform_OS_KS.as_slice(),
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_atoms,
    m_path,
    m_maxFrictionForce,
    m_angularConstrainedDOF,
    m_transform_OS_KS,
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
            "atoms" => Ok(__Field::m_atoms),
            "path" => Ok(__Field::m_path),
            "maxFrictionForce" => Ok(__Field::m_maxFrictionForce),
            "angularConstrainedDOF" => Ok(__Field::m_angularConstrainedDOF),
            "transform_OS_KS" => Ok(__Field::m_transform_OS_KS),
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
pub(super) struct __hkpPointToPathConstraintDataVisitor<'de> {
    marker: core::marker::PhantomData<hkpPointToPathConstraintData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPointToPathConstraintDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPointToPathConstraintData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPointToPathConstraintData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPointToPathConstraintDataVisitor<'de> {
    type Value = hkpPointToPathConstraintData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPointToPathConstraintData",
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
        let mut m_atoms: _serde::__private::Option<hkpBridgeAtoms> = _serde::__private::None;
        let mut m_path: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_maxFrictionForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_angularConstrainedDOF: _serde::__private::Option<
            OrientationConstraintType,
        > = _serde::__private::None;
        let mut m_transform_OS_KS: _serde::__private::Option<[Transform; 2usize]> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_atoms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                        );
                    }
                    m_atoms = _serde::__private::Some(
                        match __A::next_value::<hkpBridgeAtoms>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_path) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("path"),
                        );
                    }
                    m_path = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_maxFrictionForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFrictionForce",
                            ),
                        );
                    }
                    m_maxFrictionForce = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_angularConstrainedDOF) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "angularConstrainedDOF",
                            ),
                        );
                    }
                    m_angularConstrainedDOF = _serde::__private::Some(
                        match __A::next_value::<OrientationConstraintType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_transform_OS_KS) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transform_OS_KS",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 15usize, 3usize)?;
                    m_transform_OS_KS = _serde::__private::Some(
                        match __A::next_value::<[Transform; 2usize]>(&mut __map) {
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
        let m_path = match m_path {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("path"),
                );
            }
        };
        let m_maxFrictionForce = match m_maxFrictionForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFrictionForce"),
                );
            }
        };
        let m_angularConstrainedDOF = match m_angularConstrainedDOF {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "angularConstrainedDOF",
                    ),
                );
            }
        };
        let m_transform_OS_KS = match m_transform_OS_KS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transform_OS_KS"),
                );
            }
        };
        _serde::__private::Ok(hkpPointToPathConstraintData {
            __ptr,
            parent,
            m_atoms,
            m_path,
            m_maxFrictionForce,
            m_angularConstrainedDOF,
            m_transform_OS_KS,
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
        let parent = __hkpConstraintDataVisitor::visit_as_parent(&mut __map)?;
        let mut m_atoms: _serde::__private::Option<hkpBridgeAtoms> = _serde::__private::None;
        let mut m_path: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_maxFrictionForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_angularConstrainedDOF: _serde::__private::Option<
            OrientationConstraintType,
        > = _serde::__private::None;
        let mut m_transform_OS_KS: _serde::__private::Option<[Transform; 2usize]> = _serde::__private::None;
        for _ in 0..5usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_atoms => {
                        if _serde::__private::Option::is_some(&m_atoms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                            );
                        }
                        m_atoms = _serde::__private::Some(
                            match __A::next_value::<hkpBridgeAtoms>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_path => {
                        if _serde::__private::Option::is_some(&m_path) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("path"),
                            );
                        }
                        m_path = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxFrictionForce => {
                        if _serde::__private::Option::is_some(&m_maxFrictionForce) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxFrictionForce",
                                ),
                            );
                        }
                        m_maxFrictionForce = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_angularConstrainedDOF => {
                        if _serde::__private::Option::is_some(&m_angularConstrainedDOF) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "angularConstrainedDOF",
                                ),
                            );
                        }
                        m_angularConstrainedDOF = _serde::__private::Some(
                            match __A::next_value::<
                                OrientationConstraintType,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_transform_OS_KS => {
                        if _serde::__private::Option::is_some(&m_transform_OS_KS) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transform_OS_KS",
                                ),
                            );
                        }
                        m_transform_OS_KS = _serde::__private::Some(
                            match __A::next_value::<[Transform; 2usize]>(&mut __map) {
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
        let m_atoms = match m_atoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atoms"),
                );
            }
        };
        let m_path = match m_path {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("path"),
                );
            }
        };
        let m_maxFrictionForce = match m_maxFrictionForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFrictionForce"),
                );
            }
        };
        let m_angularConstrainedDOF = match m_angularConstrainedDOF {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "angularConstrainedDOF",
                    ),
                );
            }
        };
        let m_transform_OS_KS = match m_transform_OS_KS {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transform_OS_KS"),
                );
            }
        };
        _serde::__private::Ok(hkpPointToPathConstraintData {
            __ptr,
            parent,
            m_atoms,
            m_path,
            m_maxFrictionForce,
            m_angularConstrainedDOF,
            m_transform_OS_KS,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPointToPathConstraintData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "atoms",
                "path",
                "maxFrictionForce",
                "angularConstrainedDOF",
                "transform_OS_KS",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPointToPathConstraintData",
                FIELDS,
                __hkpPointToPathConstraintDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPointToPathConstraintData,
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
pub enum OrientationConstraintType {
    #[default]
    CONSTRAIN_ORIENTATION_INVALID = 0isize,
    CONSTRAIN_ORIENTATION_NONE = 1isize,
    CONSTRAIN_ORIENTATION_ALLOW_SPIN = 2isize,
    CONSTRAIN_ORIENTATION_TO_PATH = 3isize,
    CONSTRAIN_ORIENTATION_MAX_ID = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for OrientationConstraintType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::CONSTRAIN_ORIENTATION_INVALID => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_INVALID", &0u64)
                }
                Self::CONSTRAIN_ORIENTATION_NONE => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_NONE", &1u64)
                }
                Self::CONSTRAIN_ORIENTATION_ALLOW_SPIN => {
                    __serializer
                        .serialize_field("CONSTRAIN_ORIENTATION_ALLOW_SPIN", &2u64)
                }
                Self::CONSTRAIN_ORIENTATION_TO_PATH => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_TO_PATH", &3u64)
                }
                Self::CONSTRAIN_ORIENTATION_MAX_ID => {
                    __serializer.serialize_field("CONSTRAIN_ORIENTATION_MAX_ID", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum OrientationConstraintType to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for OrientationConstraintType {
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
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        3i8 => _serde::__private::Ok(__Field::__field3),
                        4i8 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4",
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
                                    .eq_ignore_ascii_case("CONSTRAIN_ORIENTATION_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("CONSTRAIN_ORIENTATION_NONE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "CONSTRAIN_ORIENTATION_ALLOW_SPIN",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            v if v == "3"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAIN_ORIENTATION_TO_PATH") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v
                                    .eq_ignore_ascii_case("CONSTRAIN_ORIENTATION_MAX_ID") => {
                                _serde::__private::Ok(__Field::__field4)
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
                marker: _serde::__private::PhantomData<OrientationConstraintType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = OrientationConstraintType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum OrientationConstraintType",
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
                                OrientationConstraintType::CONSTRAIN_ORIENTATION_INVALID,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OrientationConstraintType::CONSTRAIN_ORIENTATION_NONE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OrientationConstraintType::CONSTRAIN_ORIENTATION_ALLOW_SPIN,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OrientationConstraintType::CONSTRAIN_ORIENTATION_TO_PATH,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OrientationConstraintType::CONSTRAIN_ORIENTATION_MAX_ID,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "CONSTRAIN_ORIENTATION_INVALID",
                "CONSTRAIN_ORIENTATION_NONE",
                "CONSTRAIN_ORIENTATION_ALLOW_SPIN",
                "CONSTRAIN_ORIENTATION_TO_PATH",
                "CONSTRAIN_ORIENTATION_MAX_ID",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "OrientationConstraintType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<OrientationConstraintType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
