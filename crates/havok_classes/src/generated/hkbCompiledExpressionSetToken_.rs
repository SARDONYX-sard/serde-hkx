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
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbCompiledExpressionSetToken {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbCompiledExpressionSetToken"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3333082312u32)
        }
    }
    impl __serde::Serialize for hkbCompiledExpressionSetToken {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
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
