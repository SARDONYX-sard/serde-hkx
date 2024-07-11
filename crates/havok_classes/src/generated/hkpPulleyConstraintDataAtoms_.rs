use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPulleyConstraintDataAtoms`
/// -         version: `0`
/// -       signature: `0xb149e5a`
/// -          size: 112(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPulleyConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `translations`(ctype: `struct hkpSetLocalTranslationsConstraintAtom`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_translations: hkpSetLocalTranslationsConstraintAtom,
    /// # C++ Info
    /// -          name: `pulley`(ctype: `struct hkpPulleyConstraintAtom`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_pulley: hkpPulleyConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPulleyConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPulleyConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb149e5a)
        }
    }
    impl _serde::Serialize for hkpPulleyConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb149e5a)));
            let mut serializer = __serializer
                .serialize_struct("hkpPulleyConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("translations", &self.m_translations)?;
            serializer.serialize_field("pulley", &self.m_pulley)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_translations,
    m_pulley,
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
            "translations" => Ok(__Field::m_translations),
            "pulley" => Ok(__Field::m_pulley),
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
pub(super) struct __hkpPulleyConstraintDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpPulleyConstraintDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPulleyConstraintDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPulleyConstraintDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPulleyConstraintDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPulleyConstraintDataAtomsVisitor<'de> {
    type Value = hkpPulleyConstraintDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPulleyConstraintDataAtoms",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_translations: _serde::__private::Option<
            hkpSetLocalTranslationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_pulley: _serde::__private::Option<hkpPulleyConstraintAtom> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_translations) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "translations",
                            ),
                        );
                    }
                    m_translations = _serde::__private::Some(
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
                    if _serde::__private::Option::is_some(&m_pulley) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("pulley"),
                        );
                    }
                    m_pulley = _serde::__private::Some(
                        match __A::next_value::<hkpPulleyConstraintAtom>(&mut __map) {
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
        let m_translations = match m_translations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translations"),
                );
            }
        };
        let m_pulley = match m_pulley {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pulley"),
                );
            }
        };
        _serde::__private::Ok(hkpPulleyConstraintDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_translations,
            m_pulley,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_translations: _serde::__private::Option<
            hkpSetLocalTranslationsConstraintAtom,
        > = _serde::__private::None;
        let mut m_pulley: _serde::__private::Option<hkpPulleyConstraintAtom> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_translations => {
                        if _serde::__private::Option::is_some(&m_translations) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "translations",
                                ),
                            );
                        }
                        m_translations = _serde::__private::Some(
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
                    __Field::m_pulley => {
                        if _serde::__private::Option::is_some(&m_pulley) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("pulley"),
                            );
                        }
                        m_pulley = _serde::__private::Some(
                            match __A::next_value::<
                                hkpPulleyConstraintAtom,
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
        }
        let m_translations = match m_translations {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("translations"),
                );
            }
        };
        let m_pulley = match m_pulley {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pulley"),
                );
            }
        };
        _serde::__private::Ok(hkpPulleyConstraintDataAtoms {
            __ptr: __A::class_ptr(&mut __map),
            m_translations,
            m_pulley,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPulleyConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["translations", "pulley"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPulleyConstraintDataAtoms",
                FIELDS,
                __hkpPulleyConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPulleyConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
