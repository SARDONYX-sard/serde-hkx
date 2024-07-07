use super::class_requires::*;
use super::*;
/// # C++ Info
/// -            name: `hkbClientCharacterState`
/// -         version: `1`
/// -       signature: `0xa2624c97`
/// -          size: 208(x86)/272(x86_64)
/// -          vtable: true
///
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbClientCharacterState<'a> {
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
    /// -          name: `deformableSkinIds`(ctype: `hkArray<hkUint64>`)
    /// -        offset:   8(x86)/ 16(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_deformableSkinIds: Vec<u64>,
    /// # C++ Info
    /// -          name: `rigidSkinIds`(ctype: `hkArray<hkUint64>`)
    /// -        offset:  20(x86)/ 32(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidSkinIds: Vec<u64>,
    /// # C++ Info
    /// -          name: `externalEventIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  32(x86)/ 48(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_externalEventIds: Vec<i16>,
    /// # C++ Info
    /// -          name: `auxiliaryInfo`(ctype: `hkArray<hkbAuxiliaryNodeInfo*>`)
    /// -        offset:  44(x86)/ 64(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_auxiliaryInfo: Vec<Pointer>,
    /// # C++ Info
    /// -          name: `activeEventIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  56(x86)/ 80(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeEventIds: Vec<i16>,
    /// # C++ Info
    /// -          name: `activeVariableIds`(ctype: `hkArray<hkInt16>`)
    /// -        offset:  68(x86)/ 96(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_activeVariableIds: Vec<i16>,
    /// # C++ Info
    /// -          name: `characterId`(ctype: `hkUint64`)
    /// -        offset:  80(x86)/112(x86_64)
    /// -     type_size:   8(x86)/  8(x86_64)
    ///
    pub m_characterId: u64,
    /// # C++ Info
    /// -          name: `instanceName`(ctype: `hkStringPtr`)
    /// -        offset:  88(x86)/120(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_instanceName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `templateName`(ctype: `hkStringPtr`)
    /// -        offset:  92(x86)/128(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_templateName: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `fullPathToProject`(ctype: `hkStringPtr`)
    /// -        offset:  96(x86)/136(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_fullPathToProject: StringPtr<'a>,
    /// # C++ Info
    /// -          name: `behaviorData`(ctype: `struct hkbBehaviorGraphData*`)
    /// -        offset: 100(x86)/144(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorData: Pointer,
    /// # C++ Info
    /// -          name: `behaviorInternalState`(ctype: `struct hkbBehaviorGraphInternalState*`)
    /// -        offset: 104(x86)/152(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_behaviorInternalState: Pointer,
    /// # C++ Info
    /// -          name: `nodeIdToInternalStateMap`(ctype: `void*`)
    /// -        offset: 108(x86)/160(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    /// -         flags: `SERIALIZE_IGNORED`
    ///
    pub m_nodeIdToInternalStateMap: Pointer,
    /// # C++ Info
    /// -          name: `visible`(ctype: `hkBool`)
    /// -        offset: 112(x86)/168(x86_64)
    /// -     type_size:   1(x86)/  1(x86_64)
    ///
    pub m_visible: bool,
    /// # C++ Info
    /// -          name: `elapsedSimulationTime`(ctype: `hkReal`)
    /// -        offset: 116(x86)/172(x86_64)
    /// -     type_size:   4(x86)/  4(x86_64)
    ///
    pub m_elapsedSimulationTime: f32,
    /// # C++ Info
    /// -          name: `skeleton`(ctype: `struct hkaSkeleton*`)
    /// -        offset: 120(x86)/176(x86_64)
    /// -     type_size:   4(x86)/  8(x86_64)
    ///
    pub m_skeleton: Pointer,
    /// # C++ Info
    /// -          name: `worldFromModel`(ctype: `hkQsTransform`)
    /// -        offset: 128(x86)/192(x86_64)
    /// -     type_size:  48(x86)/ 48(x86_64)
    ///
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// -          name: `poseModelSpace`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset: 176(x86)/240(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_poseModelSpace: Vec<QsTransform>,
    /// # C++ Info
    /// -          name: `rigidAttachmentTransforms`(ctype: `hkArray<hkQsTransform>`)
    /// -        offset: 188(x86)/256(x86_64)
    /// -     type_size:  12(x86)/ 16(x86_64)
    ///
    pub m_rigidAttachmentTransforms: Vec<QsTransform>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbClientCharacterState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbClientCharacterState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xa2624c97)
        }
    }
    impl<'a> _serde::Serialize for hkbClientCharacterState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .map(|name| (name, _serde::__private::Signature::new(0xa2624c97)));
            let mut serializer = __serializer
                .serialize_struct("hkbClientCharacterState", class_meta)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.skip_field("memSizeAndFlags", &self.parent.m_memSizeAndFlags)?;
            serializer.skip_field("referenceCount", &self.parent.m_referenceCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_meta_field(
                    "deformableSkinIds",
                    &self.m_deformableSkinIds,
                )?;
            serializer.serialize_array_meta_field("rigidSkinIds", &self.m_rigidSkinIds)?;
            serializer
                .serialize_array_meta_field(
                    "externalEventIds",
                    &self.m_externalEventIds,
                )?;
            serializer
                .serialize_array_meta_field("auxiliaryInfo", &self.m_auxiliaryInfo)?;
            serializer
                .serialize_array_meta_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_meta_field(
                    "activeVariableIds",
                    &self.m_activeVariableIds,
                )?;
            serializer.serialize_field("characterId", &self.m_characterId)?;
            serializer
                .serialize_stringptr_meta_field("instanceName", &self.m_instanceName)?;
            serializer
                .serialize_stringptr_meta_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_meta_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_field("behaviorData", &self.m_behaviorData)?;
            serializer
                .serialize_field(
                    "behaviorInternalState",
                    &self.m_behaviorInternalState,
                )?;
            serializer
                .skip_field(
                    "nodeIdToInternalStateMap",
                    &self.m_nodeIdToInternalStateMap,
                )?;
            serializer.serialize_field("visible", &self.m_visible)?;
            serializer.pad_field([0u8; 3usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "elapsedSimulationTime",
                    &self.m_elapsedSimulationTime,
                )?;
            serializer.serialize_field("skeleton", &self.m_skeleton)?;
            serializer.pad_field([0u8; 4usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer
                .serialize_array_meta_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_meta_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.pad_field([0u8; 8usize].as_slice(), [0u8; 0usize].as_slice())?;
            serializer
                .serialize_array_field("deformableSkinIds", &self.m_deformableSkinIds)?;
            serializer.serialize_array_field("rigidSkinIds", &self.m_rigidSkinIds)?;
            serializer
                .serialize_array_field("externalEventIds", &self.m_externalEventIds)?;
            serializer.serialize_array_field("auxiliaryInfo", &self.m_auxiliaryInfo)?;
            serializer.serialize_array_field("activeEventIds", &self.m_activeEventIds)?;
            serializer
                .serialize_array_field("activeVariableIds", &self.m_activeVariableIds)?;
            serializer.serialize_stringptr_field("instanceName", &self.m_instanceName)?;
            serializer.serialize_stringptr_field("templateName", &self.m_templateName)?;
            serializer
                .serialize_stringptr_field(
                    "fullPathToProject",
                    &self.m_fullPathToProject,
                )?;
            serializer.serialize_array_field("poseModelSpace", &self.m_poseModelSpace)?;
            serializer
                .serialize_array_field(
                    "rigidAttachmentTransforms",
                    &self.m_rigidAttachmentTransforms,
                )?;
            serializer.end()
        }
    }
};
