use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSetNodePropertyCommand`
/// -         version: `1`
/// -       signature: `0xc5160b64`
/// -          size:  32(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSetNodePropertyCommand<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkReferencedObject,
    /// # C++ Info
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `nodeName`(ctype: `hkStringPtr`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_nodeName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `propertyName`(ctype: `hkStringPtr`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_propertyName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `propertyValue`(ctype: `struct hkbVariableValue`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_propertyValue: hkbVariableValue,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkInt32`)
    /// -        offset:  28(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_padding: i32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbSetNodePropertyCommand<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSetNodePropertyCommand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3306556260u32)
        }
    }
    impl<'a> _serde::Serialize for hkbSetNodePropertyCommand<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3306556260u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbSetNodePropertyCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_stringptr_meta_field("nodeName", &self.m_nodeName)?;
            serializer
                .serialize_stringptr_meta_field("propertyName", &self.m_propertyName)?;
            serializer.serialize_field("propertyValue", &self.m_propertyValue)?;
            serializer.serialize_field("padding", &self.m_padding)?;
            serializer.serialize_stringptr_field("nodeName", &self.m_nodeName)?;
            serializer.serialize_stringptr_field("propertyName", &self.m_propertyName)?;
            serializer.end()
        }
    }
};
