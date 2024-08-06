use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkTrackerSerializableScanSnapshot`
/// - version: `0`
/// - signature: `0x875af1d9`
/// - size: ` 92`(x86)/`128`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkTrackerSerializableScanSnapshot {
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
    /// - name: `allocations`(ctype: `hkArray<struct hkTrackerSerializableScanSnapshotAllocation>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_allocations: Vec<hkTrackerSerializableScanSnapshotAllocation>,
    /// # C++ Info
    /// - name: `blocks`(ctype: `hkArray<struct hkTrackerSerializableScanSnapshotBlock>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_blocks: Vec<hkTrackerSerializableScanSnapshotBlock>,
    /// # C++ Info
    /// - name: `refs`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_refs: Vec<i32>,
    /// # C++ Info
    /// - name: `typeNames`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 44`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_typeNames: Vec<u8>,
    /// # C++ Info
    /// - name: `traceText`(ctype: `hkArray<hkUint8>`)
    /// - offset: ` 56`(x86)/` 80`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_traceText: Vec<u8>,
    /// # C++ Info
    /// - name: `traceAddrs`(ctype: `hkArray<hkUint64>`)
    /// - offset: ` 68`(x86)/` 96`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_traceAddrs: Vec<u64>,
    /// # C++ Info
    /// - name: `traceParents`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_traceParents: Vec<i32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkTrackerSerializableScanSnapshot {
        #[inline]
        fn name(&self) -> &'static str {
            "hkTrackerSerializableScanSnapshot"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x875af1d9)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_allocations
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v.extend(
                self
                    .m_blocks
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<usize>>(),
            );
            v
        }
    }
    impl _serde::Serialize for hkTrackerSerializableScanSnapshot {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x875af1d9)));
            let mut serializer = __serializer
                .serialize_struct("hkTrackerSerializableScanSnapshot", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("allocations", &self.m_allocations)?;
            serializer.serialize_array_meta_field("blocks", &self.m_blocks)?;
            serializer.serialize_array_meta_field("refs", &self.m_refs)?;
            serializer.serialize_array_meta_field("typeNames", &self.m_typeNames)?;
            serializer.serialize_array_meta_field("traceText", &self.m_traceText)?;
            serializer.serialize_array_meta_field("traceAddrs", &self.m_traceAddrs)?;
            serializer.serialize_array_meta_field("traceParents", &self.m_traceParents)?;
            serializer.serialize_array_field("allocations", &self.m_allocations)?;
            serializer.serialize_array_field("blocks", &self.m_blocks)?;
            serializer.serialize_array_field("refs", &self.m_refs)?;
            serializer.serialize_array_field("typeNames", &self.m_typeNames)?;
            serializer.serialize_array_field("traceText", &self.m_traceText)?;
            serializer.serialize_array_field("traceAddrs", &self.m_traceAddrs)?;
            serializer.serialize_array_field("traceParents", &self.m_traceParents)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkTrackerSerializableScanSnapshot {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_allocations,
                m_blocks,
                m_refs,
                m_typeNames,
                m_traceText,
                m_traceAddrs,
                m_traceParents,
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
                        "allocations" => Ok(__Field::m_allocations),
                        "blocks" => Ok(__Field::m_blocks),
                        "refs" => Ok(__Field::m_refs),
                        "typeNames" => Ok(__Field::m_typeNames),
                        "traceText" => Ok(__Field::m_traceText),
                        "traceAddrs" => Ok(__Field::m_traceAddrs),
                        "traceParents" => Ok(__Field::m_traceParents),
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
            struct __hkTrackerSerializableScanSnapshotVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkTrackerSerializableScanSnapshot,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkTrackerSerializableScanSnapshotVisitor<'de> {
                type Value = hkTrackerSerializableScanSnapshot;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkTrackerSerializableScanSnapshot",
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
                    let mut m_allocations: _serde::__private::Option<
                        Vec<hkTrackerSerializableScanSnapshotAllocation>,
                    > = _serde::__private::None;
                    let mut m_blocks: _serde::__private::Option<
                        Vec<hkTrackerSerializableScanSnapshotBlock>,
                    > = _serde::__private::None;
                    let mut m_refs: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_typeNames: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_traceText: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_traceAddrs: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
                    let mut m_traceParents: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_allocations) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allocations",
                                        ),
                                    );
                                }
                                m_allocations = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkTrackerSerializableScanSnapshotAllocation>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_blocks) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("blocks"),
                                    );
                                }
                                m_blocks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkTrackerSerializableScanSnapshotBlock>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_refs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("refs"),
                                    );
                                }
                                m_refs = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_typeNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "typeNames",
                                        ),
                                    );
                                }
                                m_typeNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_traceText) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "traceText",
                                        ),
                                    );
                                }
                                m_traceText = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_traceAddrs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "traceAddrs",
                                        ),
                                    );
                                }
                                m_traceAddrs = _serde::__private::Some(
                                    match __A::next_value::<Vec<u64>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_traceParents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "traceParents",
                                        ),
                                    );
                                }
                                m_traceParents = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
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
                    let m_allocations = match m_allocations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allocations",
                                ),
                            );
                        }
                    };
                    let m_blocks = match m_blocks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("blocks"),
                            );
                        }
                    };
                    let m_refs = match m_refs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("refs"),
                            );
                        }
                    };
                    let m_typeNames = match m_typeNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "typeNames",
                                ),
                            );
                        }
                    };
                    let m_traceText = match m_traceText {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "traceText",
                                ),
                            );
                        }
                    };
                    let m_traceAddrs = match m_traceAddrs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "traceAddrs",
                                ),
                            );
                        }
                    };
                    let m_traceParents = match m_traceParents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "traceParents",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkTrackerSerializableScanSnapshot {
                        __ptr,
                        parent,
                        m_allocations,
                        m_blocks,
                        m_refs,
                        m_typeNames,
                        m_traceText,
                        m_traceAddrs,
                        m_traceParents,
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
                    let mut m_allocations: _serde::__private::Option<
                        Vec<hkTrackerSerializableScanSnapshotAllocation>,
                    > = _serde::__private::None;
                    let mut m_blocks: _serde::__private::Option<
                        Vec<hkTrackerSerializableScanSnapshotBlock>,
                    > = _serde::__private::None;
                    let mut m_refs: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    let mut m_typeNames: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_traceText: _serde::__private::Option<Vec<u8>> = _serde::__private::None;
                    let mut m_traceAddrs: _serde::__private::Option<Vec<u64>> = _serde::__private::None;
                    let mut m_traceParents: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_allocations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_allocations) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allocations",
                                        ),
                                    );
                                }
                                m_allocations = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkTrackerSerializableScanSnapshotAllocation>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blocks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blocks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("blocks"),
                                    );
                                }
                                m_blocks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkTrackerSerializableScanSnapshotBlock>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_refs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_refs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("refs"),
                                    );
                                }
                                m_refs = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_typeNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_typeNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "typeNames",
                                        ),
                                    );
                                }
                                m_typeNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_traceText => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_traceText) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "traceText",
                                        ),
                                    );
                                }
                                m_traceText = _serde::__private::Some(
                                    match __A::next_value::<Vec<u8>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_traceAddrs => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_traceAddrs) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "traceAddrs",
                                        ),
                                    );
                                }
                                m_traceAddrs = _serde::__private::Some(
                                    match __A::next_value::<Vec<u64>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_traceParents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_traceParents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "traceParents",
                                        ),
                                    );
                                }
                                m_traceParents = _serde::__private::Some(
                                    match __A::next_value::<Vec<i32>>(&mut __map) {
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
                    let m_allocations = match m_allocations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allocations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blocks = match m_blocks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("blocks"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_refs = match m_refs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("refs"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_typeNames = match m_typeNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "typeNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_traceText = match m_traceText {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "traceText",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_traceAddrs = match m_traceAddrs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "traceAddrs",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_traceParents = match m_traceParents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "traceParents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkTrackerSerializableScanSnapshot {
                        __ptr,
                        parent,
                        m_allocations,
                        m_blocks,
                        m_refs,
                        m_typeNames,
                        m_traceText,
                        m_traceAddrs,
                        m_traceParents,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "allocations",
                "blocks",
                "refs",
                "typeNames",
                "traceText",
                "traceAddrs",
                "traceParents",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkTrackerSerializableScanSnapshot",
                FIELDS,
                __hkTrackerSerializableScanSnapshotVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkTrackerSerializableScanSnapshot,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
