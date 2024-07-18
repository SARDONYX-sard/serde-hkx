use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpStiffSpringConstraintDataAtoms`
/// - version: `0`
/// - signature: `0x207eb376`
/// - size: ` 64`(x86)/` 64`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStiffSpringConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `pivots`(ctype: `struct hkpSetLocalTranslationsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_pivots: hkpSetLocalTranslationsConstraintAtom,
    /// # C++ Info
    /// - name: `spring`(ctype: `struct hkpStiffSpringConstraintAtom`)
    /// - offset: ` 48`(x86)/` 48`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_spring: hkpStiffSpringConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStiffSpringConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStiffSpringConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x207eb376)
        }
    }
    impl _serde::Serialize for hkpStiffSpringConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x207eb376)));
            let mut serializer = __serializer
                .serialize_struct("hkpStiffSpringConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("pivots", &self.m_pivots)?;
            serializer.serialize_field("spring", &self.m_spring)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pivots,
    m_spring,
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
            "pivots" => Ok(__Field::m_pivots),
            "spring" => Ok(__Field::m_spring),
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
pub(super) struct __hkpStiffSpringConstraintDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpStiffSpringConstraintDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpStiffSpringConstraintDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpStiffSpringConstraintDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpStiffSpringConstraintDataAtoms,
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
impl<'de> _serde::de::Visitor<'de> for __hkpStiffSpringConstraintDataAtomsVisitor<'de> {
    type Value = hkpStiffSpringConstraintDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpStiffSpringConstraintDataAtoms",
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
        let mut m_pivots: _serde::__private::Option<
            hkpSetLocalTranslationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_spring: _serde::__private::Option<hkpStiffSpringConstraintAtom> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pivots) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pivots"),
                        );
                    }
                    m_pivots = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSetLocalTranslationsConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_spring) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("spring"),
                        );
                    }
                    m_spring = _serde::__private::Some(
                        match __A::next_value::<
                            hkpStiffSpringConstraintAtom,
                        >(&mut __map) {
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
        __A::pad(&mut __map, 8usize, 8usize)?;
        let m_pivots = match m_pivots {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivots"),
                );
            }
        };
        let m_spring = match m_spring {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spring"),
                );
            }
        };
        _serde::__private::Ok(hkpStiffSpringConstraintDataAtoms {
            __ptr,
            m_pivots,
            m_spring,
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
        let mut m_pivots: _serde::__private::Option<
            hkpSetLocalTranslationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_spring: _serde::__private::Option<hkpStiffSpringConstraintAtom> = _serde::__private::None;
        for _ in 0..2usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_pivots => {
                        if _serde::__private::Option::is_some(&m_pivots) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("pivots"),
                            );
                        }
                        m_pivots = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSetLocalTranslationsConstraintAtom,
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
                    __Field::m_spring => {
                        if _serde::__private::Option::is_some(&m_spring) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("spring"),
                            );
                        }
                        m_spring = _serde::__private::Some(
                            match __A::next_value::<
                                hkpStiffSpringConstraintAtom,
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
        }
        let m_pivots = match m_pivots {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pivots"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_spring = match m_spring {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spring"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpStiffSpringConstraintDataAtoms {
            __ptr,
            m_pivots,
            m_spring,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpStiffSpringConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["pivots", "spring"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpStiffSpringConstraintDataAtoms",
                FIELDS,
                __hkpStiffSpringConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpStiffSpringConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
