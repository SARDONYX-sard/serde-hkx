use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMonitorStreamColorTable`
/// -         version: `0`
/// -       signature: `0x79e53e85`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMonitorStreamColorTable<'a> {
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
    /// -          name: `colorPairs`(ctype: `hkArray<struct hkMonitorStreamColorTableColorPair>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_colorPairs: Vec<hkMonitorStreamColorTableColorPair<'a>>,
    /// # C++ Info
    /// -          name: `defaultColor`(ctype: `hkUint32`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_defaultColor: u32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMonitorStreamColorTable<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMonitorStreamColorTable"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x79e53e85)
        }
    }
    impl<'a> _serde::Serialize for hkMonitorStreamColorTable<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x79e53e85)));
            let mut serializer = __serializer
                .serialize_struct("hkMonitorStreamColorTable", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("colorPairs", &self.m_colorPairs)?;
            serializer.serialize_field("defaultColor", &self.m_defaultColor)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("colorPairs", &self.m_colorPairs)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_colorPairs,
    m_defaultColor,
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
            "colorPairs" => Ok(__Field::m_colorPairs),
            "defaultColor" => Ok(__Field::m_defaultColor),
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
pub(super) struct __hkMonitorStreamColorTableVisitor<'de> {
    marker: core::marker::PhantomData<hkMonitorStreamColorTable<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkMonitorStreamColorTableVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkMonitorStreamColorTable<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkMonitorStreamColorTable<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkMonitorStreamColorTableVisitor<'de> {
    type Value = hkMonitorStreamColorTable<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkMonitorStreamColorTable")
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
        let mut m_colorPairs: _serde::__private::Option<
            Vec<hkMonitorStreamColorTableColorPair<'de>>,
        > = _serde::__private::None;
        let mut m_defaultColor: _serde::__private::Option<u32> = _serde::__private::None;
        for i in 0..2usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_colorPairs) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "colorPairs",
                            ),
                        );
                    }
                    m_colorPairs = _serde::__private::Some(
                        match __A::next_value::<
                            Vec<hkMonitorStreamColorTableColorPair<'de>>,
                        >(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_defaultColor) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "defaultColor",
                            ),
                        );
                    }
                    m_defaultColor = _serde::__private::Some(
                        match __A::next_value::<u32>(&mut __map) {
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
        let m_colorPairs = match m_colorPairs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("colorPairs"),
                );
            }
        };
        let m_defaultColor = match m_defaultColor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("defaultColor"),
                );
            }
        };
        _serde::__private::Ok(hkMonitorStreamColorTable {
            __ptr,
            parent,
            m_colorPairs,
            m_defaultColor,
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
        let mut m_colorPairs: _serde::__private::Option<
            Vec<hkMonitorStreamColorTableColorPair<'de>>,
        > = _serde::__private::None;
        let mut m_defaultColor: _serde::__private::Option<u32> = _serde::__private::None;
        for _ in 0..2usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_colorPairs => {
                        if _serde::__private::Option::is_some(&m_colorPairs) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "colorPairs",
                                ),
                            );
                        }
                        m_colorPairs = _serde::__private::Some(
                            match __A::next_value::<
                                Vec<hkMonitorStreamColorTableColorPair<'de>>,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_defaultColor => {
                        if _serde::__private::Option::is_some(&m_defaultColor) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "defaultColor",
                                ),
                            );
                        }
                        m_defaultColor = _serde::__private::Some(
                            match __A::next_value::<u32>(&mut __map) {
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
        let m_colorPairs = match m_colorPairs {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("colorPairs"),
                );
            }
        };
        let m_defaultColor = match m_defaultColor {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("defaultColor"),
                );
            }
        };
        _serde::__private::Ok(hkMonitorStreamColorTable {
            __ptr,
            parent,
            m_colorPairs,
            m_defaultColor,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkMonitorStreamColorTable<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["colorPairs", "defaultColor"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkMonitorStreamColorTable",
                FIELDS,
                __hkMonitorStreamColorTableVisitor {
                    marker: _serde::__private::PhantomData::<hkMonitorStreamColorTable>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
