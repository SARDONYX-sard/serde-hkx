use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandIkDriverInfo`
/// -         version: `0`
/// -       signature: `0xc299090a`
/// -          size:  24(x86)/ 40(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkDriverInfo<'a> {
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
    /// -          name: `hands`(ctype: `hkArray<struct hkbHandIkDriverInfoHand>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_hands: Vec<hkbHandIkDriverInfoHand<'a>>,
    /// # C++ Info
    /// -          name: `fadeInOutCurve`(ctype: `enum BlendCurve`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_fadeInOutCurve: BlendCurve,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbHandIkDriverInfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbHandIkDriverInfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc299090a)
        }
    }
    impl<'a> _serde::Serialize for hkbHandIkDriverInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xc299090a)));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkDriverInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("hands", &self.m_hands)?;
            serializer.serialize_field("fadeInOutCurve", &self.m_fadeInOutCurve)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_array_field("hands", &self.m_hands)?;
            serializer.end()
        }
    }
};
