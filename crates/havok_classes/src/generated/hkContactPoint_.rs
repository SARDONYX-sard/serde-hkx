use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkContactPoint`
/// -         version: `0`
/// -       signature: `0x91d7dd8e`
/// -          size:  32(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkContactPoint {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `position`(ctype: `hkVector4`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_position: Vector4,
    /// # C++ Info
    /// -          name: `separatingNormal`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_separatingNormal: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkContactPoint {
        #[inline]
        fn name(&self) -> &'static str {
            "hkContactPoint"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(2446843278u32)
        }
    }
    impl _serde::Serialize for hkContactPoint {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(2446843278u32)));
            let mut serializer = __serializer
                .serialize_struct("hkContactPoint", class_meta)?;
            serializer.serialize_field("position", &self.m_position)?;
            serializer.serialize_field("separatingNormal", &self.m_separatingNormal)?;
            serializer.end()
        }
    }
};
