use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleRayCastWheelCollide`
/// -         version: `1`
/// -       signature: `0x41efd9e3`
/// -          size:  36(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleRayCastWheelCollide {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleWheelCollide,
    /// # C++ Info
    /// -          name: `wheelCollisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_wheelCollisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `phantom`(ctype: `struct hkpAabbPhantom*`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_phantom: Pointer,
    /// # C++ Info
    /// -          name: `rejectRayChassisListener`(ctype: `struct hkpRejectChassisListener`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:  16(x86)/ 24(x86_64)
    ///
    pub m_rejectRayChassisListener: hkpRejectChassisListener,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleRayCastWheelCollide {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleRayCastWheelCollide"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x41efd9e3)
        }
    }
    impl _serde::Serialize for hkpVehicleRayCastWheelCollide {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x41efd9e3)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleRayCastWheelCollide", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("alreadyUsed", &self.parent.m_alreadyUsed)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field(
                    "wheelCollisionFilterInfo",
                    &self.m_wheelCollisionFilterInfo,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("phantom", &self.m_phantom)?;
            serializer
                .serialize_field(
                    "rejectRayChassisListener",
                    &self.m_rejectRayChassisListener,
                )?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_wheelCollisionFilterInfo,
    m_phantom,
    m_rejectRayChassisListener,
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
            "wheelCollisionFilterInfo" => Ok(__Field::m_wheelCollisionFilterInfo),
            "phantom" => Ok(__Field::m_phantom),
            "rejectRayChassisListener" => Ok(__Field::m_rejectRayChassisListener),
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
pub(super) struct __hkpVehicleRayCastWheelCollideVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleRayCastWheelCollide>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleRayCastWheelCollideVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleRayCastWheelCollide, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpVehicleRayCastWheelCollide>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleRayCastWheelCollideVisitor<'de> {
    type Value = hkpVehicleRayCastWheelCollide;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleRayCastWheelCollide",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_wheelCollisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_phantom: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_rejectRayChassisListener: _serde::__private::Option<
            hkpRejectChassisListener,
        > = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_wheelCollisionFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelCollisionFilterInfo",
                            ),
                        );
                    }
                    m_wheelCollisionFilterInfo = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_phantom) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("phantom"),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_phantom = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_rejectRayChassisListener) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "rejectRayChassisListener",
                            ),
                        );
                    }
                    m_rejectRayChassisListener = _serde::__private::Some(
                        match __A::next_value::<hkpRejectChassisListener>(&mut __map) {
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
        let m_wheelCollisionFilterInfo = match m_wheelCollisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelCollisionFilterInfo",
                    ),
                );
            }
        };
        let m_phantom = match m_phantom {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("phantom"),
                );
            }
        };
        let m_rejectRayChassisListener = match m_rejectRayChassisListener {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "rejectRayChassisListener",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleRayCastWheelCollide {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wheelCollisionFilterInfo,
            m_phantom,
            m_rejectRayChassisListener,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpVehicleWheelCollideVisitor::visit_as_parent(&mut __map)?;
        let mut m_wheelCollisionFilterInfo: _serde::__private::Option<u32> = _serde::__private::None;
        let mut m_phantom: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_rejectRayChassisListener: _serde::__private::Option<
            hkpRejectChassisListener,
        > = _serde::__private::None;
        for _ in 0..3usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_wheelCollisionFilterInfo => {
                        if _serde::__private::Option::is_some(
                            &m_wheelCollisionFilterInfo,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wheelCollisionFilterInfo",
                                ),
                            );
                        }
                        m_wheelCollisionFilterInfo = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_phantom => {
                        if _serde::__private::Option::is_some(&m_phantom) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "phantom",
                                ),
                            );
                        }
                        m_phantom = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_rejectRayChassisListener => {
                        if _serde::__private::Option::is_some(
                            &m_rejectRayChassisListener,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "rejectRayChassisListener",
                                ),
                            );
                        }
                        m_rejectRayChassisListener = _serde::__private::Some(
                            match __A::next_value::<
                                hkpRejectChassisListener,
                            >(&mut __map) {
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
        }
        let m_wheelCollisionFilterInfo = match m_wheelCollisionFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "wheelCollisionFilterInfo",
                    ),
                );
            }
        };
        let m_phantom = match m_phantom {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("phantom"),
                );
            }
        };
        let m_rejectRayChassisListener = match m_rejectRayChassisListener {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "rejectRayChassisListener",
                    ),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleRayCastWheelCollide {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wheelCollisionFilterInfo,
            m_phantom,
            m_rejectRayChassisListener,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleRayCastWheelCollide {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "wheelCollisionFilterInfo",
                "phantom",
                "rejectRayChassisListener",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleRayCastWheelCollide",
                FIELDS,
                __hkpVehicleRayCastWheelCollideVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleRayCastWheelCollide,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
