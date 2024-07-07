use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkSphere`
/// -         version: `0`
/// -       signature: `0x143dff99`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkSphere {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `pos`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pos: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkSphere {
        #[inline]
        fn name(&self) -> &'static str {
            "hkSphere"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x143dff99)
        }
    }
    impl _serde::Serialize for hkSphere {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x143dff99)));
            let mut serializer = __serializer.serialize_struct("hkSphere", class_meta)?;
            serializer.serialize_field("pos", &self.m_pos)?;
            serializer.end()
        }
    }
};
