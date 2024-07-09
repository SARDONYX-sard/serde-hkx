use super::*;

/// `hkbProjectStringData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 76
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x76ad60a`
/// -   version: 1
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbProjectStringData<'a> {
    pub __ptr: Option<Pointer>,

    pub parent: HkReferencedObject,
    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub animation_filenames: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub behavior_filenames: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"characterFilenames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub character_filenames: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"eventNames"`
    /// -   type: `hkArray<hkStringPtr>`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub event_names: Vec<StringPtr<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"animationPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub animation_path: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"behaviorPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub behavior_path: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"characterPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub character_path: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"fullPathToSource"`
    /// -   type: `hkStringPtr`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub full_path_to_source: StringPtr<'a>,
    /// # C++ Class Fields Info
    /// -   name:`"rootPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub root_path: StringPtr<'a>,
}

impl HavokClass for HkbProjectStringData<'_> {
    fn name(&self) -> &'static str {
        "hkbProjectStringData"
    }

    fn signature(&self) -> Signature {
        Signature::new(0x076ad60a)
    }
}

impl Serialize for HkbProjectStringData<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self.__ptr.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkbProjectStringData", class_meta)?;

        // flattened parent's fields
        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.skip_field("memSizeAndFlags", &self.parent.mem_size_and_flags)?;
        serializer.skip_field("referenceCount", &self.parent.reference_count)?;
        serializer.pad_field(&[0u8; 0].as_slice(), &[0u8; 4].as_slice())?;

        // self fields
        serializer.serialize_array_meta_field("animationFilenames", &self.animation_filenames)?;
        serializer.serialize_array_meta_field("behaviorFilenames", &self.behavior_filenames)?;
        serializer.serialize_array_meta_field("characterFilenames", &self.character_filenames)?;
        serializer.serialize_array_meta_field("eventNames", &self.event_names)?;
        serializer.serialize_stringptr_meta_field("animationPath", &self.animation_path)?;
        serializer.serialize_stringptr_meta_field("behaviorPath", &self.behavior_path)?;
        serializer.serialize_stringptr_meta_field("characterPath", &self.character_path)?;
        serializer.serialize_stringptr_meta_field("fullPathToSource", &self.full_path_to_source)?;
        serializer.skip_stringptr_meta_field("rootPath", &self.root_path)?;
        // Tailing alignment pads is none. already aligned.

        // For pointer type binary.
        serializer.serialize_array_field("animationFilenames", &self.animation_filenames)?;
        serializer.serialize_array_field("behaviorFilenames", &self.behavior_filenames)?;
        serializer.serialize_array_field("characterFilenames", &self.character_filenames)?;
        serializer.serialize_array_field("eventNames", &self.event_names)?;
        serializer.serialize_stringptr_field("animationPath", &self.animation_path)?;
        serializer.serialize_stringptr_field("behaviorPath", &self.behavior_path)?;
        serializer.serialize_stringptr_field("characterPath", &self.character_path)?;
        serializer.serialize_stringptr_field("fullPathToSource", &self.full_path_to_source)?;

        serializer.end()
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for HkbProjectStringData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                /// mem_size_and_flags
                __field0,
                /// reference_count
                __field1,

                /// animation_filenames
                __field2,
                /// behavior_filenames
                __field3,
                /// character_filenames
                __field4,
                /// event_names
                __field5,
                /// animation_path
                __field6,
                /// behavior_path
                __field7,
                /// character_path
                __field8,
                /// full_path_to_source
                __field9,
                /// root_path
                __field10,
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
                        2 => Ok(__Field::__field2),
                        3 => Ok(__Field::__field3),
                        4 => Ok(__Field::__field4),
                        5 => Ok(__Field::__field5),
                        6 => Ok(__Field::__field6),
                        7 => Ok(__Field::__field7),
                        8 => Ok(__Field::__field8),
                        9 => Ok(__Field::__field9),
                        _ => Ok(__Field::__ignore),
                    }
                }

                fn visit_key<__E>(self, __value: &str) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "animationFilenames" => Ok(__Field::__field2),
                        "behaviorFilenames" => Ok(__Field::__field3),
                        "characterFilenames" => Ok(__Field::__field4),
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
                marker: core::marker::PhantomData<HkbProjectStringData<'de>>,
                lifetime: core::marker::PhantomData<&'de ()>,
            }

            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HkbProjectStringData<'de>;
                fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct HkReferencedObject")
                }

                #[inline]
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let parent = __A::next_value(&mut __map)?; // hkBaseObject to vtable of ptr size
                    let mut __field2: _serde::__private::Option<Vec<StringPtr<'de>>> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Vec<StringPtr<'de>>> =
                        _serde::__private::None;

                    while let _serde::__private::Some(__key) =
                        match __A::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationFilenames",
                                        ),
                                    );
                                }
                                //
                                // As we need pad this line.
                                //
                                __field2 =
                                    _serde::__private::Some(match __A::next_value(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    });
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorFilenames",
                                        ),
                                    );
                                }
                                //
                                // As we need pad this line.
                                //
                                __field3 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value(&mut __map) {
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
                    __A::pad(&mut __map, 4, 8); // For tailing alignment.

                    let __field2 = match __field2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationFilenames",
                                ),
                            )
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorFilenames",
                                ),
                            )
                        }
                    };

                    _serde::__private::Ok(HkbProjectStringData {
                        __ptr: todo!(),
                        parent,
                        animation_filenames: __field2,
                        behavior_filenames: __field3,
                        character_filenames: todo!(),
                        event_names: todo!(),
                        animation_path: todo!(),
                        behavior_path: todo!(),
                        character_path: todo!(),
                        full_path_to_source: todo!(),
                        root_path: todo!(),
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
                    let parent_parent = HkBaseObject { __ptr: None };
                    let parent = HkReferencedObject {
                        parent: parent_parent,
                        __ptr: None,
                        mem_size_and_flags: 0,
                        reference_count: 0,
                    };

                    _serde::__private::Ok(HkbProjectStringData {
                        __ptr: todo!(),
                        parent,
                        animation_filenames: todo!(),
                        behavior_filenames: todo!(),
                        character_filenames: todo!(),
                        event_names: todo!(),
                        animation_path: todo!(),
                        behavior_path: todo!(),
                        character_path: todo!(),
                        full_path_to_source: todo!(),
                        root_path: todo!(),
                    })
                }
            }

            const FIELDS: &[&str] = &["memSizeAndFlags", "referenceCount"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbProjectStringData",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<HkbProjectStringData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
