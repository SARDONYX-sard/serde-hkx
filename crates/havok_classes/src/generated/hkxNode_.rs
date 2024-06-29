use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkxNode`
/// -         version: `1`
/// -       signature: `0x5a218502`
/// -          size:  72(x86)/112(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkxNode<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkxAttributeHolder<'a>,
    /// # C++ Info
    /// -          name: `name`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `object`(ctype: `struct hkReferencedObject*`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_object: Pointer,
    /// # C++ Info
    /// -          name: `keyFrames`(ctype: `hkArray<hkMatrix4>`)
    /// -        offset:  28(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_keyFrames: Vec<Matrix4>,
    /// # C++ Info
    /// -          name: `children`(ctype: `hkArray<hkxNode*>`)
    /// -        offset:  40(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_children: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `annotations`(ctype: `hkArray<struct hkxNodeAnnotationData>`)
    /// -        offset:  52(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_annotations: Vec<hkxNodeAnnotationData<'a>>,
    /// # C++ Info
    /// -          name: `userProperties`(ctype: `hkStringPtr`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_userProperties: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `selected`(ctype: `hkBool`)
    /// -        offset:  68(x86)/104(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_selected: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkxNode<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkxNode"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1512146178u32)
        }
    }
    impl<'a> __serde::Serialize for hkxNode<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer.serialize_struct("hkxNode", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndFlags", &self.parent.parent.m_memSizeAndFlags)?;
            serializer
                .skip_field("referenceCount", &self.parent.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                )?;
            serializer.serialize_stringptr_meta_field("name", &self.m_name)?;
            serializer.serialize_field("object", &self.m_object)?;
            serializer.serialize_array_meta_field("keyFrames", &self.m_keyFrames)?;
            serializer.serialize_array_meta_field("children", &self.m_children)?;
            serializer.serialize_array_meta_field("annotations", &self.m_annotations)?;
            serializer
                .serialize_stringptr_meta_field(
                    "userProperties",
                    &self.m_userProperties,
                )?;
            serializer.serialize_field("selected", &self.m_selected)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "attributeGroups",
                    &self.parent.m_attributeGroups,
                )?;
            serializer.serialize_stringptr_field("name", &self.m_name)?;
            serializer.serialize_array_field("keyFrames", &self.m_keyFrames)?;
            serializer.serialize_array_field("children", &self.m_children)?;
            serializer.serialize_array_field("annotations", &self.m_annotations)?;
            serializer
                .serialize_stringptr_field("userProperties", &self.m_userProperties)?;
            serializer.end()
        }
    }
};
