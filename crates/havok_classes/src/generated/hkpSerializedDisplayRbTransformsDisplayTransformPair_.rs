use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedDisplayRbTransformsDisplayTransformPair`
/// -         version: `0`
/// -       signature: `0x94ac5bec`
/// -          size:  80(x86)/ 80(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedDisplayRbTransformsDisplayTransformPair {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `rb`(ctype: `struct hkpRigidBody*`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rb: Pointer,
    /// # C++ Info
    /// -          name: `localToDisplay`(ctype: `hkTransform`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_localToDisplay: Transform,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSerializedDisplayRbTransformsDisplayTransformPair {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSerializedDisplayRbTransformsDisplayTransformPair"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2494323692u32)
        }
    }
    impl __serde::Serialize for hkpSerializedDisplayRbTransformsDisplayTransformPair {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkpSerializedDisplayRbTransformsDisplayTransformPair",
                    class_meta,
                )?;
            serializer.serialize_field("rb", &self.m_rb)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("localToDisplay", &self.m_localToDisplay)?;
            serializer.end()
        }
    }
};
