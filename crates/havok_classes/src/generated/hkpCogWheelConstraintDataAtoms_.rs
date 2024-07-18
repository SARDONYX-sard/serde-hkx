use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCogWheelConstraintDataAtoms`
/// - version: `0`
/// - signature: `0xf855ba44`
/// - size: `160`(x86)/`160`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCogWheelConstraintDataAtoms {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `transforms`(ctype: `struct hkpSetLocalTransformsConstraintAtom`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `144`(x86)/`144`(x86_64)
    pub m_transforms: hkpSetLocalTransformsConstraintAtom,
    /// # C++ Info
    /// - name: `cogWheels`(ctype: `struct hkpCogWheelConstraintAtom`)
    /// - offset: `144`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_cogWheels: hkpCogWheelConstraintAtom,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCogWheelConstraintDataAtoms {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCogWheelConstraintDataAtoms"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf855ba44)
        }
    }
    impl _serde::Serialize for hkpCogWheelConstraintDataAtoms {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf855ba44)));
            let mut serializer = __serializer
                .serialize_struct("hkpCogWheelConstraintDataAtoms", class_meta)?;
            serializer.serialize_field("transforms", &self.m_transforms)?;
            serializer.serialize_field("cogWheels", &self.m_cogWheels)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_transforms,
    m_cogWheels,
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
            "transforms" => Ok(__Field::m_transforms),
            "cogWheels" => Ok(__Field::m_cogWheels),
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
pub(super) struct __hkpCogWheelConstraintDataAtomsVisitor<'de> {
    marker: core::marker::PhantomData<hkpCogWheelConstraintDataAtoms>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpCogWheelConstraintDataAtomsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpCogWheelConstraintDataAtoms, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpCogWheelConstraintDataAtoms>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpCogWheelConstraintDataAtomsVisitor<'de> {
    type Value = hkpCogWheelConstraintDataAtoms;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpCogWheelConstraintDataAtoms",
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
        let mut m_transforms: _serde::__private::Option<
            hkpSetLocalTransformsConstraintAtom,
        > = _serde::__private::None;
        let mut m_cogWheels: _serde::__private::Option<hkpCogWheelConstraintAtom> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_transforms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "transforms",
                            ),
                        );
                    }
                    m_transforms = _serde::__private::Some(
                        match __A::next_value::<
                            hkpSetLocalTransformsConstraintAtom,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_cogWheels) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "cogWheels",
                            ),
                        );
                    }
                    m_cogWheels = _serde::__private::Some(
                        match __A::next_value::<hkpCogWheelConstraintAtom>(&mut __map) {
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
        let m_transforms = match m_transforms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transforms"),
                );
            }
        };
        let m_cogWheels = match m_cogWheels {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cogWheels"),
                );
            }
        };
        _serde::__private::Ok(hkpCogWheelConstraintDataAtoms {
            __ptr,
            m_transforms,
            m_cogWheels,
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
        let mut m_transforms: _serde::__private::Option<
            hkpSetLocalTransformsConstraintAtom,
        > = _serde::__private::None;
        let mut m_cogWheels: _serde::__private::Option<hkpCogWheelConstraintAtom> = _serde::__private::None;
        for _ in 0..2usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_transforms => {
                        if _serde::__private::Option::is_some(&m_transforms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "transforms",
                                ),
                            );
                        }
                        m_transforms = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSetLocalTransformsConstraintAtom,
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
                    __Field::m_cogWheels => {
                        if _serde::__private::Option::is_some(&m_cogWheels) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "cogWheels",
                                ),
                            );
                        }
                        m_cogWheels = _serde::__private::Some(
                            match __A::next_value::<
                                hkpCogWheelConstraintAtom,
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
        let m_transforms = match m_transforms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("transforms"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_cogWheels = match m_cogWheels {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("cogWheels"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpCogWheelConstraintDataAtoms {
            __ptr,
            m_transforms,
            m_cogWheels,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCogWheelConstraintDataAtoms {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["transforms", "cogWheels"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCogWheelConstraintDataAtoms",
                FIELDS,
                __hkpCogWheelConstraintDataAtomsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpCogWheelConstraintDataAtoms,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
