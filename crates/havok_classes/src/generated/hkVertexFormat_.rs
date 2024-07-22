use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkVertexFormat`
/// - version: `0`
/// - signature: `0xf11e3ff7`
/// - size: `260`(x86)/`260`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkVertexFormat {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `elements`(ctype: `struct hkVertexFormatElement[32]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  8`(x86)/`256`(x86_64)
    pub m_elements: [hkVertexFormatElement; 32usize],
    /// # C++ Info
    /// - name: `numElements`(ctype: `hkInt32`)
    /// - offset: `256`(x86)/`256`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numElements: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkVertexFormat {
        #[inline]
        fn name(&self) -> &'static str {
            "hkVertexFormat"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf11e3ff7)
        }
    }
    impl _serde::Serialize for hkVertexFormat {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf11e3ff7)));
            let mut serializer = __serializer
                .serialize_struct("hkVertexFormat", class_meta)?;
            serializer
                .serialize_fixed_array_field("elements", self.m_elements.as_slice())?;
            serializer.pad_field([0u8; 248usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("numElements", &self.m_numElements)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkVertexFormat {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_numElements,
                m_elements,
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
                        "numElements" => Ok(__Field::m_numElements),
                        "elements" => Ok(__Field::m_elements),
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
            struct __hkVertexFormatVisitor<'de> {
                marker: _serde::__private::PhantomData<hkVertexFormat>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkVertexFormatVisitor<'de> {
                type Value = hkVertexFormat;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkVertexFormat")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_elements: _serde::__private::Option<
                        [hkVertexFormatElement; 32usize],
                    > = _serde::__private::None;
                    let mut m_numElements: _serde::__private::Option<i32> = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_elements) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elements",
                                        ),
                                    );
                                }
                                m_elements = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hkVertexFormatElement; 32usize],
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_numElements) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numElements",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 248usize, 0usize)?;
                                m_numElements = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
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
                    let m_elements = match m_elements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("elements"),
                            );
                        }
                    };
                    let m_numElements = match m_numElements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numElements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkVertexFormat {
                        __ptr,
                        m_elements,
                        m_numElements,
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
                    let mut m_numElements: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_elements: _serde::__private::Option<
                        [hkVertexFormatElement; 32usize],
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
                            __Field::m_numElements => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numElements) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numElements",
                                        ),
                                    );
                                }
                                m_numElements = _serde::__private::Some(
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
                            __Field::m_elements => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_elements) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "elements",
                                        ),
                                    );
                                }
                                m_elements = _serde::__private::Some(
                                    match __A::next_value::<
                                        [hkVertexFormatElement; 32usize],
                                    >(&mut __map) {
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
                    let m_numElements = match m_numElements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numElements",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_elements = match m_elements {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("elements"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkVertexFormat {
                        __ptr,
                        m_elements,
                        m_numElements,
                    })
                }
            }
            const FIELDS: &[&str] = &["elements", "numElements"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkVertexFormat",
                FIELDS,
                __hkVertexFormatVisitor {
                    marker: _serde::__private::PhantomData::<hkVertexFormat>,
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
pub enum ComponentType {
    #[default]
    TYPE_NONE = 0isize,
    TYPE_INT8 = 1isize,
    TYPE_UINT8 = 2isize,
    TYPE_INT16 = 3isize,
    TYPE_UINT16 = 4isize,
    TYPE_INT32 = 5isize,
    TYPE_UINT32 = 6isize,
    TYPE_UINT8_DWORD = 7isize,
    TYPE_ARGB32 = 8isize,
    TYPE_FLOAT16 = 9isize,
    TYPE_FLOAT32 = 10isize,
    TYPE_VECTOR4 = 11isize,
    TYPE_LAST = 12isize,
}
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
pub enum ComponentUsage {
    #[default]
    USAGE_NONE = 0isize,
    USAGE_POSITION = 1isize,
    USAGE_NORMAL = 2isize,
    USAGE_COLOR = 3isize,
    USAGE_TANGENT = 4isize,
    USAGE_BINORMAL = 5isize,
    USAGE_BLEND_MATRIX_INDEX = 6isize,
    USAGE_BLEND_WEIGHTS = 7isize,
    USAGE_BLEND_WEIGHTS_LAST_IMPLIED = 8isize,
    USAGE_TEX_COORD = 9isize,
    USAGE_POINT_SIZE = 10isize,
    USAGE_USER = 11isize,
    USAGE_LAST = 12isize,
}
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT8`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    HintFlags : u8 { #[doc = "1"] const FLAG_READ = 1u8; #[doc = "2"] const FLAG_WRITE =
    2u8; #[doc = "4"] const FLAG_DYNAMIC = 4u8; #[doc = "8"] const FLAG_NOT_SHARED = 8u8;
    }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ComponentType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_NONE => __serializer.serialize_field("TYPE_NONE", &0u64),
                Self::TYPE_INT8 => __serializer.serialize_field("TYPE_INT8", &1u64),
                Self::TYPE_UINT8 => __serializer.serialize_field("TYPE_UINT8", &2u64),
                Self::TYPE_INT16 => __serializer.serialize_field("TYPE_INT16", &3u64),
                Self::TYPE_UINT16 => __serializer.serialize_field("TYPE_UINT16", &4u64),
                Self::TYPE_INT32 => __serializer.serialize_field("TYPE_INT32", &5u64),
                Self::TYPE_UINT32 => __serializer.serialize_field("TYPE_UINT32", &6u64),
                Self::TYPE_UINT8_DWORD => {
                    __serializer.serialize_field("TYPE_UINT8_DWORD", &7u64)
                }
                Self::TYPE_ARGB32 => __serializer.serialize_field("TYPE_ARGB32", &8u64),
                Self::TYPE_FLOAT16 => __serializer.serialize_field("TYPE_FLOAT16", &9u64),
                Self::TYPE_FLOAT32 => {
                    __serializer.serialize_field("TYPE_FLOAT32", &10u64)
                }
                Self::TYPE_VECTOR4 => {
                    __serializer.serialize_field("TYPE_VECTOR4", &11u64)
                }
                Self::TYPE_LAST => __serializer.serialize_field("TYPE_LAST", &12u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ComponentType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ComponentUsage {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::USAGE_NONE => __serializer.serialize_field("USAGE_NONE", &0u64),
                Self::USAGE_POSITION => {
                    __serializer.serialize_field("USAGE_POSITION", &1u64)
                }
                Self::USAGE_NORMAL => __serializer.serialize_field("USAGE_NORMAL", &2u64),
                Self::USAGE_COLOR => __serializer.serialize_field("USAGE_COLOR", &3u64),
                Self::USAGE_TANGENT => {
                    __serializer.serialize_field("USAGE_TANGENT", &4u64)
                }
                Self::USAGE_BINORMAL => {
                    __serializer.serialize_field("USAGE_BINORMAL", &5u64)
                }
                Self::USAGE_BLEND_MATRIX_INDEX => {
                    __serializer.serialize_field("USAGE_BLEND_MATRIX_INDEX", &6u64)
                }
                Self::USAGE_BLEND_WEIGHTS => {
                    __serializer.serialize_field("USAGE_BLEND_WEIGHTS", &7u64)
                }
                Self::USAGE_BLEND_WEIGHTS_LAST_IMPLIED => {
                    __serializer
                        .serialize_field("USAGE_BLEND_WEIGHTS_LAST_IMPLIED", &8u64)
                }
                Self::USAGE_TEX_COORD => {
                    __serializer.serialize_field("USAGE_TEX_COORD", &9u64)
                }
                Self::USAGE_POINT_SIZE => {
                    __serializer.serialize_field("USAGE_POINT_SIZE", &10u64)
                }
                Self::USAGE_USER => __serializer.serialize_field("USAGE_USER", &11u64),
                Self::USAGE_LAST => __serializer.serialize_field("USAGE_LAST", &12u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ComponentUsage to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HintFlags {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_READ => {
                        __serializer.serialize_field("FLAG_READ", &Self::FLAG_READ)
                    }
                    Self::FLAG_WRITE => {
                        __serializer.serialize_field("FLAG_WRITE", &Self::FLAG_WRITE)
                    }
                    Self::FLAG_DYNAMIC => {
                        __serializer.serialize_field("FLAG_DYNAMIC", &Self::FLAG_DYNAMIC)
                    }
                    Self::FLAG_NOT_SHARED => {
                        __serializer
                            .serialize_field("FLAG_NOT_SHARED", &Self::FLAG_NOT_SHARED)
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
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
    impl<'de> _serde::Deserialize<'de> for ComponentType {
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
                        9u8 => _serde::__private::Ok(__Field::__field9),
                        10u8 => _serde::__private::Ok(__Field::__field10),
                        11u8 => _serde::__private::Ok(__Field::__field11),
                        12u8 => _serde::__private::Ok(__Field::__field12),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12",
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
                            v if v == "0" || v.eq_ignore_ascii_case("TYPE_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("TYPE_INT8") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("TYPE_UINT8") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("TYPE_INT16") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("TYPE_UINT16") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("TYPE_INT32") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("TYPE_UINT32") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("TYPE_UINT8_DWORD") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("TYPE_ARGB32") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9" || v.eq_ignore_ascii_case("TYPE_FLOAT16") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10" || v.eq_ignore_ascii_case("TYPE_FLOAT32") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11" || v.eq_ignore_ascii_case("TYPE_VECTOR4") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12" || v.eq_ignore_ascii_case("TYPE_LAST") => {
                                _serde::__private::Ok(__Field::__field12)
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
                marker: _serde::__private::PhantomData<ComponentType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ComponentType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ComponentType",
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
                            _serde::__private::Ok(ComponentType::TYPE_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_INT8)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_UINT8)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_INT16)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_UINT16)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_INT32)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_UINT32)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_UINT8_DWORD)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_ARGB32)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_FLOAT16)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_FLOAT32)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_VECTOR4)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentType::TYPE_LAST)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TYPE_NONE",
                "TYPE_INT8",
                "TYPE_UINT8",
                "TYPE_INT16",
                "TYPE_UINT16",
                "TYPE_INT32",
                "TYPE_UINT32",
                "TYPE_UINT8_DWORD",
                "TYPE_ARGB32",
                "TYPE_FLOAT16",
                "TYPE_FLOAT32",
                "TYPE_VECTOR4",
                "TYPE_LAST",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ComponentType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ComponentType>,
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
    impl<'de> _serde::Deserialize<'de> for ComponentUsage {
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
                        9u8 => _serde::__private::Ok(__Field::__field9),
                        10u8 => _serde::__private::Ok(__Field::__field10),
                        11u8 => _serde::__private::Ok(__Field::__field11),
                        12u8 => _serde::__private::Ok(__Field::__field12),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12",
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
                            v if v == "0" || v.eq_ignore_ascii_case("USAGE_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("USAGE_POSITION") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("USAGE_NORMAL") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("USAGE_COLOR") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("USAGE_TANGENT") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("USAGE_BINORMAL") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6"
                                || v.eq_ignore_ascii_case("USAGE_BLEND_MATRIX_INDEX") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7"
                                || v.eq_ignore_ascii_case("USAGE_BLEND_WEIGHTS") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8"
                                || v
                                    .eq_ignore_ascii_case(
                                        "USAGE_BLEND_WEIGHTS_LAST_IMPLIED",
                                    ) => _serde::__private::Ok(__Field::__field8),
                            v if v == "9"
                                || v.eq_ignore_ascii_case("USAGE_TEX_COORD") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10"
                                || v.eq_ignore_ascii_case("USAGE_POINT_SIZE") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11" || v.eq_ignore_ascii_case("USAGE_USER") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12" || v.eq_ignore_ascii_case("USAGE_LAST") => {
                                _serde::__private::Ok(__Field::__field12)
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
                marker: _serde::__private::PhantomData<ComponentUsage>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ComponentUsage;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum ComponentUsage",
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
                            _serde::__private::Ok(ComponentUsage::USAGE_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_POSITION)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_NORMAL)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_COLOR)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_TANGENT)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_BINORMAL)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ComponentUsage::USAGE_BLEND_MATRIX_INDEX,
                            )
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_BLEND_WEIGHTS)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                ComponentUsage::USAGE_BLEND_WEIGHTS_LAST_IMPLIED,
                            )
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_TEX_COORD)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_POINT_SIZE)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_USER)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(ComponentUsage::USAGE_LAST)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "USAGE_NONE",
                "USAGE_POSITION",
                "USAGE_NORMAL",
                "USAGE_COLOR",
                "USAGE_TANGENT",
                "USAGE_BINORMAL",
                "USAGE_BLEND_MATRIX_INDEX",
                "USAGE_BLEND_WEIGHTS",
                "USAGE_BLEND_WEIGHTS_LAST_IMPLIED",
                "USAGE_TEX_COORD",
                "USAGE_POINT_SIZE",
                "USAGE_USER",
                "USAGE_LAST",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "ComponentUsage",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ComponentUsage>,
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
    impl<'de> _serde::Deserialize<'de> for HintFlags {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<HintFlags>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HintFlags;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct HintFlags(flags)",
                    )
                }
                #[inline]
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(HintFlags::from_bits_retain(__value as _))
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <HintFlags as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HintFlags>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
