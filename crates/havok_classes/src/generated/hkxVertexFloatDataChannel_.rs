use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxVertexFloatDataChannel`
/// -         version: `1`
/// -       signature: `0xbeeb397c`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxVertexFloatDataChannel {
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
    /// -          name: `perVertexFloats`(ctype: `hkArray<hkReal>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_perVertexFloats: Vec<f32>,
    /// # C++ Info
    /// -          name: `dimensions`(ctype: `enum VertexFloatDimensions`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_dimensions: VertexFloatDimensions,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxVertexFloatDataChannel {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxVertexFloatDataChannel"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3203086716u32)
        }
    }
    impl __serde::Serialize for hkxVertexFloatDataChannel {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkxVertexFloatDataChannel", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field("perVertexFloats", &self.m_perVertexFloats)?;
            serializer.serialize_field("dimensions", &self.m_dimensions)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field("perVertexFloats", &self.m_perVertexFloats)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum VertexFloatDimensions {
    #[default]
    FLOAT = 0isize,
    DISTANCE = 1isize,
    ANGLE = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for VertexFloatDimensions {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::FLOAT => __serializer.serialize_field("FLOAT", &0u64),
                Self::DISTANCE => __serializer.serialize_field("DISTANCE", &1u64),
                Self::ANGLE => __serializer.serialize_field("ANGLE", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum VertexFloatDimensions to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
