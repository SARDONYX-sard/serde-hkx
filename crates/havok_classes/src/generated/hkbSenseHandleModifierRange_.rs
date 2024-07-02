use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSenseHandleModifierRange`
/// -         version: `0`
/// -       signature: `0xfb56b692`
/// -          size:  20(x86)/ 32(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSenseHandleModifierRange {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `event`(ctype: `struct hkbEventProperty`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   8(x86)/ 16(x86_64)
    ///
    pub m_event: hkbEventProperty,
    /// # C++ Info
    /// -          name: `minDistance`(ctype: `hkReal`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_minDistance: f32,
    /// # C++ Info
    /// -          name: `maxDistance`(ctype: `hkReal`)
    /// -        offset:  12(x86)/ 20(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxDistance: f32,
    /// # C++ Info
    /// -          name: `ignoreHandle`(ctype: `hkBool`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_ignoreHandle: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbSenseHandleModifierRange {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSenseHandleModifierRange"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4216764050u32)
        }
    }
    impl __serde::Serialize for hkbSenseHandleModifierRange {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(4216764050u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbSenseHandleModifierRange", class_meta)?;
            serializer.serialize_field("event", &self.m_event)?;
            serializer.serialize_field("minDistance", &self.m_minDistance)?;
            serializer.serialize_field("maxDistance", &self.m_maxDistance)?;
            serializer.serialize_field("ignoreHandle", &self.m_ignoreHandle)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
