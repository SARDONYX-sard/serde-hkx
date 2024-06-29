use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMeshVertexBuffer`
/// -         version: `0`
/// -       signature: `0x534b08c8`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMeshVertexBuffer {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkMeshVertexBuffer {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkMeshVertexBuffer"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1397426376u32)
        }
    }
    impl __serde::Serialize for hkMeshVertexBuffer {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkMeshVertexBuffer", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
bitflags::bitflags! {
    #[doc = r" Bit flags that represented `enum hkFlags<Enum, SizeType>`(C++)."] #[doc =
    "- size(C++): `TYPE_UINT32`"] #[allow(non_upper_case_globals, non_snake_case)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)] pub
    struct Flags : u32 { #[doc = "1"] const ACCESS_READ = 1u32; #[doc = "2"] const
    ACCESS_WRITE = 2u32; #[doc = "3"] const ACCESS_READ_WRITE = 3u32; #[doc = "4"] const
    ACCESS_WRITE_DISCARD = 4u32; #[doc = "8"] const ACCESS_ELEMENT_ARRAY = 8u32; }
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
                    Self::ACCESS_READ => {
                        __serializer.serialize_field("ACCESS_READ", &Self::ACCESS_READ)
                    }
                    Self::ACCESS_WRITE => {
                        __serializer.serialize_field("ACCESS_WRITE", &Self::ACCESS_WRITE)
                    }
                    Self::ACCESS_READ_WRITE => {
                        __serializer
                            .serialize_field(
                                "ACCESS_READ_WRITE",
                                &Self::ACCESS_READ_WRITE,
                            )
                    }
                    Self::ACCESS_WRITE_DISCARD => {
                        __serializer
                            .serialize_field(
                                "ACCESS_WRITE_DISCARD",
                                &Self::ACCESS_WRITE_DISCARD,
                            )
                    }
                    Self::ACCESS_ELEMENT_ARRAY => {
                        __serializer
                            .serialize_field(
                                "ACCESS_ELEMENT_ARRAY",
                                &Self::ACCESS_ELEMENT_ARRAY,
                            )
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
