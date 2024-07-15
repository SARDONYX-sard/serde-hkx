use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaSplineCompressedAnimationAnimationCompressionParams`
/// - version: `0`
/// - signature: `0xde830789`
/// - size: `  4`(x86)/`  4`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaSplineCompressedAnimationAnimationCompressionParams {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `maxFramesPerBlock`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_maxFramesPerBlock: u16,
    /// # C++ Info
    /// - name: `enableSampleSingleTracks`(ctype: `hkBool`)
    /// - offset: `  2`(x86)/`  2`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_enableSampleSingleTracks: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkaSplineCompressedAnimationAnimationCompressionParams {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaSplineCompressedAnimationAnimationCompressionParams"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xde830789)
        }
    }
    impl _serde::Serialize for hkaSplineCompressedAnimationAnimationCompressionParams {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xde830789)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaSplineCompressedAnimationAnimationCompressionParams",
                    class_meta,
                )?;
            serializer.serialize_field("maxFramesPerBlock", &self.m_maxFramesPerBlock)?;
            serializer
                .serialize_field(
                    "enableSampleSingleTracks",
                    &self.m_enableSampleSingleTracks,
                )?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_maxFramesPerBlock,
    m_enableSampleSingleTracks,
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
            "maxFramesPerBlock" => Ok(__Field::m_maxFramesPerBlock),
            "enableSampleSingleTracks" => Ok(__Field::m_enableSampleSingleTracks),
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
pub(super) struct __hkaSplineCompressedAnimationAnimationCompressionParamsVisitor<'de> {
    marker: core::marker::PhantomData<
        hkaSplineCompressedAnimationAnimationCompressionParams,
    >,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkaSplineCompressedAnimationAnimationCompressionParamsVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkaSplineCompressedAnimationAnimationCompressionParams,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkaSplineCompressedAnimationAnimationCompressionParams,
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
for __hkaSplineCompressedAnimationAnimationCompressionParamsVisitor<'de> {
    type Value = hkaSplineCompressedAnimationAnimationCompressionParams;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkaSplineCompressedAnimationAnimationCompressionParams",
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
        let mut m_maxFramesPerBlock: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_enableSampleSingleTracks: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_maxFramesPerBlock) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxFramesPerBlock",
                            ),
                        );
                    }
                    m_maxFramesPerBlock = _serde::__private::Some(
                        match __A::next_value::<u16>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_enableSampleSingleTracks) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "enableSampleSingleTracks",
                            ),
                        );
                    }
                    m_enableSampleSingleTracks = _serde::__private::Some(
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
        __A::pad(&mut __map, 1usize, 1usize)?;
        let m_maxFramesPerBlock = match m_maxFramesPerBlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFramesPerBlock"),
                );
            }
        };
        let m_enableSampleSingleTracks = match m_enableSampleSingleTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableSampleSingleTracks",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkaSplineCompressedAnimationAnimationCompressionParams {
            __ptr,
            m_maxFramesPerBlock,
            m_enableSampleSingleTracks,
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
        let mut m_maxFramesPerBlock: _serde::__private::Option<u16> = _serde::__private::None;
        let mut m_enableSampleSingleTracks: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_maxFramesPerBlock => {
                        if _serde::__private::Option::is_some(&m_maxFramesPerBlock) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxFramesPerBlock",
                                ),
                            );
                        }
                        m_maxFramesPerBlock = _serde::__private::Some(
                            match __A::next_value::<u16>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_enableSampleSingleTracks => {
                        if _serde::__private::Option::is_some(
                            &m_enableSampleSingleTracks,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "enableSampleSingleTracks",
                                ),
                            );
                        }
                        m_enableSampleSingleTracks = _serde::__private::Some(
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
        }
        let m_maxFramesPerBlock = match m_maxFramesPerBlock {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxFramesPerBlock"),
                );
            }
        };
        let m_enableSampleSingleTracks = match m_enableSampleSingleTracks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "enableSampleSingleTracks",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkaSplineCompressedAnimationAnimationCompressionParams {
            __ptr,
            m_maxFramesPerBlock,
            m_enableSampleSingleTracks,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkaSplineCompressedAnimationAnimationCompressionParams {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["maxFramesPerBlock", "enableSampleSingleTracks"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaSplineCompressedAnimationAnimationCompressionParams",
                FIELDS,
                __hkaSplineCompressedAnimationAnimationCompressionParamsVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaSplineCompressedAnimationAnimationCompressionParams,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
