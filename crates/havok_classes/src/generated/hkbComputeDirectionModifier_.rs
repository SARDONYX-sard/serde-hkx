use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbComputeDirectionModifier`
/// -         version: `0`
/// -       signature: `0xdf358bd3`
/// -          size: 112(x86)/144(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbComputeDirectionModifier<'a> {
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
    /// -          name: `pointIn`(ctype: `hkVector4`)
    /// -        offset:  48(x86)/ 80(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointIn: Vector4,
    /// # C++ Info
    /// -          name: `pointOut`(ctype: `hkVector4`)
    /// -        offset:  64(x86)/ 96(x86_64)
    /// -     type_size:  16(x86)/ 16(x86_64)
    ///
    pub m_pointOut: Vector4,
    /// # C++ Info
    /// -          name: `groundAngleOut`(ctype: `hkReal`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_groundAngleOut: f32,
    /// # C++ Info
    /// -          name: `upAngleOut`(ctype: `hkReal`)
    /// -        offset:  84(x86)/116(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_upAngleOut: f32,
    /// # C++ Info
    /// -          name: `verticalOffset`(ctype: `hkReal`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// -          name: `reverseGroundAngle`(ctype: `hkBool`)
    /// -        offset:  92(x86)/124(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_reverseGroundAngle: bool,
    /// # C++ Info
    /// -          name: `reverseUpAngle`(ctype: `hkBool`)
    /// -        offset:  93(x86)/125(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_reverseUpAngle: bool,
    /// # C++ Info
    /// -          name: `projectPoint`(ctype: `hkBool`)
    /// -        offset:  94(x86)/126(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_projectPoint: bool,
    /// # C++ Info
    /// -          name: `normalizePoint`(ctype: `hkBool`)
    /// -        offset:  95(x86)/127(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_normalizePoint: bool,
    /// # C++ Info
    /// -          name: `computeOnlyOnce`(ctype: `hkBool`)
    /// -        offset:  96(x86)/128(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computeOnlyOnce: bool,
    /// # C++ Info
    /// -          name: `computedOutput`(ctype: `hkBool`)
    /// -        offset:  97(x86)/129(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_computedOutput: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbComputeDirectionModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbComputeDirectionModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(3744828371u32)
        }
    }
    impl<'a> _serde::Serialize for hkbComputeDirectionModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(3744828371u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbComputeDirectionModifier", class_meta)?;
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
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer.serialize_field("pointIn", &self.m_pointIn)?;
            serializer.serialize_field("pointOut", &self.m_pointOut)?;
            serializer.serialize_field("groundAngleOut", &self.m_groundAngleOut)?;
            serializer.serialize_field("upAngleOut", &self.m_upAngleOut)?;
            serializer.serialize_field("verticalOffset", &self.m_verticalOffset)?;
            serializer
                .serialize_field("reverseGroundAngle", &self.m_reverseGroundAngle)?;
            serializer.serialize_field("reverseUpAngle", &self.m_reverseUpAngle)?;
            serializer.serialize_field("projectPoint", &self.m_projectPoint)?;
            serializer.serialize_field("normalizePoint", &self.m_normalizePoint)?;
            serializer.serialize_field("computeOnlyOnce", &self.m_computeOnlyOnce)?;
            serializer.serialize_field("computedOutput", &self.m_computedOutput)?;
            serializer.pad_field([0u8; 14usize].as_slice(), [0u8; 14usize].as_slice())?;
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
