use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbDetectCloseToGroundModifier`
/// - version: `2`
/// - signature: `0x981687b2`
/// - size: ` 72`(x86)/`120`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbDetectCloseToGroundModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `closeToGroundEvent`(ctype: `struct hkbEventProperty`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    pub m_closeToGroundEvent: hkbEventProperty,
    /// # C++ Info
    /// - name: `closeToGroundHeight`(ctype: `hkReal`)
    /// - offset: ` 52`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_closeToGroundHeight: f32,
    /// # C++ Info
    /// - name: `raycastDistanceDown`(ctype: `hkReal`)
    /// - offset: ` 56`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// - name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// - offset: ` 60`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// - name: `boneIndex`(ctype: `hkInt16`)
    /// - offset: ` 64`(x86)/`108`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_boneIndex: i16,
    /// # C++ Info
    /// - name: `animBoneIndex`(ctype: `hkInt16`)
    /// - offset: ` 66`(x86)/`110`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_animBoneIndex: i16,
    /// # C++ Info
    /// - name: `isCloseToGround`(ctype: `hkBool`)
    /// - offset: ` 68`(x86)/`112`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_isCloseToGround: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbDetectCloseToGroundModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbDetectCloseToGroundModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x981687b2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v.extend(self.m_closeToGroundEvent.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbDetectCloseToGroundModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x981687b2)));
            let mut serializer = __serializer
                .serialize_struct("hkbDetectCloseToGroundModifier", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("closeToGroundEvent", &self.m_closeToGroundEvent)?;
            serializer
                .serialize_field("closeToGroundHeight", &self.m_closeToGroundHeight)?;
            serializer
                .serialize_field("raycastDistanceDown", &self.m_raycastDistanceDown)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer.serialize_field("boneIndex", &self.m_boneIndex)?;
            serializer.serialize_field("animBoneIndex", &self.m_animBoneIndex)?;
            serializer.skip_field("isCloseToGround", &self.m_isCloseToGround)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbDetectCloseToGroundModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_enable,
                m_closeToGroundEvent,
                m_closeToGroundHeight,
                m_raycastDistanceDown,
                m_collisionFilterInfo,
                m_boneIndex,
                m_animBoneIndex,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "enable" => Ok(__Field::m_enable),
                        "closeToGroundEvent" => Ok(__Field::m_closeToGroundEvent),
                        "closeToGroundHeight" => Ok(__Field::m_closeToGroundHeight),
                        "raycastDistanceDown" => Ok(__Field::m_raycastDistanceDown),
                        "collisionFilterInfo" => Ok(__Field::m_collisionFilterInfo),
                        "boneIndex" => Ok(__Field::m_boneIndex),
                        "animBoneIndex" => Ok(__Field::m_animBoneIndex),
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
            struct __hkbDetectCloseToGroundModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbDetectCloseToGroundModifier<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbDetectCloseToGroundModifierVisitor<'de> {
                type Value = hkbDetectCloseToGroundModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbDetectCloseToGroundModifier",
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
                    let mut m_closeToGroundEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_closeToGroundHeight: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_boneIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_animBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_isCloseToGround: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..7usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_closeToGroundEvent,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "closeToGroundEvent",
                                        ),
                                    );
                                }
                                m_closeToGroundEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_closeToGroundHeight,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "closeToGroundHeight",
                                        ),
                                    );
                                }
                                m_closeToGroundHeight = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceDown,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceDown",
                                        ),
                                    );
                                }
                                m_raycastDistanceDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_boneIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndex",
                                        ),
                                    );
                                }
                                m_boneIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_animBoneIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animBoneIndex",
                                        ),
                                    );
                                }
                                m_animBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_isCloseToGround) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isCloseToGround",
                                        ),
                                    );
                                }
                                m_isCloseToGround = _serde::__private::Some(
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
                    __A::pad(&mut __map, 3usize, 7usize)?;
                    let m_closeToGroundEvent = match m_closeToGroundEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "closeToGroundEvent",
                                ),
                            );
                        }
                    };
                    let m_closeToGroundHeight = match m_closeToGroundHeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "closeToGroundHeight",
                                ),
                            );
                        }
                    };
                    let m_raycastDistanceDown = match m_raycastDistanceDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceDown",
                                ),
                            );
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                        }
                    };
                    let m_boneIndex = match m_boneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndex",
                                ),
                            );
                        }
                    };
                    let m_animBoneIndex = match m_animBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animBoneIndex",
                                ),
                            );
                        }
                    };
                    let m_isCloseToGround = match m_isCloseToGround {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isCloseToGround",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbDetectCloseToGroundModifier {
                        __ptr,
                        parent,
                        m_closeToGroundEvent,
                        m_closeToGroundHeight,
                        m_raycastDistanceDown,
                        m_collisionFilterInfo,
                        m_boneIndex,
                        m_animBoneIndex,
                        m_isCloseToGround,
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
                    let mut m_variableBindingSet: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_closeToGroundEvent: _serde::__private::Option<
                        hkbEventProperty,
                    > = _serde::__private::None;
                    let mut m_closeToGroundHeight: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_raycastDistanceDown: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_collisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut m_boneIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_animBoneIndex: _serde::__private::Option<i16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
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
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                                    );
                                }
                                m_enable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_closeToGroundEvent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_closeToGroundEvent,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "closeToGroundEvent",
                                        ),
                                    );
                                }
                                m_closeToGroundEvent = _serde::__private::Some(
                                    match __A::next_value::<hkbEventProperty>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_closeToGroundHeight => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_closeToGroundHeight,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "closeToGroundHeight",
                                        ),
                                    );
                                }
                                m_closeToGroundHeight = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_raycastDistanceDown => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_raycastDistanceDown,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "raycastDistanceDown",
                                        ),
                                    );
                                }
                                m_raycastDistanceDown = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_collisionFilterInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_collisionFilterInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "collisionFilterInfo",
                                        ),
                                    );
                                }
                                m_collisionFilterInfo = _serde::__private::Some(
                                    match __A::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndex",
                                        ),
                                    );
                                }
                                m_boneIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_animBoneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_animBoneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animBoneIndex",
                                        ),
                                    );
                                }
                                m_animBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enable = match m_enable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enable"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_closeToGroundEvent = match m_closeToGroundEvent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "closeToGroundEvent",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_closeToGroundHeight = match m_closeToGroundHeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "closeToGroundHeight",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_raycastDistanceDown = match m_raycastDistanceDown {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "raycastDistanceDown",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_collisionFilterInfo = match m_collisionFilterInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "collisionFilterInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneIndex = match m_boneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_animBoneIndex = match m_animBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animBoneIndex",
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
                    let parent = hkbBindable {
                        __ptr,
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr,
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbModifier {
                        __ptr,
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbDetectCloseToGroundModifier {
                        __ptr,
                        parent,
                        m_closeToGroundEvent,
                        m_closeToGroundHeight,
                        m_raycastDistanceDown,
                        m_collisionFilterInfo,
                        m_boneIndex,
                        m_animBoneIndex,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "closeToGroundEvent",
                "closeToGroundHeight",
                "raycastDistanceDown",
                "collisionFilterInfo",
                "boneIndex",
                "animBoneIndex",
                "isCloseToGround",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbDetectCloseToGroundModifier",
                FIELDS,
                __hkbDetectCloseToGroundModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbDetectCloseToGroundModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
