use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxLight`
/// -         version: `1`
/// -       signature: `0x81c86d42`
/// -          size:  64(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxLight {
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
    /// -          name: `type`(ctype: `enum LightType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: LightType,
    /// # C++ Info
    /// -          name: `position`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_position: Vector4,
    /// # C++ Info
    /// -          name: `direction`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_direction: Vector4,
    /// # C++ Info
    /// -          name: `color`(ctype: `hkUint32`)
    /// -        offset:  48(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_color: u32,
    /// # C++ Info
    /// -          name: `angle`(ctype: `hkReal`)
    /// -        offset:  52(x86)/ 68(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_angle: f32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkxLight {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxLight"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2177396034u32)
        }
    }
    impl __serde::Serialize for hkxLight {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkxLight", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.pad_field([0u8; 7usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("direction", &self.m_direction)?;
            serializer.serialize_field("color", &self.m_color)?;
            serializer.serialize_field("angle", &self.m_angle)?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum LightType {
    #[default]
    POINT_LIGHT = 0isize,
    DIRECTIONAL_LIGHT = 1isize,
    SPOT_LIGHT = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for LightType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::POINT_LIGHT => __serializer.serialize_field("POINT_LIGHT", &0u64),
                Self::DIRECTIONAL_LIGHT => {
                    __serializer.serialize_field("DIRECTIONAL_LIGHT", &1u64)
                }
                Self::SPOT_LIGHT => __serializer.serialize_field("SPOT_LIGHT", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum LightType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
