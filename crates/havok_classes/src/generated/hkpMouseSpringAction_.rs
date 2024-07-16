use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpMouseSpringAction`
/// - version: `0`
/// - signature: `0x6e087fd6`
/// - size: ` 96`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMouseSpringAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpUnaryAction<'a>,
    /// # C++ Info
    /// - name: `positionInRbLocal`(ctype: `hkVector4`)
    /// - offset: ` 32`(x86)/` 64`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_positionInRbLocal: Vector4,
    /// # C++ Info
    /// - name: `mousePositionInWorld`(ctype: `hkVector4`)
    /// - offset: ` 48`(x86)/` 80`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_mousePositionInWorld: Vector4,
    /// # C++ Info
    /// - name: `springDamping`(ctype: `hkReal`)
    /// - offset: ` 64`(x86)/` 96`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_springDamping: f32,
    /// # C++ Info
    /// - name: `springElasticity`(ctype: `hkReal`)
    /// - offset: ` 68`(x86)/`100`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_springElasticity: f32,
    /// # C++ Info
    /// - name: `maxRelativeForce`(ctype: `hkReal`)
    /// - offset: ` 72`(x86)/`104`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_maxRelativeForce: f32,
    /// # C++ Info
    /// - name: `objectDamping`(ctype: `hkReal`)
    /// - offset: ` 76`(x86)/`108`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_objectDamping: f32,
    /// # C++ Info
    /// - name: `shapeKey`(ctype: `hkUint32`)
    /// - offset: ` 80`(x86)/`112`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_shapeKey: u32,
    /// # C++ Info
    /// - name: `applyCallbacks`(ctype: `hkArray<void*>`)
    /// - offset: ` 84`(x86)/`120`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_applyCallbacks: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpMouseSpringAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMouseSpringAction"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6e087fd6)
        }
    }
    impl<'a> _serde::Serialize for hkpMouseSpringAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6e087fd6)));
            let mut serializer = __serializer
                .serialize_struct("hkpMouseSpringAction", class_meta)?;
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
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entity", &self.parent.m_entity)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("positionInRbLocal", &self.m_positionInRbLocal)?;
            serializer
                .serialize_field("mousePositionInWorld", &self.m_mousePositionInWorld)?;
            serializer.serialize_field("springDamping", &self.m_springDamping)?;
            serializer.serialize_field("springElasticity", &self.m_springElasticity)?;
            serializer.serialize_field("maxRelativeForce", &self.m_maxRelativeForce)?;
            serializer.serialize_field("objectDamping", &self.m_objectDamping)?;
            serializer.serialize_field("shapeKey", &self.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("applyCallbacks", &self.m_applyCallbacks)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("applyCallbacks", &self.m_applyCallbacks)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_positionInRbLocal,
    m_mousePositionInWorld,
    m_springDamping,
    m_springElasticity,
    m_maxRelativeForce,
    m_objectDamping,
    m_shapeKey,
    m_applyCallbacks,
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
            "positionInRbLocal" => Ok(__Field::m_positionInRbLocal),
            "mousePositionInWorld" => Ok(__Field::m_mousePositionInWorld),
            "springDamping" => Ok(__Field::m_springDamping),
            "springElasticity" => Ok(__Field::m_springElasticity),
            "maxRelativeForce" => Ok(__Field::m_maxRelativeForce),
            "objectDamping" => Ok(__Field::m_objectDamping),
            "shapeKey" => Ok(__Field::m_shapeKey),
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
pub(super) struct __hkpMouseSpringActionVisitor<'de> {
    marker: core::marker::PhantomData<hkpMouseSpringAction<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpMouseSpringActionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpMouseSpringAction<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpMouseSpringAction<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpMouseSpringActionVisitor<'de> {
    type Value = hkpMouseSpringAction<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpMouseSpringAction")
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
        let mut m_positionInRbLocal: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_mousePositionInWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_springDamping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_springElasticity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxRelativeForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_objectDamping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_shapeKey: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_applyCallbacks: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_positionInRbLocal) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "positionInRbLocal",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 4usize, 8usize)?;
                    m_positionInRbLocal = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_mousePositionInWorld) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "mousePositionInWorld",
                            ),
                        );
                    }
                    m_mousePositionInWorld = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_springDamping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "springDamping",
                            ),
                        );
                    }
                    m_springDamping = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_springElasticity) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "springElasticity",
                            ),
                        );
                    }
                    m_springElasticity = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_maxRelativeForce) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxRelativeForce",
                            ),
                        );
                    }
                    m_maxRelativeForce = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_objectDamping) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "objectDamping",
                            ),
                        );
                    }
                    m_objectDamping = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_shapeKey) {
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
                7usize => {
                    if _serde::__private::Option::is_some(&m_applyCallbacks) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "applyCallbacks",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_applyCallbacks = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_positionInRbLocal = match m_positionInRbLocal {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("positionInRbLocal"),
                );
            }
        };
        let m_mousePositionInWorld = match m_mousePositionInWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "mousePositionInWorld",
                    ),
                );
            }
        };
        let m_springDamping = match m_springDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("springDamping"),
                );
            }
        };
        let m_springElasticity = match m_springElasticity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("springElasticity"),
                );
            }
        };
        let m_maxRelativeForce = match m_maxRelativeForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxRelativeForce"),
                );
            }
        };
        let m_objectDamping = match m_objectDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("objectDamping"),
                );
            }
        };
        let m_shapeKey = match m_shapeKey {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapeKey"),
                );
            }
        };
        let m_applyCallbacks = match m_applyCallbacks {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("applyCallbacks"),
                );
            }
        };
        _serde::__private::Ok(hkpMouseSpringAction {
            __ptr,
            parent,
            m_positionInRbLocal,
            m_mousePositionInWorld,
            m_springDamping,
            m_springElasticity,
            m_maxRelativeForce,
            m_objectDamping,
            m_shapeKey,
            m_applyCallbacks,
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
        let parent = __hkpUnaryActionVisitor::visit_as_parent(&mut __map)?;
        let mut m_positionInRbLocal: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_mousePositionInWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_springDamping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_springElasticity: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_maxRelativeForce: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_objectDamping: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_shapeKey: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_positionInRbLocal => {
                        if _serde::__private::Option::is_some(&m_positionInRbLocal) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "positionInRbLocal",
                                ),
                            );
                        }
                        m_positionInRbLocal = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_mousePositionInWorld => {
                        if _serde::__private::Option::is_some(&m_mousePositionInWorld) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "mousePositionInWorld",
                                ),
                            );
                        }
                        m_mousePositionInWorld = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_springDamping => {
                        if _serde::__private::Option::is_some(&m_springDamping) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "springDamping",
                                ),
                            );
                        }
                        m_springDamping = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_springElasticity => {
                        if _serde::__private::Option::is_some(&m_springElasticity) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "springElasticity",
                                ),
                            );
                        }
                        m_springElasticity = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_maxRelativeForce => {
                        if _serde::__private::Option::is_some(&m_maxRelativeForce) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "maxRelativeForce",
                                ),
                            );
                        }
                        m_maxRelativeForce = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_objectDamping => {
                        if _serde::__private::Option::is_some(&m_objectDamping) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "objectDamping",
                                ),
                            );
                        }
                        m_objectDamping = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_shapeKey => {
                        if _serde::__private::Option::is_some(&m_shapeKey) {
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
                    _ => {}
                }
            }
        }
        let m_positionInRbLocal = match m_positionInRbLocal {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("positionInRbLocal"),
                );
            }
        };
        let m_mousePositionInWorld = match m_mousePositionInWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "mousePositionInWorld",
                    ),
                );
            }
        };
        let m_springDamping = match m_springDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("springDamping"),
                );
            }
        };
        let m_springElasticity = match m_springElasticity {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("springElasticity"),
                );
            }
        };
        let m_maxRelativeForce = match m_maxRelativeForce {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxRelativeForce"),
                );
            }
        };
        let m_objectDamping = match m_objectDamping {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("objectDamping"),
                );
            }
        };
        let m_shapeKey = match m_shapeKey {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("shapeKey"),
                );
            }
        };
        _serde::__private::Ok(hkpMouseSpringAction {
            __ptr,
            parent,
            m_positionInRbLocal,
            m_mousePositionInWorld,
            m_springDamping,
            m_springElasticity,
            m_maxRelativeForce,
            m_objectDamping,
            m_shapeKey,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMouseSpringAction<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "positionInRbLocal",
                "mousePositionInWorld",
                "springDamping",
                "springElasticity",
                "maxRelativeForce",
                "objectDamping",
                "shapeKey",
                "applyCallbacks",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMouseSpringAction",
                FIELDS,
                __hkpMouseSpringActionVisitor {
                    marker: _serde::__private::PhantomData::<hkpMouseSpringAction>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
