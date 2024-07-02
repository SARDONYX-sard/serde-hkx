use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `BSTweenerModifier`
/// -         version: `1`
/// -       signature: `0xd2d9a04`
/// -          size: 176(x86)/208(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSTweenerModifier<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// -          name: `tweenPosition`(ctype: `hkBool`)
    /// -        offset:  44(x86)/ 80(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_tweenPosition: bool,
    /// # C++ Info
    /// -          name: `tweenRotation`(ctype: `hkBool`)
    /// -        offset:  45(x86)/ 81(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_tweenRotation: bool,
    /// # C++ Info
    /// -          name: `useTweenDuration`(ctype: `hkBool`)
    /// -        offset:  46(x86)/ 82(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useTweenDuration: bool,
    /// # C++ Info
    /// -          name: `tweenDuration`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 84(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_tweenDuration: f32,
    /// # C++ Info
    /// -          name: `targetPosition`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetPosition: Vector4,
    /// # C++ Info
    /// -          name: `targetRotation`(ctype: `hkQuaternion`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_targetRotation: Quaternion,
    /// # C++ Info
    /// -          name: `duration`(ctype: `hkReal`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_duration: f32,
    /// # C++ Info
    /// -          name: `startTransform`(ctype: `hkQsTransform`)
    /// -        offset: 112(x86)/144(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_startTransform: QsTransform,
    /// # C++ Info
    /// -          name: `time`(ctype: `hkReal`)
    /// -        offset: 160(x86)/192(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_time: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSTweenerModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSTweenerModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(221092356u32)
        }
    }
    impl<'a> _serde::Serialize for BSTweenerModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(221092356u32)));
            let mut serializer = __serializer
                .serialize_struct("BSTweenerModifier", class_meta)?;
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
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer.skip_field("padModifier", &self.parent.m_padModifier.as_slice())?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("tweenPosition", &self.m_tweenPosition)?;
            serializer.serialize_field("tweenRotation", &self.m_tweenRotation)?;
            serializer.serialize_field("useTweenDuration", &self.m_useTweenDuration)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("tweenDuration", &self.m_tweenDuration)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("targetPosition", &self.m_targetPosition)?;
            serializer.serialize_field("targetRotation", &self.m_targetRotation)?;
            serializer.skip_field("duration", &self.m_duration)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer.skip_field("startTransform", &self.m_startTransform)?;
            serializer.skip_field("time", &self.m_time)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 12usize].as_slice())?;
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
