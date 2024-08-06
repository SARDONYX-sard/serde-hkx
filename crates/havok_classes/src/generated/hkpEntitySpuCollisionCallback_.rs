use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpEntitySpuCollisionCallback`
/// - version: `0`
/// - signature: `0x81147f05`
/// - size: `  8`(x86)/` 16`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpEntitySpuCollisionCallback {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// - name: `util`(ctype: `void*`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_util: Pointer,
    /// # C++ Info
    /// - name: `capacity`(ctype: `hkUint16`)
    /// - offset: `  4`(x86)/`  8`(x86_64)
    /// - type_size: `  2`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    pub m_capacity: u16,
    /// # C++ Info
    /// - name: `eventFilter`(ctype: `hkUint8`)
    /// - offset: `  6`(x86)/` 10`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_eventFilter: u8,
    /// # C++ Info
    /// - name: `userFilter`(ctype: `hkUint8`)
    /// - offset: `  7`(x86)/` 11`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_userFilter: u8,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpEntitySpuCollisionCallback {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpEntitySpuCollisionCallback"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x81147f05)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_util.get());
            v
        }
    }
    impl _serde::Serialize for hkpEntitySpuCollisionCallback {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x81147f05)));
            let mut serializer = __serializer
                .serialize_struct("hkpEntitySpuCollisionCallback", class_meta)?;
            serializer.skip_field("util", &self.m_util)?;
            serializer.skip_field("capacity", &self.m_capacity)?;
            serializer.serialize_field("eventFilter", &self.m_eventFilter)?;
            serializer.serialize_field("userFilter", &self.m_userFilter)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpEntitySpuCollisionCallback {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_eventFilter,
                m_userFilter,
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
                        "eventFilter" => Ok(__Field::m_eventFilter),
                        "userFilter" => Ok(__Field::m_userFilter),
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
            struct __hkpEntitySpuCollisionCallbackVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpEntitySpuCollisionCallback>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpEntitySpuCollisionCallbackVisitor<'de> {
                type Value = hkpEntitySpuCollisionCallback;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpEntitySpuCollisionCallback",
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
                    let mut m_util: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_capacity: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut m_eventFilter: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_userFilter: _serde::__private::Option<u8> = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_util) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("util"),
                                    );
                                }
                                m_util = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_capacity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "capacity",
                                        ),
                                    );
                                }
                                m_capacity = _serde::__private::Some(
                                    match __A::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_eventFilter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventFilter",
                                        ),
                                    );
                                }
                                m_eventFilter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_userFilter) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userFilter",
                                        ),
                                    );
                                }
                                m_userFilter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_util = match m_util {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("util"),
                            );
                        }
                    };
                    let m_capacity = match m_capacity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("capacity"),
                            );
                        }
                    };
                    let m_eventFilter = match m_eventFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventFilter",
                                ),
                            );
                        }
                    };
                    let m_userFilter = match m_userFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userFilter",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpEntitySpuCollisionCallback {
                        __ptr,
                        m_util,
                        m_capacity,
                        m_eventFilter,
                        m_userFilter,
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
                    let mut m_eventFilter: _serde::__private::Option<u8> = _serde::__private::None;
                    let mut m_userFilter: _serde::__private::Option<u8> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_eventFilter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventFilter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventFilter",
                                        ),
                                    );
                                }
                                m_eventFilter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userFilter => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userFilter) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userFilter",
                                        ),
                                    );
                                }
                                m_userFilter = _serde::__private::Some(
                                    match __A::next_value::<u8>(&mut __map) {
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
                    let m_eventFilter = match m_eventFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventFilter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userFilter = match m_userFilter {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "userFilter",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpEntitySpuCollisionCallback {
                        __ptr,
                        m_eventFilter,
                        m_userFilter,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &["util", "capacity", "eventFilter", "userFilter"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpEntitySpuCollisionCallback",
                FIELDS,
                __hkpEntitySpuCollisionCallbackVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpEntitySpuCollisionCallback,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
