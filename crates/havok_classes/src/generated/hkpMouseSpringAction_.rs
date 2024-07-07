use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpMouseSpringAction`
/// -         version: `0`
/// -       signature: `0x6e087fd6`
/// -          size:  96(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpMouseSpringAction<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpUnaryAction<'a>,
    /// # C++ Info
    /// -          name: `positionInRbLocal`(ctype: `hkVector4`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_positionInRbLocal: Vector4,
    /// # C++ Info
    /// -          name: `mousePositionInWorld`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_mousePositionInWorld: Vector4,
    /// # C++ Info
    /// -          name: `springDamping`(ctype: `hkReal`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_springDamping: f32,
    /// # C++ Info
    /// -          name: `springElasticity`(ctype: `hkReal`)
    /// -        offset:  68(x86)/100(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_springElasticity: f32,
    /// # C++ Info
    /// -          name: `maxRelativeForce`(ctype: `hkReal`)
    /// -        offset:  72(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_maxRelativeForce: f32,
    /// # C++ Info
    /// -          name: `objectDamping`(ctype: `hkReal`)
    /// -        offset:  76(x86)/108(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_objectDamping: f32,
    /// # C++ Info
    /// -          name: `shapeKey`(ctype: `hkUint32`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_shapeKey: u32,
    /// # C++ Info
    /// -          name: `applyCallbacks`(ctype: `hkArray<void*>`)
    /// -        offset:  84(x86)/120(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_applyCallbacks: Vec<Pointer>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkpMouseSpringAction<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpMouseSpringAction"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x6e087fd6)
        }
    }
    impl<'a> _serde::Serialize for hkpMouseSpringAction<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x6e087fd6)));
            let mut serializer = __serializer
                .serialize_struct("hkpMouseSpringAction", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("world", &self.parent.parent.m_world)?;
            serializer.skip_field("island", &self.parent.parent.m_island)?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_field("entity", &self.parent.m_entity)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("positionInRbLocal", &self.m_positionInRbLocal)?;
            serializer
                .serialize_field("mousePositionInWorld", &self.m_mousePositionInWorld)?;
            serializer.serialize_field("springDamping", &self.m_springDamping)?;
            serializer.serialize_field("springElasticity", &self.m_springElasticity)?;
            serializer.serialize_field("maxRelativeForce", &self.m_maxRelativeForce)?;
            serializer.serialize_field("objectDamping", &self.m_objectDamping)?;
            serializer.serialize_field("shapeKey", &self.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_array_meta_field("applyCallbacks", &self.m_applyCallbacks)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.serialize_array_field("applyCallbacks", &self.m_applyCallbacks)?;
            serializer.end()
        }
    }
};
