use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMultiSphereShape`
/// -         version: `0`
/// -       signature: `0x61a590fc`
/// -          size: 160(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMultiSphereShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpSphereRepShape,
    /// # C++ Info
    /// -          name: `numSpheres`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numSpheres: i32,
    /// # C++ Info
    /// -          name: `spheres`(ctype: `hkVector4[8]`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_spheres: [Vector4; 8usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMultiSphereShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMultiSphereShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x61a590fc)
        }
    }
    impl _serde::Serialize for hkpMultiSphereShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x61a590fc)));
            let mut serializer = __serializer
                .serialize_struct("hkpMultiSphereShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("numSpheres", &self.m_numSpheres)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("spheres", &self.m_spheres.as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_numSpheres,
    m_spheres,
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
            "numSpheres" => Ok(__Field::m_numSpheres),
            "spheres" => Ok(__Field::m_spheres),
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
pub(super) struct __hkpMultiSphereShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpMultiSphereShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpMultiSphereShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpMultiSphereShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpMultiSphereShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpMultiSphereShapeVisitor<'de> {
    type Value = hkpMultiSphereShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpMultiSphereShape")
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
        let mut m_numSpheres: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_spheres: _serde::__private::Option<[Vector4; 8usize]> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_numSpheres) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "numSpheres",
                            ),
                        );
                    }
                    m_numSpheres = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_spheres) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("spheres"),
                        );
                    }
                    __A::pad(&mut __map, 12usize, 12usize)?;
                    m_spheres = _serde::__private::Some(
                        match __A::next_value::<[Vector4; 8usize]>(&mut __map) {
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
        let m_numSpheres = match m_numSpheres {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numSpheres"),
                );
            }
        };
        let m_spheres = match m_spheres {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spheres"),
                );
            }
        };
        _serde::__private::Ok(hkpMultiSphereShape {
            __ptr,
            parent,
            m_numSpheres,
            m_spheres,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpSphereRepShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_numSpheres: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_spheres: _serde::__private::Option<[Vector4; 8usize]> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_numSpheres => {
                        if _serde::__private::Option::is_some(&m_numSpheres) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "numSpheres",
                                ),
                            );
                        }
                        m_numSpheres = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_spheres => {
                        if _serde::__private::Option::is_some(&m_spheres) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "spheres",
                                ),
                            );
                        }
                        m_spheres = _serde::__private::Some(
                            match __A::next_value::<[Vector4; 8usize]>(&mut __map) {
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
        let m_numSpheres = match m_numSpheres {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("numSpheres"),
                );
            }
        };
        let m_spheres = match m_spheres {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("spheres"),
                );
            }
        };
        _serde::__private::Ok(hkpMultiSphereShape {
            __ptr,
            parent,
            m_numSpheres,
            m_spheres,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpMultiSphereShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["numSpheres", "spheres"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpMultiSphereShape",
                FIELDS,
                __hkpMultiSphereShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpMultiSphereShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
