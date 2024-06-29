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
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbEventInfo {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbEventInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1484058324u32)
        }
    }
    impl __serde::Serialize for hkbEventInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbEventInfo", class_meta)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.end()
        }
    }
};
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct Flags : u32 { #[doc = "1"] const FLAG_SILENT = 1u32; #[doc = "2"] const
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
