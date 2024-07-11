use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpVehicleDefaultSuspension`
/// -         version: `0`
/// -       signature: `0x21735a24`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpVehicleDefaultSuspension {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpVehicleSuspension,
    /// # C++ Info
    /// -          name: `wheelSpringParams`(ctype: `hkArray<struct hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_wheelSpringParams: Vec<
        hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
    >,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpVehicleDefaultSuspension {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpVehicleDefaultSuspension"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x21735a24)
        }
    }
    impl _serde::Serialize for hkpVehicleDefaultSuspension {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x21735a24)));
            let mut serializer = __serializer
                .serialize_struct("hkpVehicleDefaultSuspension", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("wheelParams", &self.parent.m_wheelParams)?;
            serializer
                .serialize_array_meta_field(
                    "wheelSpringParams",
                    &self.m_wheelSpringParams,
                )?;
            serializer.serialize_array_field("wheelParams", &self.parent.m_wheelParams)?;
            serializer
                .serialize_array_field("wheelSpringParams", &self.m_wheelSpringParams)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_wheelSpringParams,
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
            "wheelSpringParams" => Ok(__Field::m_wheelSpringParams),
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
pub(super) struct __hkpVehicleDefaultSuspensionVisitor<'de> {
    marker: core::marker::PhantomData<hkpVehicleDefaultSuspension>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpVehicleDefaultSuspensionVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpVehicleDefaultSuspension, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpVehicleDefaultSuspension>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpVehicleDefaultSuspensionVisitor<'de> {
    type Value = hkpVehicleDefaultSuspension;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkpVehicleDefaultSuspension",
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
        let mut m_wheelSpringParams: _serde::__private::Option<
            Vec<hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>,
        > = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_wheelSpringParams) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "wheelSpringParams",
                            ),
                        );
                    }
                    m_wheelSpringParams = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<
                                hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
                            >,
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
        let m_wheelSpringParams = match m_wheelSpringParams {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelSpringParams"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultSuspension {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wheelSpringParams,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpVehicleSuspensionVisitor::visit_as_parent(&mut __map)?;
        let mut m_wheelSpringParams: _serde::__private::Option<
            Vec<hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters>,
        > = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_wheelSpringParams => {
                        if _serde::__private::Option::is_some(&m_wheelSpringParams) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "wheelSpringParams",
                                ),
                            );
                        }
                        m_wheelSpringParams = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<
                                    hkpVehicleDefaultSuspensionWheelSpringSuspensionParameters,
                                >,
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
        let m_wheelSpringParams = match m_wheelSpringParams {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("wheelSpringParams"),
                );
            }
        };
        _serde::__private::Ok(hkpVehicleDefaultSuspension {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_wheelSpringParams,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpVehicleDefaultSuspension {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["wheelSpringParams"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpVehicleDefaultSuspension",
                FIELDS,
                __hkpVehicleDefaultSuspensionVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkpVehicleDefaultSuspension,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
