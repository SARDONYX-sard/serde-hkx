use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpGenericConstraintDataScheme`
/// - version: `0`
/// - signature: `0x11fd6f6c`
/// - size: ` 64`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpGenericConstraintDataScheme {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `info`(ctype: `struct hkpGenericConstraintDataSchemeConstraintInfo`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_info: hkpGenericConstraintDataSchemeConstraintInfo,
    /// # C++ Info
    /// - name: `data`(ctype: `hkArray<hkVector4>`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_data: Vec<Vector4>,
    /// # C++ Info
    /// - name: `commands`(ctype: `hkArray<hkInt32>`)
    /// - offset: ` 28`(x86)/` 32`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_commands: Vec<i32>,
    /// # C++ Info
    /// - name: `modifiers`(ctype: `hkArray<void*>`)
    /// - offset: ` 40`(x86)/` 48`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_modifiers: Vec<Pointer>,
    /// # C++ Info
    /// - name: `motors`(ctype: `hkArray<hkpConstraintMotor*>`)
    /// - offset: ` 52`(x86)/` 64`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_motors: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpGenericConstraintDataScheme {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpGenericConstraintDataScheme"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x11fd6f6c)
        }
    }
    impl _serde::Serialize for hkpGenericConstraintDataScheme {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x11fd6f6c)));
            let mut serializer = __serializer
                .serialize_struct("hkpGenericConstraintDataScheme", class_meta)?;
            serializer.skip_field("info", &self.m_info)?;
            serializer.serialize_array_meta_field("data", &self.m_data)?;
            serializer.serialize_array_meta_field("commands", &self.m_commands)?;
            serializer.skip_array_meta_field("modifiers", &self.m_modifiers)?;
            serializer.serialize_array_meta_field("motors", &self.m_motors)?;
            serializer.serialize_array_field("data", &self.m_data)?;
            serializer.serialize_array_field("commands", &self.m_commands)?;
            serializer.serialize_array_field("modifiers", &self.m_modifiers)?;
            serializer.serialize_array_field("motors", &self.m_motors)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_info,
    m_data,
    m_commands,
    m_modifiers,
    m_motors,
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
            "data" => Ok(__Field::m_data),
            "commands" => Ok(__Field::m_commands),
            "motors" => Ok(__Field::m_motors),
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
pub(super) struct __hkpGenericConstraintDataSchemeVisitor<'de> {
    marker: core::marker::PhantomData<hkpGenericConstraintDataScheme>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpGenericConstraintDataSchemeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpGenericConstraintDataScheme, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpGenericConstraintDataScheme>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpGenericConstraintDataSchemeVisitor<'de> {
    type Value = hkpGenericConstraintDataScheme;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpGenericConstraintDataScheme",
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
        let mut m_info: _serde::__private::Option<
            hkpGenericConstraintDataSchemeConstraintInfo,
        > = _serde::__private::None;
        let mut m_data: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_commands: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
        let mut m_modifiers: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_motors: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..5usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_info) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("info"),
                        );
                    }
                    m_info = _serde::__private::Some(
                        match __A::next_value::<
                            hkpGenericConstraintDataSchemeConstraintInfo,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_data) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                        );
                    }
                    m_data = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_commands) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "commands",
                            ),
                        );
                    }
                    m_commands = _serde::__private::Some(
                        match __A::next_value::<Vec<i32>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_modifiers) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "modifiers",
                            ),
                        );
                    }
                    m_modifiers = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_motors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("motors"),
                        );
                    }
                    m_motors = _serde::__private::Some(
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
        let m_info = match m_info {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("info"),
                );
            }
        };
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
            }
        };
        let m_commands = match m_commands {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("commands"),
                );
            }
        };
        let m_modifiers = match m_modifiers {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("modifiers"),
                );
            }
        };
        let m_motors = match m_motors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motors"),
                );
            }
        };
        _serde::__private::Ok(hkpGenericConstraintDataScheme {
            __ptr,
            m_info,
            m_data,
            m_commands,
            m_modifiers,
            m_motors,
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
        let mut m_data: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_commands: _serde::__private::Option<Vec<i32>> = _serde::__private::None;
        let mut m_motors: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for _ in 0..3usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_data => {
                        if _serde::__private::Option::is_some(&m_data) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("data"),
                            );
                        }
                        m_data = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_commands => {
                        if _serde::__private::Option::is_some(&m_commands) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "commands",
                                ),
                            );
                        }
                        m_commands = _serde::__private::Some(
                            match __A::next_value::<Vec<i32>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_motors => {
                        if _serde::__private::Option::is_some(&m_motors) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("motors"),
                            );
                        }
                        m_motors = _serde::__private::Some(
                            match __A::next_value::<Vec<Pointer>>(&mut __map) {
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
        let m_data = match m_data {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("data"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_commands = match m_commands {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("commands"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_motors = match m_motors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("motors"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpGenericConstraintDataScheme {
            __ptr,
            m_data,
            m_commands,
            m_motors,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpGenericConstraintDataScheme {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["info", "data", "commands", "modifiers", "motors"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpGenericConstraintDataScheme",
                FIELDS,
                __hkpGenericConstraintDataSchemeVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpGenericConstraintDataScheme,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
