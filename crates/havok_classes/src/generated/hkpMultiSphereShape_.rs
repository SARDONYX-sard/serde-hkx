use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMultiSphereShape`
/// -         version: `0`
/// -       signature: `0x61a590fc`
/// -          size: 160(x86)/176(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMultiSphereShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpSphereRepShape,
    /// # C++ Info
    /// -          name: `numSpheres`(ctype: `hkInt32`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_numSpheres: i32,
    /// # C++ Info
    /// -          name: `spheres`(ctype: `hkVector4[8]`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size: 128(x86)/128(x86_64)
    ///
    pub m_spheres: [Vector4; 8usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMultiSphereShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMultiSphereShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1638240508u32)
        }
    }
    impl _serde::Serialize for hkpMultiSphereShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1638240508u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpMultiSphereShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("numSpheres", &self.m_numSpheres)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.serialize_field("spheres", &self.m_spheres.as_slice())?;
            serializer.end()
        }
    }
};
