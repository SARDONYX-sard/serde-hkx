use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConstraintInstance`
/// - version: `1`
/// - signature: `0x34eba5f`
/// - size: ` 56`(x86)/`112`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConstraintInstance<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `owner`(ctype: `void*`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "owner"))]
    #[cfg_attr(feature = "serde", serde(rename = "owner"))]
    pub m_owner: Pointer<'a>,
    /// # C++ Info
    /// - name: `data`(ctype: `struct hkpConstraintData*`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "data"))]
    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    pub m_data: Pointer<'a>,
    /// # C++ Info
    /// - name: `constraintModifiers`(ctype: `struct hkpModifierConstraintAtom*`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "constraintModifiers"))]
    #[cfg_attr(feature = "serde", serde(rename = "constraintModifiers"))]
    pub m_constraintModifiers: Pointer<'a>,
    /// # C++ Info
    /// - name: `entities`(ctype: `struct hkpEntity*`)
    /// - offset: ` 20`(x86)/` 40`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "entities"))]
    #[cfg_attr(feature = "serde", serde(rename = "entities"))]
    pub m_entities: [Pointer<'a>; 2usize],
    /// # C++ Info
    /// - name: `priority`(ctype: `enum ConstraintPriority`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "priority"))]
    #[cfg_attr(feature = "serde", serde(rename = "priority"))]
    pub m_priority: ConstraintPriority,
    /// # C++ Info
    /// - name: `wantRuntime`(ctype: `hkBool`)
    /// - offset: ` 29`(x86)/` 57`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "wantRuntime"))]
    #[cfg_attr(feature = "serde", serde(rename = "wantRuntime"))]
    pub m_wantRuntime: bool,
    /// # C++ Info
    /// - name: `destructionRemapInfo`(ctype: `enum OnDestructionRemapInfo`)
    /// - offset: ` 30`(x86)/` 58`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "destructionRemapInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "destructionRemapInfo"))]
    pub m_destructionRemapInfo: OnDestructionRemapInfo,
    /// # C++ Info
    /// - name: `listeners`(ctype: `struct hkpConstraintInstanceSmallArraySerializeOverrideType`)
    /// - offset: ` 32`(x86)/` 64`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "listeners"))]
    #[cfg_attr(feature = "serde", serde(rename = "listeners"))]
    pub m_listeners: hkpConstraintInstanceSmallArraySerializeOverrideType<'a>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: ` 40`(x86)/` 80`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `userData`(ctype: `hkUlong`)
    /// - offset: ` 44`(x86)/` 88`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: Ulong,
    /// # C++ Info
    /// - name: `internal`(ctype: `void*`)
    /// - offset: ` 48`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "internal"))]
    #[cfg_attr(feature = "serde", serde(rename = "internal"))]
    pub m_internal: Pointer<'a>,
    /// # C++ Info
    /// - name: `uid`(ctype: `hkUint32`)
    /// - offset: ` 52`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "uid"))]
    #[cfg_attr(feature = "serde", serde(rename = "uid"))]
    pub m_uid: U32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpConstraintInstance<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConstraintInstance"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x34eba5f)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_owner);
            v.push(&self.m_data);
            v.push(&self.m_constraintModifiers);
            v.extend(self.m_entities.iter());
            v.extend(self.m_listeners.deps_indexes());
            v.push(&self.m_internal);
            v
        }
    }
    impl<'a> _serde::Serialize for hkpConstraintInstance<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x34eba5f)));
            let mut serializer = __serializer
                .serialize_struct("hkpConstraintInstance", class_meta, (56u64, 112u64))?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("owner", &self.m_owner)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer
                .serialize_field("constraintModifiers", &self.m_constraintModifiers)?;
            serializer
                .serialize_fixed_array_field(
                    "entities",
                    self.m_entities.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("priority", &self.m_priority)?;
            serializer.serialize_field("wantRuntime", &self.m_wantRuntime)?;
            serializer
                .serialize_field("destructionRemapInfo", &self.m_destructionRemapInfo)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.skip_field("listeners", &self.m_listeners)?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.skip_field("internal", &self.m_internal)?;
            serializer.skip_field("uid", &self.m_uid)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConstraintInstance<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_data,
                m_constraintModifiers,
                m_entities,
                m_priority,
                m_wantRuntime,
                m_destructionRemapInfo,
                m_name,
                m_userData,
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
                        "data" => Ok(__Field::m_data),
                        "constraintModifiers" => Ok(__Field::m_constraintModifiers),
                        "entities" => Ok(__Field::m_entities),
                        "priority" => Ok(__Field::m_priority),
                        "wantRuntime" => Ok(__Field::m_wantRuntime),
                        "destructionRemapInfo" => Ok(__Field::m_destructionRemapInfo),
                        "name" => Ok(__Field::m_name),
                        "userData" => Ok(__Field::m_userData),
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
            struct __hkpConstraintInstanceVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpConstraintInstance<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpConstraintInstanceVisitor<'de> {
                type Value = hkpConstraintInstance<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpConstraintInstance",
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
                    let mut m_owner: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_constraintModifiers: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_entities: _serde::__private::Option<
                        [Pointer<'de>; 2usize],
                    > = _serde::__private::None;
                    let mut m_priority: _serde::__private::Option<ConstraintPriority> = _serde::__private::None;
                    let mut m_wantRuntime: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_destructionRemapInfo: _serde::__private::Option<
                        OnDestructionRemapInfo,
                    > = _serde::__private::None;
                    let mut m_listeners: _serde::__private::Option<
                        hkpConstraintInstanceSmallArraySerializeOverrideType,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_internal: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_uid: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    for i in 0..12usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_owner) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("owner"),
                                    );
                                }
                                m_owner = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_data) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_constraintModifiers,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintModifiers",
                                        ),
                                    );
                                }
                                m_constraintModifiers = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_entities) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "entities",
                                        ),
                                    );
                                }
                                m_entities = _serde::__private::Some(
                                    match __A::next_value::<
                                        [Pointer<'de>; 2usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_priority) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "priority",
                                        ),
                                    );
                                }
                                m_priority = _serde::__private::Some(
                                    match __A::next_value::<ConstraintPriority>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_wantRuntime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wantRuntime",
                                        ),
                                    );
                                }
                                m_wantRuntime = _serde::__private::Some(
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
                                    &m_destructionRemapInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "destructionRemapInfo",
                                        ),
                                    );
                                }
                                m_destructionRemapInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        OnDestructionRemapInfo,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_listeners) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "listeners",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 1usize, 5usize)?;
                                m_listeners = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpConstraintInstanceSmallArraySerializeOverrideType,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_name) {
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
                            9usize => {
                                if _serde::__private::Option::is_some(&m_userData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_internal) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "internal",
                                        ),
                                    );
                                }
                                m_internal = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_uid) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("uid"),
                                    );
                                }
                                m_uid = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_owner = match m_owner {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("owner"),
                            );
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                        }
                    };
                    let m_constraintModifiers = match m_constraintModifiers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintModifiers",
                                ),
                            );
                        }
                    };
                    let m_entities = match m_entities {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("entities"),
                            );
                        }
                    };
                    let m_priority = match m_priority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("priority"),
                            );
                        }
                    };
                    let m_wantRuntime = match m_wantRuntime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wantRuntime",
                                ),
                            );
                        }
                    };
                    let m_destructionRemapInfo = match m_destructionRemapInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "destructionRemapInfo",
                                ),
                            );
                        }
                    };
                    let m_listeners = match m_listeners {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "listeners",
                                ),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                        }
                    };
                    let m_internal = match m_internal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("internal"),
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
                    _serde::__private::Ok(hkpConstraintInstance {
                        __ptr,
                        parent,
                        m_owner,
                        m_data,
                        m_constraintModifiers,
                        m_entities,
                        m_priority,
                        m_wantRuntime,
                        m_destructionRemapInfo,
                        m_listeners,
                        m_name,
                        m_userData,
                        m_internal,
                        m_uid,
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
                    let mut m_data: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_constraintModifiers: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_entities: _serde::__private::Option<
                        [Pointer<'de>; 2usize],
                    > = _serde::__private::None;
                    let mut m_priority: _serde::__private::Option<ConstraintPriority> = _serde::__private::None;
                    let mut m_wantRuntime: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_destructionRemapInfo: _serde::__private::Option<
                        OnDestructionRemapInfo,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_data => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_data) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_constraintModifiers => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_constraintModifiers,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "constraintModifiers",
                                        ),
                                    );
                                }
                                m_constraintModifiers = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_entities => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_entities) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "entities",
                                        ),
                                    );
                                }
                                m_entities = _serde::__private::Some(
                                    match __A::next_value::<
                                        [Pointer<'de>; 2usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_priority => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_priority) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "priority",
                                        ),
                                    );
                                }
                                m_priority = _serde::__private::Some(
                                    match __A::next_value::<ConstraintPriority>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_wantRuntime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_wantRuntime) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "wantRuntime",
                                        ),
                                    );
                                }
                                m_wantRuntime = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_destructionRemapInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_destructionRemapInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "destructionRemapInfo",
                                        ),
                                    );
                                }
                                m_destructionRemapInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        OnDestructionRemapInfo,
                                    >(&mut __map) {
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
                                    match __A::next_value::<Ulong>(&mut __map) {
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
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_constraintModifiers = match m_constraintModifiers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "constraintModifiers",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_entities = match m_entities {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("entities"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_priority = match m_priority {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("priority"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_wantRuntime = match m_wantRuntime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "wantRuntime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_destructionRemapInfo = match m_destructionRemapInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "destructionRemapInfo",
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
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpConstraintInstance {
                        __ptr: __ptr.clone(),
                        parent,
                        m_data,
                        m_constraintModifiers,
                        m_entities,
                        m_priority,
                        m_wantRuntime,
                        m_destructionRemapInfo,
                        m_name,
                        m_userData,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "owner",
                "data",
                "constraintModifiers",
                "entities",
                "priority",
                "wantRuntime",
                "destructionRemapInfo",
                "listeners",
                "name",
                "userData",
                "internal",
                "uid",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConstraintInstance",
                FIELDS,
                __hkpConstraintInstanceVisitor {
                    marker: _serde::__private::PhantomData::<hkpConstraintInstance>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `ConstraintPriority`(ctype: `hkEnum<ConstraintPriority, hkUint8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
