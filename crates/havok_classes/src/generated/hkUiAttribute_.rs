use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkUiAttribute`
/// - version: `2`
/// - signature: `0xeb6e96e3`
/// - size: ` 20`(x86)/` 40`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkUiAttribute<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `visible`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_visible: bool,
    /// # C++ Info
    /// - name: `hideInModeler`(ctype: `enum HideInModeler`)
    /// - offset: `  1`(x86)/`  1`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_hideInModeler: HideInModeler,
    /// # C++ Info
    /// - name: `label`(ctype: `char*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_label: CString<'a>,
    /// # C++ Info
    /// - name: `group`(ctype: `char*`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_group: CString<'a>,
    /// # C++ Info
    /// - name: `hideBaseClassMembers`(ctype: `char*`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_hideBaseClassMembers: CString<'a>,
    /// # C++ Info
    /// - name: `endGroup`(ctype: `hkBool`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_endGroup: bool,
    /// # C++ Info
    /// - name: `endGroup2`(ctype: `hkBool`)
    /// - offset: ` 17`(x86)/` 33`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_endGroup2: bool,
    /// # C++ Info
    /// - name: `advanced`(ctype: `hkBool`)
    /// - offset: ` 18`(x86)/` 34`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_advanced: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkUiAttribute<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkUiAttribute"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xeb6e96e3)
        }
    }
    impl<'a> _serde::Serialize for hkUiAttribute<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xeb6e96e3)));
            let mut serializer = __serializer
                .serialize_struct("hkUiAttribute", class_meta)?;
            serializer.serialize_field("visible", &self.m_visible)?;
            serializer.serialize_field("hideInModeler", &self.m_hideInModeler)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_cstring_meta_field("label", &self.m_label)?;
            serializer.serialize_cstring_meta_field("group", &self.m_group)?;
            serializer
                .serialize_cstring_meta_field(
                    "hideBaseClassMembers",
                    &self.m_hideBaseClassMembers,
                )?;
            serializer.serialize_field("endGroup", &self.m_endGroup)?;
            serializer.serialize_field("endGroup2", &self.m_endGroup2)?;
            serializer.serialize_field("advanced", &self.m_advanced)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_cstring_field("label", &self.m_label)?;
            serializer.serialize_cstring_field("group", &self.m_group)?;
            serializer
                .serialize_cstring_field(
                    "hideBaseClassMembers",
                    &self.m_hideBaseClassMembers,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_visible,
    m_hideInModeler,
    m_label,
    m_group,
    m_hideBaseClassMembers,
    m_endGroup,
    m_endGroup2,
    m_advanced,
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
            "visible" => Ok(__Field::m_visible),
            "hideInModeler" => Ok(__Field::m_hideInModeler),
            "label" => Ok(__Field::m_label),
            "group" => Ok(__Field::m_group),
            "hideBaseClassMembers" => Ok(__Field::m_hideBaseClassMembers),
            "endGroup" => Ok(__Field::m_endGroup),
            "endGroup2" => Ok(__Field::m_endGroup2),
            "advanced" => Ok(__Field::m_advanced),
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
pub(super) struct __hkUiAttributeVisitor<'de> {
    marker: core::marker::PhantomData<hkUiAttribute<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkUiAttributeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkUiAttribute<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkUiAttribute<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkUiAttributeVisitor<'de> {
    type Value = hkUiAttribute<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkUiAttribute")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_visible: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_hideInModeler: _serde::__private::Option<HideInModeler> = _serde::__private::None;
        let mut m_label: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_group: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_hideBaseClassMembers: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_endGroup: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_endGroup2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_advanced: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..8usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_visible) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("visible"),
                        );
                    }
                    m_visible = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_hideInModeler) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hideInModeler",
                            ),
                        );
                    }
                    m_hideInModeler = _serde::__private::Some(
                        match __A::next_value::<HideInModeler>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_label) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("label"),
                        );
                    }
                    __A::pad(&mut __map, 2usize, 6usize)?;
                    m_label = _serde::__private::Some(
                        match __A::next_value::<CString<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_group) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("group"),
                        );
                    }
                    m_group = _serde::__private::Some(
                        match __A::next_value::<CString<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_hideBaseClassMembers) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "hideBaseClassMembers",
                            ),
                        );
                    }
                    m_hideBaseClassMembers = _serde::__private::Some(
                        match __A::next_value::<CString<'de>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_endGroup) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endGroup",
                            ),
                        );
                    }
                    m_endGroup = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_endGroup2) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "endGroup2",
                            ),
                        );
                    }
                    m_endGroup2 = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_advanced) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "advanced",
                            ),
                        );
                    }
                    m_advanced = _serde::__private::Some(
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
        __A::pad(&mut __map, 1usize, 5usize)?;
        let m_visible = match m_visible {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("visible"),
                );
            }
        };
        let m_hideInModeler = match m_hideInModeler {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hideInModeler"),
                );
            }
        };
        let m_label = match m_label {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("label"),
                );
            }
        };
        let m_group = match m_group {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("group"),
                );
            }
        };
        let m_hideBaseClassMembers = match m_hideBaseClassMembers {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hideBaseClassMembers",
                    ),
                );
            }
        };
        let m_endGroup = match m_endGroup {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endGroup"),
                );
            }
        };
        let m_endGroup2 = match m_endGroup2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endGroup2"),
                );
            }
        };
        let m_advanced = match m_advanced {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("advanced"),
                );
            }
        };
        _serde::__private::Ok(hkUiAttribute {
            __ptr,
            m_visible,
            m_hideInModeler,
            m_label,
            m_group,
            m_hideBaseClassMembers,
            m_endGroup,
            m_endGroup2,
            m_advanced,
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
        let mut m_visible: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_hideInModeler: _serde::__private::Option<HideInModeler> = _serde::__private::None;
        let mut m_label: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_group: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_hideBaseClassMembers: _serde::__private::Option<CString<'de>> = _serde::__private::None;
        let mut m_endGroup: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_endGroup2: _serde::__private::Option<bool> = _serde::__private::None;
        let mut m_advanced: _serde::__private::Option<bool> = _serde::__private::None;
        for _ in 0..8usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_visible => {
                        if _serde::__private::Option::is_some(&m_visible) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "visible",
                                ),
                            );
                        }
                        m_visible = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_hideInModeler => {
                        if _serde::__private::Option::is_some(&m_hideInModeler) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hideInModeler",
                                ),
                            );
                        }
                        m_hideInModeler = _serde::__private::Some(
                            match __A::next_value::<HideInModeler>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_label => {
                        if _serde::__private::Option::is_some(&m_label) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("label"),
                            );
                        }
                        m_label = _serde::__private::Some(
                            match __A::next_value::<CString<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_group => {
                        if _serde::__private::Option::is_some(&m_group) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("group"),
                            );
                        }
                        m_group = _serde::__private::Some(
                            match __A::next_value::<CString<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_hideBaseClassMembers => {
                        if _serde::__private::Option::is_some(&m_hideBaseClassMembers) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "hideBaseClassMembers",
                                ),
                            );
                        }
                        m_hideBaseClassMembers = _serde::__private::Some(
                            match __A::next_value::<CString<'de>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_endGroup => {
                        if _serde::__private::Option::is_some(&m_endGroup) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endGroup",
                                ),
                            );
                        }
                        m_endGroup = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_endGroup2 => {
                        if _serde::__private::Option::is_some(&m_endGroup2) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "endGroup2",
                                ),
                            );
                        }
                        m_endGroup2 = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_advanced => {
                        if _serde::__private::Option::is_some(&m_advanced) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "advanced",
                                ),
                            );
                        }
                        m_advanced = _serde::__private::Some(
                            match __A::next_value::<bool>(&mut __map) {
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
        let m_visible = match m_visible {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("visible"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_hideInModeler = match m_hideInModeler {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hideInModeler"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_label = match m_label {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("label"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_group = match m_group {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("group"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_hideBaseClassMembers = match m_hideBaseClassMembers {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "hideBaseClassMembers",
                    ),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_endGroup = match m_endGroup {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endGroup"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_endGroup2 = match m_endGroup2 {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("endGroup2"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_advanced = match m_advanced {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("advanced"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkUiAttribute {
            __ptr,
            m_visible,
            m_hideInModeler,
            m_label,
            m_group,
            m_hideBaseClassMembers,
            m_endGroup,
            m_endGroup2,
            m_advanced,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkUiAttribute<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "visible",
                "hideInModeler",
                "label",
                "group",
                "hideBaseClassMembers",
                "endGroup",
                "endGroup2",
                "advanced",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkUiAttribute",
                FIELDS,
                __hkUiAttributeVisitor {
                    marker: _serde::__private::PhantomData::<hkUiAttribute>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_INT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum HideInModeler {
    #[default]
    NONE = 0isize,
    MAX = 1isize,
    MAYA = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for HideInModeler {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NONE => __serializer.serialize_field("NONE", &0u64),
                Self::MAX => __serializer.serialize_field("MAX", &1u64),
                Self::MAYA => __serializer.serialize_field("MAYA", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum HideInModeler to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for HideInModeler {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: i8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0i8 => _serde::__private::Ok(__Field::__field0),
                        1i8 => _serde::__private::Ok(__Field::__field1),
                        2i8 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("MAX") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("MAYA") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<HideInModeler>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HideInModeler;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum HideInModeler",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(HideInModeler::NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(HideInModeler::MAX)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(HideInModeler::MAYA)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["NONE", "MAX", "MAYA"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "HideInModeler",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HideInModeler>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
