use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSimpleShapePhantom`
/// -         version: `0`
/// -       signature: `0x32a2a8a8`
/// -          size: 368(x86)/448(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleShapePhantom<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShapePhantom<'a>,
    /// # C++ Info
    /// -          name: `collisionDetails`(ctype: `hkArray<struct hkpSimpleShapePhantomCollisionDetail>`)
    /// -        offset: 352(x86)/416(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionDetails: Vec<hkpSimpleShapePhantomCollisionDetail>,
    /// # C++ Info
    /// -          name: `orderDirty`(ctype: `hkBool`)
    /// -        offset: 364(x86)/432(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_orderDirty: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpSimpleShapePhantom<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSimpleShapePhantom"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x32a2a8a8)
        }
    }
    impl<'a> _serde::Serialize for hkpSimpleShapePhantom<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x32a2a8a8)));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleShapePhantom", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.parent.m_world)?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer
                .serialize_field("collidable", &self.parent.parent.parent.m_collidable)?;
            serializer
                .serialize_field(
                    "multiThreadCheck",
                    &self.parent.parent.parent.m_multiThreadCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "name",
                    &self.parent.parent.parent.m_name,
                )?;
            serializer
                .serialize_array_meta_field(
                    "properties",
                    &self.parent.parent.parent.m_properties,
                )?;
            serializer.skip_field("treeData", &self.parent.parent.parent.m_treeData)?;
            serializer
                .skip_array_meta_field(
                    "overlapListeners",
                    &self.parent.parent.m_overlapListeners,
                )?;
            serializer
                .skip_array_meta_field(
                    "phantomListeners",
                    &self.parent.parent.m_phantomListeners,
                )?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("motionState", &self.parent.m_motionState)?;
            serializer
                .skip_array_meta_field("collisionDetails", &self.m_collisionDetails)?;
            serializer.skip_field("orderDirty", &self.m_orderDirty)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .serialize_stringptr_field("name", &self.parent.parent.parent.m_name)?;
            serializer
                .serialize_array_field(
                    "properties",
                    &self.parent.parent.parent.m_properties,
                )?;
            serializer
                .serialize_array_field(
                    "overlapListeners",
                    &self.parent.parent.m_overlapListeners,
                )?;
            serializer
                .serialize_array_field(
                    "phantomListeners",
                    &self.parent.parent.m_phantomListeners,
                )?;
            serializer
                .serialize_array_field("collisionDetails", &self.m_collisionDetails)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_collisionDetails,
    m_orderDirty,
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
pub(super) struct __hkpSimpleShapePhantomVisitor<'de> {
    marker: core::marker::PhantomData<hkpSimpleShapePhantom<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpSimpleShapePhantomVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpSimpleShapePhantom<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpSimpleShapePhantom<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpSimpleShapePhantomVisitor<'de> {
    type Value = hkpSimpleShapePhantom<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpSimpleShapePhantom")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let parent = __A::next_value(&mut __map)?;
        let mut m_collisionDetails: _serde::__private::Option<
            Vec<hkpSimpleShapePhantomCollisionDetail>,
        > = _serde::__private::None;
        let mut m_orderDirty: _serde::__private::Option<bool> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_collisionDetails) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionDetails",
                            ),
                        );
                    }
                    m_collisionDetails = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkpSimpleShapePhantomCollisionDetail>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_orderDirty) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "orderDirty",
                            ),
                        );
                    }
                    m_orderDirty = _serde::__private::Some(
                        match __A::next_value::<bool>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 15usize)?;
        let m_collisionDetails = match m_collisionDetails {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("collisionDetails"),
                );
            }
        };
        let m_orderDirty = match m_orderDirty {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("orderDirty"),
                );
            }
        };
        _serde::__private::Ok(hkpSimpleShapePhantom {
            __ptr,
            parent,
            m_collisionDetails,
            m_orderDirty,
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
        let parent = __hkpShapePhantomVisitor::visit_as_parent(&mut __map)?;
        for _ in 0..0usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    _ => {}
                }
            }
        }
        _serde::__private::Ok(hkpSimpleShapePhantom {
            __ptr,
            parent,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpSimpleShapePhantom<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["collisionDetails", "orderDirty"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpSimpleShapePhantom",
                FIELDS,
                __hkpSimpleShapePhantomVisitor {
                    marker: _serde::__private::PhantomData::<hkpSimpleShapePhantom>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
