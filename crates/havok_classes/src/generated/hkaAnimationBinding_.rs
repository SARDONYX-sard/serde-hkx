use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaAnimationBinding`
/// - version: `1`
/// - signature: `0x66eac971`
/// - size: ` 44`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaAnimationBinding<'a> {
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
    /// - name: `originalSkeletonName`(ctype: `hkStringPtr`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_originalSkeletonName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `animation`(ctype: `struct hkaAnimation*`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_animation: Pointer,
    /// # C++ Info
    /// - name: `transformTrackToBoneIndices`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_transformTrackToBoneIndices: Vec<i16>,
    /// # C++ Info
    /// - name: `floatTrackToFloatSlotIndices`(ctype: `hkArray<hkInt16>`)
    /// - offset: ` 28`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_floatTrackToFloatSlotIndices: Vec<i16>,
    /// # C++ Info
    /// - name: `blendHint`(ctype: `enum BlendHint`)
    /// - offset: ` 40`(x86)/` 64`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_blendHint: BlendHint,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaAnimationBinding<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaAnimationBinding"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x66eac971)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_animation.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkaAnimationBinding<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x66eac971)));
            let mut serializer = __serializer
                .serialize_struct("hkaAnimationBinding", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer.serialize_field("animation", &self.m_animation)?;
            serializer
                .serialize_array_meta_field(
                    "transformTrackToBoneIndices",
                    &self.m_transformTrackToBoneIndices,
                )?;
            serializer
                .serialize_array_meta_field(
                    "floatTrackToFloatSlotIndices",
                    &self.m_floatTrackToFloatSlotIndices,
                )?;
            serializer.serialize_field("blendHint", &self.m_blendHint)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_stringptr_field(
                    "originalSkeletonName",
                    &self.m_originalSkeletonName,
                )?;
            serializer
                .serialize_array_field(
                    "transformTrackToBoneIndices",
                    &self.m_transformTrackToBoneIndices,
                )?;
            serializer
                .serialize_array_field(
                    "floatTrackToFloatSlotIndices",
                    &self.m_floatTrackToFloatSlotIndices,
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
    impl<'de> _serde::Deserialize<'de> for hkaAnimationBinding<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_originalSkeletonName,
                m_animation,
                m_transformTrackToBoneIndices,
                m_floatTrackToFloatSlotIndices,
                m_blendHint,
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
                        "originalSkeletonName" => Ok(__Field::m_originalSkeletonName),
                        "animation" => Ok(__Field::m_animation),
                        "transformTrackToBoneIndices" => {
                            Ok(__Field::m_transformTrackToBoneIndices)
                        }
                        "floatTrackToFloatSlotIndices" => {
                            Ok(__Field::m_floatTrackToFloatSlotIndices)
                        }
                        "blendHint" => Ok(__Field::m_blendHint),
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
            struct __hkaAnimationBindingVisitor<'de> {
                marker: _serde::__private::PhantomData<hkaAnimationBinding<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkaAnimationBindingVisitor<'de> {
                type Value = hkaAnimationBinding<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaAnimationBinding",
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
                    let mut m_originalSkeletonName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_animation: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_transformTrackToBoneIndices: _serde::__private::Option<
                        Vec<i16>,
                    > = _serde::__private::None;
                    let mut m_floatTrackToFloatSlotIndices: _serde::__private::Option<
                        Vec<i16>,
                    > = _serde::__private::None;
                    let mut m_blendHint: _serde::__private::Option<BlendHint> = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_originalSkeletonName,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalSkeletonName",
                                        ),
                                    );
                                }
                                m_originalSkeletonName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_animation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animation",
                                        ),
                                    );
                                }
                                m_animation = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transformTrackToBoneIndices,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformTrackToBoneIndices",
                                        ),
                                    );
                                }
                                m_transformTrackToBoneIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_floatTrackToFloatSlotIndices,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatTrackToFloatSlotIndices",
                                        ),
                                    );
                                }
                                m_floatTrackToFloatSlotIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_blendHint) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendHint",
                                        ),
                                    );
                                }
                                m_blendHint = _serde::__private::Some(
                                    match __A::next_value::<BlendHint>(&mut __map) {
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
                    let m_originalSkeletonName = match m_originalSkeletonName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalSkeletonName",
                                ),
                            );
                        }
                    };
                    let m_animation = match m_animation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animation",
                                ),
                            );
                        }
                    };
                    let m_transformTrackToBoneIndices = match m_transformTrackToBoneIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformTrackToBoneIndices",
                                ),
                            );
                        }
                    };
                    let m_floatTrackToFloatSlotIndices = match m_floatTrackToFloatSlotIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatTrackToFloatSlotIndices",
                                ),
                            );
                        }
                    };
                    let m_blendHint = match m_blendHint {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendHint",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaAnimationBinding {
                        __ptr,
                        parent,
                        m_originalSkeletonName,
                        m_animation,
                        m_transformTrackToBoneIndices,
                        m_floatTrackToFloatSlotIndices,
                        m_blendHint,
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
                    let mut m_originalSkeletonName: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_animation: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_transformTrackToBoneIndices: _serde::__private::Option<
                        Vec<i16>,
                    > = _serde::__private::None;
                    let mut m_floatTrackToFloatSlotIndices: _serde::__private::Option<
                        Vec<i16>,
                    > = _serde::__private::None;
                    let mut m_blendHint: _serde::__private::Option<BlendHint> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_originalSkeletonName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_originalSkeletonName,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "originalSkeletonName",
                                        ),
                                    );
                                }
                                m_originalSkeletonName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_animation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_animation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animation",
                                        ),
                                    );
                                }
                                m_animation = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transformTrackToBoneIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transformTrackToBoneIndices,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformTrackToBoneIndices",
                                        ),
                                    );
                                }
                                m_transformTrackToBoneIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatTrackToFloatSlotIndices => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_floatTrackToFloatSlotIndices,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatTrackToFloatSlotIndices",
                                        ),
                                    );
                                }
                                m_floatTrackToFloatSlotIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<i16>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blendHint => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendHint) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendHint",
                                        ),
                                    );
                                }
                                m_blendHint = _serde::__private::Some(
                                    match __A::next_value::<BlendHint>(&mut __map) {
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
                    let m_originalSkeletonName = match m_originalSkeletonName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "originalSkeletonName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_animation = match m_animation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transformTrackToBoneIndices = match m_transformTrackToBoneIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformTrackToBoneIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatTrackToFloatSlotIndices = match m_floatTrackToFloatSlotIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatTrackToFloatSlotIndices",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendHint = match m_blendHint {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendHint",
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
                    _serde::__private::Ok(hkaAnimationBinding {
                        __ptr,
                        parent,
                        m_originalSkeletonName,
                        m_animation,
                        m_transformTrackToBoneIndices,
                        m_floatTrackToFloatSlotIndices,
                        m_blendHint,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "originalSkeletonName",
                "animation",
                "transformTrackToBoneIndices",
                "floatTrackToFloatSlotIndices",
                "blendHint",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaAnimationBinding",
                FIELDS,
                __hkaAnimationBindingVisitor {
                    marker: _serde::__private::PhantomData::<hkaAnimationBinding>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum BlendHint {
    #[default]
    NORMAL = 0isize,
    ADDITIVE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for BlendHint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NORMAL => __serializer.serialize_field("NORMAL", &0u64),
                Self::ADDITIVE => __serializer.serialize_field("ADDITIVE", &1u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum BlendHint to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BlendHint {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("NORMAL") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("ADDITIVE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BlendHint>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BlendHint;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum BlendHint",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(BlendHint::NORMAL)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(BlendHint::ADDITIVE)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["NORMAL", "ADDITIVE"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "BlendHint",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BlendHint>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