pub enum ConstraintPriority {
    #[default]
    PRIORITY_INVALID = 0isize,
    PRIORITY_PSI = 1isize,
    PRIORITY_SIMPLIFIED_TOI_UNUSED = 2isize,
    PRIORITY_TOI = 3isize,
    PRIORITY_TOI_HIGHER = 4isize,
    PRIORITY_TOI_FORCED = 5isize,
    NUM_PRIORITIES = 6isize,
}
/// # C++ Info
/// - name: `OnDestructionRemapInfo`(ctype: `hkEnum<OnDestructionRemapInfo, hkUint8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
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
pub enum OnDestructionRemapInfo {
    #[default]
    ON_DESTRUCTION_REMAP = 0isize,
    ON_DESTRUCTION_REMOVE = 1isize,
    ON_DESTRUCTION_RESET_REMOVE = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ConstraintPriority {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::PRIORITY_INVALID => {
                    __serializer.serialize_field("PRIORITY_INVALID", &0u64)
                }
                Self::PRIORITY_PSI => __serializer.serialize_field("PRIORITY_PSI", &1u64),
                Self::PRIORITY_SIMPLIFIED_TOI_UNUSED => {
                    __serializer.serialize_field("PRIORITY_SIMPLIFIED_TOI_UNUSED", &2u64)
                }
                Self::PRIORITY_TOI => __serializer.serialize_field("PRIORITY_TOI", &3u64),
                Self::PRIORITY_TOI_HIGHER => {
                    __serializer.serialize_field("PRIORITY_TOI_HIGHER", &4u64)
                }
                Self::PRIORITY_TOI_FORCED => {
                    __serializer.serialize_field("PRIORITY_TOI_FORCED", &5u64)
                }
                Self::NUM_PRIORITIES => {
                    __serializer.serialize_field("NUM_PRIORITIES", &6u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ConstraintPriority to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for OnDestructionRemapInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::ON_DESTRUCTION_REMAP => {
                    __serializer.serialize_field("ON_DESTRUCTION_REMAP", &0u64)
                }
                Self::ON_DESTRUCTION_REMOVE => {
                    __serializer.serialize_field("ON_DESTRUCTION_REMOVE", &1u64)
                }
                Self::ON_DESTRUCTION_RESET_REMOVE => {
                    __serializer.serialize_field("ON_DESTRUCTION_RESET_REMOVE", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum OnDestructionRemapInfo to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for ConstraintPriority {
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
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        U8::Number(3u8) => _serde::__private::Ok(__Field::__field3),
                        U8::Number(4u8) => _serde::__private::Ok(__Field::__field4),
                        U8::Number(5u8) => _serde::__private::Ok(__Field::__field5),
                        U8::Number(6u8) => _serde::__private::Ok(__Field::__field6),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6",
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
                                || v.eq_ignore_ascii_case("PRIORITY_INVALID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("PRIORITY_PSI") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case("PRIORITY_SIMPLIFIED_TOI_UNUSED") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("PRIORITY_TOI") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("PRIORITY_TOI_HIGHER") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("PRIORITY_TOI_FORCED") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("NUM_PRIORITIES") => {
                                _serde::__private::Ok(__Field::__field6)
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
                marker: _serde::__private::PhantomData<ConstraintPriority>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ConstraintPriority;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ConstraintPriority",
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
                            _serde::__private::Ok(ConstraintPriority::PRIORITY_INVALID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintPriority::PRIORITY_PSI)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintPriority::PRIORITY_SIMPLIFIED_TOI_UNUSED,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintPriority::PRIORITY_TOI)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintPriority::PRIORITY_TOI_HIGHER,
                            )
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ConstraintPriority::PRIORITY_TOI_FORCED,
                            )
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ConstraintPriority::NUM_PRIORITIES)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "PRIORITY_INVALID",
                "PRIORITY_PSI",
                "PRIORITY_SIMPLIFIED_TOI_UNUSED",
                "PRIORITY_TOI",
                "PRIORITY_TOI_HIGHER",
                "PRIORITY_TOI_FORCED",
                "NUM_PRIORITIES",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ConstraintPriority",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ConstraintPriority>,
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
    impl<'de> _serde::Deserialize<'de> for OnDestructionRemapInfo {
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
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2",
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
                                || v.eq_ignore_ascii_case("ON_DESTRUCTION_REMAP") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("ON_DESTRUCTION_REMOVE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("ON_DESTRUCTION_RESET_REMOVE") => {
                                _serde::__private::Ok(__Field::__field2)
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
                marker: _serde::__private::PhantomData<OnDestructionRemapInfo>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = OnDestructionRemapInfo;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum OnDestructionRemapInfo",
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
                                OnDestructionRemapInfo::ON_DESTRUCTION_REMAP,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OnDestructionRemapInfo::ON_DESTRUCTION_REMOVE,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                OnDestructionRemapInfo::ON_DESTRUCTION_RESET_REMOVE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "ON_DESTRUCTION_REMAP",
                "ON_DESTRUCTION_REMOVE",
                "ON_DESTRUCTION_RESET_REMOVE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "OnDestructionRemapInfo",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<OnDestructionRemapInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
