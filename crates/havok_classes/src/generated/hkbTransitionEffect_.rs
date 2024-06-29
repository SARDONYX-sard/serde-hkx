use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbTransitionEffect`
/// -         version: `0`
/// -       signature: `0x945da157`
/// -          size:  44(x86)/ 80(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbTransitionEffect<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// -          name: `selfTransitionMode`(ctype: `enum SelfTransitionMode`)
    /// -        offset:  40(x86)/ 72(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_selfTransitionMode: SelfTransitionMode,
    /// # C++ Info
    /// -          name: `eventMode`(ctype: `enum EventMode`)
    /// -        offset:  41(x86)/ 73(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_eventMode: EventMode,
    /// # C++ Info
    /// -          name: `defaultEventMode`(ctype: `enum unknown`)
    /// -        offset:  42(x86)/ 74(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_defaultEventMode: i8,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl<'a> __serde::HavokClass for hkbTransitionEffect<'a> {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkbTransitionEffect"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(2489164119u32)
        }
    }
    impl<'a> __serde::Serialize for hkbTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkbTransitionEffect", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndFlags",
                    &self.parent.parent.parent.parent.m_memSizeAndFlags,
                )?;
            serializer
                .skip_field(
                    "referenceCount",
                    &self.parent.parent.parent.parent.m_referenceCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_meta_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer
                .serialize_stringptr_meta_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("padNode", &self.parent.parent.m_padNode.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field("selfTransitionMode", &self.m_selfTransitionMode)?;
            serializer.serialize_field("eventMode", &self.m_eventMode)?;
            serializer.skip_field("defaultEventMode", &self.m_defaultEventMode)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                )?;
            serializer.serialize_stringptr_field("name", &self.parent.parent.m_name)?;
            serializer.end()
        }
    }
};
///- size(C++): `TYPE_INT8`
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
pub enum SelfTransitionMode {
    #[default]
    SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC = 0isize,
    SELF_TRANSITION_MODE_CONTINUE = 1isize,
    SELF_TRANSITION_MODE_RESET = 2isize,
    SELF_TRANSITION_MODE_BLEND = 3isize,
}
///- size(C++): `TYPE_INT8`
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
pub enum EventMode {
    #[default]
    EVENT_MODE_DEFAULT = 0isize,
    EVENT_MODE_PROCESS_ALL = 1isize,
    EVENT_MODE_IGNORE_FROM_GENERATOR = 2isize,
    EVENT_MODE_IGNORE_TO_GENERATOR = 3isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SelfTransitionMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC => {
                    __serializer
                        .serialize_field(
                            "SELF_TRANSITION_MODE_CONTINUE_IF_CYCLIC_BLEND_IF_ACYCLIC",
                            &0u64,
                        )
                }
                Self::SELF_TRANSITION_MODE_CONTINUE => {
                    __serializer.serialize_field("SELF_TRANSITION_MODE_CONTINUE", &1u64)
                }
                Self::SELF_TRANSITION_MODE_RESET => {
                    __serializer.serialize_field("SELF_TRANSITION_MODE_RESET", &2u64)
                }
                Self::SELF_TRANSITION_MODE_BLEND => {
                    __serializer.serialize_field("SELF_TRANSITION_MODE_BLEND", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SelfTransitionMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for EventMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::EVENT_MODE_DEFAULT => {
                    __serializer.serialize_field("EVENT_MODE_DEFAULT", &0u64)
                }
                Self::EVENT_MODE_PROCESS_ALL => {
                    __serializer.serialize_field("EVENT_MODE_PROCESS_ALL", &1u64)
                }
                Self::EVENT_MODE_IGNORE_FROM_GENERATOR => {
                    __serializer
                        .serialize_field("EVENT_MODE_IGNORE_FROM_GENERATOR", &2u64)
                }
                Self::EVENT_MODE_IGNORE_TO_GENERATOR => {
                    __serializer.serialize_field("EVENT_MODE_IGNORE_TO_GENERATOR", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum EventMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
