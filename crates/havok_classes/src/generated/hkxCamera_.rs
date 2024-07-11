use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxCamera`
/// -         version: `1`
/// -       signature: `0xe3597b02`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxCamera {
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
    /// -          name: `from`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_from: Vector4,
    /// # C++ Info
    /// -          name: `focus`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_focus: Vector4,
    /// # C++ Info
    /// -          name: `up`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_up: Vector4,
    /// # C++ Info
    /// -          name: `fov`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_fov: f32,
    /// # C++ Info
    /// -          name: `far`(ctype: `hkReal`)
    /// -        offset:  68(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_far: f32,
    /// # C++ Info
    /// -          name: `near`(ctype: `hkReal`)
    /// -        offset:  72(x86)/ 72(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_near: f32,
    /// # C++ Info
    /// -          name: `leftHanded`(ctype: `hkBool`)
    /// -        offset:  76(x86)/ 76(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_leftHanded: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxCamera {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxCamera"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe3597b02)
        }
    }
    impl _serde::Serialize for hkxCamera {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe3597b02)));
            let mut serializer = __serializer.serialize_struct("hkxCamera", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("from", &self.m_from)?;
            serializer.serialize_field("focus", &self.m_focus)?;
            serializer.serialize_field("up", &self.m_up)?;
            serializer.serialize_field("fov", &self.m_fov)?;
            serializer.serialize_field("far", &self.m_far)?;
            serializer.serialize_field("near", &self.m_near)?;
            serializer.serialize_field("leftHanded", &self.m_leftHanded)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_from,
    m_focus,
    m_up,
    m_fov,
    m_far,
    m_near,
    m_leftHanded,
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
            "from" => Ok(__Field::m_from),
            "focus" => Ok(__Field::m_focus),
            "up" => Ok(__Field::m_up),
            "fov" => Ok(__Field::m_fov),
            "far" => Ok(__Field::m_far),
            "near" => Ok(__Field::m_near),
            "leftHanded" => Ok(__Field::m_leftHanded),
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
pub(super) struct __hkxCameraVisitor<'de> {
    marker: core::marker::PhantomData<hkxCamera>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxCameraVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxCamera, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxCamera>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxCameraVisitor<'de> {
    type Value = hkxCamera;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxCamera")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_from: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_focus: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_fov: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_far: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_near: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_leftHanded: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..7usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_from) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("from"),
                        );
                    }
                    m_from = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_focus) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("focus"),
                        );
                    }
                    m_focus = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_up) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("up"),
                        );
                    }
                    m_up = _serde::__private::Some(
                        match __A::next_value::<Vector4>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_fov) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("fov"),
                        );
                    }
                    m_fov = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_far) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("far"),
                        );
                    }
                    m_far = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_near) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("near"),
                        );
                    }
                    m_near = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_leftHanded) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "leftHanded",
                            ),
                        );
                    }
                    m_leftHanded = _serde::__private::Some(
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
        __A::pad(&mut __map, 3usize, 3usize)?;
        let m_from = match m_from {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("from"),
                );
            }
        };
        let m_focus = match m_focus {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("focus"),
                );
            }
        };
        let m_up = match m_up {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("up"),
                );
            }
        };
        let m_fov = match m_fov {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fov"),
                );
            }
        };
        let m_far = match m_far {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("far"),
                );
            }
        };
        let m_near = match m_near {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("near"),
                );
            }
        };
        let m_leftHanded = match m_leftHanded {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("leftHanded"),
                );
            }
        };
        _serde::__private::Ok(hkxCamera {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_from,
            m_focus,
            m_up,
            m_fov,
            m_far,
            m_near,
            m_leftHanded,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_from: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_focus: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_up: _serde::__private::Option<Vector4> = _serde::__private::None;
        let mut m_fov: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_far: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_near: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_leftHanded: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..7usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_from => {
                        if _serde::__private::Option::is_some(&m_from) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("from"),
                            );
                        }
                        m_from = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_focus => {
                        if _serde::__private::Option::is_some(&m_focus) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("focus"),
                            );
                        }
                        m_focus = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_up => {
                        if _serde::__private::Option::is_some(&m_up) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("up"),
                            );
                        }
                        m_up = _serde::__private::Some(
                            match __A::next_value::<Vector4>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fov => {
                        if _serde::__private::Option::is_some(&m_fov) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("fov"),
                            );
                        }
                        m_fov = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_far => {
                        if _serde::__private::Option::is_some(&m_far) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("far"),
                            );
                        }
                        m_far = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_near => {
                        if _serde::__private::Option::is_some(&m_near) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("near"),
                            );
                        }
                        m_near = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_leftHanded => {
                        if _serde::__private::Option::is_some(&m_leftHanded) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "leftHanded",
                                ),
                            );
                        }
                        m_leftHanded = _serde::__private::Some(
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
        let m_from = match m_from {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("from"),
                );
            }
        };
        let m_focus = match m_focus {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("focus"),
                );
            }
        };
        let m_up = match m_up {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("up"),
                );
            }
        };
        let m_fov = match m_fov {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fov"),
                );
            }
        };
        let m_far = match m_far {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("far"),
                );
            }
        };
        let m_near = match m_near {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("near"),
                );
            }
        };
        let m_leftHanded = match m_leftHanded {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("leftHanded"),
                );
            }
        };
        _serde::__private::Ok(hkxCamera {
            __ptr,
            parent,
            m_from,
            m_focus,
            m_up,
            m_fov,
            m_far,
            m_near,
            m_leftHanded,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxCamera {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "from",
                "focus",
                "up",
                "fov",
                "far",
                "near",
                "leftHanded",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxCamera",
                FIELDS,
                __hkxCameraVisitor {
                    marker: _serde::__private::PhantomData::<hkxCamera>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
