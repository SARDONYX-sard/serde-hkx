use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpBallSocketChainDataConstraintInfo`
/// - version: `0`
/// - signature: `0xc9cbedf2`
/// - size: ` 32`(x86)/` 32`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBallSocketChainDataConstraintInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `pivotInA`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_pivotInA: Vector4,
    /// # C++ Info
    /// - name: `pivotInB`(ctype: `hkVector4`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_pivotInB: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBallSocketChainDataConstraintInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBallSocketChainDataConstraintInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc9cbedf2)
        }
    }
    impl _serde::Serialize for hkpBallSocketChainDataConstraintInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc9cbedf2)));
            let mut serializer = __serializer
                .serialize_struct("hkpBallSocketChainDataConstraintInfo", class_meta)?;
            serializer.serialize_field("pivotInA", &self.m_pivotInA)?;
            serializer.serialize_field("pivotInB", &self.m_pivotInB)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pivotInA,
    m_pivotInB,
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
            "pivotInA" => Ok(__Field::m_pivotInA),
            "pivotInB" => Ok(__Field::m_pivotInB),
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
pub(super) struct __hkpBallSocketChainDataConstraintInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkpBallSocketChainDataConstraintInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpBallSocketChainDataConstraintInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpBallSocketChainDataConstraintInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpBallSocketChainDataConstraintInfo,
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
impl<'de> _serde::de::Visitor<'de>
for __hkpBallSocketChainDataConstraintInfoVisitor<'de> {
    type Value = hkpBallSocketChainDataConstraintInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpBallSocketChainDataConstraintInfo",
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
        let mut m_pivotInA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_pivotInB: _serde::__private::Option<Vector4> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pivotInA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pivotInA",
                            ),
                        );
                    }
                    m_pivotInA = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_pivotInB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pivotInB",
                            ),
                        );
                    }
                    m_pivotInB = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
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
        let m_pivotInA = match m_pivotInA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInA"),
                );
            }
        };
        let m_pivotInB = match m_pivotInB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInB"),
                );
            }
        };
        _serde::__private::Ok(hkpBallSocketChainDataConstraintInfo {
            __ptr,
            m_pivotInA,
            m_pivotInB,
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
        let mut m_pivotInA: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_pivotInB: _serde::__private::Option<Vector4> = _serde::__private::None;
        for _ in 0..2usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_pivotInA => {
                        if _serde::__private::Option::is_some(&m_pivotInA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pivotInA",
                                ),
                            );
                        }
                        m_pivotInA = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_pivotInB => {
                        if _serde::__private::Option::is_some(&m_pivotInB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pivotInB",
                                ),
                            );
                        }
                        m_pivotInB = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
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
        let m_pivotInA = match m_pivotInA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInA"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_pivotInB = match m_pivotInB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivotInB"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpBallSocketChainDataConstraintInfo {
            __ptr,
            m_pivotInA,
            m_pivotInB,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpBallSocketChainDataConstraintInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["pivotInA", "pivotInB"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpBallSocketChainDataConstraintInfo",
                FIELDS,
                __hkpBallSocketChainDataConstraintInfoVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpBallSocketChainDataConstraintInfo,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
