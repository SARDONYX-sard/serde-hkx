use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbPoweredRagdollControlsModifier`
/// - version: `5`
/// - signature: `0x7cb54065`
/// - size: ` 96`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbPoweredRagdollControlsModifier<'a> {
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
    /// - name: `controlData`(ctype: `struct hkbPoweredRagdollControlData`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 32`(x86)/` 32`(x86_64)
    pub m_controlData: hkbPoweredRagdollControlData,
    /// # C++ Info
    /// - name: `bones`(ctype: `struct hkbBoneIndexArray*`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_bones: Pointer,
    /// # C++ Info
    /// - name: `worldFromModelModeData`(ctype: `struct hkbWorldFromModelModeData`)
    /// - offset: ` 84`(x86)/`120`(x86_64)
    /// - type_size: `  8`(x86)/`  8`(x86_64)
    pub m_worldFromModelModeData: hkbWorldFromModelModeData,
    /// # C++ Info
    /// - name: `boneWeights`(ctype: `struct hkbBoneWeightArray*`)
    /// - offset: ` 92`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_boneWeights: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbPoweredRagdollControlsModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbPoweredRagdollControlsModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7cb54065)
        }
    }
    impl<'a> _serde::Serialize for hkbPoweredRagdollControlsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x7cb54065)));
            let mut serializer = __serializer
                .serialize_struct("hkbPoweredRagdollControlsModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_field("bones", &self.m_bones)?;
            serializer
                .serialize_field(
                    "worldFromModelModeData",
                    &self.m_worldFromModelModeData,
                )?;
            serializer.serialize_field("boneWeights", &self.m_boneWeights)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
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
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_controlData,
    m_bones,
    m_worldFromModelModeData,
    m_boneWeights,
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
            "controlData" => Ok(__Field::m_controlData),
            "bones" => Ok(__Field::m_bones),
            "worldFromModelModeData" => Ok(__Field::m_worldFromModelModeData),
            "boneWeights" => Ok(__Field::m_boneWeights),
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
pub(super) struct __hkbPoweredRagdollControlsModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbPoweredRagdollControlsModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbPoweredRagdollControlsModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbPoweredRagdollControlsModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkbPoweredRagdollControlsModifier<'de>,
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
impl<'de> _serde::de::Visitor<'de> for __hkbPoweredRagdollControlsModifierVisitor<'de> {
    type Value = hkbPoweredRagdollControlsModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkbPoweredRagdollControlsModifier",
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
        let mut m_controlData: _serde::__private::Option<hkbPoweredRagdollControlData> = _serde::__private::None;
        let mut m_bones: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_worldFromModelModeData: _serde::__private::Option<
            hkbWorldFromModelModeData,
        > = _serde::__private::None;
        let mut m_boneWeights: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..4usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_controlData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "controlData",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 0usize)?;
                    m_controlData = _serde::__private::Some(
                        match __A::next_value::<
                            hkbPoweredRagdollControlData,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_bones) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                        );
                    }
                    m_bones = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_worldFromModelModeData) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "worldFromModelModeData",
                            ),
                        );
                    }
                    m_worldFromModelModeData = _serde::__private::Some(
                        match __A::next_value::<hkbWorldFromModelModeData>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_boneWeights) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "boneWeights",
                            ),
                        );
                    }
                    m_boneWeights = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 8usize)?;
        let m_controlData = match m_controlData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("controlData"),
                );
            }
        };
        let m_bones = match m_bones {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bones"),
                );
            }
        };
        let m_worldFromModelModeData = match m_worldFromModelModeData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelModeData",
                    ),
                );
            }
        };
        let m_boneWeights = match m_boneWeights {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneWeights"),
                );
            }
        };
        _serde::__private::Ok(hkbPoweredRagdollControlsModifier {
            __ptr,
            parent,
            m_controlData,
            m_bones,
            m_worldFromModelModeData,
            m_boneWeights,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_controlData: _serde::__private::Option<hkbPoweredRagdollControlData> = _serde::__private::None;
        let mut m_bones: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_worldFromModelModeData: _serde::__private::Option<
            hkbWorldFromModelModeData,
        > = _serde::__private::None;
        let mut m_boneWeights: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..4usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_controlData => {
                        if _serde::__private::Option::is_some(&m_controlData) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "controlData",
                                ),
                            );
                        }
                        m_controlData = _serde::__private::Some(
                            match __A::next_value::<
                                hkbPoweredRagdollControlData,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_bones => {
                        if _serde::__private::Option::is_some(&m_bones) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("bones"),
                            );
                        }
                        m_bones = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_worldFromModelModeData => {
                        if _serde::__private::Option::is_some(
                            &m_worldFromModelModeData,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "worldFromModelModeData",
                                ),
                            );
                        }
                        m_worldFromModelModeData = _serde::__private::Some(
                            match __A::next_value::<
                                hkbWorldFromModelModeData,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_boneWeights => {
                        if _serde::__private::Option::is_some(&m_boneWeights) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "boneWeights",
                                ),
                            );
                        }
                        m_boneWeights = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        let m_controlData = match m_controlData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("controlData"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_bones = match m_bones {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("bones"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_worldFromModelModeData = match m_worldFromModelModeData {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "worldFromModelModeData",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_boneWeights = match m_boneWeights {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("boneWeights"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkbPoweredRagdollControlsModifier {
            __ptr,
            parent,
            m_controlData,
            m_bones,
            m_worldFromModelModeData,
            m_boneWeights,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbPoweredRagdollControlsModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "controlData",
                "bones",
                "worldFromModelModeData",
                "boneWeights",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbPoweredRagdollControlsModifier",
                FIELDS,
                __hkbPoweredRagdollControlsModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbPoweredRagdollControlsModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
