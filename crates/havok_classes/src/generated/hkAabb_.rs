use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkAabb`
/// -         version: `0`
/// -       signature: `0x4a948b16`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkAabb {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `min`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_min: Vector4,
    /// # C++ Info
    /// -          name: `max`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_max: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkAabb {
        #[inline]
        fn name(&self) -> &'static str {
            "hkAabb"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4a948b16)
        }
    }
    impl _serde::Serialize for hkAabb {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x4a948b16)));
            let mut serializer = __serializer.serialize_struct("hkAabb", class_meta)?;
            serializer.serialize_field("min", &self.m_min)?;
            serializer.serialize_field("max", &self.m_max)?;
            serializer.end()
        }
    }
};
