use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbFootIkDriverInfo`
/// -         version: `0`
/// -       signature: `0xc6a09dbf`
/// -          size:  56(x86)/ 72(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbFootIkDriverInfo {
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
    /// -          name: `legs`(ctype: `hkArray<struct hkbFootIkDriverInfoLeg>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_legs: Vec<hkbFootIkDriverInfoLeg>,
    /// # C++ Info
    /// -          name: `raycastDistanceUp`(ctype: `hkReal`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceUp: f32,
    /// # C++ Info
    /// -          name: `raycastDistanceDown`(ctype: `hkReal`)
    /// -        offset:  24(x86)/ 36(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_raycastDistanceDown: f32,
    /// # C++ Info
    /// -          name: `originalGroundHeightMS`(ctype: `hkReal`)
    /// -        offset:  28(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_originalGroundHeightMS: f32,
    /// # C++ Info
    /// -          name: `verticalOffset`(ctype: `hkReal`)
    /// -        offset:  32(x86)/ 44(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_verticalOffset: f32,
    /// # C++ Info
    /// -          name: `collisionFilterInfo`(ctype: `hkUint32`)
    /// -        offset:  36(x86)/ 48(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_collisionFilterInfo: u32,
    /// # C++ Info
    /// -          name: `forwardAlignFraction`(ctype: `hkReal`)
    /// -        offset:  40(x86)/ 52(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_forwardAlignFraction: f32,
    /// # C++ Info
    /// -          name: `sidewaysAlignFraction`(ctype: `hkReal`)
    /// -        offset:  44(x86)/ 56(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sidewaysAlignFraction: f32,
    /// # C++ Info
    /// -          name: `sidewaysSampleWidth`(ctype: `hkReal`)
    /// -        offset:  48(x86)/ 60(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_sidewaysSampleWidth: f32,
    /// # C++ Info
    /// -          name: `lockFeetWhenPlanted`(ctype: `hkBool`)
    /// -        offset:  52(x86)/ 64(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_lockFeetWhenPlanted: bool,
    /// # C++ Info
    /// -          name: `useCharacterUpVector`(ctype: `hkBool`)
    /// -        offset:  53(x86)/ 65(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useCharacterUpVector: bool,
    /// # C++ Info
    /// -          name: `isQuadrupedNarrow`(ctype: `hkBool`)
    /// -        offset:  54(x86)/ 66(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_isQuadrupedNarrow: bool,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkbFootIkDriverInfo {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbFootIkDriverInfo"
        }
        #[inline]
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3332414911u32)
        }
    }
    impl __serde::Serialize for hkbFootIkDriverInfo {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, __serde::__private::Signature::new(3332414911u32)));
            let mut serializer = __serializer
                .serialize_struct("hkbFootIkDriverInfo", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_array_meta_field("legs", &self.m_legs)?;
            serializer.serialize_field("raycastDistanceUp", &self.m_raycastDistanceUp)?;
            serializer
                .serialize_field("raycastDistanceDown", &self.m_raycastDistanceDown)?;
            serializer
                .serialize_field(
                    "originalGroundHeightMS",
                    &self.m_originalGroundHeightMS,
                )?;
            serializer.serialize_field("verticalOffset", &self.m_verticalOffset)?;
            serializer
                .serialize_field("collisionFilterInfo", &self.m_collisionFilterInfo)?;
            serializer
                .serialize_field("forwardAlignFraction", &self.m_forwardAlignFraction)?;
            serializer
                .serialize_field(
                    "sidewaysAlignFraction",
                    &self.m_sidewaysAlignFraction,
                )?;
            serializer
                .serialize_field("sidewaysSampleWidth", &self.m_sidewaysSampleWidth)?;
            serializer
                .serialize_field("lockFeetWhenPlanted", &self.m_lockFeetWhenPlanted)?;
            serializer
                .serialize_field("useCharacterUpVector", &self.m_useCharacterUpVector)?;
            serializer.serialize_field("isQuadrupedNarrow", &self.m_isQuadrupedNarrow)?;
            serializer.pad_field([0u8; 1usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_array_field("legs", &self.m_legs)?;
            serializer.end()
        }
    }
};
