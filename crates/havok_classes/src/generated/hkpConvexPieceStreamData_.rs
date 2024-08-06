use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConvexPieceStreamData`
/// - version: `0`
/// - signature: `0xa5bd1d6e`
/// - size: ` 44`(x86)/` 64`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexPieceStreamData {
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
    /// - name: `convexPieceStream`(ctype: `hkArray<hkUint32>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_convexPieceStream: Vec<u32>,
    /// # C++ Info
    /// - name: `convexPieceOffsets`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_convexPieceOffsets: Vec<u32>,
    /// # C++ Info
    /// - name: `convexPieceSingleTriangles`(ctype: `hkArray<hkUint32>`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_convexPieceSingleTriangles: Vec<u32>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConvexPieceStreamData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConvexPieceStreamData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa5bd1d6e)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v
        }
    }
    impl _serde::Serialize for hkpConvexPieceStreamData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa5bd1d6e)));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexPieceStreamData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "convexPieceStream",
                    &self.m_convexPieceStream,
                )?;
            serializer
                .serialize_array_meta_field(
                    "convexPieceOffsets",
                    &self.m_convexPieceOffsets,
                )?;
            serializer
                .serialize_array_meta_field(
                    "convexPieceSingleTriangles",
                    &self.m_convexPieceSingleTriangles,
                )?;
            serializer
                .serialize_array_field("convexPieceStream", &self.m_convexPieceStream)?;
            serializer
                .serialize_array_field(
                    "convexPieceOffsets",
                    &self.m_convexPieceOffsets,
                )?;
            serializer
                .serialize_array_field(
                    "convexPieceSingleTriangles",
                    &self.m_convexPieceSingleTriangles,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConvexPieceStreamData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_convexPieceStream,
                m_convexPieceOffsets,
                m_convexPieceSingleTriangles,
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
                        "convexPieceStream" => Ok(__Field::m_convexPieceStream),
                        "convexPieceOffsets" => Ok(__Field::m_convexPieceOffsets),
                        "convexPieceSingleTriangles" => {
                            Ok(__Field::m_convexPieceSingleTriangles)
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
            struct __hkpConvexPieceStreamDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpConvexPieceStreamData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpConvexPieceStreamDataVisitor<'de> {
                type Value = hkpConvexPieceStreamData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpConvexPieceStreamData",
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
                    let mut m_convexPieceStream: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_convexPieceOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_convexPieceSingleTriangles: _serde::__private::Option<
                        Vec<u32>,
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_convexPieceStream,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieceStream",
                                        ),
                                    );
                                }
                                m_convexPieceStream = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_convexPieceOffsets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieceOffsets",
                                        ),
                                    );
                                }
                                m_convexPieceOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_convexPieceSingleTriangles,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieceSingleTriangles",
                                        ),
                                    );
                                }
                                m_convexPieceSingleTriangles = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
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
                    let m_convexPieceStream = match m_convexPieceStream {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieceStream",
                                ),
                            );
                        }
                    };
                    let m_convexPieceOffsets = match m_convexPieceOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieceOffsets",
                                ),
                            );
                        }
                    };
                    let m_convexPieceSingleTriangles = match m_convexPieceSingleTriangles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieceSingleTriangles",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpConvexPieceStreamData {
                        __ptr,
                        parent,
                        m_convexPieceStream,
                        m_convexPieceOffsets,
                        m_convexPieceSingleTriangles,
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
                    let mut m_convexPieceStream: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_convexPieceOffsets: _serde::__private::Option<Vec<u32>> = _serde::__private::None;
                    let mut m_convexPieceSingleTriangles: _serde::__private::Option<
                        Vec<u32>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_convexPieceStream => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_convexPieceStream,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieceStream",
                                        ),
                                    );
                                }
                                m_convexPieceStream = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_convexPieceOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_convexPieceOffsets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieceOffsets",
                                        ),
                                    );
                                }
                                m_convexPieceOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_convexPieceSingleTriangles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_convexPieceSingleTriangles,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "convexPieceSingleTriangles",
                                        ),
                                    );
                                }
                                m_convexPieceSingleTriangles = _serde::__private::Some(
                                    match __A::next_value::<Vec<u32>>(&mut __map) {
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
                    let m_convexPieceStream = match m_convexPieceStream {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieceStream",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_convexPieceOffsets = match m_convexPieceOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieceOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_convexPieceSingleTriangles = match m_convexPieceSingleTriangles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "convexPieceSingleTriangles",
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
                    _serde::__private::Ok(hkpConvexPieceStreamData {
                        __ptr,
                        parent,
                        m_convexPieceStream,
                        m_convexPieceOffsets,
                        m_convexPieceSingleTriangles,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "convexPieceStream",
                "convexPieceOffsets",
                "convexPieceSingleTriangles",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConvexPieceStreamData",
                FIELDS,
                __hkpConvexPieceStreamDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpConvexPieceStreamData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
