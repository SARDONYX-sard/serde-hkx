use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkReferencedObject {
    pub parent: HkBaseObject,

    /// Class index of itself.
    ///
    /// In XML, this takes the place of a pointer.
    /// This index is generated when deserializing a binary file.
    ///
    /// # Note
    /// The case of [`Option::None`] assumes that the class is embedded directly in a field within the class.
    pub __ptr_name_attr: Option<Pointer>,

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
        let class_meta = self.__ptr_name_attr.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkReferencedObject", class_meta)?;

        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.skip_field("memSizeAndFlags", &self.mem_size_and_flags)?; // offset: 4(+2), 8(+2)
        serializer.skip_field("referenceCount", &self.reference_count)?; // offset: 6(+2), 10(+2)
        serializer.pad_field(&[0u8; 0].as_slice(), &[0u8; 4].as_slice())?; // offset: 8(+0), 12(+4) Tailing align by ptr size bytes.
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

                /// Index for binary
                fn visit_uint64<E>(self, __value: u64) -> Result<Self::Value, E>
                where
                    E: havok_serde::de::Error,
                {
                    match __value {
                        0 => Ok(__Field::__field0),
                        1 => Ok(__Field::__field1),
                        _ => Ok(__Field::__ignore),
                    }
                }

                fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "memSizeAndFlags" => Ok(__Field::__field0),
                        "referenceCount" => Ok(__Field::__field1),
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
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u16> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i16> = _serde::__private::None;

                    __A::pad(&mut __map, 4, 8); // hkBaseObject to vtable of ptr size
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
                                        _serde::__private::Ok(__val) => {
                                            __A::pad(&mut __map, 4, 8);
                                            __val
                                        }
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
                        __ptr_name_attr: __map.class_ptr(),
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
