use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMultiRayShape`
/// -         version: `0`
/// -       signature: `0xea2e7ec9`
/// -          size:  32(x86)/ 56(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMultiRayShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpShape,
    /// # C++ Info
    /// -          name: `rays`(ctype: `hkArray<struct hkpMultiRayShapeRay>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rays: Vec<hkpMultiRayShapeRay>,
    /// # C++ Info
    /// -          name: `rayPenetrationDistance`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_rayPenetrationDistance: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpMultiRayShape {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMultiRayShape"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xea2e7ec9)
        }
    }
    impl _serde::Serialize for hkpMultiRayShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xea2e7ec9)));
            let mut serializer = __serializer
                .serialize_struct("hkpMultiRayShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.m_userData)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("rays", &self.m_rays)?;
            serializer
                .serialize_field(
                    "rayPenetrationDistance",
                    &self.m_rayPenetrationDistance,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_field("rays", &self.m_rays)?;
            serializer.end()
        }
    }
};
