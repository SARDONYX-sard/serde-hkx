use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpStiffSpringChainDataConstraintInfo`
/// -         version: `0`
/// -       signature: `0xc624a180`
/// -          size:  48(x86)/ 48(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpStiffSpringChainDataConstraintInfo {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pivotInA`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pivotInA: Vector4,
    /// # C++ Info
    /// -          name: `pivotInB`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pivotInB: Vector4,
    /// # C++ Info
    /// -          name: `springLength`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_springLength: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpStiffSpringChainDataConstraintInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpStiffSpringChainDataConstraintInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc624a180)
        }
    }
    impl _serde::Serialize for hkpStiffSpringChainDataConstraintInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc624a180)));
            let mut serializer = __serializer
                .serialize_struct("hkpStiffSpringChainDataConstraintInfo", class_meta)?;
            serializer.serialize_field("pivotInA", &self.m_pivotInA)?;
            serializer.serialize_field("pivotInB", &self.m_pivotInB)?;
            serializer.serialize_field("springLength", &self.m_springLength)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.end()
        }
    }
};
