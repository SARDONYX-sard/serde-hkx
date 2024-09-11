use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkClassMember`
/// - version: `0`
/// - signature: `0x5c7ea4c2`
/// - size: ` 24`(x86)/` 40`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkClassMember<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `name`(ctype: `char*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_name: CString<'a>,
    /// # C++ Info
    /// - name: `class`(ctype: `struct hkClass*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_class: Pointer,
    /// # C++ Info
    /// - name: `enum`(ctype: `struct hkClassEnum*`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_enum: Pointer,
    /// # C++ Info
    /// - name: `type`(ctype: `enum Type`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_type: Type,
    /// # C++ Info
    /// - name: `subtype`(ctype: `enum Type`)
    /// - offset: ` 13`(x86)/` 25`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_subtype: Type,
    /// # C++ Info
    /// - name: `cArraySize`(ctype: `hkInt16`)
    /// - offset: ` 14`(x86)/` 26`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_cArraySize: i16,
    /// # C++ Info
    /// - name: `flags`(ctype: `flags FlagValues`)
    /// - offset: ` 16`(x86)/` 28`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_flags: FlagValues,
    /// # C++ Info
    /// - name: `offset`(ctype: `hkUint16`)
    /// - offset: ` 18`(x86)/` 30`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    pub m_offset: u16,
    /// # C++ Info
    /// - name: `attributes`(ctype: `struct hkCustomAttributes*`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_attributes: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkClassMember<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkClassMember"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5c7ea4c2)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_class.get());
            v.push(self.m_enum.get());
            v.push(self.m_attributes.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkClassMember<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5c7ea4c2)));
            let mut serializer = __serializer
                .serialize_struct("hkClassMember", class_meta, (24u64, 40u64))?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("class", &self.m_class)?;
            serializer.serialize_field("enum", &self.m_enum)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("subtype", &self.m_subtype)?;
            serializer.serialize_field("cArraySize", &self.m_cArraySize)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("offset", &self.m_offset)?;
            serializer.skip_field("attributes", &self.m_attributes)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkClassMember<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_class,
                m_enum,
                m_type,
                m_subtype,
                m_cArraySize,
                m_flags,
                m_offset,
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
                        "name" => Ok(__Field::m_name),
                        "class" => Ok(__Field::m_class),
                        "enum" => Ok(__Field::m_enum),
                        "type" => Ok(__Field::m_type),
                        "subtype" => Ok(__Field::m_subtype),
                        "cArraySize" => Ok(__Field::m_cArraySize),
                        "flags" => Ok(__Field::m_flags),
                        "offset" => Ok(__Field::m_offset),
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
            struct __hkClassMemberVisitor<'de> {
                marker: _serde::__private::PhantomData<hkClassMember<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkClassMemberVisitor<'de> {
                type Value = hkClassMember<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkClassMember")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_name: _serde::__private::Option<CString<'de>> = _serde::__private::None;
                    let mut m_class: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_enum: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_type: _serde::__private::Option<Type> = _serde::__private::None;
                    let mut m_subtype: _serde::__private::Option<Type> = _serde::__private::None;
                    let mut m_cArraySize: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagValues> = _serde::__private::None;
                    let mut m_offset: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_attributes: _serde::__private::Option<Pointer> = _serde::__private::None;
                    for i in 0..9usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_name) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_class) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("class"),
                                    );
                                }
                                m_class = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_enum) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enum"),
                                    );
                                }
                                m_enum = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_type) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<Type>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_subtype) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "subtype",
                                        ),
                                    );
                                }
                                m_subtype = _serde::__private::Some(
                                    match __A::next_value::<Type>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_cArraySize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cArraySize",
                                        ),
                                    );
                                }
                                m_cArraySize = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<FlagValues>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_offset) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                                    );
                                }
                                m_offset = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_attributes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributes",
                                        ),
                                    );
                                }
                                m_attributes = _serde::__private::Some(
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_class = match m_class {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("class"),
                            );
                        }
                    };
                    let m_enum = match m_enum {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enum"),
                            );
                        }
                    };
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                        }
                    };
                    let m_subtype = match m_subtype {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("subtype"),
                            );
                        }
                    };
                    let m_cArraySize = match m_cArraySize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cArraySize",
                                ),
                            );
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    let m_offset = match m_offset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offset"),
                            );
                        }
                    };
                    let m_attributes = match m_attributes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributes",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkClassMember {
                        __ptr,
                        m_name,
                        m_class,
                        m_enum,
                        m_type,
                        m_subtype,
                        m_cArraySize,
                        m_flags,
                        m_offset,
                        m_attributes,
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
                    let mut m_name: _serde::__private::Option<CString<'de>> = _serde::__private::None;
                    let mut m_class: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_enum: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_type: _serde::__private::Option<Type> = _serde::__private::None;
                    let mut m_subtype: _serde::__private::Option<Type> = _serde::__private::None;
                    let mut m_cArraySize: _serde::__private::Option<i16> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagValues> = _serde::__private::None;
                    let mut m_offset: _serde::__private::Option<u16> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
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
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_class => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_class) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("class"),
                                    );
                                }
                                m_class = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_enum => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enum) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enum"),
                                    );
                                }
                                m_enum = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<Type>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_subtype => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_subtype) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "subtype",
                                        ),
                                    );
                                }
                                m_subtype = _serde::__private::Some(
                                    match __A::next_value::<Type>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cArraySize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_cArraySize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cArraySize",
                                        ),
                                    );
                                }
                                m_cArraySize = _serde::__private::Some(
                                    match __A::next_value::<i16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_flags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_flags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<FlagValues>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_offset => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_offset) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("offset"),
                                    );
                                }
                                m_offset = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
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
                    let m_class = match m_class {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("class"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_enum = match m_enum {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enum"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_subtype = match m_subtype {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("subtype"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cArraySize = match m_cArraySize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cArraySize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_offset = match m_offset {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("offset"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkClassMember {
                        __ptr,
                        m_name,
                        m_class,
                        m_enum,
                        m_type,
                        m_subtype,
                        m_cArraySize,
                        m_flags,
                        m_offset,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "class",
                "enum",
                "type",
                "subtype",
                "cArraySize",
                "flags",
                "offset",
                "attributes",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkClassMember",
                FIELDS,
                __hkClassMemberVisitor {
                    marker: _serde::__private::PhantomData::<hkClassMember>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
///- size(C++): `TYPE_UINT8`
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
pub enum Type {
    #[default]
    TYPE_VOID = 0isize,
    TYPE_BOOL = 1isize,
    TYPE_CHAR = 2isize,
    TYPE_INT8 = 3isize,
    TYPE_UINT8 = 4isize,
    TYPE_INT16 = 5isize,
    TYPE_UINT16 = 6isize,
    TYPE_INT32 = 7isize,
    TYPE_UINT32 = 8isize,
    TYPE_INT64 = 9isize,
    TYPE_UINT64 = 10isize,
    TYPE_REAL = 11isize,
    TYPE_VECTOR4 = 12isize,
    TYPE_QUATERNION = 13isize,
    TYPE_MATRIX3 = 14isize,
    TYPE_ROTATION = 15isize,
    TYPE_QSTRANSFORM = 16isize,
    TYPE_MATRIX4 = 17isize,
    TYPE_TRANSFORM = 18isize,
    TYPE_ZERO = 19isize,
    TYPE_POINTER = 20isize,
    TYPE_FUNCTIONPOINTER = 21isize,
    TYPE_ARRAY = 22isize,
    TYPE_INPLACEARRAY = 23isize,
    TYPE_ENUM = 24isize,
    TYPE_STRUCT = 25isize,
    TYPE_SIMPLEARRAY = 26isize,
    TYPE_HOMOGENEOUSARRAY = 27isize,
    TYPE_VARIANT = 28isize,
    TYPE_CSTRING = 29isize,
    TYPE_ULONG = 30isize,
    TYPE_FLAGS = 31isize,
    TYPE_HALF = 32isize,
    TYPE_STRINGPTR = 33isize,
    TYPE_RELARRAY = 34isize,
    TYPE_MAX = 35isize,
}
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagValues : u32 { #[doc = "0"] const FLAGS_NONE = 0u32; #[doc = "128"] const ALIGN_8
    = 128u32; #[doc = "256"] const ALIGN_16 = 256u32; #[doc = "512"] const NOT_OWNED =
    512u32; #[doc = "1024"] const SERIALIZE_IGNORED = 1024u32; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Type {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TYPE_VOID => __serializer.serialize_field("TYPE_VOID", &0u64),
                Self::TYPE_BOOL => __serializer.serialize_field("TYPE_BOOL", &1u64),
                Self::TYPE_CHAR => __serializer.serialize_field("TYPE_CHAR", &2u64),
                Self::TYPE_INT8 => __serializer.serialize_field("TYPE_INT8", &3u64),
                Self::TYPE_UINT8 => __serializer.serialize_field("TYPE_UINT8", &4u64),
                Self::TYPE_INT16 => __serializer.serialize_field("TYPE_INT16", &5u64),
                Self::TYPE_UINT16 => __serializer.serialize_field("TYPE_UINT16", &6u64),
                Self::TYPE_INT32 => __serializer.serialize_field("TYPE_INT32", &7u64),
                Self::TYPE_UINT32 => __serializer.serialize_field("TYPE_UINT32", &8u64),
                Self::TYPE_INT64 => __serializer.serialize_field("TYPE_INT64", &9u64),
                Self::TYPE_UINT64 => __serializer.serialize_field("TYPE_UINT64", &10u64),
                Self::TYPE_REAL => __serializer.serialize_field("TYPE_REAL", &11u64),
                Self::TYPE_VECTOR4 => {
                    __serializer.serialize_field("TYPE_VECTOR4", &12u64)
                }
                Self::TYPE_QUATERNION => {
                    __serializer.serialize_field("TYPE_QUATERNION", &13u64)
                }
                Self::TYPE_MATRIX3 => {
                    __serializer.serialize_field("TYPE_MATRIX3", &14u64)
                }
                Self::TYPE_ROTATION => {
                    __serializer.serialize_field("TYPE_ROTATION", &15u64)
                }
                Self::TYPE_QSTRANSFORM => {
                    __serializer.serialize_field("TYPE_QSTRANSFORM", &16u64)
                }
                Self::TYPE_MATRIX4 => {
                    __serializer.serialize_field("TYPE_MATRIX4", &17u64)
                }
                Self::TYPE_TRANSFORM => {
                    __serializer.serialize_field("TYPE_TRANSFORM", &18u64)
                }
                Self::TYPE_ZERO => __serializer.serialize_field("TYPE_ZERO", &19u64),
                Self::TYPE_POINTER => {
                    __serializer.serialize_field("TYPE_POINTER", &20u64)
                }
                Self::TYPE_FUNCTIONPOINTER => {
                    __serializer.serialize_field("TYPE_FUNCTIONPOINTER", &21u64)
                }
                Self::TYPE_ARRAY => __serializer.serialize_field("TYPE_ARRAY", &22u64),
                Self::TYPE_INPLACEARRAY => {
                    __serializer.serialize_field("TYPE_INPLACEARRAY", &23u64)
                }
                Self::TYPE_ENUM => __serializer.serialize_field("TYPE_ENUM", &24u64),
                Self::TYPE_STRUCT => __serializer.serialize_field("TYPE_STRUCT", &25u64),
                Self::TYPE_SIMPLEARRAY => {
                    __serializer.serialize_field("TYPE_SIMPLEARRAY", &26u64)
                }
                Self::TYPE_HOMOGENEOUSARRAY => {
                    __serializer.serialize_field("TYPE_HOMOGENEOUSARRAY", &27u64)
                }
                Self::TYPE_VARIANT => {
                    __serializer.serialize_field("TYPE_VARIANT", &28u64)
                }
                Self::TYPE_CSTRING => {
                    __serializer.serialize_field("TYPE_CSTRING", &29u64)
                }
                Self::TYPE_ULONG => __serializer.serialize_field("TYPE_ULONG", &30u64),
                Self::TYPE_FLAGS => __serializer.serialize_field("TYPE_FLAGS", &31u64),
                Self::TYPE_HALF => __serializer.serialize_field("TYPE_HALF", &32u64),
                Self::TYPE_STRINGPTR => {
                    __serializer.serialize_field("TYPE_STRINGPTR", &33u64)
                }
                Self::TYPE_RELARRAY => {
                    __serializer.serialize_field("TYPE_RELARRAY", &34u64)
                }
                Self::TYPE_MAX => __serializer.serialize_field("TYPE_MAX", &35u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_u8().ok_or(S::Error::custom("Failed enum Type to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagValues {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAGS_NONE => {
                        __serializer.serialize_field("FLAGS_NONE", &Self::FLAGS_NONE)
                    }
                    Self::ALIGN_8 => {
                        __serializer.serialize_field("ALIGN_8", &Self::ALIGN_8)
                    }
                    Self::ALIGN_16 => {
                        __serializer.serialize_field("ALIGN_16", &Self::ALIGN_16)
                    }
                    Self::NOT_OWNED => {
                        __serializer.serialize_field("NOT_OWNED", &Self::NOT_OWNED)
                    }
                    Self::SERIALIZE_IGNORED => {
                        __serializer
                            .serialize_field(
                                "SERIALIZE_IGNORED",
                                &Self::SERIALIZE_IGNORED,
                            )
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
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
    impl<'de> _serde::Deserialize<'de> for Type {
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
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
                __field15,
                __field16,
                __field17,
                __field18,
                __field19,
                __field20,
                __field21,
                __field22,
                __field23,
                __field24,
                __field25,
                __field26,
                __field27,
                __field28,
                __field29,
                __field30,
                __field31,
                __field32,
                __field33,
                __field34,
                __field35,
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
                fn visit_uint8<__E>(
                    self,
                    __value: u8,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u8 => _serde::__private::Ok(__Field::__field0),
                        1u8 => _serde::__private::Ok(__Field::__field1),
                        2u8 => _serde::__private::Ok(__Field::__field2),
                        3u8 => _serde::__private::Ok(__Field::__field3),
                        4u8 => _serde::__private::Ok(__Field::__field4),
                        5u8 => _serde::__private::Ok(__Field::__field5),
                        6u8 => _serde::__private::Ok(__Field::__field6),
                        7u8 => _serde::__private::Ok(__Field::__field7),
                        8u8 => _serde::__private::Ok(__Field::__field8),
                        9u8 => _serde::__private::Ok(__Field::__field9),
                        10u8 => _serde::__private::Ok(__Field::__field10),
                        11u8 => _serde::__private::Ok(__Field::__field11),
                        12u8 => _serde::__private::Ok(__Field::__field12),
                        13u8 => _serde::__private::Ok(__Field::__field13),
                        14u8 => _serde::__private::Ok(__Field::__field14),
                        15u8 => _serde::__private::Ok(__Field::__field15),
                        16u8 => _serde::__private::Ok(__Field::__field16),
                        17u8 => _serde::__private::Ok(__Field::__field17),
                        18u8 => _serde::__private::Ok(__Field::__field18),
                        19u8 => _serde::__private::Ok(__Field::__field19),
                        20u8 => _serde::__private::Ok(__Field::__field20),
                        21u8 => _serde::__private::Ok(__Field::__field21),
                        22u8 => _serde::__private::Ok(__Field::__field22),
                        23u8 => _serde::__private::Ok(__Field::__field23),
                        24u8 => _serde::__private::Ok(__Field::__field24),
                        25u8 => _serde::__private::Ok(__Field::__field25),
                        26u8 => _serde::__private::Ok(__Field::__field26),
                        27u8 => _serde::__private::Ok(__Field::__field27),
                        28u8 => _serde::__private::Ok(__Field::__field28),
                        29u8 => _serde::__private::Ok(__Field::__field29),
                        30u8 => _serde::__private::Ok(__Field::__field30),
                        31u8 => _serde::__private::Ok(__Field::__field31),
                        32u8 => _serde::__private::Ok(__Field::__field32),
                        33u8 => _serde::__private::Ok(__Field::__field33),
                        34u8 => _serde::__private::Ok(__Field::__field34),
                        35u8 => _serde::__private::Ok(__Field::__field35),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35",
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
                            v if v == "0" || v.eq_ignore_ascii_case("TYPE_VOID") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("TYPE_BOOL") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("TYPE_CHAR") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3" || v.eq_ignore_ascii_case("TYPE_INT8") => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            v if v == "4" || v.eq_ignore_ascii_case("TYPE_UINT8") => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            v if v == "5" || v.eq_ignore_ascii_case("TYPE_INT16") => {
                                _serde::__private::Ok(__Field::__field5)
                            }
                            v if v == "6" || v.eq_ignore_ascii_case("TYPE_UINT16") => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            v if v == "7" || v.eq_ignore_ascii_case("TYPE_INT32") => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            v if v == "8" || v.eq_ignore_ascii_case("TYPE_UINT32") => {
                                _serde::__private::Ok(__Field::__field8)
                            }
                            v if v == "9" || v.eq_ignore_ascii_case("TYPE_INT64") => {
                                _serde::__private::Ok(__Field::__field9)
                            }
                            v if v == "10" || v.eq_ignore_ascii_case("TYPE_UINT64") => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            v if v == "11" || v.eq_ignore_ascii_case("TYPE_REAL") => {
                                _serde::__private::Ok(__Field::__field11)
                            }
                            v if v == "12" || v.eq_ignore_ascii_case("TYPE_VECTOR4") => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            v if v == "13"
                                || v.eq_ignore_ascii_case("TYPE_QUATERNION") => {
                                _serde::__private::Ok(__Field::__field13)
                            }
                            v if v == "14" || v.eq_ignore_ascii_case("TYPE_MATRIX3") => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            v if v == "15" || v.eq_ignore_ascii_case("TYPE_ROTATION") => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            v if v == "16"
                                || v.eq_ignore_ascii_case("TYPE_QSTRANSFORM") => {
                                _serde::__private::Ok(__Field::__field16)
                            }
                            v if v == "17" || v.eq_ignore_ascii_case("TYPE_MATRIX4") => {
                                _serde::__private::Ok(__Field::__field17)
                            }
                            v if v == "18"
                                || v.eq_ignore_ascii_case("TYPE_TRANSFORM") => {
                                _serde::__private::Ok(__Field::__field18)
                            }
                            v if v == "19" || v.eq_ignore_ascii_case("TYPE_ZERO") => {
                                _serde::__private::Ok(__Field::__field19)
                            }
                            v if v == "20" || v.eq_ignore_ascii_case("TYPE_POINTER") => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            v if v == "21"
                                || v.eq_ignore_ascii_case("TYPE_FUNCTIONPOINTER") => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            v if v == "22" || v.eq_ignore_ascii_case("TYPE_ARRAY") => {
                                _serde::__private::Ok(__Field::__field22)
                            }
                            v if v == "23"
                                || v.eq_ignore_ascii_case("TYPE_INPLACEARRAY") => {
                                _serde::__private::Ok(__Field::__field23)
                            }
                            v if v == "24" || v.eq_ignore_ascii_case("TYPE_ENUM") => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            v if v == "25" || v.eq_ignore_ascii_case("TYPE_STRUCT") => {
                                _serde::__private::Ok(__Field::__field25)
                            }
                            v if v == "26"
                                || v.eq_ignore_ascii_case("TYPE_SIMPLEARRAY") => {
                                _serde::__private::Ok(__Field::__field26)
                            }
                            v if v == "27"
                                || v.eq_ignore_ascii_case("TYPE_HOMOGENEOUSARRAY") => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            v if v == "28" || v.eq_ignore_ascii_case("TYPE_VARIANT") => {
                                _serde::__private::Ok(__Field::__field28)
                            }
                            v if v == "29" || v.eq_ignore_ascii_case("TYPE_CSTRING") => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            v if v == "30" || v.eq_ignore_ascii_case("TYPE_ULONG") => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            v if v == "31" || v.eq_ignore_ascii_case("TYPE_FLAGS") => {
                                _serde::__private::Ok(__Field::__field31)
                            }
                            v if v == "32" || v.eq_ignore_ascii_case("TYPE_HALF") => {
                                _serde::__private::Ok(__Field::__field32)
                            }
                            v if v == "33"
                                || v.eq_ignore_ascii_case("TYPE_STRINGPTR") => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            v if v == "34" || v.eq_ignore_ascii_case("TYPE_RELARRAY") => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            v if v == "35" || v.eq_ignore_ascii_case("TYPE_MAX") => {
                                _serde::__private::Ok(__Field::__field35)
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Type>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Type;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Type")
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
                            _serde::__private::Ok(Type::TYPE_VOID)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_BOOL)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_CHAR)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_INT8)
                        }
                        (__Field::__field4, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_UINT8)
                        }
                        (__Field::__field5, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_INT16)
                        }
                        (__Field::__field6, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_UINT16)
                        }
                        (__Field::__field7, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_INT32)
                        }
                        (__Field::__field8, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_UINT32)
                        }
                        (__Field::__field9, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_INT64)
                        }
                        (__Field::__field10, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_UINT64)
                        }
                        (__Field::__field11, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_REAL)
                        }
                        (__Field::__field12, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_VECTOR4)
                        }
                        (__Field::__field13, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_QUATERNION)
                        }
                        (__Field::__field14, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_MATRIX3)
                        }
                        (__Field::__field15, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_ROTATION)
                        }
                        (__Field::__field16, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_QSTRANSFORM)
                        }
                        (__Field::__field17, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_MATRIX4)
                        }
                        (__Field::__field18, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_TRANSFORM)
                        }
                        (__Field::__field19, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_ZERO)
                        }
                        (__Field::__field20, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_POINTER)
                        }
                        (__Field::__field21, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_FUNCTIONPOINTER)
                        }
                        (__Field::__field22, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_ARRAY)
                        }
                        (__Field::__field23, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_INPLACEARRAY)
                        }
                        (__Field::__field24, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_ENUM)
                        }
                        (__Field::__field25, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_STRUCT)
                        }
                        (__Field::__field26, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_SIMPLEARRAY)
                        }
                        (__Field::__field27, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_HOMOGENEOUSARRAY)
                        }
                        (__Field::__field28, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_VARIANT)
                        }
                        (__Field::__field29, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_CSTRING)
                        }
                        (__Field::__field30, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_ULONG)
                        }
                        (__Field::__field31, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_FLAGS)
                        }
                        (__Field::__field32, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_HALF)
                        }
                        (__Field::__field33, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_STRINGPTR)
                        }
                        (__Field::__field34, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_RELARRAY)
                        }
                        (__Field::__field35, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(Type::TYPE_MAX)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TYPE_VOID",
                "TYPE_BOOL",
                "TYPE_CHAR",
                "TYPE_INT8",
                "TYPE_UINT8",
                "TYPE_INT16",
                "TYPE_UINT16",
                "TYPE_INT32",
                "TYPE_UINT32",
                "TYPE_INT64",
                "TYPE_UINT64",
                "TYPE_REAL",
                "TYPE_VECTOR4",
                "TYPE_QUATERNION",
                "TYPE_MATRIX3",
                "TYPE_ROTATION",
                "TYPE_QSTRANSFORM",
                "TYPE_MATRIX4",
                "TYPE_TRANSFORM",
                "TYPE_ZERO",
                "TYPE_POINTER",
                "TYPE_FUNCTIONPOINTER",
                "TYPE_ARRAY",
                "TYPE_INPLACEARRAY",
                "TYPE_ENUM",
                "TYPE_STRUCT",
                "TYPE_SIMPLEARRAY",
                "TYPE_HOMOGENEOUSARRAY",
                "TYPE_VARIANT",
                "TYPE_CSTRING",
                "TYPE_ULONG",
                "TYPE_FLAGS",
                "TYPE_HALF",
                "TYPE_STRINGPTR",
                "TYPE_RELARRAY",
                "TYPE_MAX",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Type",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Type>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for FlagValues {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagValues>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagValues;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlagValues(flags)",
                    )
                }
                #[inline]
                fn visit_uint32<__E>(
                    self,
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(FlagValues::from_bits_retain(__value as _))
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <FlagValues as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagValues>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
