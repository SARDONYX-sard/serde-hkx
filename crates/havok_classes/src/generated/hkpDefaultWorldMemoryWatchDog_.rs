use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpDefaultWorldMemoryWatchDog`
/// - version: `1`
/// - signature: `0x77d6b19f`
/// - size: ` 12`(x86)/` 24`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpDefaultWorldMemoryWatchDog<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hkWorldMemoryAvailableWatchDog<'a>,
    /// # C++ Info
    /// - name: `freeHeapMemoryRequested`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "freeHeapMemoryRequested"))]
    #[cfg_attr(feature = "serde", serde(rename = "freeHeapMemoryRequested"))]
    pub m_freeHeapMemoryRequested: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpDefaultWorldMemoryWatchDog<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpDefaultWorldMemoryWatchDog"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x77d6b19f)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkpDefaultWorldMemoryWatchDog<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x77d6b19f)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpDefaultWorldMemoryWatchDog",
                    class_meta,
                    (12u64, 24u64),
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "freeHeapMemoryRequested",
                    &self.m_freeHeapMemoryRequested,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpDefaultWorldMemoryWatchDog<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_freeHeapMemoryRequested,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "freeHeapMemoryRequested" => {
                            Ok(__Field::m_freeHeapMemoryRequested)
                        }
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hkpDefaultWorldMemoryWatchDogVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkpDefaultWorldMemoryWatchDog<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpDefaultWorldMemoryWatchDogVisitor<'de> {
                type Value = hkpDefaultWorldMemoryWatchDog<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpDefaultWorldMemoryWatchDog",
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
                    let mut m_freeHeapMemoryRequested: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_freeHeapMemoryRequested,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "freeHeapMemoryRequested",
                                        ),
                                    );
                                }
                                m_freeHeapMemoryRequested = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_freeHeapMemoryRequested = match m_freeHeapMemoryRequested {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "freeHeapMemoryRequested",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpDefaultWorldMemoryWatchDog {
                        __ptr,
                        parent,
                        m_freeHeapMemoryRequested,
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
                    let mut m_freeHeapMemoryRequested: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_freeHeapMemoryRequested => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_freeHeapMemoryRequested,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "freeHeapMemoryRequested",
                                        ),
                                    );
                                }
                                m_freeHeapMemoryRequested = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_freeHeapMemoryRequested = match m_freeHeapMemoryRequested {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "freeHeapMemoryRequested",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkWorldMemoryAvailableWatchDog {
                        __ptr: __ptr.clone(),
                        parent,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpDefaultWorldMemoryWatchDog {
                        __ptr: __ptr.clone(),
                        parent,
                        m_freeHeapMemoryRequested,
                    })
                }
            }
            const FIELDS: &[&str] = &["freeHeapMemoryRequested"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpDefaultWorldMemoryWatchDog",
                FIELDS,
                __hkpDefaultWorldMemoryWatchDogVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpDefaultWorldMemoryWatchDog,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
