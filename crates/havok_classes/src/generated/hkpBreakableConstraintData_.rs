use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpBreakableConstraintData`
/// - version: `0`
/// - signature: `0x7d6310c8`
/// - size: ` 40`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBreakableConstraintData {
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
    /// - name: `constraintData`(ctype: `struct hkpConstraintData*`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_constraintData: Pointer,
    /// # C++ Info
    /// - name: `childRuntimeSize`(ctype: `hkUint16`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_childRuntimeSize: u16,
    /// # C++ Info
    /// - name: `childNumSolverResults`(ctype: `hkUint16`)
    /// - offset: ` 30`(x86)/` 58`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_childNumSolverResults: u16,
    /// # C++ Info
    /// - name: `solverResultLimit`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 60`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_solverResultLimit: f32,
    /// # C++ Info
    /// - name: `removeWhenBroken`(ctype: `hkBool`)
    /// - offset: ` 36`(x86)/` 64`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_removeWhenBroken: bool,
    /// # C++ Info
    /// - name: `revertBackVelocityOnBreak`(ctype: `hkBool`)
    /// - offset: ` 37`(x86)/` 65`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_revertBackVelocityOnBreak: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBreakableConstraintData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBreakableConstraintData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7d6310c8)
        }
    }
    impl _serde::Serialize for hkpBreakableConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7d6310c8)));
            let mut serializer = __serializer
                .serialize_struct("hkpBreakableConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("constraintData", &self.m_constraintData)?;
            serializer.serialize_field("childRuntimeSize", &self.m_childRuntimeSize)?;
            serializer
                .serialize_field(
                    "childNumSolverResults",
                    &self.m_childNumSolverResults,
                )?;
            serializer.serialize_field("solverResultLimit", &self.m_solverResultLimit)?;
            serializer.serialize_field("removeWhenBroken", &self.m_removeWhenBroken)?;
            serializer
                .serialize_field(
                    "revertBackVelocityOnBreak",
                    &self.m_revertBackVelocityOnBreak,
                )?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpBreakableConstraintData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_revertBackVelocityOnBreak,
                m_removeWhenBroken,
                m_solverResultLimit,
                m_childNumSolverResults,
                m_childRuntimeSize,
                m_constraintData,
                m_atoms,
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
                        "revertBackVelocityOnBreak" => {
                            Ok(__Field::m_revertBackVelocityOnBreak)
                        }
                        "removeWhenBroken" => Ok(__Field::m_removeWhenBroken),
                        "solverResultLimit" => Ok(__Field::m_solverResultLimit),
                        "childNumSolverResults" => Ok(__Field::m_childNumSolverResults),
                        "childRuntimeSize" => Ok(__Field::m_childRuntimeSize),
                        "constraintData" => Ok(__Field::m_constraintData),
                        "atoms" => Ok(__Field::m_atoms),
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
            struct __hkpBreakableConstraintDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpBreakableConstraintData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpBreakableConstraintDataVisitor<'de> {
                type Value = hkpBreakableConstraintData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpBreakableConstraintData",
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
                    let mut m_constraintData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_childRuntimeSize: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_childNumSolverResults: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_solverResultLimit: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_removeWhenBroken: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_revertBackVelocityOnBreak: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..7usize {
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
                                if _serde::__private::Option::is_some(&m_constraintData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintData",
                                        ),
                                    );
                                }
                                m_constraintData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_childRuntimeSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "childRuntimeSize",
                                        ),
                                    );
                                }
                                m_childRuntimeSize = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_childNumSolverResults,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "childNumSolverResults",
                                        ),
                                    );
                                }
                                m_childNumSolverResults = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_solverResultLimit,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverResultLimit",
                                        ),
                                    );
                                }
                                m_solverResultLimit = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_removeWhenBroken) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "removeWhenBroken",
                                        ),
                                    );
                                }
                                m_removeWhenBroken = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_revertBackVelocityOnBreak,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "revertBackVelocityOnBreak",
                                        ),
                                    );
                                }
                                m_revertBackVelocityOnBreak = _serde::__private::Some(
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
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    let m_atoms = match m_atoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("atoms"),
                            );
                        }
                    };
                    let m_constraintData = match m_constraintData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintData",
                                ),
                            );
                        }
                    };
                    let m_childRuntimeSize = match m_childRuntimeSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "childRuntimeSize",
                                ),
                            );
                        }
                    };
                    let m_childNumSolverResults = match m_childNumSolverResults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "childNumSolverResults",
                                ),
                            );
                        }
                    };
                    let m_solverResultLimit = match m_solverResultLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverResultLimit",
                                ),
                            );
                        }
                    };
                    let m_removeWhenBroken = match m_removeWhenBroken {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "removeWhenBroken",
                                ),
                            );
                        }
                    };
                    let m_revertBackVelocityOnBreak = match m_revertBackVelocityOnBreak {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "revertBackVelocityOnBreak",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpBreakableConstraintData {
                        __ptr,
                        parent,
                        m_atoms,
                        m_constraintData,
                        m_childRuntimeSize,
                        m_childNumSolverResults,
                        m_solverResultLimit,
                        m_removeWhenBroken,
                        m_revertBackVelocityOnBreak,
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
                    let mut m_revertBackVelocityOnBreak: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_removeWhenBroken: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_solverResultLimit: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_childNumSolverResults: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_childRuntimeSize: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_constraintData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_atoms: _serde::__private::Option<hkpBridgeAtoms> = _serde::__private::None;
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
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_revertBackVelocityOnBreak => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_revertBackVelocityOnBreak,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "revertBackVelocityOnBreak",
                                        ),
                                    );
                                }
                                m_revertBackVelocityOnBreak = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_removeWhenBroken => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_removeWhenBroken) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "removeWhenBroken",
                                        ),
                                    );
                                }
                                m_removeWhenBroken = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            #[cfg(feature = "strict")]
                                            return _serde::__private::Err(__err);
                                            #[cfg(not(feature = "strict"))] Default::default()
                                        }
                                    },
                                );
                            }
                            __Field::m_solverResultLimit => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_solverResultLimit,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverResultLimit",
                                        ),
                                    );
                                }
                                m_solverResultLimit = _serde::__private::Some(
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
                            __Field::m_childNumSolverResults => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_childNumSolverResults,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "childNumSolverResults",
                                        ),
                                    );
                                }
                                m_childNumSolverResults = _serde::__private::Some(
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
                            __Field::m_childRuntimeSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_childRuntimeSize) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "childRuntimeSize",
                                        ),
                                    );
                                }
                                m_childRuntimeSize = _serde::__private::Some(
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
                            __Field::m_constraintData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_constraintData) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintData",
                                        ),
                                    );
                                }
                                m_constraintData = _serde::__private::Some(
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
                                    match __A::next_value::<hkpBridgeAtoms>(&mut __map) {
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
                    let m_revertBackVelocityOnBreak = match m_revertBackVelocityOnBreak {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "revertBackVelocityOnBreak",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_removeWhenBroken = match m_removeWhenBroken {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "removeWhenBroken",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_solverResultLimit = match m_solverResultLimit {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "solverResultLimit",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_childNumSolverResults = match m_childNumSolverResults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "childNumSolverResults",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_childRuntimeSize = match m_childRuntimeSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "childRuntimeSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constraintData = match m_constraintData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintData",
                                ),
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
                    _serde::__private::Ok(hkpBreakableConstraintData {
                        __ptr,
                        parent,
                        m_atoms,
                        m_constraintData,
                        m_childRuntimeSize,
                        m_childNumSolverResults,
                        m_solverResultLimit,
                        m_removeWhenBroken,
                        m_revertBackVelocityOnBreak,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "atoms",
                "constraintData",
                "childRuntimeSize",
                "childNumSolverResults",
                "solverResultLimit",
                "removeWhenBroken",
                "revertBackVelocityOnBreak",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpBreakableConstraintData",
                FIELDS,
                __hkpBreakableConstraintDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpBreakableConstraintData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
