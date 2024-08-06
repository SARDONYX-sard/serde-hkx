use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCombineTransformsModifier`
/// - version: `0`
/// - signature: `0xfd1f0b79`
/// - size: `160`(x86)/`192`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCombineTransformsModifier<'a> {
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
    /// - name: `translationOut`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_translationOut: Vector4,
    /// # C++ Info
    /// - name: `rotationOut`(ctype: `hkQuaternion`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_rotationOut: Quaternion,
    /// # C++ Info
    /// - name: `leftTranslation`(ctype: `hkVector4`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_leftTranslation: Vector4,
    /// # C++ Info
    /// - name: `leftRotation`(ctype: `hkQuaternion`)
    /// - offset: ` 96`(x86)/`128`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_leftRotation: Quaternion,
    /// # C++ Info
    /// - name: `rightTranslation`(ctype: `hkVector4`)
    /// - offset: `112`(x86)/`144`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_rightTranslation: Vector4,
    /// # C++ Info
    /// - name: `rightRotation`(ctype: `hkQuaternion`)
    /// - offset: `128`(x86)/`160`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_rightRotation: Quaternion,
    /// # C++ Info
    /// - name: `invertLeftTransform`(ctype: `hkBool`)
    /// - offset: `144`(x86)/`176`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_invertLeftTransform: bool,
    /// # C++ Info
    /// - name: `invertRightTransform`(ctype: `hkBool`)
    /// - offset: `145`(x86)/`177`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_invertRightTransform: bool,
    /// # C++ Info
    /// - name: `invertResult`(ctype: `hkBool`)
    /// - offset: `146`(x86)/`178`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_invertResult: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCombineTransformsModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCombineTransformsModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfd1f0b79)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.parent.parent.parent.m_variableBindingSet.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCombineTransformsModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xfd1f0b79)));
            let mut serializer = __serializer
                .serialize_struct("hkbCombineTransformsModifier", class_meta)?;
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
            serializer.serialize_field("translationOut", &self.m_translationOut)?;
            serializer.serialize_field("rotationOut", &self.m_rotationOut)?;
            serializer.serialize_field("leftTranslation", &self.m_leftTranslation)?;
            serializer.serialize_field("leftRotation", &self.m_leftRotation)?;
            serializer.serialize_field("rightTranslation", &self.m_rightTranslation)?;
            serializer.serialize_field("rightRotation", &self.m_rightRotation)?;
            serializer
                .serialize_field("invertLeftTransform", &self.m_invertLeftTransform)?;
            serializer
                .serialize_field("invertRightTransform", &self.m_invertRightTransform)?;
            serializer.serialize_field("invertResult", &self.m_invertResult)?;
            serializer.pad_field([0u8; 13usize].as_slice(), [0u8; 13usize].as_slice())?;
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
    impl<'de> _serde::Deserialize<'de> for hkbCombineTransformsModifier<'de> {
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
                m_translationOut,
                m_rotationOut,
                m_leftTranslation,
                m_leftRotation,
                m_rightTranslation,
                m_rightRotation,
                m_invertLeftTransform,
                m_invertRightTransform,
                m_invertResult,
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
                        "translationOut" => Ok(__Field::m_translationOut),
                        "rotationOut" => Ok(__Field::m_rotationOut),
                        "leftTranslation" => Ok(__Field::m_leftTranslation),
                        "leftRotation" => Ok(__Field::m_leftRotation),
                        "rightTranslation" => Ok(__Field::m_rightTranslation),
                        "rightRotation" => Ok(__Field::m_rightRotation),
                        "invertLeftTransform" => Ok(__Field::m_invertLeftTransform),
                        "invertRightTransform" => Ok(__Field::m_invertRightTransform),
                        "invertResult" => Ok(__Field::m_invertResult),
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
            struct __hkbCombineTransformsModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbCombineTransformsModifier<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbCombineTransformsModifierVisitor<'de> {
                type Value = hkbCombineTransformsModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCombineTransformsModifier",
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
                    let mut m_translationOut: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_leftTranslation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_leftRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_rightTranslation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_rightRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_invertLeftTransform: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_invertRightTransform: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_invertResult: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_translationOut) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "translationOut",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 4usize, 0usize)?;
                                m_translationOut = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_rotationOut) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationOut",
                                        ),
                                    );
                                }
                                m_rotationOut = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_leftTranslation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leftTranslation",
                                        ),
                                    );
                                }
                                m_leftTranslation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_leftRotation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leftRotation",
                                        ),
                                    );
                                }
                                m_leftRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_rightTranslation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rightTranslation",
                                        ),
                                    );
                                }
                                m_rightTranslation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_rightRotation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rightRotation",
                                        ),
                                    );
                                }
                                m_rightRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_invertLeftTransform,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invertLeftTransform",
                                        ),
                                    );
                                }
                                m_invertLeftTransform = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_invertRightTransform,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invertRightTransform",
                                        ),
                                    );
                                }
                                m_invertRightTransform = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_invertResult) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invertResult",
                                        ),
                                    );
                                }
                                m_invertResult = _serde::__private::Some(
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
                    __A::pad(&mut __map, 13usize, 13usize)?;
                    let m_translationOut = match m_translationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "translationOut",
                                ),
                            );
                        }
                    };
                    let m_rotationOut = match m_rotationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationOut",
                                ),
                            );
                        }
                    };
                    let m_leftTranslation = match m_leftTranslation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leftTranslation",
                                ),
                            );
                        }
                    };
                    let m_leftRotation = match m_leftRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leftRotation",
                                ),
                            );
                        }
                    };
                    let m_rightTranslation = match m_rightTranslation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rightTranslation",
                                ),
                            );
                        }
                    };
                    let m_rightRotation = match m_rightRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rightRotation",
                                ),
                            );
                        }
                    };
                    let m_invertLeftTransform = match m_invertLeftTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invertLeftTransform",
                                ),
                            );
                        }
                    };
                    let m_invertRightTransform = match m_invertRightTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invertRightTransform",
                                ),
                            );
                        }
                    };
                    let m_invertResult = match m_invertResult {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invertResult",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCombineTransformsModifier {
                        __ptr,
                        parent,
                        m_translationOut,
                        m_rotationOut,
                        m_leftTranslation,
                        m_leftRotation,
                        m_rightTranslation,
                        m_rightRotation,
                        m_invertLeftTransform,
                        m_invertRightTransform,
                        m_invertResult,
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
                    let mut m_translationOut: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_leftTranslation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_leftRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_rightTranslation: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_rightRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_invertLeftTransform: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_invertRightTransform: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_invertResult: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_translationOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_translationOut) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "translationOut",
                                        ),
                                    );
                                }
                                m_translationOut = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rotationOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rotationOut) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationOut",
                                        ),
                                    );
                                }
                                m_rotationOut = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_leftTranslation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_leftTranslation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leftTranslation",
                                        ),
                                    );
                                }
                                m_leftTranslation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_leftRotation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_leftRotation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "leftRotation",
                                        ),
                                    );
                                }
                                m_leftRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rightTranslation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rightTranslation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rightTranslation",
                                        ),
                                    );
                                }
                                m_rightTranslation = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rightRotation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rightRotation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rightRotation",
                                        ),
                                    );
                                }
                                m_rightRotation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invertLeftTransform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_invertLeftTransform,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invertLeftTransform",
                                        ),
                                    );
                                }
                                m_invertLeftTransform = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invertRightTransform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_invertRightTransform,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invertRightTransform",
                                        ),
                                    );
                                }
                                m_invertRightTransform = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invertResult => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invertResult) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invertResult",
                                        ),
                                    );
                                }
                                m_invertResult = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_translationOut = match m_translationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "translationOut",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rotationOut = match m_rotationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationOut",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_leftTranslation = match m_leftTranslation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leftTranslation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_leftRotation = match m_leftRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "leftRotation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rightTranslation = match m_rightTranslation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rightTranslation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rightRotation = match m_rightRotation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rightRotation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invertLeftTransform = match m_invertLeftTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invertLeftTransform",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invertRightTransform = match m_invertRightTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invertRightTransform",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invertResult = match m_invertResult {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invertResult",
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
                    _serde::__private::Ok(hkbCombineTransformsModifier {
                        __ptr,
                        parent,
                        m_translationOut,
                        m_rotationOut,
                        m_leftTranslation,
                        m_leftRotation,
                        m_rightTranslation,
                        m_rightRotation,
                        m_invertLeftTransform,
                        m_invertRightTransform,
                        m_invertResult,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "translationOut",
                "rotationOut",
                "leftTranslation",
                "leftRotation",
                "rightTranslation",
                "rightRotation",
                "invertLeftTransform",
                "invertRightTransform",
                "invertResult",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCombineTransformsModifier",
                FIELDS,
                __hkbCombineTransformsModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCombineTransformsModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
