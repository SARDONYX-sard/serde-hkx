use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSetupStabilizationAtom`
/// -         version: `1`
/// -       signature: `0xf05d137e`
/// -          size:  16(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSetupStabilizationAtom {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConstraintAtom,
    /// # C++ Info
    /// -          name: `enabled`(ctype: `hkBool`)
    /// -        offset:   2(x86)/  2(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_enabled: bool,
    /// # C++ Info
    /// -          name: `maxAngle`(ctype: `hkReal`)
    /// -        offset:   4(x86)/  4(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxAngle: f32,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkUint8[8]`)
    /// -        offset:   8(x86)/  8(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_padding: [u8; 8usize],
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSetupStabilizationAtom {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSetupStabilizationAtom"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(4032631678u32)
        }
    }
    impl __serde::Serialize for hkpSetupStabilizationAtom {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSetupStabilizationAtom", class_meta)?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("enabled", &self.m_enabled)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("maxAngle", &self.m_maxAngle)?;
            serializer.serialize_field("padding", &self.m_padding.as_slice())?;
            serializer.end()
        }
    }
};
