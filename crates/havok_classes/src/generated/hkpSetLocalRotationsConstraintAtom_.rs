use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpSetLocalRotationsConstraintAtom`
/// - version: `0`
/// - signature: `0xf81db8e`
/// - size: `112`(x86)/`112`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetLocalRotationsConstraintAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// - name: `rotationA`(ctype: `hkRotation`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_rotationA: Rotation,
    /// # C++ Info
    /// - name: `rotationB`(ctype: `hkRotation`)
    /// - offset: ` 64`(x86)/` 64`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    pub m_rotationB: Rotation,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSetLocalRotationsConstraintAtom {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSetLocalRotationsConstraintAtom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf81db8e)
        }
    }
    impl _serde::Serialize for hkpSetLocalRotationsConstraintAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf81db8e)));
            let mut serializer = __serializer
                .serialize_struct("hkpSetLocalRotationsConstraintAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("rotationA", &self.m_rotationA)?;
            serializer.serialize_field("rotationB", &self.m_rotationB)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_rotationA,
    m_rotationB,
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
            "rotationA" => Ok(__Field::m_rotationA),
            "rotationB" => Ok(__Field::m_rotationB),
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
pub(super) struct __hkpSetLocalRotationsConstraintAtomVisitor<'de> {
    marker: core::marker::PhantomData<hkpSetLocalRotationsConstraintAtom>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSetLocalRotationsConstraintAtomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSetLocalRotationsConstraintAtom, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpSetLocalRotationsConstraintAtom,
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
impl<'de> _serde::de::Visitor<'de> for __hkpSetLocalRotationsConstraintAtomVisitor<'de> {
    type Value = hkpSetLocalRotationsConstraintAtom;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpSetLocalRotationsConstraintAtom",
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
        let parent = __A::parent_value(&mut __map)?;
        let mut m_rotationA: _serde::__private::Option<Rotation> = _serde::__private::None;
        let mut m_rotationB: _serde::__private::Option<Rotation> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_rotationA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotationA",
                            ),
                        );
                    }
                    m_rotationA = _serde::__private::Some(
                        match __A::next_value::<Rotation>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_rotationB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rotationB",
                            ),
                        );
                    }
                    m_rotationB = _serde::__private::Some(
                        match __A::next_value::<Rotation>(&mut __map) {
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
        let m_rotationA = match m_rotationA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationA"),
                );
            }
        };
        let m_rotationB = match m_rotationB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationB"),
                );
            }
        };
        _serde::__private::Ok(hkpSetLocalRotationsConstraintAtom {
            __ptr,
            parent,
            m_rotationA,
            m_rotationB,
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
        let parent = __hkpConstraintAtomVisitor::visit_as_parent(&mut __map)?;
        let mut m_rotationA: _serde::__private::Option<Rotation> = _serde::__private::None;
        let mut m_rotationB: _serde::__private::Option<Rotation> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_rotationA => {
                        if _serde::__private::Option::is_some(&m_rotationA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotationA",
                                ),
                            );
                        }
                        m_rotationA = _serde::__private::Some(
                            match __A::next_value::<Rotation>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rotationB => {
                        if _serde::__private::Option::is_some(&m_rotationB) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rotationB",
                                ),
                            );
                        }
                        m_rotationB = _serde::__private::Some(
                            match __A::next_value::<Rotation>(&mut __map) {
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
        let m_rotationA = match m_rotationA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationA"),
                );
            }
        };
        let m_rotationB = match m_rotationB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rotationB"),
                );
            }
        };
        _serde::__private::Ok(hkpSetLocalRotationsConstraintAtom {
            __ptr,
            parent,
            m_rotationA,
            m_rotationB,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSetLocalRotationsConstraintAtom {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["rotationA", "rotationB"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSetLocalRotationsConstraintAtom",
                FIELDS,
                __hkpSetLocalRotationsConstraintAtomVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpSetLocalRotationsConstraintAtom,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
