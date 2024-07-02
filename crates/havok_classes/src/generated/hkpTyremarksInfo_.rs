use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpTyremarksInfo`
/// -         version: `0`
/// -       signature: `0x3d0433d6`
/// -          size:  28(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpTyremarksInfo {
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
    /// -          name: `minTyremarkEnergy`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minTyremarkEnergy: f32,
    /// # C++ Info
    /// -          name: `maxTyremarkEnergy`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxTyremarkEnergy: f32,
    /// # C++ Info
    /// -          name: `tyremarksWheel`(ctype: `hkArray<hkpTyremarksWheel*>`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_tyremarksWheel: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpTyremarksInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpTyremarksInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(1023685590u32)
        }
    }
    impl _serde::Serialize for hkpTyremarksInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(1023685590u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpTyremarksInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("minTyremarkEnergy", &self.m_minTyremarkEnergy)?;
            serializer.serialize_field("maxTyremarkEnergy", &self.m_maxTyremarkEnergy)?;
            serializer
                .serialize_array_meta_field("tyremarksWheel", &self.m_tyremarksWheel)?;
            serializer.serialize_array_field("tyremarksWheel", &self.m_tyremarksWheel)?;
            serializer.end()
        }
    }
};
