use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkReferencedObject {
    pub parent: HkBaseObject,

    pub _name: Option<Pointer>,

    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,
}

impl HavokClass for HkReferencedObject {
    fn name(&self) -> &'static CStr {
        c"hkReferencedObject"
    }

    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}

impl Serialize for HkReferencedObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self._name.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.skip_field("memSizeAndFlags", &self.mem_size_and_flags)?;
        serializer.skip_field("referenceCount", &self.reference_count)?;
        serializer.pad_field(&[0u8; 0].as_slice(), &[0u8; 4].as_slice())?; // tailing align by ptr size bytes
        serializer.end()
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for HkReferencedObject {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;

                fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }

                fn visit_uint64<E>(self, __value: u64) -> Result<Self::Value, E>
                where
                    E: havok_serde::de::Error,
                {
                    match __value {
                        0u64 => Ok(__Field::__field0),
                        1u64 => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }

                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(s) = __value.into_inner() {
                        match s.as_ref() {
                            "memSizeAndFlags" => Ok(__Field::__field0),
                            "referenceCount" => Ok(__Field::__field1),
                            _ => Ok(__Field::__ignore),
                        }
                    } else {
                        Ok(__Field::__ignore)
                    }
                }
            }

            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_stringptr(__deserializer, __FieldVisitor)
                }
            }

            struct __Visitor<'de> {
                marker: core::marker::PhantomData<HkReferencedObject>,
                lifetime: core::marker::PhantomData<&'de ()>,
            }

            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HkReferencedObject;
                fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct HkReferencedObject")
                }

                #[inline]
                fn visit_array<__A>(
                    self,
                    mut __seq: __A,
                ) -> core::result::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_primitive_element::<u16>(
                        &mut __seq,
                    ) {
                        core::result::Result::Ok(__val) => __val,
                        core::result::Result::Err(__err) => {
                            return core::result::Result::Err(__err);
                        }
                    } {
                        core::option::Option::Some(__value) => __value,
                        core::option::Option::None => {
                            return core::result::Result::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct HkReferencedObject with 1 element",
                            ));
                        }
                    };

                    let __field1 = match match _serde::de::SeqAccess::next_primitive_element::<i16>(
                        &mut __seq,
                    ) {
                        core::result::Result::Ok(__val) => __val,
                        core::result::Result::Err(__err) => {
                            return core::result::Result::Err(__err);
                        }
                    } {
                        core::option::Option::Some(__value) => __value,
                        core::option::Option::None => {
                            return core::result::Result::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct HkReferencedObject with 2 element",
                            ));
                        }
                    };
                    core::result::Result::Ok(HkReferencedObject {
                        parent: HkBaseObject { _name: None },
                        _name: None,

                        mem_size_and_flags: __field0,
                        reference_count: __field1,
                    })
                }

                #[inline]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i16> = _serde::__private::None;

                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "memSizeAndFlags",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referenceCount",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<i16>(&mut __map) {
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

                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("memSizeAndFlags"),
                            )
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("referenceCount"),
                            )
                        }
                    };
                    _serde::__private::Ok(HkReferencedObject {
                        parent: HkBaseObject { _name: None },
                        _name: None,
                        mem_size_and_flags: __field0,
                        reference_count: __field1,
                    })
                }
            }

            const FIELDS: &'static [&'static str] = &["memSizeAndFlags", "referenceCount"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkReferencedObject",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HkReferencedObject>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
