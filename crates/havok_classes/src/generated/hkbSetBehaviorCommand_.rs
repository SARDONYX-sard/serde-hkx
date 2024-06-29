use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbSetBehaviorCommand`
/// -         version: `1`
/// -       signature: `0xe18b74b9`
/// -          size:  48(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbSetBehaviorCommand {
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
    /// -          name: `behavior`(ctype: `struct hkbBehaviorGraph*`)
    /// -        offset:  16(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behavior: Pointer,
    /// # C++ Info
    /// -          name: `rootGenerator`(ctype: `struct hkbGenerator*`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_rootGenerator: Pointer,
    /// # C++ Info
    /// -          name: `referencedBehaviors`(ctype: `hkArray<hkbBehaviorGraph*>`)
    /// -        offset:  24(x86)/ 40(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_referencedBehaviors: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `startStateIndex`(ctype: `hkInt32`)
    /// -        offset:  36(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_startStateIndex: i32,
    /// # C++ Info
    /// -          name: `randomizeSimulation`(ctype: `hkBool`)
    /// -        offset:  40(x86)/ 60(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_randomizeSimulation: bool,
    /// # C++ Info
    /// -          name: `padding`(ctype: `hkInt32`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_padding: i32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbSetBehaviorCommand {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbSetBehaviorCommand"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3784012985u32)
        }
    }
    impl __serde::Serialize for hkbSetBehaviorCommand {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbSetBehaviorCommand", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer.serialize_field("behavior", &self.m_behavior)?;
            serializer.serialize_field("rootGenerator", &self.m_rootGenerator)?;
            serializer
                .serialize_array_meta_field(
                    "referencedBehaviors",
                    &self.m_referencedBehaviors,
                )?;
            serializer.serialize_field("startStateIndex", &self.m_startStateIndex)?;
            serializer
                .serialize_field("randomizeSimulation", &self.m_randomizeSimulation)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("padding", &self.m_padding)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "referencedBehaviors",
                    &self.m_referencedBehaviors,
                )?;
            serializer.end()
        }
    }
};
