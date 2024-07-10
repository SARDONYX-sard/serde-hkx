use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpDisplayBindingData`
/// -         version: `1`
/// -       signature: `0xdc46c906`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpDisplayBindingData {
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
    /// -          name: `rigidBodyBindings`(ctype: `hkArray<hkpDisplayBindingDataRigidBody*>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidBodyBindings: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `physicsSystemBindings`(ctype: `hkArray<hkpDisplayBindingDataPhysicsSystem*>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_physicsSystemBindings: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpDisplayBindingData {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpDisplayBindingData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xdc46c906)
        }
    }
    impl _serde::Serialize for hkpDisplayBindingData {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xdc46c906)));
            let mut serializer = __serializer
                .serialize_struct("hkpDisplayBindingData", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "rigidBodyBindings",
                    &self.m_rigidBodyBindings,
                )?;
            serializer
                .serialize_array_meta_field(
                    "physicsSystemBindings",
                    &self.m_physicsSystemBindings,
                )?;
            serializer
                .serialize_array_field("rigidBodyBindings", &self.m_rigidBodyBindings)?;
            serializer
                .serialize_array_field(
                    "physicsSystemBindings",
                    &self.m_physicsSystemBindings,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_rigidBodyBindings,
    m_physicsSystemBindings,
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
            "rigidBodyBindings" => Ok(__Field::m_rigidBodyBindings),
            "physicsSystemBindings" => Ok(__Field::m_physicsSystemBindings),
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
pub(super) struct __hkpDisplayBindingDataVisitor<'de> {
    marker: core::marker::PhantomData<hkpDisplayBindingData>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpDisplayBindingDataVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpDisplayBindingData, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpDisplayBindingData>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpDisplayBindingDataVisitor<'de> {
    type Value = hkpDisplayBindingData;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpDisplayBindingData")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_rigidBodyBindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_physicsSystemBindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_rigidBodyBindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rigidBodyBindings",
                            ),
                        );
                    }
                    m_rigidBodyBindings = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_physicsSystemBindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "physicsSystemBindings",
                            ),
                        );
                    }
                    m_physicsSystemBindings = _serde::__private::Some(
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
        let m_rigidBodyBindings = match m_rigidBodyBindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigidBodyBindings"),
                );
            }
        };
        let m_physicsSystemBindings = match m_physicsSystemBindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "physicsSystemBindings",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpDisplayBindingData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_rigidBodyBindings,
            m_physicsSystemBindings,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_rigidBodyBindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        let mut m_physicsSystemBindings: _serde::__private::Option<Vec<Pointer>> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_rigidBodyBindings => {
                    if _serde::__private::Option::is_some(&m_rigidBodyBindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rigidBodyBindings",
                            ),
                        );
                    }
                    m_rigidBodyBindings = _serde::__private::Some(
                        match __A::next_value::<Vec<Pointer>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_physicsSystemBindings => {
                    if _serde::__private::Option::is_some(&m_physicsSystemBindings) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "physicsSystemBindings",
                            ),
                        );
                    }
                    m_physicsSystemBindings = _serde::__private::Some(
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
        let m_rigidBodyBindings = match m_rigidBodyBindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("rigidBodyBindings"),
                );
            }
        };
        let m_physicsSystemBindings = match m_physicsSystemBindings {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "physicsSystemBindings",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpDisplayBindingData {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_rigidBodyBindings,
            m_physicsSystemBindings,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpDisplayBindingData {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["rigidBodyBindings", "physicsSystemBindings"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpDisplayBindingData",
                FIELDS,
                __hkpDisplayBindingDataVisitor {
                    marker: _serde::__private::PhantomData::<hkpDisplayBindingData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
