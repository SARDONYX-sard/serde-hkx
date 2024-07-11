use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbCompiledExpressionSetToken`
/// -         version: `0`
/// -       signature: `0xc6aaccc8`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCompiledExpressionSetToken {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkReal`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_data: f32,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum TokenType`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: TokenType,
    /// # C++ Info
    /// -          name: `operator`(ctype: `enum Operator`)
    /// -        offset:   5(x86)/  5(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_operator: Operator,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbCompiledExpressionSetToken {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCompiledExpressionSetToken"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc6aaccc8)
        }
    }
    impl _serde::Serialize for hkbCompiledExpressionSetToken {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc6aaccc8)));
            let mut serializer = __serializer
                .serialize_struct("hkbCompiledExpressionSetToken", class_meta)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("operator", &self.m_operator)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_data,
    m_type,
    m_operator,
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
            "data" => Ok(__Field::m_data),
            "type" => Ok(__Field::m_type),
            "operator" => Ok(__Field::m_operator),
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
pub(super) struct __hkbCompiledExpressionSetTokenVisitor<'de> {
    marker: core::marker::PhantomData<hkbCompiledExpressionSetToken>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbCompiledExpressionSetTokenVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbCompiledExpressionSetToken, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbCompiledExpressionSetToken>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbCompiledExpressionSetTokenVisitor<'de> {
    type Value = hkbCompiledExpressionSetToken;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbCompiledExpressionSetToken",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_data: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_type: _serde::__private::Option<TokenType> = _serde::__private::None;
        let mut m_operator: _serde::__private::Option<Operator> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                        );
                    }
                    m_data = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_type) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("type"),
                        );
                    }
                    m_type = _serde::__private::Some(
                        match __A::next_value::<TokenType>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_operator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "operator",
                            ),
                        );
                    }
                    m_operator = _serde::__private::Some(
                        match __A::next_value::<Operator>(&mut __map) {
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
        __A::pad(&mut __map, 2usize, 2usize)?;
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_operator = match m_operator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("operator"),
                );
            }
        };
        _serde::__private::Ok(hkbCompiledExpressionSetToken {
            __ptr: __A::class_ptr(&mut __map),
            m_data,
            m_type,
            m_operator,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_data: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_type: _serde::__private::Option<TokenType> = _serde::__private::None;
        let mut m_operator: _serde::__private::Option<Operator> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_data => {
                        if _serde::__private::Option::is_some(&m_data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                            );
                        }
                        m_data = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_type => {
                        if _serde::__private::Option::is_some(&m_type) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                            );
                        }
                        m_type = _serde::__private::Some(
                            match __A::next_value::<TokenType>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_operator => {
                        if _serde::__private::Option::is_some(&m_operator) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "operator",
                                ),
                            );
                        }
                        m_operator = _serde::__private::Some(
                            match __A::next_value::<Operator>(&mut __map) {
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
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        let m_type = match m_type {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("type"),
                );
            }
        };
        let m_operator = match m_operator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("operator"),
                );
            }
        };
        _serde::__private::Ok(hkbCompiledExpressionSetToken {
            __ptr: __A::class_ptr(&mut __map),
            m_data,
            m_type,
            m_operator,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCompiledExpressionSetToken {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["data", "type", "operator"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCompiledExpressionSetToken",
                FIELDS,
                __hkbCompiledExpressionSetTokenVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCompiledExpressionSetToken,
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
pub enum TokenType {
    #[default]
    TOKEN_TYPE_NONE = 0isize,
    TOKEN_TYPE_OPERATOR = 1isize,
    TOKEN_TYPE_NUMBER = 2isize,
    TOKEN_TYPE_VARIABLE_INDEX = 3isize,
    TOKEN_TYPE_OPENING_PAREN = 4isize,
    TOKEN_TYPE_CLOSING_PAREN = 5isize,
    TOKEN_TYPE_COMMA = 6isize,
    TOKEN_TYPE_CHARACTER_PROPERTY_INDEX = 7isize,
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
pub enum Operator {
    #[default]
    OP_NOP = 0isize,
    OP_RAND01 = 1isize,
    OP_LOGICAL_NOT = 2isize,
    OP_UNARY_MINUS = 3isize,
    OP_UNARY_PLUS = 4isize,
    OP_SIN = 5isize,
    OP_COS = 6isize,
    OP_ASIN = 7isize,
    OP_ACOS = 8isize,
    OP_SQRT = 9isize,
    OP_FABS = 10isize,
    OP_CEIL = 11isize,
    OP_FLOOR = 12isize,
    OP_SQRTINV = 13isize,
    OP_MUL = 14isize,
    OP_DIV = 15isize,
    OP_ADD = 16isize,
    OP_SUB = 17isize,
    OP_LOGICAL_OR = 18isize,
    OP_LOGICAL_AND = 19isize,
    OP_EQ = 20isize,
    OP_NEQ = 21isize,
    OP_LT = 22isize,
    OP_GT = 23isize,
    OP_LEQ = 24isize,
    OP_GEQ = 25isize,
    OP_POW = 26isize,
    OP_MAX2 = 27isize,
    OP_MIN2 = 28isize,
    OP_RANDRANGE = 29isize,
    OP_ATAN2APPROX = 30isize,
    OP_CLAMP = 31isize,
    OP_MOD = 32isize,
    OP_DEG2RAD = 33isize,
    OP_RAD2DEG = 34isize,
    OP_COSD = 35isize,
    OP_SIND = 36isize,
    OP_ACOSD = 37isize,
    OP_ASIND = 38isize,
    OP_ATAN2APPROXD = 39isize,
    OP_SIGN = 40isize,
    OP_LERP = 41isize,
    OP_CLERP = 42isize,
    OP_COND = 43isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TokenType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TOKEN_TYPE_NONE => {
                    __serializer.serialize_field("TOKEN_TYPE_NONE", &0u64)
                }
                Self::TOKEN_TYPE_OPERATOR => {
                    __serializer.serialize_field("TOKEN_TYPE_OPERATOR", &1u64)
                }
                Self::TOKEN_TYPE_NUMBER => {
                    __serializer.serialize_field("TOKEN_TYPE_NUMBER", &2u64)
                }
                Self::TOKEN_TYPE_VARIABLE_INDEX => {
                    __serializer.serialize_field("TOKEN_TYPE_VARIABLE_INDEX", &3u64)
                }
                Self::TOKEN_TYPE_OPENING_PAREN => {
                    __serializer.serialize_field("TOKEN_TYPE_OPENING_PAREN", &4u64)
                }
                Self::TOKEN_TYPE_CLOSING_PAREN => {
                    __serializer.serialize_field("TOKEN_TYPE_CLOSING_PAREN", &5u64)
                }
                Self::TOKEN_TYPE_COMMA => {
                    __serializer.serialize_field("TOKEN_TYPE_COMMA", &6u64)
                }
                Self::TOKEN_TYPE_CHARACTER_PROPERTY_INDEX => {
                    __serializer
                        .serialize_field("TOKEN_TYPE_CHARACTER_PROPERTY_INDEX", &7u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum TokenType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Operator {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::OP_NOP => __serializer.serialize_field("OP_NOP", &0u64),
                Self::OP_RAND01 => __serializer.serialize_field("OP_RAND01", &1u64),
                Self::OP_LOGICAL_NOT => {
                    __serializer.serialize_field("OP_LOGICAL_NOT", &2u64)
                }
                Self::OP_UNARY_MINUS => {
                    __serializer.serialize_field("OP_UNARY_MINUS", &3u64)
                }
                Self::OP_UNARY_PLUS => {
                    __serializer.serialize_field("OP_UNARY_PLUS", &4u64)
                }
                Self::OP_SIN => __serializer.serialize_field("OP_SIN", &5u64),
                Self::OP_COS => __serializer.serialize_field("OP_COS", &6u64),
                Self::OP_ASIN => __serializer.serialize_field("OP_ASIN", &7u64),
                Self::OP_ACOS => __serializer.serialize_field("OP_ACOS", &8u64),
                Self::OP_SQRT => __serializer.serialize_field("OP_SQRT", &9u64),
                Self::OP_FABS => __serializer.serialize_field("OP_FABS", &10u64),
                Self::OP_CEIL => __serializer.serialize_field("OP_CEIL", &11u64),
                Self::OP_FLOOR => __serializer.serialize_field("OP_FLOOR", &12u64),
                Self::OP_SQRTINV => __serializer.serialize_field("OP_SQRTINV", &13u64),
                Self::OP_MUL => __serializer.serialize_field("OP_MUL", &14u64),
                Self::OP_DIV => __serializer.serialize_field("OP_DIV", &15u64),
                Self::OP_ADD => __serializer.serialize_field("OP_ADD", &16u64),
                Self::OP_SUB => __serializer.serialize_field("OP_SUB", &17u64),
                Self::OP_LOGICAL_OR => {
                    __serializer.serialize_field("OP_LOGICAL_OR", &18u64)
                }
                Self::OP_LOGICAL_AND => {
                    __serializer.serialize_field("OP_LOGICAL_AND", &19u64)
                }
                Self::OP_EQ => __serializer.serialize_field("OP_EQ", &20u64),
                Self::OP_NEQ => __serializer.serialize_field("OP_NEQ", &21u64),
                Self::OP_LT => __serializer.serialize_field("OP_LT", &22u64),
                Self::OP_GT => __serializer.serialize_field("OP_GT", &23u64),
                Self::OP_LEQ => __serializer.serialize_field("OP_LEQ", &24u64),
                Self::OP_GEQ => __serializer.serialize_field("OP_GEQ", &25u64),
                Self::OP_POW => __serializer.serialize_field("OP_POW", &26u64),
                Self::OP_MAX2 => __serializer.serialize_field("OP_MAX2", &27u64),
                Self::OP_MIN2 => __serializer.serialize_field("OP_MIN2", &28u64),
                Self::OP_RANDRANGE => {
                    __serializer.serialize_field("OP_RANDRANGE", &29u64)
                }
                Self::OP_ATAN2APPROX => {
                    __serializer.serialize_field("OP_ATAN2APPROX", &30u64)
                }
                Self::OP_CLAMP => __serializer.serialize_field("OP_CLAMP", &31u64),
                Self::OP_MOD => __serializer.serialize_field("OP_MOD", &32u64),
                Self::OP_DEG2RAD => __serializer.serialize_field("OP_DEG2RAD", &33u64),
                Self::OP_RAD2DEG => __serializer.serialize_field("OP_RAD2DEG", &34u64),
                Self::OP_COSD => __serializer.serialize_field("OP_COSD", &35u64),
                Self::OP_SIND => __serializer.serialize_field("OP_SIND", &36u64),
                Self::OP_ACOSD => __serializer.serialize_field("OP_ACOSD", &37u64),
                Self::OP_ASIND => __serializer.serialize_field("OP_ASIND", &38u64),
                Self::OP_ATAN2APPROXD => {
                    __serializer.serialize_field("OP_ATAN2APPROXD", &39u64)
                }
                Self::OP_SIGN => __serializer.serialize_field("OP_SIGN", &40u64),
                Self::OP_LERP => __serializer.serialize_field("OP_LERP", &41u64),
                Self::OP_CLERP => __serializer.serialize_field("OP_CLERP", &42u64),
                Self::OP_COND => __serializer.serialize_field("OP_COND", &43u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum Operator to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for TokenType {
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
                        5i8 => _serde::__private::Ok(__Field::__field5),
                        6i8 => _serde::__private::Ok(__Field::__field6),
                        7i8 => _serde::__private::Ok(__Field::__field7),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7",
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
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_OPERATOR") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_NUMBER") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_VARIABLE_INDEX") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4"
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_OPENING_PAREN") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_CLOSING_PAREN") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("TOKEN_TYPE_COMMA") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v
                                    .eq_ignore_ascii_case(
                                        "TOKEN_TYPE_CHARACTER_PROPERTY_INDEX",
                                    ) => _serde::__private::Ok(__Field::__field7),
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
                marker: _serde::__private::PhantomData<TokenType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TokenType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TokenType",
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
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_OPERATOR)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_NUMBER)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_VARIABLE_INDEX)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_OPENING_PAREN)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_CLOSING_PAREN)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TokenType::TOKEN_TYPE_COMMA)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                TokenType::TOKEN_TYPE_CHARACTER_PROPERTY_INDEX,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TOKEN_TYPE_NONE",
                "TOKEN_TYPE_OPERATOR",
                "TOKEN_TYPE_NUMBER",
                "TOKEN_TYPE_VARIABLE_INDEX",
                "TOKEN_TYPE_OPENING_PAREN",
                "TOKEN_TYPE_CLOSING_PAREN",
                "TOKEN_TYPE_COMMA",
                "TOKEN_TYPE_CHARACTER_PROPERTY_INDEX",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TokenType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TokenType>,
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
    impl<'de> _serde::Deserialize<'de> for Operator {
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
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
                __field15,
                __field16,
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
                __field23,
                __field24,
                __field25,
                __field26,
                __field27,
                __field28,
                __field29,
                __field30,
                __field31,
                __field32,
                __field33,
                __field34,
                __field35,
                __field36,
                __field37,
                __field38,
                __field39,
                __field40,
                __field41,
                __field42,
                __field43,
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
                        5i8 => _serde::__private::Ok(__Field::__field5),
                        6i8 => _serde::__private::Ok(__Field::__field6),
                        7i8 => _serde::__private::Ok(__Field::__field7),
                        8i8 => _serde::__private::Ok(__Field::__field8),
                        9i8 => _serde::__private::Ok(__Field::__field9),
                        10i8 => _serde::__private::Ok(__Field::__field10),
                        11i8 => _serde::__private::Ok(__Field::__field11),
                        12i8 => _serde::__private::Ok(__Field::__field12),
                        13i8 => _serde::__private::Ok(__Field::__field13),
                        14i8 => _serde::__private::Ok(__Field::__field14),
                        15i8 => _serde::__private::Ok(__Field::__field15),
                        16i8 => _serde::__private::Ok(__Field::__field16),
                        17i8 => _serde::__private::Ok(__Field::__field17),
                        18i8 => _serde::__private::Ok(__Field::__field18),
                        19i8 => _serde::__private::Ok(__Field::__field19),
                        20i8 => _serde::__private::Ok(__Field::__field20),
                        21i8 => _serde::__private::Ok(__Field::__field21),
                        22i8 => _serde::__private::Ok(__Field::__field22),
                        23i8 => _serde::__private::Ok(__Field::__field23),
                        24i8 => _serde::__private::Ok(__Field::__field24),
                        25i8 => _serde::__private::Ok(__Field::__field25),
                        26i8 => _serde::__private::Ok(__Field::__field26),
                        27i8 => _serde::__private::Ok(__Field::__field27),
                        28i8 => _serde::__private::Ok(__Field::__field28),
                        29i8 => _serde::__private::Ok(__Field::__field29),
                        30i8 => _serde::__private::Ok(__Field::__field30),
                        31i8 => _serde::__private::Ok(__Field::__field31),
                        32i8 => _serde::__private::Ok(__Field::__field32),
                        33i8 => _serde::__private::Ok(__Field::__field33),
                        34i8 => _serde::__private::Ok(__Field::__field34),
                        35i8 => _serde::__private::Ok(__Field::__field35),
                        36i8 => _serde::__private::Ok(__Field::__field36),
                        37i8 => _serde::__private::Ok(__Field::__field37),
                        38i8 => _serde::__private::Ok(__Field::__field38),
                        39i8 => _serde::__private::Ok(__Field::__field39),
                        40i8 => _serde::__private::Ok(__Field::__field40),
                        41i8 => _serde::__private::Ok(__Field::__field41),
                        42i8 => _serde::__private::Ok(__Field::__field42),
                        43i8 => _serde::__private::Ok(__Field::__field43),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43",
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
                            v if v == "0" || v.eq_ignore_ascii_case("OP_NOP") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("OP_RAND01") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("OP_LOGICAL_NOT") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("OP_UNARY_MINUS") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("OP_UNARY_PLUS") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("OP_SIN") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("OP_COS") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7" || v.eq_ignore_ascii_case("OP_ASIN") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("OP_ACOS") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9" || v.eq_ignore_ascii_case("OP_SQRT") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10" || v.eq_ignore_ascii_case("OP_FABS") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11" || v.eq_ignore_ascii_case("OP_CEIL") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12" || v.eq_ignore_ascii_case("OP_FLOOR") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "13" || v.eq_ignore_ascii_case("OP_SQRTINV") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "14" || v.eq_ignore_ascii_case("OP_MUL") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "15" || v.eq_ignore_ascii_case("OP_DIV") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "16" || v.eq_ignore_ascii_case("OP_ADD") => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            v if v == "17" || v.eq_ignore_ascii_case("OP_SUB") => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            v if v == "18" || v.eq_ignore_ascii_case("OP_LOGICAL_OR") => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            v if v == "19"
                                || v.eq_ignore_ascii_case("OP_LOGICAL_AND") => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            v if v == "20" || v.eq_ignore_ascii_case("OP_EQ") => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            v if v == "21" || v.eq_ignore_ascii_case("OP_NEQ") => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            v if v == "22" || v.eq_ignore_ascii_case("OP_LT") => {
                                _serde::__private::Ok(__Field::__field22)
                            }
                            v if v == "23" || v.eq_ignore_ascii_case("OP_GT") => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            v if v == "24" || v.eq_ignore_ascii_case("OP_LEQ") => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            v if v == "25" || v.eq_ignore_ascii_case("OP_GEQ") => {
                                _serde::__private::Ok(__Field::__field25)
                            }
                            v if v == "26" || v.eq_ignore_ascii_case("OP_POW") => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            v if v == "27" || v.eq_ignore_ascii_case("OP_MAX2") => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            v if v == "28" || v.eq_ignore_ascii_case("OP_MIN2") => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            v if v == "29" || v.eq_ignore_ascii_case("OP_RANDRANGE") => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            v if v == "30"
                                || v.eq_ignore_ascii_case("OP_ATAN2APPROX") => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            v if v == "31" || v.eq_ignore_ascii_case("OP_CLAMP") => {
                                _serde::__private::Ok(__Field::__field31)
                            }
                            v if v == "32" || v.eq_ignore_ascii_case("OP_MOD") => {
                                _serde::__private::Ok(__Field::__field32)
                            }
                            v if v == "33" || v.eq_ignore_ascii_case("OP_DEG2RAD") => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            v if v == "34" || v.eq_ignore_ascii_case("OP_RAD2DEG") => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            v if v == "35" || v.eq_ignore_ascii_case("OP_COSD") => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            v if v == "36" || v.eq_ignore_ascii_case("OP_SIND") => {
                                _serde::__private::Ok(__Field::__field36)
                            }
                            v if v == "37" || v.eq_ignore_ascii_case("OP_ACOSD") => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            v if v == "38" || v.eq_ignore_ascii_case("OP_ASIND") => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            v if v == "39"
                                || v.eq_ignore_ascii_case("OP_ATAN2APPROXD") => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            v if v == "40" || v.eq_ignore_ascii_case("OP_SIGN") => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            v if v == "41" || v.eq_ignore_ascii_case("OP_LERP") => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            v if v == "42" || v.eq_ignore_ascii_case("OP_CLERP") => {
                                _serde::__private::Ok(__Field::__field42)
                            }
                            v if v == "43" || v.eq_ignore_ascii_case("OP_COND") => {
                                _serde::__private::Ok(__Field::__field43)
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
                marker: _serde::__private::PhantomData<Operator>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Operator;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Operator")
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
                            _serde::__private::Ok(Operator::OP_NOP)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_RAND01)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_LOGICAL_NOT)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_UNARY_MINUS)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_UNARY_PLUS)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_SIN)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_COS)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ASIN)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ACOS)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_SQRT)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_FABS)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_CEIL)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_FLOOR)
                        }
                        (__Field::__field13, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_SQRTINV)
                        }
                        (__Field::__field14, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_MUL)
                        }
                        (__Field::__field15, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_DIV)
                        }
                        (__Field::__field16, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ADD)
                        }
                        (__Field::__field17, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_SUB)
                        }
                        (__Field::__field18, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_LOGICAL_OR)
                        }
                        (__Field::__field19, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_LOGICAL_AND)
                        }
                        (__Field::__field20, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_EQ)
                        }
                        (__Field::__field21, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_NEQ)
                        }
                        (__Field::__field22, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_LT)
                        }
                        (__Field::__field23, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_GT)
                        }
                        (__Field::__field24, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_LEQ)
                        }
                        (__Field::__field25, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_GEQ)
                        }
                        (__Field::__field26, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_POW)
                        }
                        (__Field::__field27, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_MAX2)
                        }
                        (__Field::__field28, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_MIN2)
                        }
                        (__Field::__field29, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_RANDRANGE)
                        }
                        (__Field::__field30, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ATAN2APPROX)
                        }
                        (__Field::__field31, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_CLAMP)
                        }
                        (__Field::__field32, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_MOD)
                        }
                        (__Field::__field33, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_DEG2RAD)
                        }
                        (__Field::__field34, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_RAD2DEG)
                        }
                        (__Field::__field35, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_COSD)
                        }
                        (__Field::__field36, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_SIND)
                        }
                        (__Field::__field37, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ACOSD)
                        }
                        (__Field::__field38, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ASIND)
                        }
                        (__Field::__field39, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_ATAN2APPROXD)
                        }
                        (__Field::__field40, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_SIGN)
                        }
                        (__Field::__field41, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_LERP)
                        }
                        (__Field::__field42, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_CLERP)
                        }
                        (__Field::__field43, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Operator::OP_COND)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "OP_NOP",
                "OP_RAND01",
                "OP_LOGICAL_NOT",
                "OP_UNARY_MINUS",
                "OP_UNARY_PLUS",
                "OP_SIN",
                "OP_COS",
                "OP_ASIN",
                "OP_ACOS",
                "OP_SQRT",
                "OP_FABS",
                "OP_CEIL",
                "OP_FLOOR",
                "OP_SQRTINV",
                "OP_MUL",
                "OP_DIV",
                "OP_ADD",
                "OP_SUB",
                "OP_LOGICAL_OR",
                "OP_LOGICAL_AND",
                "OP_EQ",
                "OP_NEQ",
                "OP_LT",
                "OP_GT",
                "OP_LEQ",
                "OP_GEQ",
                "OP_POW",
                "OP_MAX2",
                "OP_MIN2",
                "OP_RANDRANGE",
                "OP_ATAN2APPROX",
                "OP_CLAMP",
                "OP_MOD",
                "OP_DEG2RAD",
                "OP_RAD2DEG",
                "OP_COSD",
                "OP_SIND",
                "OP_ACOSD",
                "OP_ASIND",
                "OP_ATAN2APPROXD",
                "OP_SIGN",
                "OP_LERP",
                "OP_CLERP",
                "OP_COND",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Operator",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Operator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
