use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkBaseObject {
    pub __ptr: Option<Pointer>,
}

impl HavokClass for HkBaseObject {
    #[inline]
    fn name(&self) -> &'static str {
        "hkBaseObject"
    }

    #[inline]
    fn signature(&self) -> Signature {
        Signature::new(0xea7f1d08)
    }
}
impl Serialize for HkBaseObject {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let class_meta = self.__ptr.map(|name| (name, self.signature()));
        let mut serializer = serializer.serialize_struct("hkBaseObject", class_meta)?;

        serializer.pad_field([0u8; 4].as_slice(), [0u8; 8].as_slice())?; // hkBaseObject ptr size
        serializer.end()
    }
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for HkBaseObject {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            struct __Visitor<'de> {
                marker: core::marker::PhantomData<HkBaseObject>,
                lifetime: core::marker::PhantomData<&'de ()>,
            }

            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = HkBaseObject;
                fn expecting(&self, __formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct HkBaseObject")
                }

                #[inline]
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    __A::pad(&mut __map, 4, 8); // hkBaseObject to vtable of ptr size
                    _serde::__private::Ok(HkBaseObject {
                        __ptr: __A::class_ptr(&mut __map),
                    })
                }

                #[inline]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    _serde::__private::Ok(HkBaseObject {
                        __ptr: __A::class_ptr(&mut __map),
                    })
                }
            }

            const FIELDS: &[&str] = &[];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkBaseObject",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Self>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
