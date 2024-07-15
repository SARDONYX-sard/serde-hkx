use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpBvShape`
/// - version: `0`
/// - signature: `0x286eb64c`
/// - size: ` 28`(x86)/` 56`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpBvShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShape,
    /// # C++ Info
    /// - name: `boundingVolumeShape`(ctype: `struct hkpShape*`)
    /// - offset: ` 16`(x86)/` 32`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_boundingVolumeShape: Pointer,
    /// # C++ Info
    /// - name: `childShape`(ctype: `struct hkpSingleShapeContainer`)
    /// - offset: ` 20`(x86)/` 40`(x86_64)
    /// - type_size: `  8`(x86)/` 16`(x86_64)
    pub m_childShape: hkpSingleShapeContainer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpBvShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpBvShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x286eb64c)
        }
    }
    impl _serde::Serialize for hkpBvShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x286eb64c)));
            let mut serializer = __serializer
                .serialize_struct("hkpBvShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("boundingVolumeShape", &self.m_boundingVolumeShape)?;
            serializer.serialize_field("childShape", &self.m_childShape)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_boundingVolumeShape,
    m_childShape,
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
            "boundingVolumeShape" => Ok(__Field::m_boundingVolumeShape),
            "childShape" => Ok(__Field::m_childShape),
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
pub(super) struct __hkpBvShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpBvShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpBvShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpBvShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpBvShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpBvShapeVisitor<'de> {
    type Value = hkpBvShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpBvShape")
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
        let mut m_boundingVolumeShape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_childShape: _serde::__private::Option<hkpSingleShapeContainer> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_boundingVolumeShape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "boundingVolumeShape",
                            ),
                        );
                    }
                    m_boundingVolumeShape = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_childShape) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "childShape",
                            ),
                        );
                    }
                    m_childShape = _serde::__private::Some(
                        match __A::next_value::<hkpSingleShapeContainer>(&mut __map) {
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
        let m_boundingVolumeShape = match m_boundingVolumeShape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "boundingVolumeShape",
                    ),
                );
            }
        };
        let m_childShape = match m_childShape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShape"),
                );
            }
        };
        _serde::__private::Ok(hkpBvShape {
            __ptr,
            parent,
            m_boundingVolumeShape,
            m_childShape,
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
        let parent = __hkpShapeVisitor::visit_as_parent(&mut __map)?;
        let mut m_boundingVolumeShape: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_childShape: _serde::__private::Option<hkpSingleShapeContainer> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_boundingVolumeShape => {
                        if _serde::__private::Option::is_some(&m_boundingVolumeShape) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "boundingVolumeShape",
                                ),
                            );
                        }
                        m_boundingVolumeShape = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_childShape => {
                        if _serde::__private::Option::is_some(&m_childShape) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "childShape",
                                ),
                            );
                        }
                        m_childShape = _serde::__private::Some(
                            match __A::next_value::<
                                hkpSingleShapeContainer,
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
        let m_boundingVolumeShape = match m_boundingVolumeShape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "boundingVolumeShape",
                    ),
                );
            }
        };
        let m_childShape = match m_childShape {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("childShape"),
                );
            }
        };
        _serde::__private::Ok(hkpBvShape {
            __ptr,
            parent,
            m_boundingVolumeShape,
            m_childShape,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpBvShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["boundingVolumeShape", "childShape"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpBvShape",
                FIELDS,
                __hkpBvShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpBvShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
