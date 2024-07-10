use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkTrackerSerializableScanSnapshotAllocation`
/// -         version: `0`
/// -       signature: `0x9ab3a6ac`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkTrackerSerializableScanSnapshotAllocation {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `start`(ctype: `hkUlong`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_start: u64,
    /// # C++ Info
    /// -          name: `size`(ctype: `hkUlong`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_size: u64,
    /// # C++ Info
    /// -          name: `traceId`(ctype: `hkInt32`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_traceId: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkTrackerSerializableScanSnapshotAllocation {
        #[inline]
        fn name(&self) -> &'static str {
            "hkTrackerSerializableScanSnapshotAllocation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9ab3a6ac)
        }
    }
    impl _serde::Serialize for hkTrackerSerializableScanSnapshotAllocation {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9ab3a6ac)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkTrackerSerializableScanSnapshotAllocation",
                    class_meta,
                )?;
            serializer.serialize_field("start", &self.m_start)?;
            serializer.serialize_field("size", &self.m_size)?;
            serializer.serialize_field("traceId", &self.m_traceId)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_start,
    m_size,
    m_traceId,
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
            "start" => Ok(__Field::m_start),
            "size" => Ok(__Field::m_size),
            "traceId" => Ok(__Field::m_traceId),
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
pub(super) struct __hkTrackerSerializableScanSnapshotAllocationVisitor<'de> {
    marker: core::marker::PhantomData<hkTrackerSerializableScanSnapshotAllocation>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkTrackerSerializableScanSnapshotAllocationVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<
        hkTrackerSerializableScanSnapshotAllocation,
        __A::Error,
    >
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<
                    hkTrackerSerializableScanSnapshotAllocation,
                >,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de>
for __hkTrackerSerializableScanSnapshotAllocationVisitor<'de> {
    type Value = hkTrackerSerializableScanSnapshotAllocation;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(
            __formatter,
            "struct hkTrackerSerializableScanSnapshotAllocation",
        )
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_start: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_size: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_traceId: _serde::__private::Option<i32> = _serde::__private::None;
        for i in 0..3usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_start) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("start"),
                        );
                    }
                    m_start = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                1usize => {
                    if _serde::__private::Option::is_some(&m_size) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("size"),
                        );
                    }
                    m_size = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                2usize => {
                    if _serde::__private::Option::is_some(&m_traceId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("traceId"),
                        );
                    }
                    m_traceId = _serde::__private::Some(
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
        __A::pad(&mut __map, 0usize, 4usize)?;
        let m_start = match m_start {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("start"),
                );
            }
        };
        let m_size = match m_size {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("size"),
                );
            }
        };
        let m_traceId = match m_traceId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("traceId"),
                );
            }
        };
        _serde::__private::Ok(hkTrackerSerializableScanSnapshotAllocation {
            __ptr: __A::class_ptr(&mut __map),
            m_start,
            m_size,
            m_traceId,
        })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let mut m_start: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_size: _serde::__private::Option<u64> = _serde::__private::None;
        let mut m_traceId: _serde::__private::Option<i32> = _serde::__private::None;
        while let _serde::__private::Some(__key) = match __A::next_key::<
            __Field,
        >(&mut __map) {
            _serde::__private::Ok(__val) => __val,
            _serde::__private::Err(__err) => {
                return _serde::__private::Err(__err);
            }
        } {
            match __key {
                __Field::m_start => {
                    if _serde::__private::Option::is_some(&m_start) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("start"),
                        );
                    }
                    m_start = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_size => {
                    if _serde::__private::Option::is_some(&m_size) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("size"),
                        );
                    }
                    m_size = _serde::__private::Some(
                        match __A::next_value::<u64>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        },
                    );
                }
                __Field::m_traceId => {
                    if _serde::__private::Option::is_some(&m_traceId) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("traceId"),
                        );
                    }
                    m_traceId = _serde::__private::Some(
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
        let m_start = match m_start {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("start"),
                );
            }
        };
        let m_size = match m_size {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("size"),
                );
            }
        };
        let m_traceId = match m_traceId {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("traceId"),
                );
            }
        };
        _serde::__private::Ok(hkTrackerSerializableScanSnapshotAllocation {
            __ptr: __A::class_ptr(&mut __map),
            m_start,
            m_size,
            m_traceId,
        })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkTrackerSerializableScanSnapshotAllocation {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["start", "size", "traceId"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkTrackerSerializableScanSnapshotAllocation",
                FIELDS,
                __hkTrackerSerializableScanSnapshotAllocationVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkTrackerSerializableScanSnapshotAllocation,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
