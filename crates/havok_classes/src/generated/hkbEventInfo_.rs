use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbEventInfo`
/// -         version: `0`
/// -       signature: `0x5874eed4`
/// -          size:   4(x86)/  4(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbEventInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `flags`(ctype: `flags Flags`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_flags: Flags,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbEventInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbEventInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x5874eed4)
        }
    }
    impl _serde::Serialize for hkbEventInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x5874eed4)));
            let mut serializer = __serializer
                .serialize_struct("hkbEventInfo", class_meta)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.end()
        }
    }
};
use havok_serde as _serde;
#[allow(non_camel_case_types)]
enum __Field {
    m_flags,
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
            "flags" => Ok(__Field::m_flags),
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
pub(super) struct __hkbEventInfoVisitor<'de> {
    marker: core::marker::PhantomData<hkbEventInfo>,
    lifetime: core::marker::PhantomData<&'de ()>,
}
impl<'de> __hkbEventInfoVisitor<'de> {
    /// # Purpose of this method
    /// To reproduce C++ field inheritance, we will have the field internal implementation
    /// of deserialization partially exposed and reused.
    #[inline]
    pub(super) fn visit_as_parent<__A>(
        __map: &mut __A,
    ) -> _serde::__private::Result<hkbEventInfo, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        _serde::de::Visitor::visit_struct(
            Self {
                marker: _serde::__private::PhantomData::<hkbEventInfo>,
                lifetime: _serde::__private::PhantomData,
            },
            __map,
        )
    }
}
#[allow(clippy::match_single_binding)]
#[allow(clippy::reversed_empty_ranges)]
#[allow(clippy::single_match)]
impl<'de> _serde::de::Visitor<'de> for __hkbEventInfoVisitor<'de> {
    type Value = hkbEventInfo;
    fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Formatter::write_str(__formatter, "struct hkbEventInfo")
    }
    fn visit_struct_for_bytes<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_flags: _serde::__private::Option<Flags> = _serde::__private::None;
        for i in 0..1usize {
            match i {
                0usize => {
                    if _serde::__private::Option::is_some(&m_flags) {
                        return _serde::__private::Err(
                            <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                        );
                    }
                    m_flags = _serde::__private::Some(
                        match __A::next_value::<Flags>(&mut __map) {
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
        let m_flags = match m_flags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("flags"),
                );
            }
        };
        _serde::__private::Ok(hkbEventInfo { __ptr, m_flags })
    }
    fn visit_struct<__A>(
        self,
        mut __map: __A,
    ) -> _serde::__private::Result<Self::Value, __A::Error>
    where
        __A: _serde::de::MapAccess<'de>,
    {
        let __ptr = __A::class_ptr(&mut __map);
        let mut m_flags: _serde::__private::Option<Flags> = _serde::__private::None;
        for _ in 0..1usize {
            if let _serde::__private::Some(__key) = __A::next_key::<
                __Field,
            >(&mut __map)? {
                match __key {
                    __Field::m_flags => {
                        if _serde::__private::Option::is_some(&m_flags) {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                            );
                        }
                        m_flags = _serde::__private::Some(
                            match __A::next_value::<Flags>(&mut __map) {
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
        let m_flags = match m_flags {
            _serde::__private::Some(__field) => __field,
            _serde::__private::None => {
                return _serde::__private::Err(
                    <__A::Error as _serde::de::Error>::missing_field("flags"),
                );
            }
        };
        _serde::__private::Ok(hkbEventInfo { __ptr, m_flags })
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbEventInfo {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            const FIELDS: &[&str] = &["flags"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbEventInfo",
                FIELDS,
                __hkbEventInfoVisitor {
                    marker: _serde::__private::PhantomData::<hkbEventInfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    Flags : u32 { #[doc = "1"] const FLAG_SILENT = 1u32; #[doc = "2"] const
    FLAG_SYNC_POINT = 2u32; }
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for Flags {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_SILENT => {
                        __serializer.serialize_field("FLAG_SILENT", &Self::FLAG_SILENT)
                    }
                    Self::FLAG_SYNC_POINT => {
                        __serializer
                            .serialize_field("FLAG_SYNC_POINT", &Self::FLAG_SYNC_POINT)
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Flags {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Flags>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Flags;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct Flags(flags)",
                    )
                }
                #[inline]
                fn visit_uint32<__E>(
                    self,
                    __value: u32,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(Flags::from_bits_retain(__value as _))
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <Flags as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint32,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Flags>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
