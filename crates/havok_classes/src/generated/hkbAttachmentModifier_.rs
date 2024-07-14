use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAttachmentModifier`
/// -         version: `1`
/// -       signature: `0xcc0aab32`
/// -          size: 108(x86)/200(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAttachmentModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `sendToAttacherOnAttach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacherOnAttach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `sendToAttacheeOnAttach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  52(x86)/ 96(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacheeOnAttach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `sendToAttacherOnDetach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  60(x86)/112(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacherOnDetach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `sendToAttacheeOnDetach`(ctype: `struct hkbEventProperty`)
    /// -        offset:  68(x86)/128(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_sendToAttacheeOnDetach: hkbEventProperty,
    /// # C++ Info
    /// -          name: `attachmentSetup`(ctype: `struct hkbAttachmentSetup*`)
    /// -        offset:  76(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attachmentSetup: Pointer,
    /// # C++ Info
    /// -          name: `attacherHandle`(ctype: `struct hkbHandle*`)
    /// -        offset:  80(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attacherHandle: Pointer,
    /// # C++ Info
    /// -          name: `attacheeHandle`(ctype: `struct hkbHandle*`)
    /// -        offset:  84(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_attacheeHandle: Pointer,
    /// # C++ Info
    /// -          name: `attacheeLayer`(ctype: `hkInt32`)
    /// -        offset:  88(x86)/168(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_attacheeLayer: i32,
    /// # C++ Info
    /// -          name: `attacheeRB`(ctype: `void*`)
    /// -        offset:  92(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attacheeRB: Pointer,
    /// # C++ Info
    /// -          name: `oldMotionType`(ctype: `enum unknown`)
    /// -        offset:  96(x86)/184(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldMotionType: u8,
    /// # C++ Info
    /// -          name: `oldFilterInfo`(ctype: `hkInt32`)
    /// -        offset: 100(x86)/188(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_oldFilterInfo: i32,
    /// # C++ Info
    /// -          name: `attachment`(ctype: `void*`)
    /// -        offset: 104(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_attachment: Pointer,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbAttachmentModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbAttachmentModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xcc0aab32)
        }
    }
    impl<'a> _serde::Serialize for hkbAttachmentModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xcc0aab32)));
            let mut serializer = __serializer
                .serialize_struct("hkbAttachmentModifier", class_meta)?;
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
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "sendToAttacherOnAttach",
                    &self.m_sendToAttacherOnAttach,
                )?;
            serializer
                .serialize_field(
                    "sendToAttacheeOnAttach",
                    &self.m_sendToAttacheeOnAttach,
                )?;
            serializer
                .serialize_field(
                    "sendToAttacherOnDetach",
                    &self.m_sendToAttacherOnDetach,
                )?;
            serializer
                .serialize_field(
                    "sendToAttacheeOnDetach",
                    &self.m_sendToAttacheeOnDetach,
                )?;
            serializer.serialize_field("attachmentSetup", &self.m_attachmentSetup)?;
            serializer.serialize_field("attacherHandle", &self.m_attacherHandle)?;
            serializer.serialize_field("attacheeHandle", &self.m_attacheeHandle)?;
            serializer.serialize_field("attacheeLayer", &self.m_attacheeLayer)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("attacheeRB", &self.m_attacheeRB)?;
            serializer.skip_field("oldMotionType", &self.m_oldMotionType)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.skip_field("oldFilterInfo", &self.m_oldFilterInfo)?;
            serializer.skip_field("attachment", &self.m_attachment)?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_sendToAttacherOnAttach,
    m_sendToAttacheeOnAttach,
    m_sendToAttacherOnDetach,
    m_sendToAttacheeOnDetach,
    m_attachmentSetup,
    m_attacherHandle,
    m_attacheeHandle,
    m_attacheeLayer,
    m_attacheeRB,
    m_oldMotionType,
    m_oldFilterInfo,
    m_attachment,
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
            "sendToAttacherOnAttach" => Ok(__Field::m_sendToAttacherOnAttach),
            "sendToAttacheeOnAttach" => Ok(__Field::m_sendToAttacheeOnAttach),
            "sendToAttacherOnDetach" => Ok(__Field::m_sendToAttacherOnDetach),
            "sendToAttacheeOnDetach" => Ok(__Field::m_sendToAttacheeOnDetach),
            "attachmentSetup" => Ok(__Field::m_attachmentSetup),
            "attacherHandle" => Ok(__Field::m_attacherHandle),
            "attacheeHandle" => Ok(__Field::m_attacheeHandle),
            "attacheeLayer" => Ok(__Field::m_attacheeLayer),
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
pub(super) struct __hkbAttachmentModifierVisitor<'de> {
    marker: core::marker::PhantomData<hkbAttachmentModifier<'de>>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbAttachmentModifierVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbAttachmentModifier<'de>, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbAttachmentModifier<'de>>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbAttachmentModifierVisitor<'de> {
    type Value = hkbAttachmentModifier<'de>;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbAttachmentModifier")
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
        let mut m_sendToAttacherOnAttach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_sendToAttacheeOnAttach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_sendToAttacherOnDetach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_sendToAttacheeOnDetach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_attachmentSetup: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_attacherHandle: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_attacheeHandle: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_attacheeLayer: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_attacheeRB: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_oldMotionType: _serde::__private::Option<u8> = _serde::__private::None;
        let mut m_oldFilterInfo: _serde::__private::Option<i32> = _serde::__private::None;
        let mut m_attachment: _serde::__private::Option<Pointer> = _serde::__private::None;
        for i in 0..12usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_sendToAttacherOnAttach) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sendToAttacherOnAttach",
                            ),
                        );
                    }
                    m_sendToAttacherOnAttach = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_sendToAttacheeOnAttach) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sendToAttacheeOnAttach",
                            ),
                        );
                    }
                    m_sendToAttacheeOnAttach = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_sendToAttacherOnDetach) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sendToAttacherOnDetach",
                            ),
                        );
                    }
                    m_sendToAttacherOnDetach = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                3usize => {
                    if _serde::__private::Option::is_some(&m_sendToAttacheeOnDetach) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "sendToAttacheeOnDetach",
                            ),
                        );
                    }
                    m_sendToAttacheeOnDetach = _serde::__private::Some(
                        match __A::next_value::<hkbEventProperty>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                4usize => {
                    if _serde::__private::Option::is_some(&m_attachmentSetup) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attachmentSetup",
                            ),
                        );
                    }
                    m_attachmentSetup = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                5usize => {
                    if _serde::__private::Option::is_some(&m_attacherHandle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attacherHandle",
                            ),
                        );
                    }
                    m_attacherHandle = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                6usize => {
                    if _serde::__private::Option::is_some(&m_attacheeHandle) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attacheeHandle",
                            ),
                        );
                    }
                    m_attacheeHandle = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                7usize => {
                    if _serde::__private::Option::is_some(&m_attacheeLayer) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attacheeLayer",
                            ),
                        );
                    }
                    m_attacheeLayer = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                8usize => {
                    if _serde::__private::Option::is_some(&m_attacheeRB) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attacheeRB",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    m_attacheeRB = _serde::__private::Some(
                        match __A::next_value::<Pointer>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                9usize => {
                    if _serde::__private::Option::is_some(&m_oldMotionType) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "oldMotionType",
                            ),
                        );
                    }
                    m_oldMotionType = _serde::__private::Some(
                        match __A::next_value::<u8>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                10usize => {
                    if _serde::__private::Option::is_some(&m_oldFilterInfo) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "oldFilterInfo",
                            ),
                        );
                    }
                    __A::pad(&mut __map, 3usize, 3usize)?;
                    m_oldFilterInfo = _serde::__private::Some(
                        match __A::next_value::<i32>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                11usize => {
                    if _serde::__private::Option::is_some(&m_attachment) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field(
                                "attachment",
                            ),
                        );
                    }
                    m_attachment = _serde::__private::Some(
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
        let m_sendToAttacherOnAttach = match m_sendToAttacherOnAttach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacherOnAttach",
                    ),
                );
            }
        };
        let m_sendToAttacheeOnAttach = match m_sendToAttacheeOnAttach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacheeOnAttach",
                    ),
                );
            }
        };
        let m_sendToAttacherOnDetach = match m_sendToAttacherOnDetach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacherOnDetach",
                    ),
                );
            }
        };
        let m_sendToAttacheeOnDetach = match m_sendToAttacheeOnDetach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacheeOnDetach",
                    ),
                );
            }
        };
        let m_attachmentSetup = match m_attachmentSetup {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attachmentSetup"),
                );
            }
        };
        let m_attacherHandle = match m_attacherHandle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacherHandle"),
                );
            }
        };
        let m_attacheeHandle = match m_attacheeHandle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacheeHandle"),
                );
            }
        };
        let m_attacheeLayer = match m_attacheeLayer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacheeLayer"),
                );
            }
        };
        let m_attacheeRB = match m_attacheeRB {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacheeRB"),
                );
            }
        };
        let m_oldMotionType = match m_oldMotionType {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("oldMotionType"),
                );
            }
        };
        let m_oldFilterInfo = match m_oldFilterInfo {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("oldFilterInfo"),
                );
            }
        };
        let m_attachment = match m_attachment {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attachment"),
                );
            }
        };
        _serde::__private::Ok(hkbAttachmentModifier {
            __ptr,
            parent,
            m_sendToAttacherOnAttach,
            m_sendToAttacheeOnAttach,
            m_sendToAttacherOnDetach,
            m_sendToAttacheeOnDetach,
            m_attachmentSetup,
            m_attacherHandle,
            m_attacheeHandle,
            m_attacheeLayer,
            m_attacheeRB,
            m_oldMotionType,
            m_oldFilterInfo,
            m_attachment,
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
        let parent = __hkbModifierVisitor::visit_as_parent(&mut __map)?;
        let mut m_sendToAttacherOnAttach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_sendToAttacheeOnAttach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_sendToAttacherOnDetach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_sendToAttacheeOnDetach: _serde::__private::Option<hkbEventProperty> = _serde::__private::None;
        let mut m_attachmentSetup: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_attacherHandle: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_attacheeHandle: _serde::__private::Option<Pointer> = _serde::__private::None;
        let mut m_attacheeLayer: _serde::__private::Option<i32> = _serde::__private::None;
        for _ in 0..8usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_sendToAttacherOnAttach => {
                        if _serde::__private::Option::is_some(
                            &m_sendToAttacherOnAttach,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sendToAttacherOnAttach",
                                ),
                            );
                        }
                        m_sendToAttacherOnAttach = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sendToAttacheeOnAttach => {
                        if _serde::__private::Option::is_some(
                            &m_sendToAttacheeOnAttach,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sendToAttacheeOnAttach",
                                ),
                            );
                        }
                        m_sendToAttacheeOnAttach = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sendToAttacherOnDetach => {
                        if _serde::__private::Option::is_some(
                            &m_sendToAttacherOnDetach,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sendToAttacherOnDetach",
                                ),
                            );
                        }
                        m_sendToAttacherOnDetach = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_sendToAttacheeOnDetach => {
                        if _serde::__private::Option::is_some(
                            &m_sendToAttacheeOnDetach,
                        ) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "sendToAttacheeOnDetach",
                                ),
                            );
                        }
                        m_sendToAttacheeOnDetach = _serde::__private::Some(
                            match __A::next_value::<hkbEventProperty>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_attachmentSetup => {
                        if _serde::__private::Option::is_some(&m_attachmentSetup) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "attachmentSetup",
                                ),
                            );
                        }
                        m_attachmentSetup = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_attacherHandle => {
                        if _serde::__private::Option::is_some(&m_attacherHandle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "attacherHandle",
                                ),
                            );
                        }
                        m_attacherHandle = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_attacheeHandle => {
                        if _serde::__private::Option::is_some(&m_attacheeHandle) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "attacheeHandle",
                                ),
                            );
                        }
                        m_attacheeHandle = _serde::__private::Some(
                            match __A::next_value::<Pointer>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            },
                        );
                    }
                    __Field::m_attacheeLayer => {
                        if _serde::__private::Option::is_some(&m_attacheeLayer) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field(
                                    "attacheeLayer",
                                ),
                            );
                        }
                        m_attacheeLayer = _serde::__private::Some(
                            match __A::next_value::<i32>(&mut __map) {
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
        let m_sendToAttacherOnAttach = match m_sendToAttacherOnAttach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacherOnAttach",
                    ),
                );
            }
        };
        let m_sendToAttacheeOnAttach = match m_sendToAttacheeOnAttach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacheeOnAttach",
                    ),
                );
            }
        };
        let m_sendToAttacherOnDetach = match m_sendToAttacherOnDetach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacherOnDetach",
                    ),
                );
            }
        };
        let m_sendToAttacheeOnDetach = match m_sendToAttacheeOnDetach {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field(
                        "sendToAttacheeOnDetach",
                    ),
                );
            }
        };
        let m_attachmentSetup = match m_attachmentSetup {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attachmentSetup"),
                );
            }
        };
        let m_attacherHandle = match m_attacherHandle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacherHandle"),
                );
            }
        };
        let m_attacheeHandle = match m_attacheeHandle {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacheeHandle"),
                );
            }
        };
        let m_attacheeLayer = match m_attacheeLayer {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("attacheeLayer"),
                );
            }
        };
        _serde::__private::Ok(hkbAttachmentModifier {
            __ptr,
            parent,
            m_sendToAttacherOnAttach,
            m_sendToAttacheeOnAttach,
            m_sendToAttacherOnDetach,
            m_sendToAttacheeOnDetach,
            m_attachmentSetup,
            m_attacherHandle,
            m_attacheeHandle,
            m_attacheeLayer,
            ..Default::default()
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbAttachmentModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &[
                "sendToAttacherOnAttach",
                "sendToAttacheeOnAttach",
                "sendToAttacherOnDetach",
                "sendToAttacheeOnDetach",
                "attachmentSetup",
                "attacherHandle",
                "attacheeHandle",
                "attacheeLayer",
                "attacheeRB",
                "oldMotionType",
                "oldFilterInfo",
                "attachment",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbAttachmentModifier",
                FIELDS,
                __hkbAttachmentModifierVisitor {
                    marker: _serde::__private::PhantomData::<hkbAttachmentModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
