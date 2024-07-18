use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbVariableBindingSet`
/// - version: `2`
/// - signature: `0x338ad4ff`
/// - size: ` 28`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbVariableBindingSet<'a> {
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
    /// - name: `bindings`(ctype: `hkArray<struct hkbVariableBindingSetBinding>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_bindings: Vec<hkbVariableBindingSetBinding<'a>>,
    /// # C++ Info
    /// - name: `indexOfBindingToEnable`(ctype: `hkInt32`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_indexOfBindingToEnable: i32,
    /// # C++ Info
    /// - name: `hasOutputBinding`(ctype: `hkBool`)
    /// - offset: ` 24`(x86)/` 36`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_hasOutputBinding: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbVariableBindingSet<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbVariableBindingSet"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x338ad4ff)
        }
    }
    impl<'a> _serde::Serialize for hkbVariableBindingSet<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x338ad4ff)));
            let mut serializer = __serializer
                .serialize_struct("hkbVariableBindingSet", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("bindings", &self.m_bindings)?;
            serializer
                .serialize_field(
                    "indexOfBindingToEnable",
                    &self.m_indexOfBindingToEnable,
                )?;
            serializer.skip_field("hasOutputBinding", &self.m_hasOutputBinding)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_array_field("bindings", &self.m_bindings)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_bindings,
    m_indexOfBindingToEnable,
    m_hasOutputBinding,
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
            "bindings" => Ok(__Field::m_bindings),
            "indexOfBindingToEnable" => Ok(__Field::m_indexOfBindingToEnable),
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
pub(super) struct __hkbVariableBindingSetVisitor<'de> {
    marker: core::marker::PhantomData<hkbVariableBindingSet<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbVariableBindingSetVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbVariableBindingSet<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbVariableBindingSet<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbVariableBindingSetVisitor<'de> {
    type Value = hkbVariableBindingSet<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbVariableBindingSet")
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
        let mut m_bindings: _serde::__private::Option<
            Vec<hkbVariableBindingSetBinding<'de>>,
        > = _serde::__private::None;
        let mut m_indexOfBindingToEnable: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_hasOutputBinding: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_bindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "bindings",
                            ),
                        );
                    }
                    m_bindings = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkbVariableBindingSetBinding<'de>>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_indexOfBindingToEnable) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "indexOfBindingToEnable",
                            ),
                        );
                    }
                    m_indexOfBindingToEnable = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_hasOutputBinding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hasOutputBinding",
                            ),
                        );
                    }
                    m_hasOutputBinding = _serde::__private::Some(
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
        __A::pad(&mut __map, 3usize, 3usize)?;
        let m_bindings = match m_bindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bindings"),
                );
            }
        };
        let m_indexOfBindingToEnable = match m_indexOfBindingToEnable {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "indexOfBindingToEnable",
                    ),
                );
            }
        };
        let m_hasOutputBinding = match m_hasOutputBinding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hasOutputBinding"),
                );
            }
        };
        _serde::__private::Ok(hkbVariableBindingSet {
            __ptr,
            parent,
            m_bindings,
            m_indexOfBindingToEnable,
            m_hasOutputBinding,
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
        let mut m_bindings: _serde::__private::Option<
            Vec<hkbVariableBindingSetBinding<'de>>,
        > = _serde::__private::None;
        let mut m_indexOfBindingToEnable: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..2usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_bindings => {
                        if _serde::__private::Option::is_some(&m_bindings) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "bindings",
                                ),
                            );
                        }
                        m_bindings = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkbVariableBindingSetBinding<'de>>,
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
                    __Field::m_indexOfBindingToEnable => {
                        if _serde::__private::Option::is_some(
                            &m_indexOfBindingToEnable,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "indexOfBindingToEnable",
                                ),
                            );
                        }
                        m_indexOfBindingToEnable = _serde::__private::Some(
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
                    _ => {}
                }
            }
        }
        let m_bindings = match m_bindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bindings"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_indexOfBindingToEnable = match m_indexOfBindingToEnable {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "indexOfBindingToEnable",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkbVariableBindingSet {
            __ptr,
            parent,
            m_bindings,
            m_indexOfBindingToEnable,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbVariableBindingSet<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "bindings",
                "indexOfBindingToEnable",
                "hasOutputBinding",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbVariableBindingSet",
                FIELDS,
                __hkbVariableBindingSetVisitor {
                    marker: _serde::__private::PhantomData::<hkbVariableBindingSet>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
