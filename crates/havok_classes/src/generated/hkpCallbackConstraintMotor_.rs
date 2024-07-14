use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCallbackConstraintMotor`
/// -         version: `0`
/// -       signature: `0xafcd79ad`
/// -          size:  40(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCallbackConstraintMotor {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpLimitedForceConstraintMotor,
    /// # C++ Info
    /// -          name: `callbackFunc`(ctype: `void*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_callbackFunc: Pointer,
    /// # C++ Info
    /// -          name: `callbackType`(ctype: `enum CallbackType`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_callbackType: CallbackType,
    /// # C++ Info
    /// -          name: `userData0`(ctype: `hkUlong`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData0: u64,
    /// # C++ Info
    /// -          name: `userData1`(ctype: `hkUlong`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData1: u64,
    /// # C++ Info
    /// -          name: `userData2`(ctype: `hkUlong`)
    /// -        offset:  36(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData2: u64,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCallbackConstraintMotor {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCallbackConstraintMotor"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xafcd79ad)
        }
    }
    impl _serde::Serialize for hkpCallbackConstraintMotor {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xafcd79ad)));
            let mut serializer = __serializer
                .serialize_struct("hkpCallbackConstraintMotor", class_meta)?;
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
            serializer.serialize_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("minForce", &self.parent.m_minForce)?;
            serializer.serialize_field("maxForce", &self.parent.m_maxForce)?;
            serializer.skip_field("callbackFunc", &self.m_callbackFunc)?;
            serializer.serialize_field("callbackType", &self.m_callbackType)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData0", &self.m_userData0)?;
            serializer.serialize_field("userData1", &self.m_userData1)?;
            serializer.serialize_field("userData2", &self.m_userData2)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_callbackFunc,
    m_callbackType,
    m_userData0,
    m_userData1,
    m_userData2,
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
            "callbackType" => Ok(__Field::m_callbackType),
            "userData0" => Ok(__Field::m_userData0),
            "userData1" => Ok(__Field::m_userData1),
            "userData2" => Ok(__Field::m_userData2),
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
pub(super) struct __hkpCallbackConstraintMotorVisitor<'de> {
    marker: core::marker::PhantomData<hkpCallbackConstraintMotor>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCallbackConstraintMotorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCallbackConstraintMotor, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpCallbackConstraintMotor>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpCallbackConstraintMotorVisitor<'de> {
    type Value = hkpCallbackConstraintMotor;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpCallbackConstraintMotor")
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
        let mut m_callbackFunc: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_callbackType: _serde::__private::Option<CallbackType> = _serde::__private::None;
        let mut m_userData0: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_userData1: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_userData2: _serde::__private::Option<u64> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_callbackFunc) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "callbackFunc",
                            ),
                        );
                    }
                    m_callbackFunc = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_callbackType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "callbackType",
                            ),
                        );
                    }
                    m_callbackType = _serde::__private::Some(
                        match __A::next_value::<CallbackType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_userData0) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userData0",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_userData0 = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_userData1) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userData1",
                            ),
                        );
                    }
                    m_userData1 = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_userData2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "userData2",
                            ),
                        );
                    }
                    m_userData2 = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
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
        let m_callbackFunc = match m_callbackFunc {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("callbackFunc"),
                );
            }
        };
        let m_callbackType = match m_callbackType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("callbackType"),
                );
            }
        };
        let m_userData0 = match m_userData0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData0"),
                );
            }
        };
        let m_userData1 = match m_userData1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData1"),
                );
            }
        };
        let m_userData2 = match m_userData2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData2"),
                );
            }
        };
        _serde::__private::Ok(hkpCallbackConstraintMotor {
            __ptr,
            parent,
            m_callbackFunc,
            m_callbackType,
            m_userData0,
            m_userData1,
            m_userData2,
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
        let parent = __hkpLimitedForceConstraintMotorVisitor::visit_as_parent(
            &mut __map,
        )?;
        let mut m_callbackType: _serde::__private::Option<CallbackType> = _serde::__private::None;
        let mut m_userData0: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_userData1: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_userData2: _serde::__private::Option<u64> = _serde::__private::None;
        for _ in 0..4usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_callbackType => {
                        if _serde::__private::Option::is_some(&m_callbackType) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "callbackType",
                                ),
                            );
                        }
                        m_callbackType = _serde::__private::Some(
                            match __A::next_value::<CallbackType>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userData0 => {
                        if _serde::__private::Option::is_some(&m_userData0) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userData0",
                                ),
                            );
                        }
                        m_userData0 = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userData1 => {
                        if _serde::__private::Option::is_some(&m_userData1) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userData1",
                                ),
                            );
                        }
                        m_userData1 = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_userData2 => {
                        if _serde::__private::Option::is_some(&m_userData2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "userData2",
                                ),
                            );
                        }
                        m_userData2 = _serde::__private::Some(
                            match __A::next_value::<u64>(&mut __map) {
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
        let m_callbackType = match m_callbackType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("callbackType"),
                );
            }
        };
        let m_userData0 = match m_userData0 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData0"),
                );
            }
        };
        let m_userData1 = match m_userData1 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData1"),
                );
            }
        };
        let m_userData2 = match m_userData2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("userData2"),
                );
            }
        };
        _serde::__private::Ok(hkpCallbackConstraintMotor {
            __ptr,
            parent,
            m_callbackType,
            m_userData0,
            m_userData1,
            m_userData2,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCallbackConstraintMotor {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "callbackFunc",
                "callbackType",
                "userData0",
                "userData1",
                "userData2",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCallbackConstraintMotor",
                FIELDS,
                __hkpCallbackConstraintMotorVisitor {
                    marker: _serde::__private::PhantomData::<hkpCallbackConstraintMotor>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT32`
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
pub enum CallbackType {
    #[default]
    CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER = 0isize,
    CALLBACK_MOTOR_TYPE_USER_0 = 1isize,
    CALLBACK_MOTOR_TYPE_USER_1 = 2isize,
    CALLBACK_MOTOR_TYPE_USER_2 = 3isize,
    CALLBACK_MOTOR_TYPE_USER_3 = 4isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CallbackType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER => {
                    __serializer
                        .serialize_field(
                            "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER",
                            &0u64,
                        )
                }
                Self::CALLBACK_MOTOR_TYPE_USER_0 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_0", &1u64)
                }
                Self::CALLBACK_MOTOR_TYPE_USER_1 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_1", &2u64)
                }
                Self::CALLBACK_MOTOR_TYPE_USER_2 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_2", &3u64)
                }
                Self::CALLBACK_MOTOR_TYPE_USER_3 => {
                    __serializer.serialize_field("CALLBACK_MOTOR_TYPE_USER_3", &4u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u32()
                .ok_or(S::Error::custom("Failed enum CallbackType to_u32"))?;
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
    impl<'de> _serde::Deserialize<'de> for CallbackType {
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
                fn visit_uint32<__E>(
                    self,
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u32 => _serde::__private::Ok(__Field::__field0),
                        1u32 => _serde::__private::Ok(__Field::__field1),
                        2u32 => _serde::__private::Ok(__Field::__field2),
                        3u32 => _serde::__private::Ok(__Field::__field3),
                        4u32 => _serde::__private::Ok(__Field::__field4),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint32(__value),
                                    &"value(u32) of variant is one of 0, 1, 2, 3, 4",
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
                                        "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v.eq_ignore_ascii_case("CALLBACK_MOTOR_TYPE_USER_0") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("CALLBACK_MOTOR_TYPE_USER_1") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("CALLBACK_MOTOR_TYPE_USER_2") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("CALLBACK_MOTOR_TYPE_USER_3") => {
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
                        _serde::de::ReadEnumSize::Uint32,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CallbackType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CallbackType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum CallbackType",
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
                                CallbackType::CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                CallbackType::CALLBACK_MOTOR_TYPE_USER_0,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                CallbackType::CALLBACK_MOTOR_TYPE_USER_1,
                            )
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                CallbackType::CALLBACK_MOTOR_TYPE_USER_2,
                            )
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                CallbackType::CALLBACK_MOTOR_TYPE_USER_3,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "CALLBACK_MOTOR_TYPE_HAVOK_DEMO_SPRING_DAMPER",
                "CALLBACK_MOTOR_TYPE_USER_0",
                "CALLBACK_MOTOR_TYPE_USER_1",
                "CALLBACK_MOTOR_TYPE_USER_2",
                "CALLBACK_MOTOR_TYPE_USER_3",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CallbackType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CallbackType>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
