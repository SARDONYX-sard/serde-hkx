use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkpSerializedAgentNnEntry`
/// -         version: `0`
/// -       signature: `0x49ec7de3`
/// -          size: 320(x86)/368(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkpSerializedAgentNnEntry {
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
    /// -          name: `bodyA`(ctype: `struct hkpEntity*`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyA: Pointer,
    /// # C++ Info
    /// -          name: `bodyB`(ctype: `struct hkpEntity*`)
    /// -        offset:  12(x86)/ 24(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyB: Pointer,
    /// # C++ Info
    /// -          name: `bodyAId`(ctype: `hkUlong`)
    /// -        offset:  16(x86)/ 32(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyAId: u64,
    /// # C++ Info
    /// -          name: `bodyBId`(ctype: `hkUlong`)
    /// -        offset:  20(x86)/ 40(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_bodyBId: u64,
    /// # C++ Info
    /// -          name: `useEntityIds`(ctype: `hkBool`)
    /// -        offset:  24(x86)/ 48(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_useEntityIds: bool,
    /// # C++ Info
    /// -          name: `agentType`(ctype: `enum SerializedAgentType`)
    /// -        offset:  25(x86)/ 49(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_agentType: SerializedAgentType,
    /// # C++ Info
    /// -          name: `atom`(ctype: `struct hkpSimpleContactConstraintAtom`)
    /// -        offset:  32(x86)/ 64(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_atom: hkpSimpleContactConstraintAtom,
    /// # C++ Info
    /// -          name: `propertiesStream`(ctype: `hkArray<hkUint8>`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_propertiesStream: Vec<u8>,
    /// # C++ Info
    /// -          name: `contactPoints`(ctype: `hkArray<struct hkContactPoint>`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_contactPoints: Vec<hkContactPoint>,
    /// # C++ Info
    /// -          name: `cpIdMgr`(ctype: `hkArray<hkUint8>`)
    /// -        offset: 104(x86)/144(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_cpIdMgr: Vec<u8>,
    /// # C++ Info
    /// -          name: `nnEntryData`(ctype: `hkUint8[160]`)
    /// -        offset: 116(x86)/160(x86_64)
    /// -     type_size: 160(x86)/160(x86_64)
    ///
    #[cfg_attr(feature = "serde", serde_as(as = "[_; 160]"))]
    #[educe(Default = [0;160usize])]
    pub m_nnEntryData: [u8; 160usize],
    /// # C++ Info
    /// -          name: `trackInfo`(ctype: `struct hkpSerializedTrack1nInfo`)
    /// -        offset: 276(x86)/320(x86_64)
    /// -     type_size:  24(x86)/ 32(x86_64)
    ///
    pub m_trackInfo: hkpSerializedTrack1nInfo,
    /// # C++ Info
    /// -          name: `endianCheckBuffer`(ctype: `hkUint8[4]`)
    /// -        offset: 300(x86)/352(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_endianCheckBuffer: [u8; 4usize],
    /// # C++ Info
    /// -          name: `version`(ctype: `hkUint32`)
    /// -        offset: 304(x86)/356(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_version: u32,
}
const _: () = {
    use havok_serde as __serde;
    use __serde::HavokClass;
    impl __serde::HavokClass for hkpSerializedAgentNnEntry {
        fn name(&self) -> &'static core::ffi::CStr {
            c"hkpSerializedAgentNnEntry"
        }
        fn signature(&self) -> __serde::__private::Signature {
            __serde::__private::Signature::new(1240235491u32)
        }
    }
    impl __serde::Serialize for hkpSerializedAgentNnEntry {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let class_meta = self.__ptr.map(|name| (name, self.signature()));
            let mut serializer = __serializer
                .serialize_struct("hkpSerializedAgentNnEntry", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("bodyA", &self.m_bodyA)?;
            serializer.serialize_field("bodyB", &self.m_bodyB)?;
            serializer.serialize_field("bodyAId", &self.m_bodyAId)?;
            serializer.serialize_field("bodyBId", &self.m_bodyBId)?;
            serializer.serialize_field("useEntityIds", &self.m_useEntityIds)?;
            serializer.serialize_field("agentType", &self.m_agentType)?;
            serializer.pad_field([0u8; 6usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("atom", &self.m_atom)?;
            serializer
                .serialize_array_meta_field(
                    "propertiesStream",
                    &self.m_propertiesStream,
                )?;
            serializer
                .serialize_array_meta_field("contactPoints", &self.m_contactPoints)?;
            serializer.serialize_array_meta_field("cpIdMgr", &self.m_cpIdMgr)?;
            serializer.serialize_field("nnEntryData", &self.m_nnEntryData.as_slice())?;
            serializer.serialize_field("trackInfo", &self.m_trackInfo)?;
            serializer
                .serialize_field(
                    "endianCheckBuffer",
                    &self.m_endianCheckBuffer.as_slice(),
                )?;
            serializer.serialize_field("version", &self.m_version)?;
            serializer.pad_field([0u8; 12usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_array_field("propertiesStream", &self.m_propertiesStream)?;
            serializer.serialize_array_field("contactPoints", &self.m_contactPoints)?;
            serializer.serialize_array_field("cpIdMgr", &self.m_cpIdMgr)?;
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
pub enum SerializedAgentType {
    #[default]
    INVALID_AGENT_TYPE = 0isize,
    BOX_BOX_AGENT3 = 1isize,
    CAPSULE_TRIANGLE_AGENT3 = 2isize,
    PRED_GSK_AGENT3 = 3isize,
    PRED_GSK_CYLINDER_AGENT3 = 4isize,
    CONVEX_LIST_AGENT3 = 5isize,
    LIST_AGENT3 = 6isize,
    BV_TREE_AGENT3 = 7isize,
    COLLECTION_COLLECTION_AGENT3 = 8isize,
    COLLECTION_AGENT3 = 9isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for SerializedAgentType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::INVALID_AGENT_TYPE => {
                    __serializer.serialize_field("INVALID_AGENT_TYPE", &0u64)
                }
                Self::BOX_BOX_AGENT3 => {
                    __serializer.serialize_field("BOX_BOX_AGENT3", &1u64)
                }
                Self::CAPSULE_TRIANGLE_AGENT3 => {
                    __serializer.serialize_field("CAPSULE_TRIANGLE_AGENT3", &2u64)
                }
                Self::PRED_GSK_AGENT3 => {
                    __serializer.serialize_field("PRED_GSK_AGENT3", &3u64)
                }
                Self::PRED_GSK_CYLINDER_AGENT3 => {
                    __serializer.serialize_field("PRED_GSK_CYLINDER_AGENT3", &4u64)
                }
                Self::CONVEX_LIST_AGENT3 => {
                    __serializer.serialize_field("CONVEX_LIST_AGENT3", &5u64)
                }
                Self::LIST_AGENT3 => __serializer.serialize_field("LIST_AGENT3", &6u64),
                Self::BV_TREE_AGENT3 => {
                    __serializer.serialize_field("BV_TREE_AGENT3", &7u64)
                }
                Self::COLLECTION_COLLECTION_AGENT3 => {
                    __serializer.serialize_field("COLLECTION_COLLECTION_AGENT3", &8u64)
                }
                Self::COLLECTION_AGENT3 => {
                    __serializer.serialize_field("COLLECTION_AGENT3", &9u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum SerializedAgentType to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
