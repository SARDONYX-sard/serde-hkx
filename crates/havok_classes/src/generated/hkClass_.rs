use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkClass`
/// - version: `0`
/// - signature: `0x75585ef6`
/// - size: ` 48`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkClass<'a> {
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
    /// - name: `parent`(ctype: `struct hkClass*`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_parent: Pointer,
    /// # C++ Info
    /// - name: `objectSize`(ctype: `hkInt32`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_objectSize: i32,
    /// # C++ Info
    /// - name: `numImplementedInterfaces`(ctype: `hkInt32`)
    /// - offset: ` 12`(x86)/` 20`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_numImplementedInterfaces: i32,
    /// # C++ Info
    /// - name: `declaredEnums`(ctype: `hkSimpleArray<struct hkClassEnum>`)
    /// - offset: ` 16`(x86)/` 24`(x86_64)
    /// - type_size: `  8`(x86)/` 12`(x86_64)
    pub m_declaredEnums: Vec<hkClassEnum<'a>>,
    /// # C++ Info
    /// - name: `declaredMembers`(ctype: `hkSimpleArray<struct hkClassMember>`)
    /// - offset: ` 24`(x86)/` 40`(x86_64)
    /// - type_size: `  8`(x86)/` 12`(x86_64)
    pub m_declaredMembers: Vec<hkClassMember<'a>>,
    /// # C++ Info
    /// - name: `defaults`(ctype: `void*`)
    /// - offset: ` 32`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_defaults: Pointer,
    /// # C++ Info
    /// - name: `attributes`(ctype: `struct hkCustomAttributes*`)
    /// - offset: ` 36`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_attributes: Pointer,
    /// # C++ Info
    /// - name: `flags`(ctype: `flags FlagValues`)
    /// - offset: ` 40`(x86)/` 72`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_flags: FlagValues,
    /// # C++ Info
    /// - name: `describedVersion`(ctype: `hkInt32`)
    /// - offset: ` 44`(x86)/` 76`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_describedVersion: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkClass<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkClass"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x75585ef6)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_parent.get());
            v.push(self.m_defaults.get());
            v.push(self.m_attributes.get());
            v
        }
    }
    impl<'a> _serde::Serialize for hkClass<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x75585ef6)));
            let mut serializer = __serializer.serialize_struct("hkClass", class_meta)?;
            serializer.serialize_cstring_meta_field("name", &self.m_name)?;
            serializer.serialize_field("parent", &self.m_parent)?;
            serializer.serialize_field("objectSize", &self.m_objectSize)?;
            serializer
                .serialize_field(
                    "numImplementedInterfaces",
                    &self.m_numImplementedInterfaces,
                )?;
            serializer.serialize_field("declaredEnums", &self.m_declaredEnums)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("declaredMembers", &self.m_declaredMembers)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("defaults", &self.m_defaults)?;
            serializer.skip_field("attributes", &self.m_attributes)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("describedVersion", &self.m_describedVersion)?;
            serializer.serialize_cstring_field("name", &self.m_name)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkClass<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_parent,
                m_objectSize,
                m_numImplementedInterfaces,
                m_declaredEnums,
                m_declaredMembers,
                m_flags,
                m_describedVersion,
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
                        "parent" => Ok(__Field::m_parent),
                        "objectSize" => Ok(__Field::m_objectSize),
                        "numImplementedInterfaces" => {
                            Ok(__Field::m_numImplementedInterfaces)
                        }
                        "declaredEnums" => Ok(__Field::m_declaredEnums),
                        "declaredMembers" => Ok(__Field::m_declaredMembers),
                        "flags" => Ok(__Field::m_flags),
                        "describedVersion" => Ok(__Field::m_describedVersion),
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
            struct __hkClassVisitor<'de> {
                marker: _serde::__private::PhantomData<hkClass<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkClassVisitor<'de> {
                type Value = hkClass<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkClass")
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
                    let mut m_parent: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_objectSize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_numImplementedInterfaces: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_declaredEnums: _serde::__private::Option<
                        Vec<hkClassEnum<'de>>,
                    > = _serde::__private::None;
                    let mut m_declaredMembers: _serde::__private::Option<
                        Vec<hkClassMember<'de>>,
                    > = _serde::__private::None;
                    let mut m_defaults: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_attributes: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagValues> = _serde::__private::None;
                    let mut m_describedVersion: _serde::__private::Option<i32> = _serde::__private::None;
                    for i in 0..10usize {
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
                                if _serde::__private::Option::is_some(&m_parent) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("parent"),
                                    );
                                }
                                m_parent = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_objectSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "objectSize",
                                        ),
                                    );
                                }
                                m_objectSize = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numImplementedInterfaces,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numImplementedInterfaces",
                                        ),
                                    );
                                }
                                m_numImplementedInterfaces = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_declaredEnums) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "declaredEnums",
                                        ),
                                    );
                                }
                                m_declaredEnums = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkClassEnum<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_declaredMembers) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "declaredMembers",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_declaredMembers = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkClassMember<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_defaults) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "defaults",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_defaults = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
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
                            8usize => {
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
                            9usize => {
                                if _serde::__private::Option::is_some(&m_describedVersion) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "describedVersion",
                                        ),
                                    );
                                }
                                m_describedVersion = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
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
                    let m_parent = match m_parent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("parent"),
                            );
                        }
                    };
                    let m_objectSize = match m_objectSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "objectSize",
                                ),
                            );
                        }
                    };
                    let m_numImplementedInterfaces = match m_numImplementedInterfaces {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numImplementedInterfaces",
                                ),
                            );
                        }
                    };
                    let m_declaredEnums = match m_declaredEnums {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "declaredEnums",
                                ),
                            );
                        }
                    };
                    let m_declaredMembers = match m_declaredMembers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "declaredMembers",
                                ),
                            );
                        }
                    };
                    let m_defaults = match m_defaults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("defaults"),
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
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    let m_describedVersion = match m_describedVersion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "describedVersion",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkClass {
                        __ptr,
                        m_name,
                        m_parent,
                        m_objectSize,
                        m_numImplementedInterfaces,
                        m_declaredEnums,
                        m_declaredMembers,
                        m_defaults,
                        m_attributes,
                        m_flags,
                        m_describedVersion,
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
                    let mut m_parent: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_objectSize: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_numImplementedInterfaces: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut m_declaredEnums: _serde::__private::Option<
                        Vec<hkClassEnum<'de>>,
                    > = _serde::__private::None;
                    let mut m_declaredMembers: _serde::__private::Option<
                        Vec<hkClassMember<'de>>,
                    > = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagValues> = _serde::__private::None;
                    let mut m_describedVersion: _serde::__private::Option<i32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
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
                            __Field::m_parent => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_parent) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("parent"),
                                    );
                                }
                                m_parent = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_objectSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_objectSize) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "objectSize",
                                        ),
                                    );
                                }
                                m_objectSize = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numImplementedInterfaces => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numImplementedInterfaces,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numImplementedInterfaces",
                                        ),
                                    );
                                }
                                m_numImplementedInterfaces = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_declaredEnums => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_declaredEnums) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "declaredEnums",
                                        ),
                                    );
                                }
                                m_declaredEnums = _serde::__private::Some(
                                    match __A::next_value::<Vec<hkClassEnum<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_declaredMembers => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_declaredMembers) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "declaredMembers",
                                        ),
                                    );
                                }
                                m_declaredMembers = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkClassMember<'de>>,
                                    >(&mut __map) {
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
                                    #[cfg(feature = "ignore_duplicates")] continue;
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
                            __Field::m_describedVersion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_describedVersion) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "describedVersion",
                                        ),
                                    );
                                }
                                m_describedVersion = _serde::__private::Some(
                                    match __A::next_value::<i32>(&mut __map) {
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
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_parent = match m_parent {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("parent"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_objectSize = match m_objectSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "objectSize",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numImplementedInterfaces = match m_numImplementedInterfaces {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numImplementedInterfaces",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_declaredEnums = match m_declaredEnums {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "declaredEnums",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_declaredMembers = match m_declaredMembers {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "declaredMembers",
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
                    let m_describedVersion = match m_describedVersion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "describedVersion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkClass {
                        __ptr,
                        m_name,
                        m_parent,
                        m_objectSize,
                        m_numImplementedInterfaces,
                        m_declaredEnums,
                        m_declaredMembers,
                        m_flags,
                        m_describedVersion,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "parent",
                "objectSize",
                "numImplementedInterfaces",
                "declaredEnums",
                "declaredMembers",
                "defaults",
                "attributes",
                "flags",
                "describedVersion",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkClass",
                FIELDS,
                __hkClassVisitor {
                    marker: _serde::__private::PhantomData::<hkClass>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagValues : u32 { #[doc = "0"] const FLAGS_NONE = 0u32; #[doc = "1"] const
    FLAGS_NOT_SERIALIZABLE = 1u32; }
}
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
                    Self::FLAGS_NOT_SERIALIZABLE => {
                        __serializer
                            .serialize_field(
                                "FLAGS_NOT_SERIALIZABLE",
                                &Self::FLAGS_NOT_SERIALIZABLE,
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
