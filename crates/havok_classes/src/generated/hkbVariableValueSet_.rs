use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbVariableValueSet`
/// - version: `0`
/// - signature: `0x27812d8d`
/// - size: ` 44`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableValueSet {
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
    /// - name: `wordVariableValues`(ctype: `hkArray<struct hkbVariableValue>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_wordVariableValues: Vec<hkbVariableValue>,
    /// # C++ Info
    /// - name: `quadVariableValues`(ctype: `hkArray<hkVector4>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_quadVariableValues: Vec<Vector4>,
    /// # C++ Info
    /// - name: `variantVariableValues`(ctype: `hkArray<hkReferencedObject*>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_variantVariableValues: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbVariableValueSet {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableValueSet"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x27812d8d)
        }
    }
    impl _serde::Serialize for hkbVariableValueSet {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x27812d8d)));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableValueSet", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "wordVariableValues",
                    &self.m_wordVariableValues,
                )?;
            serializer
                .serialize_array_meta_field(
                    "quadVariableValues",
                    &self.m_quadVariableValues,
                )?;
            serializer
                .serialize_array_meta_field(
                    "variantVariableValues",
                    &self.m_variantVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "wordVariableValues",
                    &self.m_wordVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "quadVariableValues",
                    &self.m_quadVariableValues,
                )?;
            serializer
                .serialize_array_field(
                    "variantVariableValues",
                    &self.m_variantVariableValues,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_wordVariableValues,
    m_quadVariableValues,
    m_variantVariableValues,
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
            "wordVariableValues" => Ok(__Field::m_wordVariableValues),
            "quadVariableValues" => Ok(__Field::m_quadVariableValues),
            "variantVariableValues" => Ok(__Field::m_variantVariableValues),
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
pub(super) struct __hkbVariableValueSetVisitor<'de> {
    marker: core::marker::PhantomData<hkbVariableValueSet>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbVariableValueSetVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbVariableValueSet, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbVariableValueSet>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbVariableValueSetVisitor<'de> {
    type Value = hkbVariableValueSet;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbVariableValueSet")
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
        let mut m_wordVariableValues: _serde::__private::Option<Vec<hkbVariableValue>> = _serde::__private::None;
        let mut m_quadVariableValues: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_variantVariableValues: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_wordVariableValues) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wordVariableValues",
                            ),
                        );
                    }
                    m_wordVariableValues = _serde::__private::Some(
                        match __A::next_value::<Vec<hkbVariableValue>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_quadVariableValues) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "quadVariableValues",
                            ),
                        );
                    }
                    m_quadVariableValues = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_variantVariableValues) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "variantVariableValues",
                            ),
                        );
                    }
                    m_variantVariableValues = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_wordVariableValues = match m_wordVariableValues {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wordVariableValues",
                    ),
                );
            }
        };
        let m_quadVariableValues = match m_quadVariableValues {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "quadVariableValues",
                    ),
                );
            }
        };
        let m_variantVariableValues = match m_variantVariableValues {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "variantVariableValues",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkbVariableValueSet {
            __ptr,
            parent,
            m_wordVariableValues,
            m_quadVariableValues,
            m_variantVariableValues,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_wordVariableValues: _serde::__private::Option<Vec<hkbVariableValue>> = _serde::__private::None;
        let mut m_quadVariableValues: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_variantVariableValues: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..3usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_wordVariableValues => {
                        if _serde::__private::Option::is_some(&m_wordVariableValues) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wordVariableValues",
                                ),
                            );
                        }
                        m_wordVariableValues = _serde::__private::Some(
                            match __A::next_value::<Vec<hkbVariableValue>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_quadVariableValues => {
                        if _serde::__private::Option::is_some(&m_quadVariableValues) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "quadVariableValues",
                                ),
                            );
                        }
                        m_quadVariableValues = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_variantVariableValues => {
                        if _serde::__private::Option::is_some(&m_variantVariableValues) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "variantVariableValues",
                                ),
                            );
                        }
                        m_variantVariableValues = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        }
        let m_wordVariableValues = match m_wordVariableValues {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wordVariableValues",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_quadVariableValues = match m_quadVariableValues {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "quadVariableValues",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_variantVariableValues = match m_variantVariableValues {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "variantVariableValues",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkbVariableValueSet {
            __ptr,
            parent,
            m_wordVariableValues,
            m_quadVariableValues,
            m_variantVariableValues,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbVariableValueSet {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "wordVariableValues",
                "quadVariableValues",
                "variantVariableValues",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbVariableValueSet",
                FIELDS,
                __hkbVariableValueSetVisitor {
                    marker: _serde::__private::PhantomData::<hkbVariableValueSet>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
