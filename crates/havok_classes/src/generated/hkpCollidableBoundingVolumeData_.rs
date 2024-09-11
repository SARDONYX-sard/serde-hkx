use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCollidableBoundingVolumeData`
/// - version: `0`
/// - signature: `0xb5f0e6b1`
/// - size: ` 44`(x86)/` 56`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollidableBoundingVolumeData {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `min`(ctype: `hkUint32[3]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_min: [u32; 3usize],
    /// # C++ Info
    /// - name: `expansionMin`(ctype: `hkUint8[3]`)
    /// - offset: ` 12`(x86)/` 12`(x86_64)
    /// - type_size: `  3`(x86)/`  3`(x86_64)
    pub m_expansionMin: [u8; 3usize],
    /// # C++ Info
    /// - name: `expansionShift`(ctype: `hkUint8`)
    /// - offset: ` 15`(x86)/` 15`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_expansionShift: u8,
    /// # C++ Info
    /// - name: `max`(ctype: `hkUint32[3]`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    pub m_max: [u32; 3usize],
    /// # C++ Info
    /// - name: `expansionMax`(ctype: `hkUint8[3]`)
    /// - offset: ` 28`(x86)/` 28`(x86_64)
    /// - type_size: `  3`(x86)/`  3`(x86_64)
    pub m_expansionMax: [u8; 3usize],
    /// # C++ Info
    /// - name: `padding`(ctype: `hkUint8`)
    /// - offset: ` 31`(x86)/` 31`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_padding: u8,
    /// # C++ Info
    /// - name: `numChildShapeAabbs`(ctype: `hkUint16`)
    /// - offset: ` 32`(x86)/` 32`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_numChildShapeAabbs: u16,
    /// # C++ Info
    /// - name: `capacityChildShapeAabbs`(ctype: `hkUint16`)
    /// - offset: ` 34`(x86)/` 34`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_capacityChildShapeAabbs: u16,
    /// # C++ Info
    /// - name: `childShapeAabbs`(ctype: `void*`)
    /// - offset: ` 36`(x86)/` 40`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_childShapeAabbs: Pointer,
    /// # C++ Info
    /// - name: `childShapeKeys`(ctype: `void*`)
    /// - offset: ` 40`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_childShapeKeys: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCollidableBoundingVolumeData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCollidableBoundingVolumeData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb5f0e6b1)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_childShapeAabbs.get());
            v.push(self.m_childShapeKeys.get());
            v
        }
    }
    impl _serde::Serialize for hkpCollidableBoundingVolumeData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb5f0e6b1)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpCollidableBoundingVolumeData",
                    class_meta,
                    (44u64, 56u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "min",
                    self.m_min.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "expansionMin",
                    self.m_expansionMin.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("expansionShift", &self.m_expansionShift)?;
            serializer
                .serialize_fixed_array_field(
                    "max",
                    self.m_max.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "expansionMax",
                    self.m_expansionMax.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("padding", &self.m_padding)?;
            serializer.skip_field("numChildShapeAabbs", &self.m_numChildShapeAabbs)?;
            serializer
                .skip_field("capacityChildShapeAabbs", &self.m_capacityChildShapeAabbs)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("childShapeAabbs", &self.m_childShapeAabbs)?;
            serializer.skip_field("childShapeKeys", &self.m_childShapeKeys)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpCollidableBoundingVolumeData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_min,
                m_expansionMin,
                m_expansionShift,
                m_max,
                m_expansionMax,
                m_padding,
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
                        "min" => Ok(__Field::m_min),
                        "expansionMin" => Ok(__Field::m_expansionMin),
                        "expansionShift" => Ok(__Field::m_expansionShift),
                        "max" => Ok(__Field::m_max),
                        "expansionMax" => Ok(__Field::m_expansionMax),
                        "padding" => Ok(__Field::m_padding),
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
            struct __hkpCollidableBoundingVolumeDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpCollidableBoundingVolumeData>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpCollidableBoundingVolumeDataVisitor<'de> {
                type Value = hkpCollidableBoundingVolumeData;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpCollidableBoundingVolumeData",
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
                    let mut m_min: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
                    let mut m_expansionMin: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
                    let mut m_expansionShift: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_max: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
                    let mut m_expansionMax: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
                    let mut m_padding: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_numChildShapeAabbs: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_capacityChildShapeAabbs: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_childShapeAabbs: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_childShapeKeys: _serde::__private::Option<Pointer> = _serde::__private::None;
                    for i in 0..10usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_min) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("min"),
                                    );
                                }
                                m_min = _serde::__private::Some(
                                    match __A::next_value::<[u32; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_expansionMin) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expansionMin",
                                        ),
                                    );
                                }
                                m_expansionMin = _serde::__private::Some(
                                    match __A::next_value::<[u8; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_expansionShift) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expansionShift",
                                        ),
                                    );
                                }
                                m_expansionShift = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_max) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("max"),
                                    );
                                }
                                m_max = _serde::__private::Some(
                                    match __A::next_value::<[u32; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_expansionMax) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expansionMax",
                                        ),
                                    );
                                }
                                m_expansionMax = _serde::__private::Some(
                                    match __A::next_value::<[u8; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_padding) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "padding",
                                        ),
                                    );
                                }
                                m_padding = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numChildShapeAabbs,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numChildShapeAabbs",
                                        ),
                                    );
                                }
                                m_numChildShapeAabbs = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_capacityChildShapeAabbs,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "capacityChildShapeAabbs",
                                        ),
                                    );
                                }
                                m_capacityChildShapeAabbs = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_childShapeAabbs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "childShapeAabbs",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_childShapeAabbs = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_childShapeKeys) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "childShapeKeys",
                                        ),
                                    );
                                }
                                m_childShapeKeys = _serde::__private::Some(
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
                    let m_min = match m_min {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("min"),
                            );
                        }
                    };
                    let m_expansionMin = match m_expansionMin {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expansionMin",
                                ),
                            );
                        }
                    };
                    let m_expansionShift = match m_expansionShift {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expansionShift",
                                ),
                            );
                        }
                    };
                    let m_max = match m_max {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("max"),
                            );
                        }
                    };
                    let m_expansionMax = match m_expansionMax {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expansionMax",
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
                    let m_numChildShapeAabbs = match m_numChildShapeAabbs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numChildShapeAabbs",
                                ),
                            );
                        }
                    };
                    let m_capacityChildShapeAabbs = match m_capacityChildShapeAabbs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "capacityChildShapeAabbs",
                                ),
                            );
                        }
                    };
                    let m_childShapeAabbs = match m_childShapeAabbs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "childShapeAabbs",
                                ),
                            );
                        }
                    };
                    let m_childShapeKeys = match m_childShapeKeys {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "childShapeKeys",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpCollidableBoundingVolumeData {
                        __ptr,
                        m_min,
                        m_expansionMin,
                        m_expansionShift,
                        m_max,
                        m_expansionMax,
                        m_padding,
                        m_numChildShapeAabbs,
                        m_capacityChildShapeAabbs,
                        m_childShapeAabbs,
                        m_childShapeKeys,
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
                    let mut m_min: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
                    let mut m_expansionMin: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
                    let mut m_expansionShift: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_max: _serde::__private::Option<[u32; 3usize]> = _serde::__private::None;
                    let mut m_expansionMax: _serde::__private::Option<[u8; 3usize]> = _serde::__private::None;
                    let mut m_padding: _serde::__private::Option<u8> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_min => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_min) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("min"),
                                    );
                                }
                                m_min = _serde::__private::Some(
                                    match __A::next_value::<[u32; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_expansionMin => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_expansionMin) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expansionMin",
                                        ),
                                    );
                                }
                                m_expansionMin = _serde::__private::Some(
                                    match __A::next_value::<[u8; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_expansionShift => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_expansionShift) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expansionShift",
                                        ),
                                    );
                                }
                                m_expansionShift = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_max => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_max) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("max"),
                                    );
                                }
                                m_max = _serde::__private::Some(
                                    match __A::next_value::<[u32; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_expansionMax => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_expansionMax) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "expansionMax",
                                        ),
                                    );
                                }
                                m_expansionMax = _serde::__private::Some(
                                    match __A::next_value::<[u8; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_padding => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_padding) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "padding",
                                        ),
                                    );
                                }
                                m_padding = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
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
                    let m_min = match m_min {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("min"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_expansionMin = match m_expansionMin {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expansionMin",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_expansionShift = match m_expansionShift {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expansionShift",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_max = match m_max {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("max"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_expansionMax = match m_expansionMax {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "expansionMax",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_padding = match m_padding {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("padding"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpCollidableBoundingVolumeData {
                        __ptr,
                        m_min,
                        m_expansionMin,
                        m_expansionShift,
                        m_max,
                        m_expansionMax,
                        m_padding,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "min",
                "expansionMin",
                "expansionShift",
                "max",
                "expansionMax",
                "padding",
                "numChildShapeAabbs",
                "capacityChildShapeAabbs",
                "childShapeAabbs",
                "childShapeKeys",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCollidableBoundingVolumeData",
                FIELDS,
                __hkpCollidableBoundingVolumeDataVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpCollidableBoundingVolumeData,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
