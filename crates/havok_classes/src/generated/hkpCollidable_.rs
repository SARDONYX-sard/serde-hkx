use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpCollidable`
/// - version: `0`
/// - signature: `0x9a0e42a5`
/// - size: ` 80`(x86)/`112`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollidable {
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
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub parent: hkpCdBody,
    /// # C++ Info
    /// - name: `ownerOffset`(ctype: `hkInt8`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "ownerOffset"))]
    #[cfg_attr(feature = "serde", serde(rename = "ownerOffset"))]
    pub m_ownerOffset: i8,
    /// # C++ Info
    /// - name: `forceCollideOntoPpu`(ctype: `hkUint8`)
    /// - offset: ` 17`(x86)/` 33`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "forceCollideOntoPpu"))]
    #[cfg_attr(feature = "serde", serde(rename = "forceCollideOntoPpu"))]
    pub m_forceCollideOntoPpu: u8,
    /// # C++ Info
    /// - name: `shapeSizeOnSpu`(ctype: `hkUint16`)
    /// - offset: ` 18`(x86)/` 34`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "shapeSizeOnSpu"))]
    #[cfg_attr(feature = "serde", serde(rename = "shapeSizeOnSpu"))]
    pub m_shapeSizeOnSpu: u16,
    /// # C++ Info
    /// - name: `broadPhaseHandle`(ctype: `struct hkpTypedBroadPhaseHandle`)
    /// - offset: ` 20`(x86)/` 36`(x86_64)
    /// - type_size: ` 12`(x86)/` 12`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "broadPhaseHandle"))]
    #[cfg_attr(feature = "serde", serde(rename = "broadPhaseHandle"))]
    pub m_broadPhaseHandle: hkpTypedBroadPhaseHandle,
    /// # C++ Info
    /// - name: `boundingVolumeData`(ctype: `struct hkpCollidableBoundingVolumeData`)
    /// - offset: ` 32`(x86)/` 48`(x86_64)
    /// - type_size: ` 44`(x86)/` 56`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "json_schema", schemars(rename = "boundingVolumeData"))]
    #[cfg_attr(feature = "serde", serde(rename = "boundingVolumeData"))]
    pub m_boundingVolumeData: hkpCollidableBoundingVolumeData,
    /// # C++ Info
    /// - name: `allowedPenetrationDepth`(ctype: `hkReal`)
    /// - offset: ` 76`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "json_schema", schemars(rename = "allowedPenetrationDepth"))]
    #[cfg_attr(feature = "serde", serde(rename = "allowedPenetrationDepth"))]
    pub m_allowedPenetrationDepth: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCollidable {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCollidable"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9a0e42a5)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.m_shape.get());
            v.push(self.parent.m_motion.get());
            v.push(self.parent.m_parent.get());
            v.extend(self.m_broadPhaseHandle.deps_indexes());
            v.extend(self.m_boundingVolumeData.deps_indexes());
            v
        }
    }
    impl _serde::Serialize for hkpCollidable {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9a0e42a5)));
            let mut serializer = __serializer
                .serialize_struct("hkpCollidable", class_meta, (80u64, 112u64))?;
            serializer.serialize_field("shape", &self.parent.m_shape)?;
            serializer.serialize_field("shapeKey", &self.parent.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("motion", &self.parent.m_motion)?;
            serializer.skip_field("parent", &self.parent.m_parent)?;
            serializer.skip_field("ownerOffset", &self.m_ownerOffset)?;
            serializer
                .serialize_field("forceCollideOntoPpu", &self.m_forceCollideOntoPpu)?;
            serializer.skip_field("shapeSizeOnSpu", &self.m_shapeSizeOnSpu)?;
            serializer.serialize_field("broadPhaseHandle", &self.m_broadPhaseHandle)?;
            serializer.skip_field("boundingVolumeData", &self.m_boundingVolumeData)?;
            serializer
                .serialize_field(
                    "allowedPenetrationDepth",
                    &self.m_allowedPenetrationDepth,
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
    impl<'de> _serde::Deserialize<'de> for hkpCollidable {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_shape,
                m_shapeKey,
                m_forceCollideOntoPpu,
                m_broadPhaseHandle,
                m_allowedPenetrationDepth,
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
                        "shape" => Ok(__Field::m_shape),
                        "shapeKey" => Ok(__Field::m_shapeKey),
                        "forceCollideOntoPpu" => Ok(__Field::m_forceCollideOntoPpu),
                        "broadPhaseHandle" => Ok(__Field::m_broadPhaseHandle),
                        "allowedPenetrationDepth" => {
                            Ok(__Field::m_allowedPenetrationDepth)
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
            struct __hkpCollidableVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpCollidable>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkpCollidableVisitor<'de> {
                type Value = hkpCollidable;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkpCollidable")
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
                    let mut m_ownerOffset: _serde::__private::Option<i8> = _serde::__private::None;
                    let mut m_forceCollideOntoPpu: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_shapeSizeOnSpu: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_broadPhaseHandle: _serde::__private::Option<
                        hkpTypedBroadPhaseHandle,
                    > = _serde::__private::None;
                    let mut m_boundingVolumeData: _serde::__private::Option<
                        hkpCollidableBoundingVolumeData,
                    > = _serde::__private::None;
                    let mut m_allowedPenetrationDepth: _serde::__private::Option<f32> = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_ownerOffset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ownerOffset",
                                        ),
                                    );
                                }
                                m_ownerOffset = _serde::__private::Some(
                                    match __A::next_value::<i8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_forceCollideOntoPpu,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forceCollideOntoPpu",
                                        ),
                                    );
                                }
                                m_forceCollideOntoPpu = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_shapeSizeOnSpu) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeSizeOnSpu",
                                        ),
                                    );
                                }
                                m_shapeSizeOnSpu = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_broadPhaseHandle) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseHandle",
                                        ),
                                    );
                                }
                                m_broadPhaseHandle = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpTypedBroadPhaseHandle,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_boundingVolumeData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boundingVolumeData",
                                        ),
                                    );
                                }
                                m_boundingVolumeData = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpCollidableBoundingVolumeData,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allowedPenetrationDepth,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowedPenetrationDepth",
                                        ),
                                    );
                                }
                                m_allowedPenetrationDepth = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_ownerOffset = match m_ownerOffset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ownerOffset",
                                ),
                            );
                        }
                    };
                    let m_forceCollideOntoPpu = match m_forceCollideOntoPpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forceCollideOntoPpu",
                                ),
                            );
                        }
                    };
                    let m_shapeSizeOnSpu = match m_shapeSizeOnSpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "shapeSizeOnSpu",
                                ),
                            );
                        }
                    };
                    let m_broadPhaseHandle = match m_broadPhaseHandle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseHandle",
                                ),
                            );
                        }
                    };
                    let m_boundingVolumeData = match m_boundingVolumeData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boundingVolumeData",
                                ),
                            );
                        }
                    };
                    let m_allowedPenetrationDepth = match m_allowedPenetrationDepth {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowedPenetrationDepth",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpCollidable {
                        __ptr,
                        parent,
                        m_ownerOffset,
                        m_forceCollideOntoPpu,
                        m_shapeSizeOnSpu,
                        m_broadPhaseHandle,
                        m_boundingVolumeData,
                        m_allowedPenetrationDepth,
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
                    let mut m_shape: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_shapeKey: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_forceCollideOntoPpu: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_broadPhaseHandle: _serde::__private::Option<
                        hkpTypedBroadPhaseHandle,
                    > = _serde::__private::None;
                    let mut m_allowedPenetrationDepth: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_shape => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shape) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("shape"),
                                    );
                                }
                                m_shape = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_shapeKey => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_shapeKey) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "shapeKey",
                                        ),
                                    );
                                }
                                m_shapeKey = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_forceCollideOntoPpu => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_forceCollideOntoPpu,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "forceCollideOntoPpu",
                                        ),
                                    );
                                }
                                m_forceCollideOntoPpu = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_broadPhaseHandle => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_broadPhaseHandle) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "broadPhaseHandle",
                                        ),
                                    );
                                }
                                m_broadPhaseHandle = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkpTypedBroadPhaseHandle,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_allowedPenetrationDepth => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_allowedPenetrationDepth,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allowedPenetrationDepth",
                                        ),
                                    );
                                }
                                m_allowedPenetrationDepth = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                    let m_shape = match m_shape {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shape"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_shapeKey = match m_shapeKey {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("shapeKey"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_forceCollideOntoPpu = match m_forceCollideOntoPpu {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "forceCollideOntoPpu",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_broadPhaseHandle = match m_broadPhaseHandle {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "broadPhaseHandle",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_allowedPenetrationDepth = match m_allowedPenetrationDepth {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allowedPenetrationDepth",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkpCdBody {
                        __ptr,
                        m_shape,
                        m_shapeKey,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpCollidable {
                        __ptr,
                        parent,
                        m_forceCollideOntoPpu,
                        m_broadPhaseHandle,
                        m_allowedPenetrationDepth,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "ownerOffset",
                "forceCollideOntoPpu",
                "shapeSizeOnSpu",
                "broadPhaseHandle",
                "boundingVolumeData",
                "allowedPenetrationDepth",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpCollidable",
                FIELDS,
                __hkpCollidableVisitor {
                    marker: _serde::__private::PhantomData::<hkpCollidable>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
