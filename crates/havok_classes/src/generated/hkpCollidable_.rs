use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpCollidable`
/// -         version: `0`
/// -       signature: `0x9a0e42a5`
/// -          size:  80(x86)/112(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpCollidable {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCdBody,
    /// # C++ Info
    /// -          name: `ownerOffset`(ctype: `hkInt8`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_ownerOffset: i8,
    /// # C++ Info
    /// -          name: `forceCollideOntoPpu`(ctype: `hkUint8`)
    /// -        offset:  17(x86)/ 33(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_forceCollideOntoPpu: u8,
    /// # C++ Info
    /// -          name: `shapeSizeOnSpu`(ctype: `hkUint16`)
    /// -        offset:  18(x86)/ 34(x86_64)
    /// -     type_size:   2(x86)/  2(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_shapeSizeOnSpu: u16,
    /// # C++ Info
    /// -          name: `broadPhaseHandle`(ctype: `struct hkpTypedBroadPhaseHandle`)
    /// -        offset:  20(x86)/ 36(x86_64)
    /// -     type_size:  12(x86)/ 12(x86_64)
    ///
    pub m_broadPhaseHandle: hkpTypedBroadPhaseHandle,
    /// # C++ Info
    /// -          name: `boundingVolumeData`(ctype: `struct hkpCollidableBoundingVolumeData`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  44(x86)/ 56(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_boundingVolumeData: hkpCollidableBoundingVolumeData,
    /// # C++ Info
    /// -          name: `allowedPenetrationDepth`(ctype: `hkReal`)
    /// -        offset:  76(x86)/104(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_allowedPenetrationDepth: f32,
}
const _: () = {
    use havok_serde as _serde;
    impl _serde::HavokClass for hkpCollidable {
        #[inline]
        fn name(&self) -> &'static str {
            "hkpCollidable"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9a0e42a5)
        }
    }
    impl _serde::Serialize for hkpCollidable {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0x9a0e42a5)));
            let mut serializer = __serializer
                .serialize_struct("hkpCollidable", class_meta)?;
            serializer.serialize_field("shape", &self.parent.m_shape)?;
            serializer.serialize_field("shapeKey", &self.parent.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("motion", &self.parent.m_motion)?;
            serializer.skip_field("parent", &self.parent.m_parent)?;
            serializer.skip_field("ownerOffset", &self.m_ownerOffset)?;
            serializer
                .serialize_field("forceCollideOntoPpu", &self.m_forceCollideOntoPpu)?;
            serializer.skip_field("shapeSizeOnSpu", &self.m_shapeSizeOnSpu)?;
            serializer.serialize_field("broadPhaseHandle", &self.m_broadPhaseHandle)?;
            serializer.skip_field("boundingVolumeData", &self.m_boundingVolumeData)?;
            serializer
                .serialize_field(
                    "allowedPenetrationDepth",
                    &self.m_allowedPenetrationDepth,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
