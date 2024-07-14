use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxSparselyAnimatedEnum`
/// -         version: `1`
/// -       signature: `0x68a47b64`
/// -          size:  36(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxSparselyAnimatedEnum {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkxSparselyAnimatedInt,
    /// # C++ Info
    /// -          name: `enum`(ctype: `struct hkxEnum*`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_enum: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkxSparselyAnimatedEnum {
        #[inline]
        fn name(&self) -> &'static str {
            "hkxSparselyAnimatedEnum"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x68a47b64)
        }
    }
    impl _serde::Serialize for hkxSparselyAnimatedEnum {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x68a47b64)));
            let mut serializer = __serializer
                .serialize_struct("hkxSparselyAnimatedEnum", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("ints", &self.parent.m_ints)?;
            serializer.serialize_array_meta_field("times", &self.parent.m_times)?;
            serializer.serialize_field("enum", &self.m_enum)?;
            serializer.serialize_array_field("ints", &self.parent.m_ints)?;
            serializer.serialize_array_field("times", &self.parent.m_times)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_enum,
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
            "enum" => Ok(__Field::m_enum),
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
pub(super) struct __hkxSparselyAnimatedEnumVisitor<'de> {
    marker: core::marker::PhantomData<hkxSparselyAnimatedEnum>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkxSparselyAnimatedEnumVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkxSparselyAnimatedEnum, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkxSparselyAnimatedEnum>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkxSparselyAnimatedEnumVisitor<'de> {
    type Value = hkxSparselyAnimatedEnum;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkxSparselyAnimatedEnum")
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
        let mut m_enum: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_enum) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("enum"),
                        );
                    }
                    m_enum = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
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
        let m_enum = match m_enum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enum"),
                );
            }
        };
        _serde::__private::Ok(hkxSparselyAnimatedEnum {
            __ptr,
            parent,
            m_enum,
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
        let parent = __hkxSparselyAnimatedIntVisitor::visit_as_parent(&mut __map)?;
        let mut m_enum: _serde::__private::Option<Pointer> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_enum => {
                        if _serde::__private::Option::is_some(&m_enum) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("enum"),
                            );
                        }
                        m_enum = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
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
        let m_enum = match m_enum {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("enum"),
                );
            }
        };
        _serde::__private::Ok(hkxSparselyAnimatedEnum {
            __ptr,
            parent,
            m_enum,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkxSparselyAnimatedEnum {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["enum"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkxSparselyAnimatedEnum",
                FIELDS,
                __hkxSparselyAnimatedEnumVisitor {
                    marker: _serde::__private::PhantomData::<hkxSparselyAnimatedEnum>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
