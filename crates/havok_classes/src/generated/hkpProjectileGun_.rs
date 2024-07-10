use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpProjectileGun`
/// -         version: `0`
/// -       signature: `0xb4f30148`
/// -          size:  64(x86)/104(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpProjectileGun<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpFirstPersonGun<'a>,
    /// # C++ Info
    /// -          name: `maxProjectiles`(ctype: `hkInt32`)
    /// -        offset:  32(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxProjectiles: i32,
    /// # C++ Info
    /// -          name: `reloadTime`(ctype: `hkReal`)
    /// -        offset:  36(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_reloadTime: f32,
    /// # C++ Info
    /// -          name: `reload`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_reload: f32,
    /// # C++ Info
    /// -          name: `projectiles`(ctype: `hkArray<void*>`)
    /// -        offset:  44(x86)/ 72(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_projectiles: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `world`(ctype: `void*`)
    /// -        offset:  56(x86)/ 88(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_world: Pointer,
    /// # C++ Info
    /// -          name: `destructionWorld`(ctype: `void*`)
    /// -        offset:  60(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_destructionWorld: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpProjectileGun<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpProjectileGun"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb4f30148)
        }
    }
    impl<'a> _serde::Serialize for hkpProjectileGun<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xb4f30148)));
            let mut serializer = __serializer
                .serialize_struct("hkpProjectileGun", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_stringptr_meta_field("name", &self.parent.m_name)?;
            serializer.serialize_field("keyboardKey", &self.parent.m_keyboardKey)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.skip_array_meta_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_field("maxProjectiles", &self.m_maxProjectiles)?;
            serializer.serialize_field("reloadTime", &self.m_reloadTime)?;
            serializer.skip_field("reload", &self.m_reload)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("projectiles", &self.m_projectiles)?;
            serializer.skip_field("world", &self.m_world)?;
            serializer.skip_field("destructionWorld", &self.m_destructionWorld)?;
            serializer.serialize_stringptr_field("name", &self.parent.m_name)?;
            serializer.serialize_array_field("listeners", &self.parent.m_listeners)?;
            serializer.serialize_array_field("projectiles", &self.m_projectiles)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_maxProjectiles,
    m_reloadTime,
    m_reload,
    m_projectiles,
    m_world,
    m_destructionWorld,
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
            "maxProjectiles" => Ok(__Field::m_maxProjectiles),
            "reloadTime" => Ok(__Field::m_reloadTime),
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
pub(super) struct __hkpProjectileGunVisitor<'de> {
    marker: core::marker::PhantomData<hkpProjectileGun<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpProjectileGunVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpProjectileGun<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpProjectileGun<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpProjectileGunVisitor<'de> {
    type Value = hkpProjectileGun<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpProjectileGun")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_maxProjectiles: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_reloadTime: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_reload: _serde::__private::Option<f32> = _serde::__private::None;
        let mut m_projectiles: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_world: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_destructionWorld: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..6usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_maxProjectiles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxProjectiles",
                            ),
                        );
                    }
                    m_maxProjectiles = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_reloadTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reloadTime",
                            ),
                        );
                    }
                    m_reloadTime = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_reload) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("reload"),
                        );
                    }
                    m_reload = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_projectiles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "projectiles",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_projectiles = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_world) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("world"),
                        );
                    }
                    m_world = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_destructionWorld) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "destructionWorld",
                            ),
                        );
                    }
                    m_destructionWorld = _serde::__private::Some(
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
        let m_maxProjectiles = match m_maxProjectiles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxProjectiles"),
                );
            }
        };
        let m_reloadTime = match m_reloadTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reloadTime"),
                );
            }
        };
        let m_reload = match m_reload {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reload"),
                );
            }
        };
        let m_projectiles = match m_projectiles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("projectiles"),
                );
            }
        };
        let m_world = match m_world {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("world"),
                );
            }
        };
        let m_destructionWorld = match m_destructionWorld {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("destructionWorld"),
                );
            }
        };
        _serde::__private::Ok(hkpProjectileGun {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_maxProjectiles,
            m_reloadTime,
            m_reload,
            m_projectiles,
            m_world,
            m_destructionWorld,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpFirstPersonGunVisitor::visit_as_parent(&mut __map)?;
        let mut m_maxProjectiles: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_reloadTime: _serde::__private::Option<f32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_maxProjectiles => {
                    if _serde::__private::Option::is_some(&m_maxProjectiles) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "maxProjectiles",
                            ),
                        );
                    }
                    m_maxProjectiles = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_reloadTime => {
                    if _serde::__private::Option::is_some(&m_reloadTime) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "reloadTime",
                            ),
                        );
                    }
                    m_reloadTime = _serde::__private::Some(
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
        let m_maxProjectiles = match m_maxProjectiles {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("maxProjectiles"),
                );
            }
        };
        let m_reloadTime = match m_reloadTime {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("reloadTime"),
                );
            }
        };
        _serde::__private::Ok(hkpProjectileGun {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_maxProjectiles,
            m_reloadTime,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpProjectileGun<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "maxProjectiles",
                "reloadTime",
                "reload",
                "projectiles",
                "world",
                "destructionWorld",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpProjectileGun",
                FIELDS,
                __hkpProjectileGunVisitor {
                    marker: _serde::__private::PhantomData::<hkpProjectileGun>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
