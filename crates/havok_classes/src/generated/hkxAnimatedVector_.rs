use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxAnimatedVector`
/// -         version: `1`
/// -       signature: `0x34b1a197`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxAnimatedVector {
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
    /// -          name: `vectors`(ctype: `hkArray<hkVector4>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_vectors: Vec<Vector4>,
    /// # C++ Info
    /// -          name: `hint`(ctype: `enum Hint`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_hint: Hint,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxAnimatedVector {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxAnimatedVector"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x34b1a197)
        }
    }
    impl _serde::Serialize for hkxAnimatedVector {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x34b1a197)));
            let mut serializer = __serializer
                .serialize_struct("hkxAnimatedVector", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("vectors", &self.m_vectors)?;
            serializer.serialize_field("hint", &self.m_hint)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("vectors", &self.m_vectors)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_vectors,
    m_hint,
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
            "vectors" => Ok(__Field::m_vectors),
            "hint" => Ok(__Field::m_hint),
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
pub(super) struct __hkxAnimatedVectorVisitor<'de> {
    marker: core::marker::PhantomData<hkxAnimatedVector>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxAnimatedVectorVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxAnimatedVector, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxAnimatedVector>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxAnimatedVectorVisitor<'de> {
    type Value = hkxAnimatedVector;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxAnimatedVector")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let parent = __A::next_value(&mut __map)?;
        let mut m_vectors: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_hint: _serde::__private::Option<Hint> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_vectors) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("vectors"),
                        );
                    }
                    m_vectors = _serde::__private::Some(
                        match __A::next_value::<Vec<Vector4>>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_hint) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("hint"),
                        );
                    }
                    m_hint = _serde::__private::Some(
                        match __A::next_value::<Hint>(&mut __map) {
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
        __A::pad(&mut __map, 3usize, 7usize)?;
        let m_vectors = match m_vectors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectors"),
                );
            }
        };
        let m_hint = match m_hint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hint"),
                );
            }
        };
        _serde::__private::Ok(hkxAnimatedVector {
            __ptr: __A::class_ptr(&mut __map),
            parent,
            m_vectors,
            m_hint,
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
        let parent = __hkReferencedObjectVisitor::visit_as_parent(&mut __map)?;
        let mut m_vectors: _serde::__private::Option<Vec<Vector4>> = _serde::__private::None;
        let mut m_hint: _serde::__private::Option<Hint> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_vectors => {
                        if _serde::__private::Option::is_some(&m_vectors) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "vectors",
                                ),
                            );
                        }
                        m_vectors = _serde::__private::Some(
                            match __A::next_value::<Vec<Vector4>>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_hint => {
                        if _serde::__private::Option::is_some(&m_hint) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("hint"),
                            );
                        }
                        m_hint = _serde::__private::Some(
                            match __A::next_value::<Hint>(&mut __map) {
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
        let m_vectors = match m_vectors {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("vectors"),
                );
            }
        };
        let m_hint = match m_hint {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hint"),
                );
            }
        };
        _serde::__private::Ok(hkxAnimatedVector {
            __ptr,
            parent,
            m_vectors,
            m_hint,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxAnimatedVector {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["vectors", "hint"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxAnimatedVector",
                FIELDS,
                __hkxAnimatedVectorVisitor {
                    marker: _serde::__private::PhantomData::<hkxAnimatedVector>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
