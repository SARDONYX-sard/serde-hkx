use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkMoppBvTreeShapeBase`
/// - version: `0`
/// - signature: `0x7c338c66`
/// - size: ` 48`(x86)/` 80`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMoppBvTreeShapeBase {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpBvTreeShape,
    /// # C++ Info
    /// - name: `code`(ctype: `struct hkpMoppCode*`)
    /// - offset: ` 20`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_code: Pointer,
    /// # C++ Info
    /// - name: `moppData`(ctype: `void*`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_moppData: Pointer,
    /// # C++ Info
    /// - name: `moppDataSize`(ctype: `hkUint32`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_moppDataSize: u32,
    /// # C++ Info
    /// - name: `codeInfoCopy`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_codeInfoCopy: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkMoppBvTreeShapeBase {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMoppBvTreeShapeBase"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7c338c66)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_code.get());
            v.push(self.m_moppData.get());
            v
        }
    }
    impl _serde::Serialize for hkMoppBvTreeShapeBase {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7c338c66)));
            let mut serializer = __serializer
                .serialize_struct("hkMoppBvTreeShapeBase", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bvTreeType", &self.parent.m_bvTreeType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("code", &self.m_code)?;
            serializer.skip_field("moppData", &self.m_moppData)?;
            serializer.skip_field("moppDataSize", &self.m_moppDataSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("codeInfoCopy", &self.m_codeInfoCopy)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMoppBvTreeShapeBase {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_userData,
                m_bvTreeType,
                m_code,
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
                        "userData" => Ok(__Field::m_userData),
                        "bvTreeType" => Ok(__Field::m_bvTreeType),
                        "code" => Ok(__Field::m_code),
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
            struct __hkMoppBvTreeShapeBaseVisitor<'de> {
                marker: _serde::__private::PhantomData<hkMoppBvTreeShapeBase>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkMoppBvTreeShapeBaseVisitor<'de> {
                type Value = hkMoppBvTreeShapeBase;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkMoppBvTreeShapeBase",
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
                    let mut m_code: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_moppData: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_moppDataSize: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_codeInfoCopy: _serde::__private::Option<Vector4> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_code) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("code"),
                                    );
                                }
                                m_code = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_moppData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "moppData",
                                        ),
                                    );
                                }
                                m_moppData = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_moppDataSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "moppDataSize",
                                        ),
                                    );
                                }
                                m_moppDataSize = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_codeInfoCopy) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "codeInfoCopy",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_codeInfoCopy = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_code = match m_code {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("code"),
                            );
                        }
                    };
                    let m_moppData = match m_moppData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("moppData"),
                            );
                        }
                    };
                    let m_moppDataSize = match m_moppDataSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "moppDataSize",
                                ),
                            );
                        }
                    };
                    let m_codeInfoCopy = match m_codeInfoCopy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "codeInfoCopy",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkMoppBvTreeShapeBase {
                        __ptr,
                        parent,
                        m_code,
                        m_moppData,
                        m_moppDataSize,
                        m_codeInfoCopy,
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
                    let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_bvTreeType: _serde::__private::Option<BvTreeType> = _serde::__private::None;
                    let mut m_code: _serde::__private::Option<Pointer> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<u64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bvTreeType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bvTreeType) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bvTreeType",
                                        ),
                                    );
                                }
                                m_bvTreeType = _serde::__private::Some(
                                    match __A::next_value::<BvTreeType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_code => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_code) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("code"),
                                    );
                                }
                                m_code = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
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
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bvTreeType = match m_bvTreeType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bvTreeType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_code = match m_code {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("code"),
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
                    let parent = hkpShape {
                        __ptr,
                        parent,
                        m_userData,
                        ..Default::default()
                    };
                    let parent = hkpBvTreeShape {
                        __ptr,
                        parent,
                        m_bvTreeType,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkMoppBvTreeShapeBase {
                        __ptr,
                        parent,
                        m_code,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "code",
                "moppData",
                "moppDataSize",
                "codeInfoCopy",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMoppBvTreeShapeBase",
                FIELDS,
                __hkMoppBvTreeShapeBaseVisitor {
                    marker: _serde::__private::PhantomData::<hkMoppBvTreeShapeBase>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
