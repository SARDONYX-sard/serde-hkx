use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BGSGamebryoSequenceGenerator`
/// -         version: `2`
/// -       signature: `0xc8df2d77`
/// -          size:  72(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BGSGamebryoSequenceGenerator<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `pSequence`(ctype: `char*`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_pSequence: CString<'a>,
    /// # C++ Info
    /// -          name: `eBlendModeFunction`(ctype: `enum BlendModeFunction`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eBlendModeFunction: BlendModeFunction,
    /// # C++ Info
    /// -          name: `fPercent`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fPercent: f32,
    /// # C++ Info
    /// -          name: `events`(ctype: `hkArray<void>`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_events: Vec<()>,
    /// # C++ Info
    /// -          name: `fTime`(ctype: `hkReal`)
    /// -        offset:  64(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_fTime: f32,
    /// # C++ Info
    /// -          name: `bDelayedActivate`(ctype: `hkBool`)
    /// -        offset:  68(x86)/108(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bDelayedActivate: bool,
    /// # C++ Info
    /// -          name: `bLooping`(ctype: `hkBool`)
    /// -        offset:  69(x86)/109(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_bLooping: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BGSGamebryoSequenceGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BGSGamebryoSequenceGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc8df2d77)
        }
    }
    impl<'a> _serde::Serialize for BGSGamebryoSequenceGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc8df2d77)));
            let mut serializer = __serializer
                .serialize_struct("BGSGamebryoSequenceGenerator", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_cstring_meta_field("pSequence", &self.m_pSequence)?;
            serializer
                .serialize_field("eBlendModeFunction", &self.m_eBlendModeFunction)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("fPercent", &self.m_fPercent)?;
            serializer.skip_array_meta_field("events", &self.m_events)?;
            serializer.skip_field("fTime", &self.m_fTime)?;
            serializer.skip_field("bDelayedActivate", &self.m_bDelayedActivate)?;
            serializer.skip_field("bLooping", &self.m_bLooping)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_cstring_field("pSequence", &self.m_pSequence)?;
            serializer.serialize_array_field("events", &self.m_events)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pSequence,
    m_eBlendModeFunction,
    m_fPercent,
    m_events,
    m_fTime,
    m_bDelayedActivate,
    m_bLooping,
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
            "pSequence" => Ok(__Field::m_pSequence),
            "eBlendModeFunction" => Ok(__Field::m_eBlendModeFunction),
            "fPercent" => Ok(__Field::m_fPercent),
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
pub(super) struct __BGSGamebryoSequenceGeneratorVisitor<'de> {
    marker: core::marker::PhantomData<BGSGamebryoSequenceGenerator<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BGSGamebryoSequenceGeneratorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BGSGamebryoSequenceGenerator<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    BGSGamebryoSequenceGenerator<'de>,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BGSGamebryoSequenceGeneratorVisitor<'de> {
    type Value = BGSGamebryoSequenceGenerator<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct BGSGamebryoSequenceGenerator",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_pSequence: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_eBlendModeFunction: _serde::__private::Option<BlendModeFunction> = _serde::__private::None;
        let mut m_fPercent: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_events: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        let mut m_fTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_bDelayedActivate: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_bLooping: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pSequence) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pSequence",
                            ),
                        );
                    }
                    m_pSequence = _serde::__private::Some(
                        match __A::next_value::<CString<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_eBlendModeFunction) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "eBlendModeFunction",
                            ),
                        );
                    }
                    m_eBlendModeFunction = _serde::__private::Some(
                        match __A::next_value::<BlendModeFunction>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_fPercent) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fPercent",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_fPercent = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_events) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("events"),
                        );
                    }
                    m_events = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_fTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("fTime"),
                        );
                    }
                    m_fTime = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_bDelayedActivate) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bDelayedActivate",
                            ),
                        );
                    }
                    m_bDelayedActivate = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_bLooping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bLooping",
                            ),
                        );
                    }
                    m_bLooping = _serde::__private::Some(
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
        __A::pad(&mut __map, 2usize, 2usize)?;
        let m_pSequence = match m_pSequence {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pSequence"),
                );
            }
        };
        let m_eBlendModeFunction = match m_eBlendModeFunction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "eBlendModeFunction",
                    ),
                );
            }
        };
        let m_fPercent = match m_fPercent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fPercent"),
                );
            }
        };
        let m_events = match m_events {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("events"),
                );
            }
        };
        let m_fTime = match m_fTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fTime"),
                );
            }
        };
        let m_bDelayedActivate = match m_bDelayedActivate {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bDelayedActivate"),
                );
            }
        };
        let m_bLooping = match m_bLooping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bLooping"),
                );
            }
        };
        _serde::__private::Ok(BGSGamebryoSequenceGenerator {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_pSequence,
            m_eBlendModeFunction,
            m_fPercent,
            m_events,
            m_fTime,
            m_bDelayedActivate,
            m_bLooping,
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
        let parent = __hkbGeneratorVisitor::visit_as_parent(&mut __map)?;
        let mut m_pSequence: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_eBlendModeFunction: _serde::__private::Option<BlendModeFunction> = _serde::__private::None;
        let mut m_fPercent: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_pSequence => {
                        if _serde::__private::Option::is_some(&m_pSequence) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pSequence",
                                ),
                            );
                        }
                        m_pSequence = _serde::__private::Some(
                            match __A::next_value::<CString<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_eBlendModeFunction => {
                        if _serde::__private::Option::is_some(&m_eBlendModeFunction) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "eBlendModeFunction",
                                ),
                            );
                        }
                        m_eBlendModeFunction = _serde::__private::Some(
                            match __A::next_value::<BlendModeFunction>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fPercent => {
                        if _serde::__private::Option::is_some(&m_fPercent) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fPercent",
                                ),
                            );
                        }
                        m_fPercent = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        let m_pSequence = match m_pSequence {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pSequence"),
                );
            }
        };
        let m_eBlendModeFunction = match m_eBlendModeFunction {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "eBlendModeFunction",
                    ),
                );
            }
        };
        let m_fPercent = match m_fPercent {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fPercent"),
                );
            }
        };
        _serde::__private::Ok(BGSGamebryoSequenceGenerator {
            __ptr,
            parent,
            m_pSequence,
            m_eBlendModeFunction,
            m_fPercent,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BGSGamebryoSequenceGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "pSequence",
                "eBlendModeFunction",
                "fPercent",
                "events",
                "fTime",
                "bDelayedActivate",
                "bLooping",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BGSGamebryoSequenceGenerator",
                FIELDS,
                __BGSGamebryoSequenceGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<
                        BGSGamebryoSequenceGenerator,
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
pub enum BlendModeFunction {
    #[default]
    BMF_NONE = 0isize,
    BMF_PERCENT = 1isize,
    BMF_ONE_MINUS_PERCENT = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BlendModeFunction {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::BMF_NONE => __serializer.serialize_field("BMF_NONE", &0u64),
                Self::BMF_PERCENT => __serializer.serialize_field("BMF_PERCENT", &1u64),
                Self::BMF_ONE_MINUS_PERCENT => {
                    __serializer.serialize_field("BMF_ONE_MINUS_PERCENT", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BlendModeFunction to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for BlendModeFunction {
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
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
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
                            v if v == "0" || v.eq_ignore_ascii_case("BMF_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("BMF_PERCENT") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("BMF_ONE_MINUS_PERCENT") => {
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BlendModeFunction>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BlendModeFunction;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum BlendModeFunction",
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
                            _serde::__private::Ok(BlendModeFunction::BMF_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(BlendModeFunction::BMF_PERCENT)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                BlendModeFunction::BMF_ONE_MINUS_PERCENT,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "BMF_NONE",
                "BMF_PERCENT",
                "BMF_ONE_MINUS_PERCENT",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "BlendModeFunction",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BlendModeFunction>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
