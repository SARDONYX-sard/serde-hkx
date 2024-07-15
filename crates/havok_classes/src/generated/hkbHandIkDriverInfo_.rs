use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbHandIkDriverInfo`
/// - version: `0`
/// - signature: `0xc299090a`
/// - size: ` 24`(x86)/` 40`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkDriverInfo<'a> {
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
    /// - name: `hands`(ctype: `hkArray<struct hkbHandIkDriverInfoHand>`)
    /// - offset: `  8`(x86)/` 16`(x86_64)
    /// - type_size: ` 12`(x86)/` 16`(x86_64)
    pub m_hands: Vec<hkbHandIkDriverInfoHand<'a>>,
    /// # C++ Info
    /// - name: `fadeInOutCurve`(ctype: `enum BlendCurve`)
    /// - offset: ` 20`(x86)/` 32`(x86_64)
    /// - type_size: `  1`(x86)/`  1`(x86_64)
    pub m_fadeInOutCurve: BlendCurve,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbHandIkDriverInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbHandIkDriverInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc299090a)
        }
    }
    impl<'a> _serde::Serialize for hkbHandIkDriverInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc299090a)));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkDriverInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("hands", &self.m_hands)?;
            serializer.serialize_field("fadeInOutCurve", &self.m_fadeInOutCurve)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("hands", &self.m_hands)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_hands,
    m_fadeInOutCurve,
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
            "hands" => Ok(__Field::m_hands),
            "fadeInOutCurve" => Ok(__Field::m_fadeInOutCurve),
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
pub(super) struct __hkbHandIkDriverInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbHandIkDriverInfo<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbHandIkDriverInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbHandIkDriverInfo<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbHandIkDriverInfo<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbHandIkDriverInfoVisitor<'de> {
    type Value = hkbHandIkDriverInfo<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbHandIkDriverInfo")
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
        let mut m_hands: _serde::__private::Option<Vec<hkbHandIkDriverInfoHand<'de>>> = _serde::__private::None;
        let mut m_fadeInOutCurve: _serde::__private::Option<BlendCurve> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_hands) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("hands"),
                        );
                    }
                    m_hands = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkbHandIkDriverInfoHand<'de>>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_fadeInOutCurve) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "fadeInOutCurve",
                            ),
                        );
                    }
                    m_fadeInOutCurve = _serde::__private::Some(
                        match __A::next_value::<BlendCurve>(&mut __map) {
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
        let m_hands = match m_hands {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hands"),
                );
            }
        };
        let m_fadeInOutCurve = match m_fadeInOutCurve {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fadeInOutCurve"),
                );
            }
        };
        _serde::__private::Ok(hkbHandIkDriverInfo {
            __ptr,
            parent,
            m_hands,
            m_fadeInOutCurve,
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
        let mut m_hands: _serde::__private::Option<Vec<hkbHandIkDriverInfoHand<'de>>> = _serde::__private::None;
        let mut m_fadeInOutCurve: _serde::__private::Option<BlendCurve> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_hands => {
                        if _serde::__private::Option::is_some(&m_hands) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("hands"),
                            );
                        }
                        m_hands = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkbHandIkDriverInfoHand<'de>>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_fadeInOutCurve => {
                        if _serde::__private::Option::is_some(&m_fadeInOutCurve) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "fadeInOutCurve",
                                ),
                            );
                        }
                        m_fadeInOutCurve = _serde::__private::Some(
                            match __A::next_value::<BlendCurve>(&mut __map) {
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
        let m_hands = match m_hands {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("hands"),
                );
            }
        };
        let m_fadeInOutCurve = match m_fadeInOutCurve {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("fadeInOutCurve"),
                );
            }
        };
        _serde::__private::Ok(hkbHandIkDriverInfo {
            __ptr,
            parent,
            m_hands,
            m_fadeInOutCurve,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbHandIkDriverInfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["hands", "fadeInOutCurve"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbHandIkDriverInfo",
                FIELDS,
                __hkbHandIkDriverInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbHandIkDriverInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
