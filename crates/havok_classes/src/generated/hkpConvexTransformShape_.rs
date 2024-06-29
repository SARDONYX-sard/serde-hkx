use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpConvexTransformShape`
/// -         version: `0`
/// -       signature: `0xae3e5017`
/// -          size:  96(x86)/128(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpConvexTransformShape {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpConvexTransformShapeBase,
    /// # C++ Info
    /// -          name: `transform`(ctype: `hkTransform`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  64(x86)/ 64(x86_64)
    ///
    pub m_transform: Transform,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpConvexTransformShape {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpConvexTransformShape"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2923319319u32)
        }
    }
    impl __serde::Serialize for hkpConvexTransformShape {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpConvexTransformShape", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "userData",
                    &self.parent.parent.parent.parent.m_userData,
                )?;
            serializer.skip_field("type", &self.parent.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("radius", &self.parent.parent.m_radius)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("childShape", &self.parent.m_childShape)?;
            serializer.skip_field("childShapeSize", &self.parent.m_childShapeSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("transform", &self.m_transform)?;
            serializer.end()
        }
    }
};
