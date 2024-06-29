use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbBehaviorInfoIdToNamePair`
/// -         version: `0`
/// -       signature: `0x35a0439a`
/// -          size:  12(x86)/ 24(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorInfoIdToNamePair<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// # C++ Info
    /// -          name: `behaviorName`(ctype: `hkStringPtr`)
    /// -        offset:   0(x86)/  0(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `nodeName`(ctype: `hkStringPtr`)
    /// -        offset:   4(x86)/  8(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_nodeName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `toolType`(ctype: `enum ToolNodeType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_toolType: ToolNodeType,
    /// # C++ Info
    /// -          name: `id`(ctype: `hkInt16`)
    /// -        offset:  10(x86)/ 18(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    ///
    pub m_id: i16,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbBehaviorInfoIdToNamePair<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbBehaviorInfoIdToNamePair"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(899695514u32)
        }
    }
    impl<'a> __serde::Serialize for hkbBehaviorInfoIdToNamePair<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorInfoIdToNamePair", class_meta)?;
            serializer
                .serialize_stringptr_meta_field("behaviorName", &self.m_behaviorName)?;
            serializer.serialize_stringptr_meta_field("nodeName", &self.m_nodeName)?;
            serializer.serialize_field("toolType", &self.m_toolType)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("id", &self.m_id)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_stringptr_field("behaviorName", &self.m_behaviorName)?;
            serializer.serialize_stringptr_field("nodeName", &self.m_nodeName)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_UINT8`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum ToolNodeType {
    #[default]
    NODE_TYPE_UNKNOWN = 0isize,
    NODE_TYPE_STATE_MACHINE = 1isize,
    NODE_TYPE_CLIP = 2isize,
    NODE_TYPE_BLEND = 3isize,
    NODE_TYPE_MODIFIER = 4isize,
    NODE_TYPE_GENERATOR = 5isize,
    NODE_TYPE_MODIFIER_GENERATOR = 6isize,
    NODE_TYPE_TRANSITION_EFFECT = 7isize,
    NODE_TYPE_BEHAVIOR_FILE_REFERENCE = 8isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for ToolNodeType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NODE_TYPE_UNKNOWN => {
                    __serializer.serialize_field("NODE_TYPE_UNKNOWN", &0u64)
                }
                Self::NODE_TYPE_STATE_MACHINE => {
                    __serializer.serialize_field("NODE_TYPE_STATE_MACHINE", &1u64)
                }
                Self::NODE_TYPE_CLIP => {
                    __serializer.serialize_field("NODE_TYPE_CLIP", &2u64)
                }
                Self::NODE_TYPE_BLEND => {
                    __serializer.serialize_field("NODE_TYPE_BLEND", &3u64)
                }
                Self::NODE_TYPE_MODIFIER => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER", &4u64)
                }
                Self::NODE_TYPE_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_GENERATOR", &5u64)
                }
                Self::NODE_TYPE_MODIFIER_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER_GENERATOR", &6u64)
                }
                Self::NODE_TYPE_TRANSITION_EFFECT => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION_EFFECT", &7u64)
                }
                Self::NODE_TYPE_BEHAVIOR_FILE_REFERENCE => {
                    __serializer
                        .serialize_field("NODE_TYPE_BEHAVIOR_FILE_REFERENCE", &8u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum ToolNodeType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
