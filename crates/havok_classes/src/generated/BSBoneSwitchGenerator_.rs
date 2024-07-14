use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSBoneSwitchGenerator`
/// -         version: `1`
/// -       signature: `0xf33d3eea`
/// -          size:  64(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSBoneSwitchGenerator<'a> {
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
    /// -          name: `pDefaultGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_pDefaultGenerator: Pointer,
    /// # C++ Info
    /// -          name: `ChildrenA`(ctype: `hkArray<BSBoneSwitchGeneratorBoneData*>`)
    /// -        offset:  52(x86)/ 88(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_ChildrenA: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSBoneSwitchGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSBoneSwitchGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf33d3eea)
        }
    }
    impl<'a> _serde::Serialize for BSBoneSwitchGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf33d3eea)));
            let mut serializer = __serializer
                .serialize_struct("BSBoneSwitchGenerator", class_meta)?;
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
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("pDefaultGenerator", &self.m_pDefaultGenerator)?;
            serializer.serialize_array_meta_field("ChildrenA", &self.m_ChildrenA)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("ChildrenA", &self.m_ChildrenA)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_pDefaultGenerator,
    m_ChildrenA,
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
            "pDefaultGenerator" => Ok(__Field::m_pDefaultGenerator),
            "ChildrenA" => Ok(__Field::m_ChildrenA),
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
pub(super) struct __BSBoneSwitchGeneratorVisitor<'de> {
    marker: core::marker::PhantomData<BSBoneSwitchGenerator<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSBoneSwitchGeneratorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSBoneSwitchGenerator<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSBoneSwitchGenerator<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSBoneSwitchGeneratorVisitor<'de> {
    type Value = BSBoneSwitchGenerator<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSBoneSwitchGenerator")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_pDefaultGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_ChildrenA: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_pDefaultGenerator) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "pDefaultGenerator",
                            ),
                        );
                    }
                    m_pDefaultGenerator = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_ChildrenA) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "ChildrenA",
                            ),
                        );
                    }
                    m_ChildrenA = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 8usize)?;
        let m_pDefaultGenerator = match m_pDefaultGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pDefaultGenerator"),
                );
            }
        };
        let m_ChildrenA = match m_ChildrenA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ChildrenA"),
                );
            }
        };
        _serde::__private::Ok(BSBoneSwitchGenerator {
            __ptr,
            parent,
            m_pDefaultGenerator,
            m_ChildrenA,
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
        let mut m_pDefaultGenerator: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_ChildrenA: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_pDefaultGenerator => {
                        if _serde::__private::Option::is_some(&m_pDefaultGenerator) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "pDefaultGenerator",
                                ),
                            );
                        }
                        m_pDefaultGenerator = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_ChildrenA => {
                        if _serde::__private::Option::is_some(&m_ChildrenA) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "ChildrenA",
                                ),
                            );
                        }
                        m_ChildrenA = _serde::__private::Some(
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
        }
        let m_pDefaultGenerator = match m_pDefaultGenerator {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("pDefaultGenerator"),
                );
            }
        };
        let m_ChildrenA = match m_ChildrenA {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("ChildrenA"),
                );
            }
        };
        _serde::__private::Ok(BSBoneSwitchGenerator {
            __ptr,
            parent,
            m_pDefaultGenerator,
            m_ChildrenA,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSBoneSwitchGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["pDefaultGenerator", "ChildrenA"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSBoneSwitchGenerator",
                FIELDS,
                __BSBoneSwitchGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<BSBoneSwitchGenerator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
