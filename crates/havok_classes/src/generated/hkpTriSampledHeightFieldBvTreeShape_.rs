use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpTriSampledHeightFieldBvTreeShape`
/// - version: `0`
/// - signature: `0x58e1e585`
/// - size: ` 48`(x86)/` 80`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTriSampledHeightFieldBvTreeShape {
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
    /// - name: `childContainer`(ctype: `struct hkpSingleShapeContainer`)
    /// - offset: ` 20`(x86)/` 40`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    pub m_childContainer: hkpSingleShapeContainer,
    /// # C++ Info
    /// - name: `childSize`(ctype: `hkInt32`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_childSize: i32,
    /// # C++ Info
    /// - name: `wantAabbRejectionTest`(ctype: `hkBool`)
    /// - offset: ` 32`(x86)/` 60`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_wantAabbRejectionTest: bool,
    /// # C++ Info
    /// - name: `padding`(ctype: `hkUint8[12]`)
    /// - offset: ` 33`(x86)/` 61`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_padding: [u8; 12usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTriSampledHeightFieldBvTreeShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTriSampledHeightFieldBvTreeShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x58e1e585)
        }
    }
    impl _serde::Serialize for hkpTriSampledHeightFieldBvTreeShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x58e1e585)));
            let mut serializer = __serializer
                .serialize_struct("hkpTriSampledHeightFieldBvTreeShape", class_meta)?;
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
            serializer.serialize_field("childContainer", &self.m_childContainer)?;
            serializer.skip_field("childSize", &self.m_childSize)?;
            serializer
                .serialize_field(
                    "wantAabbRejectionTest",
                    &self.m_wantAabbRejectionTest,
                )?;
            serializer.serialize_field("padding", &self.m_padding.as_slice())?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_childContainer,
    m_childSize,
    m_wantAabbRejectionTest,
    m_padding,
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
            "childContainer" => Ok(__Field::m_childContainer),
            "wantAabbRejectionTest" => Ok(__Field::m_wantAabbRejectionTest),
            "padding" => Ok(__Field::m_padding),
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
pub(super) struct __hkpTriSampledHeightFieldBvTreeShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpTriSampledHeightFieldBvTreeShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpTriSampledHeightFieldBvTreeShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpTriSampledHeightFieldBvTreeShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkpTriSampledHeightFieldBvTreeShape,
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
impl<'de> _serde::de::Visitor<'de>
for __hkpTriSampledHeightFieldBvTreeShapeVisitor<'de> {
    type Value = hkpTriSampledHeightFieldBvTreeShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpTriSampledHeightFieldBvTreeShape",
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
        let mut m_childContainer: _serde::__private::Option<hkpSingleShapeContainer> = _serde::__private::None;
        let mut m_childSize: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_wantAabbRejectionTest: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<[u8; 12usize]> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_childContainer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childContainer",
                            ),
                        );
                    }
                    m_childContainer = _serde::__private::Some(
                        match __A::next_value::<hkpSingleShapeContainer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_childSize) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childSize",
                            ),
                        );
                    }
                    m_childSize = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_wantAabbRejectionTest) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wantAabbRejectionTest",
                            ),
                        );
                    }
                    m_wantAabbRejectionTest = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_padding) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("padding"),
                        );
                    }
                    m_padding = _serde::__private::Some(
                        match __A::next_value::<[u8; 12usize]>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 7usize)?;
        let m_childContainer = match m_childContainer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childContainer"),
                );
            }
        };
        let m_childSize = match m_childSize {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childSize"),
                );
            }
        };
        let m_wantAabbRejectionTest = match m_wantAabbRejectionTest {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wantAabbRejectionTest",
                    ),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkpTriSampledHeightFieldBvTreeShape {
            __ptr,
            parent,
            m_childContainer,
            m_childSize,
            m_wantAabbRejectionTest,
            m_padding,
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
        let parent = __hkpBvTreeShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_childContainer: _serde::__private::Option<hkpSingleShapeContainer> = _serde::__private::None;
        let mut m_wantAabbRejectionTest: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_padding: _serde::__private::Option<[u8; 12usize]> = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_childContainer => {
                        if _serde::__private::Option::is_some(&m_childContainer) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "childContainer",
                                ),
                            );
                        }
                        m_childContainer = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSingleShapeContainer,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_wantAabbRejectionTest => {
                        if _serde::__private::Option::is_some(&m_wantAabbRejectionTest) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wantAabbRejectionTest",
                                ),
                            );
                        }
                        m_wantAabbRejectionTest = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_padding => {
                        if _serde::__private::Option::is_some(&m_padding) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "padding",
                                ),
                            );
                        }
                        m_padding = _serde::__private::Some(
                            match __A::next_value::<[u8; 12usize]>(&mut __map) {
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
        let m_childContainer = match m_childContainer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childContainer"),
                );
            }
        };
        let m_wantAabbRejectionTest = match m_wantAabbRejectionTest {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wantAabbRejectionTest",
                    ),
                );
            }
        };
        let m_padding = match m_padding {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("padding"),
                );
            }
        };
        _serde::__private::Ok(hkpTriSampledHeightFieldBvTreeShape {
            __ptr,
            parent,
            m_childContainer,
            m_wantAabbRejectionTest,
            m_padding,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpTriSampledHeightFieldBvTreeShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "childContainer",
                "childSize",
                "wantAabbRejectionTest",
                "padding",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpTriSampledHeightFieldBvTreeShape",
                FIELDS,
                __hkpTriSampledHeightFieldBvTreeShapeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpTriSampledHeightFieldBvTreeShape,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
