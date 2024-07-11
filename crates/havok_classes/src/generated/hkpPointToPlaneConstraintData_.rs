use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpPointToPlaneConstraintData`
/// -         version: `0`
/// -       signature: `0x65c56e17`
/// -          size: 176(x86)/192(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpPointToPlaneConstraintData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintData,
    /// # C++ Info
    /// -          name: `atoms`(ctype: `struct hkpPointToPlaneConstraintDataAtoms`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size: 160(x86)/160(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_atoms: hkpPointToPlaneConstraintDataAtoms,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpPointToPlaneConstraintData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpPointToPlaneConstraintData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x65c56e17)
        }
    }
    impl _serde::Serialize for hkpPointToPlaneConstraintData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x65c56e17)));
            let mut serializer = __serializer
                .serialize_struct("hkpPointToPlaneConstraintData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("atoms", &self.m_atoms)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_atoms,
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
            "atoms" => Ok(__Field::m_atoms),
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
pub(super) struct __hkpPointToPlaneConstraintDataVisitor<'de> {
    marker: core::marker::PhantomData<hkpPointToPlaneConstraintData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpPointToPlaneConstraintDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpPointToPlaneConstraintData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpPointToPlaneConstraintData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpPointToPlaneConstraintDataVisitor<'de> {
    type Value = hkpPointToPlaneConstraintData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpPointToPlaneConstraintData",
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
        let mut m_atoms: _serde::__private::Option<hkpPointToPlaneConstraintDataAtoms> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_atoms) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                        );
                    }
                    m_atoms = _serde::__private::Some(
                        match __A::next_value::<
                            hkpPointToPlaneConstraintDataAtoms,
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
        let m_atoms = match m_atoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atoms"),
                );
            }
        };
        _serde::__private::Ok(hkpPointToPlaneConstraintData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_atoms,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpConstraintDataVisitor::visit_as_parent(&mut __map)?;
        let mut m_atoms: _serde::__private::Option<hkpPointToPlaneConstraintDataAtoms> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_atoms => {
                        if _serde::__private::Option::is_some(&m_atoms) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                            );
                        }
                        m_atoms = _serde::__private::Some(
                            match __A::next_value::<
                                hkpPointToPlaneConstraintDataAtoms,
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
        let m_atoms = match m_atoms {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("atoms"),
                );
            }
        };
        _serde::__private::Ok(hkpPointToPlaneConstraintData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_atoms,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpPointToPlaneConstraintData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["atoms"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpPointToPlaneConstraintData",
                FIELDS,
                __hkpPointToPlaneConstraintDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpPointToPlaneConstraintData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
