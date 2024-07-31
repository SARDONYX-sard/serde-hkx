use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpDisplayBindingDataRigidBody`
/// - version: `2`
/// - signature: `0xfe16e2a3`
/// - size: ` 80`(x86)/` 96`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpDisplayBindingDataRigidBody {
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
    /// - name: `rigidBody`(ctype: `struct hkpRigidBody*`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_rigidBody: Pointer,
    /// # C++ Info
    /// - name: `displayObjectPtr`(ctype: `struct hkReferencedObject*`)
    /// - offset: ` 12`(x86)/` 24`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_displayObjectPtr: Pointer,
    /// # C++ Info
    /// - name: `rigidBodyFromDisplayObjectTransform`(ctype: `hkMatrix4`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: ` 64`(x86)/` 64`(x86_64)
    pub m_rigidBodyFromDisplayObjectTransform: Matrix4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpDisplayBindingDataRigidBody {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpDisplayBindingDataRigidBody"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfe16e2a3)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<usize> {
            let mut v = Vec::new();
            v.push(self.m_rigidBody.get());
            v.push(self.m_displayObjectPtr.get());
            v
        }
    }
    impl _serde::Serialize for hkpDisplayBindingDataRigidBody {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xfe16e2a3)));
            let mut serializer = __serializer
                .serialize_struct("hkpDisplayBindingDataRigidBody", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("rigidBody", &self.m_rigidBody)?;
            serializer.serialize_field("displayObjectPtr", &self.m_displayObjectPtr)?;
            serializer
                .serialize_field(
                    "rigidBodyFromDisplayObjectTransform",
                    &self.m_rigidBodyFromDisplayObjectTransform,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpDisplayBindingDataRigidBody {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_rigidBody,
                m_displayObjectPtr,
                m_rigidBodyFromDisplayObjectTransform,
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
                        "rigidBody" => Ok(__Field::m_rigidBody),
                        "displayObjectPtr" => Ok(__Field::m_displayObjectPtr),
                        "rigidBodyFromDisplayObjectTransform" => {
                            Ok(__Field::m_rigidBodyFromDisplayObjectTransform)
                        }
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
            struct __hkpDisplayBindingDataRigidBodyVisitor<'de> {
                marker: _serde::__private::PhantomData<hkpDisplayBindingDataRigidBody>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkpDisplayBindingDataRigidBodyVisitor<'de> {
                type Value = hkpDisplayBindingDataRigidBody;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkpDisplayBindingDataRigidBody",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_rigidBody: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_displayObjectPtr: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_rigidBodyFromDisplayObjectTransform: _serde::__private::Option<
                        Matrix4,
                    > = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_rigidBody) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBody",
                                        ),
                                    );
                                }
                                m_rigidBody = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_displayObjectPtr) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "displayObjectPtr",
                                        ),
                                    );
                                }
                                m_displayObjectPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_rigidBodyFromDisplayObjectTransform,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBodyFromDisplayObjectTransform",
                                        ),
                                    );
                                }
                                m_rigidBodyFromDisplayObjectTransform = _serde::__private::Some(
                                    match __A::next_value::<Matrix4>(&mut __map) {
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
                    let m_rigidBody = match m_rigidBody {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBody",
                                ),
                            );
                        }
                    };
                    let m_displayObjectPtr = match m_displayObjectPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "displayObjectPtr",
                                ),
                            );
                        }
                    };
                    let m_rigidBodyFromDisplayObjectTransform = match m_rigidBodyFromDisplayObjectTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBodyFromDisplayObjectTransform",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkpDisplayBindingDataRigidBody {
                        __ptr,
                        parent,
                        m_rigidBody,
                        m_displayObjectPtr,
                        m_rigidBodyFromDisplayObjectTransform,
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
                    let mut m_rigidBody: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_displayObjectPtr: _serde::__private::Option<Pointer> = _serde::__private::None;
                    let mut m_rigidBodyFromDisplayObjectTransform: _serde::__private::Option<
                        Matrix4,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_rigidBody => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rigidBody) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBody",
                                        ),
                                    );
                                }
                                m_rigidBody = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_displayObjectPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_displayObjectPtr) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "displayObjectPtr",
                                        ),
                                    );
                                }
                                m_displayObjectPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rigidBodyFromDisplayObjectTransform => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_rigidBodyFromDisplayObjectTransform,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigidBodyFromDisplayObjectTransform",
                                        ),
                                    );
                                }
                                m_rigidBodyFromDisplayObjectTransform = _serde::__private::Some(
                                    match __A::next_value::<Matrix4>(&mut __map) {
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
                    let m_rigidBody = match m_rigidBody {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBody",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_displayObjectPtr = match m_displayObjectPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "displayObjectPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rigidBodyFromDisplayObjectTransform = match m_rigidBodyFromDisplayObjectTransform {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rigidBodyFromDisplayObjectTransform",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject { __ptr };
                    let parent = hkReferencedObject {
                        __ptr,
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkpDisplayBindingDataRigidBody {
                        __ptr,
                        parent,
                        m_rigidBody,
                        m_displayObjectPtr,
                        m_rigidBodyFromDisplayObjectTransform,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "rigidBody",
                "displayObjectPtr",
                "rigidBodyFromDisplayObjectTransform",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpDisplayBindingDataRigidBody",
                FIELDS,
                __hkpDisplayBindingDataRigidBodyVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpDisplayBindingDataRigidBody,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
