use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSetWordVariableCommand`
/// -         version: `0`
/// -       signature: `0xf3ae5fca`
/// -          size:  64(x86)/ 64(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSetWordVariableCommand {
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
    /// -          name: `quadValue`(ctype: `hkVector4`)
    /// -        offset:  16(x86)/ 16(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_quadValue: Vector4,
    /// # C++ Info
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:  32(x86)/ 32(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `variableId`(ctype: `hkInt32`)
    /// -        offset:  40(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_variableId: i32,
    /// # C++ Info
    /// -          name: `value`(ctype: `struct hkbVariableValue`)
    /// -        offset:  44(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_value: hkbVariableValue,
    /// # C++ Info
    /// -          name: `type`(ctype: `enum VariableType`)
    /// -        offset:  48(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: VariableType,
    /// # C++ Info
    /// -          name: `global`(ctype: `hkBool`)
    /// -        offset:  49(x86)/ 49(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_global: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkbSetWordVariableCommand {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbSetWordVariableCommand"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf3ae5fca)
        }
    }
    impl _serde::Serialize for hkbSetWordVariableCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xf3ae5fca)));
            let mut serializer = __serializer
                .serialize_struct("hkbSetWordVariableCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("quadValue", &self.m_quadValue)?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("variableId", &self.m_variableId)?;
            serializer.serialize_field("value", &self.m_value)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("global", &self.m_global)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.end()
        }
    }
};
