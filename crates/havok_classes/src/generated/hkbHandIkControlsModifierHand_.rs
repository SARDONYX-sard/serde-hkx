use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbHandIkControlsModifierHand`
/// -         version: `0`
/// -       signature: `0x9c72e9e3`
/// -          size:  96(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHandIkControlsModifierHand {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `controlData`(ctype: `struct hkbHandIkControlData`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:  80(x86)/ 96(x86_64)
    ///
    pub m_controlData: hkbHandIkControlData,
    /// # C++ Info
    /// -          name: `handIndex`(ctype: `hkInt32`)
    /// -        offset:  80(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_handIndex: i32,
    /// # C++ Info
    /// -          name: `enable`(ctype: `hkBool`)
    /// -        offset:  84(x86)/100(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enable: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbHandIkControlsModifierHand {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbHandIkControlsModifierHand"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2624776675u32)
        }
    }
    impl __serde::Serialize for hkbHandIkControlsModifierHand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbHandIkControlsModifierHand", class_meta)?;
            serializer.serialize_field("controlData", &self.m_controlData)?;
            serializer.serialize_field("handIndex", &self.m_handIndex)?;
            serializer.serialize_field("enable", &self.m_enable)?;
            serializer.pad_field([0u8; 11usize].as_slice(), [0u8; 11usize].as_slice())?;
            serializer.end()
        }
    }
};
