use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbComputeRotationToTargetModifierInternalState`
/// - version: `0`
/// - signature: `0x71cd1eb0`
/// - size: ` 32`(x86)/` 32`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbComputeRotationToTargetModifierInternalState {
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
    /// - name: `rotationOut`(ctype: `hkQuaternion`)
    /// - offset: ` 16`(x86)/` 16`(x86_64)
    /// - type_size: ` 16`(x86)/` 16`(x86_64)
    pub m_rotationOut: Quaternion,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbComputeRotationToTargetModifierInternalState {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbComputeRotationToTargetModifierInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x71cd1eb0)
        }
    }
    impl _serde::Serialize for hkbComputeRotationToTargetModifierInternalState {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x71cd1eb0)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbComputeRotationToTargetModifierInternalState",
                    class_meta,
                )?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("rotationOut", &self.m_rotationOut)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de>
    for hkbComputeRotationToTargetModifierInternalState {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_rotationOut,
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
                        "rotationOut" => Ok(__Field::m_rotationOut),
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
            struct __hkbComputeRotationToTargetModifierInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbComputeRotationToTargetModifierInternalState,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbComputeRotationToTargetModifierInternalStateVisitor<'de> {
                type Value = hkbComputeRotationToTargetModifierInternalState;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbComputeRotationToTargetModifierInternalState",
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
                    let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    for i in 0..1usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_rotationOut) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationOut",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 8usize, 0usize)?;
                                m_rotationOut = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
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
                    let m_rotationOut = match m_rotationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationOut",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbComputeRotationToTargetModifierInternalState {
                        __ptr,
                        parent,
                        m_rotationOut,
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
                    let mut m_rotationOut: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        #[cfg(not(feature = "strict"))]
                        let __key = __A::next_key::<__Field>(&mut __map)
                            .unwrap_or(Some(__Field::__ignore));
                        #[cfg(feature = "strict")]
                        let __key = __A::next_key::<__Field>(&mut __map)?;
                        __key
                    } {
                        match __key {
                            __Field::m_rotationOut => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rotationOut) {
                                    #[cfg(feature = "ignore_duplicates")] continue;
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationOut",
                                        ),
                                    );
                                }
                                m_rotationOut = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
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
                    let m_rotationOut = match m_rotationOut {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationOut",
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
                    _serde::__private::Ok(hkbComputeRotationToTargetModifierInternalState {
                        __ptr,
                        parent,
                        m_rotationOut,
                    })
                }
            }
            const FIELDS: &[&str] = &["rotationOut"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbComputeRotationToTargetModifierInternalState",
                FIELDS,
                __hkbComputeRotationToTargetModifierInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbComputeRotationToTargetModifierInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
