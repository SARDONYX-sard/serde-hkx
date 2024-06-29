use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkContactPointMaterial`
/// -         version: `0`
/// -       signature: `0x4e32287c`
/// -          size:   8(x86)/ 16(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkContactPointMaterial {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `userData`(ctype: `hkUlong`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userData: u64,
    /// # C++ Info
    /// -          name: `friction`(ctype: `hkUint8`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_friction: u8,
    /// # C++ Info
    /// -          name: `restitution`(ctype: `hkUint8`)
    /// -        offset:   5(x86)/  9(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_restitution: u8,
    /// # C++ Info
    /// -          name: `maxImpulse`(ctype: `hkUint8`)
    /// -        offset:   6(x86)/ 10(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_maxImpulse: u8,
    /// # C++ Info
    /// -          name: `flags`(ctype: `hkUint8`)
    /// -        offset:   7(x86)/ 11(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_flags: u8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkContactPointMaterial {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkContactPointMaterial"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1311910012u32)
        }
    }
    impl __serde::Serialize for hkContactPointMaterial {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkContactPointMaterial", class_meta)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_field("friction", &self.m_friction)?;
            serializer.serialize_field("restitution", &self.m_restitution)?;
            serializer.serialize_field("maxImpulse", &self.m_maxImpulse)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
