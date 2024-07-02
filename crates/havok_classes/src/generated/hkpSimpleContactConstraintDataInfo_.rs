use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSimpleContactConstraintDataInfo`
/// -         version: `1`
/// -       signature: `0xb59d1734`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSimpleContactConstraintDataInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `flags`(ctype: `hkUint16`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `ALIGN_16`
    ///
    pub m_flags: u16,
    /// # C++ Info
    /// -          name: `index`(ctype: `hkUint16`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_index: u16,
    /// # C++ Info
    /// -          name: `internalData0`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_internalData0: f32,
    /// # C++ Info
    /// -          name: `rollingFrictionMultiplier`(ctype: `hkHalf`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_rollingFrictionMultiplier: f16,
    /// # C++ Info
    /// -          name: `internalData1`(ctype: `hkHalf`)
    /// -        offset:  10(x86)/ 10(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_internalData1: f16,
    /// # C++ Info
    /// -          name: `data`(ctype: `hkUint32[5]`)
    /// -        offset:  12(x86)/ 12(x86_64)
    /// -     type_size:  20(x86)/ 20(x86_64)
    ///
    pub m_data: [u32; 5usize],
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpSimpleContactConstraintDataInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpSimpleContactConstraintDataInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3046971188u32)
        }
    }
    impl _serde::Serialize for hkpSimpleContactConstraintDataInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3046971188u32)));
            let mut serializer = __serializer
                .serialize_struct("hkpSimpleContactConstraintDataInfo", class_meta)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("index", &self.m_index)?;
            serializer.serialize_field("internalData0", &self.m_internalData0)?;
            serializer
                .serialize_field(
                    "rollingFrictionMultiplier",
                    &self.m_rollingFrictionMultiplier,
                )?;
            serializer.serialize_field("internalData1", &self.m_internalData1)?;
            serializer.serialize_field("data", &self.m_data.as_slice())?;
            serializer.end()
        }
    }
};
