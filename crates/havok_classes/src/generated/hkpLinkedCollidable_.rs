use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLinkedCollidable`
/// -         version: `0`
/// -       signature: `0xe1a81497`
/// -          size:  92(x86)/128(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinkedCollidable {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCollidable,
    /// # C++ Info
    /// -          name: `collisionEntries`(ctype: `hkArray<void>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionEntries: Vec<()>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpLinkedCollidable {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpLinkedCollidable"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xe1a81497)
        }
    }
    impl _serde::Serialize for hkpLinkedCollidable {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xe1a81497)));
            let mut serializer = __serializer
                .serialize_struct("hkpLinkedCollidable", class_meta)?;
            serializer.serialize_field("shape", &self.parent.parent.m_shape)?;
            serializer.serialize_field("shapeKey", &self.parent.parent.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("motion", &self.parent.parent.m_motion)?;
            serializer.skip_field("parent", &self.parent.parent.m_parent)?;
            serializer.skip_field("ownerOffset", &self.parent.m_ownerOffset)?;
            serializer
                .serialize_field(
                    "forceCollideOntoPpu",
                    &self.parent.m_forceCollideOntoPpu,
                )?;
            serializer.skip_field("shapeSizeOnSpu", &self.parent.m_shapeSizeOnSpu)?;
            serializer
                .serialize_field("broadPhaseHandle", &self.parent.m_broadPhaseHandle)?;
            serializer
                .skip_field("boundingVolumeData", &self.parent.m_boundingVolumeData)?;
            serializer
                .serialize_field(
                    "allowedPenetrationDepth",
                    &self.parent.m_allowedPenetrationDepth,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_array_meta_field("collisionEntries", &self.m_collisionEntries)?;
            serializer
                .serialize_array_field("collisionEntries", &self.m_collisionEntries)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_collisionEntries,
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
pub(super) struct __hkpLinkedCollidableVisitor<'de> {
    marker: core::marker::PhantomData<hkpLinkedCollidable>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkpLinkedCollidableVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkpLinkedCollidable, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkpLinkedCollidable>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkpLinkedCollidableVisitor<'de> {
    type Value = hkpLinkedCollidable;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkpLinkedCollidable")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_collisionEntries: _serde::__private::Option<Vec<()>> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_collisionEntries) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "collisionEntries",
                            ),
                        );
                    }
                    m_collisionEntries = _serde::__private::Some(
                        match __A::next_value::<Vec<()>>(&mut __map) {
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
        let m_collisionEntries = match m_collisionEntries {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("collisionEntries"),
                );
            }
        };
        _serde::__private::Ok(hkpLinkedCollidable {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_collisionEntries,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __hkpCollidableVisitor::visit_as_parent(&mut __map)?;
        for _ in 0..0usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    _ => {}
                }
            }
        }
        _serde::__private::Ok(hkpLinkedCollidable {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkpLinkedCollidable {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["collisionEntries"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkpLinkedCollidable",
                FIELDS,
                __hkpLinkedCollidableVisitor {
                    marker: _serde::__private::PhantomData::<hkpLinkedCollidable>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
