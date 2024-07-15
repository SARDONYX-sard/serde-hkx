use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSTweenerModifier`
/// - version: `1`
/// - signature: `0xd2d9a04`
/// - size: `176`(x86)/`208`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSTweenerModifier<'a> {
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
    /// - name: `tweenPosition`(ctype: `hkBool`)
    /// - offset: ` 44`(x86)/` 80`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_tweenPosition: bool,
    /// # C++ Info
    /// - name: `tweenRotation`(ctype: `hkBool`)
    /// - offset: ` 45`(x86)/` 81`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_tweenRotation: bool,
    /// # C++ Info
    /// - name: `useTweenDuration`(ctype: `hkBool`)
    /// - offset: ` 46`(x86)/` 82`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_useTweenDuration: bool,
    /// # C++ Info
    /// - name: `tweenDuration`(ctype: `hkReal`)
    /// - offset: ` 48`(x86)/` 84`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_tweenDuration: f32,
    /// # C++ Info
    /// - name: `targetPosition`(ctype: `hkVector4`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_targetPosition: Vector4,
    /// # C++ Info
    /// - name: `targetRotation`(ctype: `hkQuaternion`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_targetRotation: Quaternion,
    /// # C++ Info
    /// - name: `duration`(ctype: `hkReal`)
    /// - offset: ` 96`(x86)/`128`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_duration: f32,
    /// # C++ Info
    /// - name: `startTransform`(ctype: `hkQsTransform`)
    /// - offset: `112`(x86)/`144`(x86_64)
    /// - type_size: ` 48`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_startTransform: QsTransform,
    /// # C++ Info
    /// - name: `time`(ctype: `hkReal`)
    /// - offset: `160`(x86)/`192`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_time: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSTweenerModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSTweenerModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd2d9a04)
        }
    }
    impl<'a> _serde::Serialize for BSTweenerModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xd2d9a04)));
            let mut serializer = __serializer
                .serialize_struct("BSTweenerModifier", class_meta)?;
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
            serializer.serialize_field("tweenPosition", &self.m_tweenPosition)?;
            serializer.serialize_field("tweenRotation", &self.m_tweenRotation)?;
            serializer.serialize_field("useTweenDuration", &self.m_useTweenDuration)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("tweenDuration", &self.m_tweenDuration)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("targetPosition", &self.m_targetPosition)?;
            serializer.serialize_field("targetRotation", &self.m_targetRotation)?;
            serializer.skip_field("duration", &self.m_duration)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.skip_field("startTransform", &self.m_startTransform)?;
            serializer.skip_field("time", &self.m_time)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
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
    m_tweenPosition,
    m_tweenRotation,
    m_useTweenDuration,
    m_tweenDuration,
    m_targetPosition,
    m_targetRotation,
    m_duration,
    m_startTransform,
    m_time,
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
            "tweenPosition" => Ok(__Field::m_tweenPosition),
            "tweenRotation" => Ok(__Field::m_tweenRotation),
            "useTweenDuration" => Ok(__Field::m_useTweenDuration),
            "tweenDuration" => Ok(__Field::m_tweenDuration),
            "targetPosition" => Ok(__Field::m_targetPosition),
            "targetRotation" => Ok(__Field::m_targetRotation),
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
pub(super) struct __BSTweenerModifierVisitor<'de> {
    marker: core::marker::PhantomData<BSTweenerModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __BSTweenerModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<BSTweenerModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<BSTweenerModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __BSTweenerModifierVisitor<'de> {
    type Value = BSTweenerModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct BSTweenerModifier")
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
        let mut m_tweenPosition: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_tweenRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_useTweenDuration: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_tweenDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_targetPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_targetRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_startTransform: _serde::__private::Option<QsTransform> = _serde::__private::None;
        let mut m_time: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..9usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_tweenPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "tweenPosition",
                            ),
                        );
                    }
                    m_tweenPosition = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_tweenRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "tweenRotation",
                            ),
                        );
                    }
                    m_tweenRotation = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_useTweenDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "useTweenDuration",
                            ),
                        );
                    }
                    m_useTweenDuration = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_tweenDuration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "tweenDuration",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 1usize, 1usize)?;
                    m_tweenDuration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_targetPosition) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "targetPosition",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 8usize)?;
                    m_targetPosition = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_targetRotation) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "targetRotation",
                            ),
                        );
                    }
                    m_targetRotation = _serde::__private::Some(
                        match __A::next_value::<Quaternion>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_duration) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "duration",
                            ),
                        );
                    }
                    m_duration = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_startTransform) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "startTransform",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 12usize)?;
                    m_startTransform = _serde::__private::Some(
                        match __A::next_value::<QsTransform>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_time) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("time"),
                        );
                    }
                    m_time = _serde::__private::Some(
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
        __A::pad(&mut __map, 12usize, 12usize)?;
        let m_tweenPosition = match m_tweenPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tweenPosition"),
                );
            }
        };
        let m_tweenRotation = match m_tweenRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tweenRotation"),
                );
            }
        };
        let m_useTweenDuration = match m_useTweenDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("useTweenDuration"),
                );
            }
        };
        let m_tweenDuration = match m_tweenDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tweenDuration"),
                );
            }
        };
        let m_targetPosition = match m_targetPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetPosition"),
                );
            }
        };
        let m_targetRotation = match m_targetRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetRotation"),
                );
            }
        };
        let m_duration = match m_duration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("duration"),
                );
            }
        };
        let m_startTransform = match m_startTransform {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("startTransform"),
                );
            }
        };
        let m_time = match m_time {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("time"),
                );
            }
        };
        _serde::__private::Ok(BSTweenerModifier {
            __ptr,
            parent,
            m_tweenPosition,
            m_tweenRotation,
            m_useTweenDuration,
            m_tweenDuration,
            m_targetPosition,
            m_targetRotation,
            m_duration,
            m_startTransform,
            m_time,
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
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_tweenPosition: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_tweenRotation: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_useTweenDuration: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_tweenDuration: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_targetPosition: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_targetRotation: _serde::__private::Option<Quaternion> = _serde::__private::None;
        for _ in 0..6usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_tweenPosition => {
                        if _serde::__private::Option::is_some(&m_tweenPosition) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "tweenPosition",
                                ),
                            );
                        }
                        m_tweenPosition = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_tweenRotation => {
                        if _serde::__private::Option::is_some(&m_tweenRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "tweenRotation",
                                ),
                            );
                        }
                        m_tweenRotation = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_useTweenDuration => {
                        if _serde::__private::Option::is_some(&m_useTweenDuration) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "useTweenDuration",
                                ),
                            );
                        }
                        m_useTweenDuration = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_tweenDuration => {
                        if _serde::__private::Option::is_some(&m_tweenDuration) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "tweenDuration",
                                ),
                            );
                        }
                        m_tweenDuration = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_targetPosition => {
                        if _serde::__private::Option::is_some(&m_targetPosition) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "targetPosition",
                                ),
                            );
                        }
                        m_targetPosition = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_targetRotation => {
                        if _serde::__private::Option::is_some(&m_targetRotation) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "targetRotation",
                                ),
                            );
                        }
                        m_targetRotation = _serde::__private::Some(
                            match __A::next_value::<Quaternion>(&mut __map) {
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
        let m_tweenPosition = match m_tweenPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tweenPosition"),
                );
            }
        };
        let m_tweenRotation = match m_tweenRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tweenRotation"),
                );
            }
        };
        let m_useTweenDuration = match m_useTweenDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("useTweenDuration"),
                );
            }
        };
        let m_tweenDuration = match m_tweenDuration {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("tweenDuration"),
                );
            }
        };
        let m_targetPosition = match m_targetPosition {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetPosition"),
                );
            }
        };
        let m_targetRotation = match m_targetRotation {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("targetRotation"),
                );
            }
        };
        _serde::__private::Ok(BSTweenerModifier {
            __ptr,
            parent,
            m_tweenPosition,
            m_tweenRotation,
            m_useTweenDuration,
            m_tweenDuration,
            m_targetPosition,
            m_targetRotation,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSTweenerModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "tweenPosition",
                "tweenRotation",
                "useTweenDuration",
                "tweenDuration",
                "targetPosition",
                "targetRotation",
                "duration",
                "startTransform",
                "time",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSTweenerModifier",
                FIELDS,
                __BSTweenerModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSTweenerModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
