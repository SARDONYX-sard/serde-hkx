use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkpConvexPieceMeshShape`
/// - version: `0`
/// - signature: `0x38fd3d97`
/// - size: ` 36`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexPieceMeshShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapeCollection,
    /// # C++ Info
    /// - name: `convexPieceStream`(ctype: `struct hkpConvexPieceStreamData*`)
    /// - offset: ` 24`(x86)/` 48`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_convexPieceStream: Pointer,
    /// # C++ Info
    /// - name: `displayMesh`(ctype: `struct hkpShapeCollection*`)
    /// - offset: ` 28`(x86)/` 56`(x86_64)
    /// - type_size: `  4`(x86)/`  8`(x86_64)
    pub m_displayMesh: Pointer,
    /// # C++ Info
    /// - name: `radius`(ctype: `hkReal`)
    /// - offset: ` 32`(x86)/` 64`(x86_64)
    /// - type_size: `  4`(x86)/`  4`(x86_64)
    pub m_radius: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpConvexPieceMeshShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpConvexPieceMeshShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x38fd3d97)
        }
    }
    impl _serde::Serialize for hkpConvexPieceMeshShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x38fd3d97)));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexPieceMeshShape", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("disableWelding", &self.parent.m_disableWelding)?;
            serializer.serialize_field("collectionType", &self.parent.m_collectionType)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("convexPieceStream", &self.m_convexPieceStream)?;
            serializer.serialize_field("displayMesh", &self.m_displayMesh)?;
            serializer.serialize_field("radius", &self.m_radius)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_convexPieceStream,
    m_displayMesh,
    m_radius,
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
            "convexPieceStream" => Ok(__Field::m_convexPieceStream),
            "displayMesh" => Ok(__Field::m_displayMesh),
            "radius" => Ok(__Field::m_radius),
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
pub(super) struct __hkpConvexPieceMeshShapeVisitor<'de> {
    marker: core::marker::PhantomData<hkpConvexPieceMeshShape>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpConvexPieceMeshShapeVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpConvexPieceMeshShape, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpConvexPieceMeshShape>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpConvexPieceMeshShapeVisitor<'de> {
    type Value = hkpConvexPieceMeshShape;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpConvexPieceMeshShape")
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
        let mut m_convexPieceStream: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_displayMesh: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_convexPieceStream) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "convexPieceStream",
                            ),
                        );
                    }
                    m_convexPieceStream = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_displayMesh) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "displayMesh",
                            ),
                        );
                    }
                    m_displayMesh = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_radius) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                        );
                    }
                    m_radius = _serde::__private::Some(
                        match __A::next_value::<f32>(&mut __map) {
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_convexPieceStream = match m_convexPieceStream {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("convexPieceStream"),
                );
            }
        };
        let m_displayMesh = match m_displayMesh {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("displayMesh"),
                );
            }
        };
        let m_radius = match m_radius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("radius"),
                );
            }
        };
        _serde::__private::Ok(hkpConvexPieceMeshShape {
            __ptr,
            parent,
            m_convexPieceStream,
            m_displayMesh,
            m_radius,
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
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __hkpShapeCollectionVisitor::visit_as_parent(&mut __map)?;
        let mut m_convexPieceStream: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_displayMesh: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_radius: _serde::__private::Option<f32> = _serde::__private::None;
        for _ in 0..3usize {
            #[cfg(not(feature = "strict"))]
            let __res = __A::next_key::<__Field>(&mut __map)
                .unwrap_or(Some(__Field::__ignore));
            #[cfg(feature = "strict")]
            let __res = __A::next_key::<__Field>(&mut __map)?;
            if let _serde::__private::Some(__key) = __res {
                match __key {
                    __Field::m_convexPieceStream => {
                        if _serde::__private::Option::is_some(&m_convexPieceStream) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "convexPieceStream",
                                ),
                            );
                        }
                        m_convexPieceStream = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_displayMesh => {
                        if _serde::__private::Option::is_some(&m_displayMesh) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "displayMesh",
                                ),
                            );
                        }
                        m_displayMesh = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(__err);
                                    #[cfg(not(feature = "strict"))] Default::default()
                                }
                            },
                        );
                    }
                    __Field::m_radius => {
                        if _serde::__private::Option::is_some(&m_radius) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("radius"),
                            );
                        }
                        m_radius = _serde::__private::Some(
                            match __A::next_value::<f32>(&mut __map) {
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
        }
        let m_convexPieceStream = match m_convexPieceStream {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("convexPieceStream"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_displayMesh = match m_displayMesh {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("displayMesh"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        let m_radius = match m_radius {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                #[cfg(feature = "strict")]
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("radius"),
                );
                #[cfg(not(feature = "strict"))] Default::default()
            }
        };
        _serde::__private::Ok(hkpConvexPieceMeshShape {
            __ptr,
            parent,
            m_convexPieceStream,
            m_displayMesh,
            m_radius,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpConvexPieceMeshShape {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["convexPieceStream", "displayMesh", "radius"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpConvexPieceMeshShape",
                FIELDS,
                __hkpConvexPieceMeshShapeVisitor {
                    marker: _serde::__private::PhantomData::<hkpConvexPieceMeshShape>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
