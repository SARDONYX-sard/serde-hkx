use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpLinkedCollidable`
/// -         version: `0`
/// -       signature: `0xe1a81497`
/// -          size:  92(x86)/128(x86_64)
/// -          vtable: false
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpLinkedCollidable {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    pub __ptr: Option<Pointer>,
    /// Alternative to C++ class inheritance.
    pub parent: hkpCollidable,
    /// # C++ Info
    /// -          name: `collisionEntries`(ctype: `hkArray<void>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_collisionEntries: Vec<()>,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpLinkedCollidable {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpLinkedCollidable"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(3785888919u32)
        }
    }
    impl __serde::Serialize for hkpLinkedCollidable {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpLinkedCollidable", class_meta)?;
            serializer.serialize_field("shape", &self.parent.parent.m_shape)?;
            serializer.serialize_field("shapeKey", &self.parent.parent.m_shapeKey)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("motion", &self.parent.parent.m_motion)?;
            serializer.skip_field("parent", &self.parent.parent.m_parent)?;
            serializer.skip_field("ownerOffset", &self.parent.m_ownerOffset)?;
            serializer
                .serialize_field(
                    "forceCollideOntoPpu",
                    &self.parent.m_forceCollideOntoPpu,
                )?;
            serializer.skip_field("shapeSizeOnSpu", &self.parent.m_shapeSizeOnSpu)?;
            serializer
                .serialize_field("broadPhaseHandle", &self.parent.m_broadPhaseHandle)?;
            serializer
                .skip_field("boundingVolumeData", &self.parent.m_boundingVolumeData)?;
            serializer
                .serialize_field(
                    "allowedPenetrationDepth",
                    &self.parent.m_allowedPenetrationDepth,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_array_meta_field("collisionEntries", &self.m_collisionEntries)?;
            serializer
                .serialize_array_field("collisionEntries", &self.m_collisionEntries)?;
            serializer.end()
        }
    }
};
