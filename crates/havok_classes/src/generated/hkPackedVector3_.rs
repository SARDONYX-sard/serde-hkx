use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkPackedVector3`
/// -         version: `0`
/// -       signature: `0x9c16df5b`
/// -          size:   8(x86)/  8(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkPackedVector3 {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `values`(ctype: `hkInt16[4]`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_values: [i16; 4usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkPackedVector3 {
        #[inline]
        fn name(&self) -> &'static str {
            "hkPackedVector3"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9c16df5b)
        }
    }
    impl _serde::Serialize for hkPackedVector3 {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9c16df5b)));
            let mut serializer = __serializer
                .serialize_struct("hkPackedVector3", class_meta)?;
            serializer.serialize_field("values", &self.m_values.as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_values,
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
            "values" => Ok(__Field::m_values),
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
pub(super) struct __hkPackedVector3Visitor<'de> {
    marker: core::marker::PhantomData<hkPackedVector3>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkPackedVector3Visitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkPackedVector3, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkPackedVector3>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkPackedVector3Visitor<'de> {
    type Value = hkPackedVector3;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkPackedVector3")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_values: _serde::__private::Option<[i16; 4usize]> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_values) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("values"),
                        );
                    }
                    m_values = _serde::__private::Some(
                        match __A::next_value::<[i16; 4usize]>(&mut __map) {
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
        let m_values = match m_values {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("values"),
                );
            }
        };
        _serde::__private::Ok(hkPackedVector3 {
            __ptr: __A::class_ptr(&mut __map),
            m_values,
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
        let mut m_values: _serde::__private::Option<[i16; 4usize]> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_values => {
                        if _serde::__private::Option::is_some(&m_values) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("values"),
                            );
                        }
                        m_values = _serde::__private::Some(
                            match __A::next_value::<[i16; 4usize]>(&mut __map) {
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
        let m_values = match m_values {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("values"),
                );
            }
        };
        _serde::__private::Ok(hkPackedVector3 { __ptr, m_values })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkPackedVector3 {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["values"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkPackedVector3",
                FIELDS,
                __hkPackedVector3Visitor {
                    marker: _serde::__private::PhantomData::<hkPackedVector3>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
