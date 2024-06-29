use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbAuxiliaryNodeInfo`
/// -         version: `1`
/// -       signature: `0xca0888ca`
/// -          size:  28(x86)/ 48(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbAuxiliaryNodeInfo<'a> {
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
    /// -          name: `type`(ctype: `enum NodeType`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_type: NodeType,
    /// # C++ Info
    /// -          name: `depth`(ctype: `hkUint8`)
    /// -        offset:   9(x86)/ 17(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_depth: u8,
    /// # C++ Info
    /// -          name: `referenceBehaviorName`(ctype: `hkStringPtr`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_referenceBehaviorName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `selfTransitionNames`(ctype: `hkArray<hkStringPtr>`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_selfTransitionNames: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbAuxiliaryNodeInfo<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbAuxiliaryNodeInfo"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3389556938u32)
        }
    }
    impl<'a> __serde::Serialize for hkbAuxiliaryNodeInfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbAuxiliaryNodeInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.serialize_field("depth", &self.m_depth)?;
            serializer.pad_field([0u8; 2usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_stringptr_meta_field(
                    "referenceBehaviorName",
                    &self.m_referenceBehaviorName,
                )?;
            serializer
                .serialize_array_meta_field(
                    "selfTransitionNames",
                    &self.m_selfTransitionNames,
                )?;
            serializer
                .serialize_stringptr_field(
                    "referenceBehaviorName",
                    &self.m_referenceBehaviorName,
                )?;
            serializer
                .serialize_array_field(
                    "selfTransitionNames",
                    &self.m_selfTransitionNames,
                )?;
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
pub enum NodeType {
    #[default]
    NODE_TYPE_UNKNOWN = 0isize,
    NODE_TYPE_NODE = 1isize,
    NODE_TYPE_TRANSITION = 2isize,
    NODE_TYPE_WILDCARD_TRANSITION = 3isize,
    NODE_TYPE_STATE = 4isize,
    NODE_TYPE_STATE_MACHINE = 5isize,
    NODE_TYPE_MODIFIER_GENERATOR = 6isize,
    NODE_TYPE_MODIFIER = 7isize,
    NODE_TYPE_CLIP = 8isize,
    NODE_TYPE_BLEND = 9isize,
    NODE_TYPE_TRANSITION_EFFECT = 10isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for NodeType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::NODE_TYPE_UNKNOWN => {
                    __serializer.serialize_field("NODE_TYPE_UNKNOWN", &0u64)
                }
                Self::NODE_TYPE_NODE => {
                    __serializer.serialize_field("NODE_TYPE_NODE", &1u64)
                }
                Self::NODE_TYPE_TRANSITION => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION", &2u64)
                }
                Self::NODE_TYPE_WILDCARD_TRANSITION => {
                    __serializer.serialize_field("NODE_TYPE_WILDCARD_TRANSITION", &3u64)
                }
                Self::NODE_TYPE_STATE => {
                    __serializer.serialize_field("NODE_TYPE_STATE", &4u64)
                }
                Self::NODE_TYPE_STATE_MACHINE => {
                    __serializer.serialize_field("NODE_TYPE_STATE_MACHINE", &5u64)
                }
                Self::NODE_TYPE_MODIFIER_GENERATOR => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER_GENERATOR", &6u64)
                }
                Self::NODE_TYPE_MODIFIER => {
                    __serializer.serialize_field("NODE_TYPE_MODIFIER", &7u64)
                }
                Self::NODE_TYPE_CLIP => {
                    __serializer.serialize_field("NODE_TYPE_CLIP", &8u64)
                }
                Self::NODE_TYPE_BLEND => {
                    __serializer.serialize_field("NODE_TYPE_BLEND", &9u64)
                }
                Self::NODE_TYPE_TRANSITION_EFFECT => {
                    __serializer.serialize_field("NODE_TYPE_TRANSITION_EFFECT", &10u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum NodeType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
