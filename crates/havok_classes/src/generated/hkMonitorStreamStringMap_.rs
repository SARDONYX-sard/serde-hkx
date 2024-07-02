use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkMonitorStreamStringMap`
/// -         version: `0`
/// -       signature: `0xc4d3a8b4`
/// -          size:  12(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkMonitorStreamStringMap<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `map`(ctype: `hkArray<struct hkMonitorStreamStringMapStringMap>`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_map: Vec<hkMonitorStreamStringMapStringMap<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkMonitorStreamStringMap<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkMonitorStreamStringMap"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3302205620u32)
        }
    }
    impl<'a> _serde::Serialize for hkMonitorStreamStringMap<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3302205620u32)));
            let mut serializer = __serializer
                .serialize_struct("hkMonitorStreamStringMap", class_meta)?;
            serializer.serialize_array_meta_field("map", &self.m_map)?;
            serializer.serialize_array_field("map", &self.m_map)?;
            serializer.end()
        }
    }
};
