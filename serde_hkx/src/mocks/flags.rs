use super::class_requires::*;

#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    /// Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++).
    ///
    /// # On XML
    /// When all bits are 0, "0" is inserted.
    /// (Even if `FLAGS_NONE = 0` and 0 is replaced by `FLAGS_NONE`, "0" will appear when reconverting xml -> hkx -> xml.)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct FlagValues: u16 {
        /// Flags is empty: 0
        const FLAGS_NONE = 0;
        /// Force 8-byte align: 1 << 7 => 128
        const ALIGN_8 = 128;
        /// Forced 16-byte align: 1 << 8 => 256
        const ALIGN_16 = 256;
        /// Not owned by class: 1 << 9
        const NOT_OWNED = 512;
        /// Skip serializing: 1 << 10 => 1024
        const SERIALIZE_IGNORED = 1024;
    }
}

impl Serialize for FlagValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut sv = serializer.serialize_enum_flags()?;
        if self.is_empty() {
            sv.serialize_empty_bit()?;
            return sv.end();
        };

        for flag in self.iter() {
            match flag {
                Self::FLAGS_NONE => sv.serialize_field("FLAGS_NONE", &Self::FLAGS_NONE),
                Self::ALIGN_8 => sv.serialize_field("ALIGN_8", &Self::ALIGN_8),
                Self::ALIGN_16 => sv.serialize_field("ALIGN_16", &Self::ALIGN_16),
                Self::NOT_OWNED => sv.serialize_field("NOT_OWNED", &Self::NOT_OWNED),
                Self::SERIALIZE_IGNORED => {
                    sv.serialize_field("SERIALIZE_IGNORED", &Self::SERIALIZE_IGNORED)
                }
                remain => sv.serialize_field(&remain.bits().to_string(), &remain.bits()),
            }?
        }

        // For binary
        sv.serialize_bits(&self.bits())?;
        sv.end()
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for FlagValues {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagValues>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagValues;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct FlagValues")
                }

                #[inline]
                fn visit_uint16<__E>(
                    self,
                    __value: u16,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    Ok(FlagValues::from_bits_retain(__value as _)) // Contain unknown bits.
                }

                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <FlagValues as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }

            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint16,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagValues>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
